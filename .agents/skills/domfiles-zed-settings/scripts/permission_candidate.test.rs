#[path = "permission_candidate.rs"]
mod helper;
use helper::permission_patterns as patterns;

use serde_json::{Value, json};
use std::{
    cell::Cell,
    collections::BTreeMap,
    env,
    ffi::OsString,
    fs, io,
    path::{Component, Path, PathBuf},
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt, symlink};

const ALLOW_SCOPE: &str = "/agent/tool_permissions/tools/terminal/always_allow";
const CONFIRM_SCOPE: &str = "/agent/tool_permissions/tools/terminal/always_confirm";
const DEFAULT_SCOPE: &str = "/agent/tool_permissions/tools/terminal/default";
const DENY_SCOPE: &str = "/agent/tool_permissions/tools/terminal/always_deny";
static NEXT_FIXTURE_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let fixture_id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock must be after the Unix epoch")
            .as_nanos();
        let temporary_root = fs::canonicalize(env::temp_dir())
            .expect("Temporary directory must resolve to a real directory");
        let root = temporary_root.join(format!(
            "domfiles-permission-candidate-{}-{timestamp}-{fixture_id}",
            process::id()
        ));
        fs::create_dir(&root).expect("Failed to create fixture directory");

        Self { root }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn create_dir(&self, relative: &str) -> PathBuf {
        let path = self.path(relative);
        fs::create_dir_all(&path).expect("Failed to create fixture subdirectory");
        path
    }

    fn write(&self, relative: &str, bytes: &[u8]) -> PathBuf {
        let path = self.path(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create fixture file parent");
        }
        fs::write(&path, bytes).expect("Failed to write fixture file");
        path
    }

    fn write_json(&self, relative: &str, value: &Value) -> PathBuf {
        let bytes = serde_json::to_vec(value).expect("Fixture JSON must serialize");
        self.write(relative, &bytes)
    }

    fn write_pretty_json(&self, relative: &str, value: &Value) -> PathBuf {
        let bytes = helper::serialize_pretty_json(value).expect("Fixture JSON must serialize");
        self.write(relative, &bytes)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct RunResult {
    status: u8,
    stdout: String,
    stderr: String,
}

struct CaptureResult {
    candidate: PathBuf,
    output: PathBuf,
    pattern_file: PathBuf,
    settings: PathBuf,
    state: PathBuf,
}

struct CaptureArtifacts {
    candidate: PathBuf,
    output: PathBuf,
    settings: PathBuf,
    state: PathBuf,
}

fn run(arguments: Vec<OsString>) -> RunResult {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);

    RunResult {
        status,
        stdout: String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        stderr: String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    }
}

fn run_matcher(arguments: Vec<OsString>) -> RunResult {
    let binary = option_env!("CARGO_BIN_EXE_domfiles-zed-settings-pattern-match")
        .or(option_env!("CARGO_BIN_EXE_pattern-match"))
        .expect("Cargo must provide the matcher binary path");
    let output = process::Command::new(binary)
        .args(arguments)
        .output()
        .expect("Failed to run matcher binary");
    let status = output
        .status
        .code()
        .and_then(|code| u8::try_from(code).ok())
        .expect("Matcher exit status must fit in a byte");

    RunResult {
        status,
        stdout: String::from_utf8(output.stdout).expect("Standard output must be valid UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("Standard error must be valid UTF-8"),
    }
}

fn capture_arguments(settings: &Path, selection: &Path, output: &Path) -> Vec<OsString> {
    vec![
        OsString::from("capture"),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
        OsString::from("--selection"),
        selection.as_os_str().to_owned(),
        OsString::from("--output"),
        output.as_os_str().to_owned(),
    ]
}

fn materialize_arguments(
    candidate: &Path,
    state: &Path,
    selection: &Path,
    output: &Path,
) -> Vec<OsString> {
    vec![
        OsString::from("materialize"),
        OsString::from("--candidate"),
        candidate.as_os_str().to_owned(),
        OsString::from("--state"),
        state.as_os_str().to_owned(),
        OsString::from("--selection"),
        selection.as_os_str().to_owned(),
        OsString::from("--output"),
        output.as_os_str().to_owned(),
    ]
}

const FIXTURE_OWNER: &str = "fx";

/// Strip the fixture-only ownership marker that drives owner-spec construction.
fn strip_owner_marker(patterns: &[Value]) -> Vec<Value> {
    patterns
        .iter()
        .map(|pattern| {
            let mut pattern = pattern.clone();
            pattern
                .as_object_mut()
                .expect("Materialization selection pattern must be an object")
                .remove("owner_replacement");
            pattern
        })
        .collect()
}

fn marked_ids(patterns: &[Value], owned: bool) -> Vec<String> {
    patterns
        .iter()
        .filter(|pattern| {
            pattern
                .get("owner_replacement")
                .and_then(Value::as_bool)
                .unwrap_or(true)
                == owned
        })
        .map(|pattern| {
            pattern
                .get("id")
                .and_then(Value::as_str)
                .expect("Fixture pattern must declare an ID")
                .to_owned()
        })
        .collect()
}

/// Build the stable owner specification a fixture graph implies.
fn owner_spec_value(state: &Value, patterns: &[Value]) -> Value {
    let baseline_members = state["patterns"]
        .as_array()
        .expect("State patterns must be an array")
        .iter()
        .map(|pattern| {
            pattern["id"]
                .as_str()
                .expect("State ID must be a string")
                .to_owned()
        })
        .collect::<Vec<_>>();
    let candidate_members = marked_ids(patterns, true);
    let overlaps = marked_ids(patterns, false);
    let operation = if baseline_members.is_empty() {
        "insert"
    } else if candidate_members.is_empty() {
        "delete"
    } else {
        "replace"
    };

    let owners = if baseline_members.is_empty() && candidate_members.is_empty() {
        json!([])
    } else {
        json!([{
            "id": "fixture-owner",
            "inventory_owner": FIXTURE_OWNER,
            "operation": operation,
            "baseline_members": baseline_members,
            "candidate_members": candidate_members
        }])
    };

    json!({ "owners": owners, "overlaps": overlaps })
}

fn verify_arguments(settings: &Path, state: &Path) -> Vec<OsString> {
    vec![
        OsString::from("verify"),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
        OsString::from("--state"),
        state.as_os_str().to_owned(),
    ]
}

fn promote_arguments(settings: &Path, bundle: &Path, write: bool) -> Vec<OsString> {
    let mut arguments = vec![
        OsString::from("promote"),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
        OsString::from("--bundle"),
        bundle.as_os_str().to_owned(),
    ];
    if write {
        arguments.push(OsString::from("--write"));
    }

    arguments
}

fn pattern(pattern: &str, case_sensitive: bool) -> Value {
    json!({
        "pattern": pattern,
        "case_sensitive": case_sensitive
    })
}

fn settings(allow_patterns: Vec<Value>, generation: u64) -> Value {
    json!({
        "outside": {
            "generation": generation,
            "note": "outside-value"
        },
        "agent": {
            "tool_permissions": {
                "tools": {
                    "terminal": {
                        "default": "confirm",
                        "always_allow": allow_patterns,
                        "always_confirm": [
                            pattern("^fx confirm$", true)
                        ],
                        "always_deny": [
                            pattern("^fx deny$", true)
                        ]
                    }
                }
            }
        }
    })
}

fn selection(id: &str, index: usize) -> Value {
    json!({
        "scopes": [ALLOW_SCOPE],
        "patterns": [
            {
                "id": id,
                "bucket": "always_allow",
                "index": index
            }
        ]
    })
}

fn materialization_selection(patterns: Vec<Value>) -> Value {
    json!({
        "patterns": strip_owner_marker(&patterns)
    })
}

fn state_value(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("Failed to read state manifest"))
        .expect("State manifest must be valid JSON")
}

fn captured_pattern_file(output: &Path, state: &Value) -> PathBuf {
    output.join(
        state["patterns"][0]["pattern_file"]
            .as_str()
            .expect("State pattern file must be a string"),
    )
}

fn catalog_value(output: &Path) -> Value {
    serde_json::from_slice(
        &fs::read(output.join("artifact-catalog.json")).expect("Failed to read artifact catalog"),
    )
    .expect("Artifact catalog must be valid JSON")
}

fn materialized_pattern_file(output: &Path, catalog: &Value, index: usize) -> PathBuf {
    output.join(
        catalog["patterns"][index]["pattern_file"]
            .as_str()
            .expect("Catalog pattern file must be a string"),
    )
}

fn graph_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("Fixture artifact must live inside the graph root")
        .to_str()
        .expect("Fixture paths must be UTF-8")
        .replace(std::path::MAIN_SEPARATOR, "/")
}

fn file_sha256(path: &Path) -> String {
    patterns::sha256_hex(&fs::read(path).expect("Fixture artifact must be readable"))
}

/// Record hash-bound workflow evidence for a fixture graph. Results are evidence, not proof that a
/// validator ran, so the fixture computes the same closure the verifier recomputes.
fn write_evidence(
    fixture: &Fixture,
    root: &Path,
    name: &str,
    kind: &str,
    manifest: &Path,
    audit_settings: Option<&Path>,
) -> (String, String) {
    let mut builder =
        patterns::InputClosureBuilder::new(root).expect("Graph root must be a directory");
    let context = patterns::ClosureContext { overlay: None };
    match kind {
        "matcher_suite" => patterns::resolve_suite_closure(&mut builder, &context, manifest),
        "comparison" => patterns::resolve_comparison_closure(&mut builder, &context, manifest),
        "layer_decision" => patterns::resolve_layer_closure(&mut builder, &context, manifest),
        _ => patterns::resolve_audit_closure(
            &mut builder,
            manifest,
            audit_settings.expect("Audit evidence requires settings"),
            None,
        ),
    }
    .expect("Fixture closure must resolve");
    let closure = builder.finish().expect("Fixture closure must finish");

    let result = json!({
        "kind": kind,
        "evaluator": "fixture",
        "outcome": "passed",
        "bound_inputs": {
            "manifest_sha256": file_sha256(manifest),
            "input_closure": serde_json::to_value(&closure).expect("Closure must serialize")
        },
        "counts": {}
    });
    let result_path = fixture.write_json(&format!("{name}-result.json"), &result);

    (
        graph_relative(root, manifest),
        graph_relative(root, &result_path),
    )
}

/// Seal one fixture graph with the evidence kinds the contract requires for its shape.
fn seal_bundle(
    fixture: &Fixture,
    prefix: &str,
    candidate: &Path,
    state: &Path,
    catalog: &Path,
    owner_spec: &Value,
) -> PathBuf {
    let root = fixture.path("");
    let root = root.as_path();
    let spec_path = fixture.write_json(&format!("{prefix}-owner-spec.json"), owner_spec);
    let candidate_relative = graph_relative(root, candidate);
    let state_relative = graph_relative(root, state);
    let catalog_relative = graph_relative(root, catalog);

    let owners = owner_spec["owners"]
        .as_array()
        .expect("Owner spec must declare owners")
        .clone();
    let catalog_document: Value =
        serde_json::from_slice(&fs::read(catalog).expect("Catalog must be readable"))
            .expect("Catalog must parse");
    let changes_patterns = !owners.is_empty()
        || !catalog_document["patterns"]
            .as_array()
            .expect("Catalog patterns must be an array")
            .is_empty();

    let mut results = Vec::new();
    if changes_patterns {
        let ordinary_pattern = fixture.write(&format!("{prefix}-ordinary.regex"), b"^fx alpha$");
        let validation_input =
            fixture.write(&format!("{prefix}-validation-input.txt"), b"fx alpha");
        let ordinary_pattern_relative = graph_relative(root, &ordinary_pattern);
        let validation_input_relative = graph_relative(root, &validation_input);
        let suite = fixture.write(
            &format!("{prefix}-suite.txt"),
            format!(
                "default\tconfirm\npattern\tfixture-ordinary\talways_allow\tcase-sensitive\t{ordinary_pattern_relative}\npattern-case-file\tfixture-ordinary\tmatch\t{validation_input_relative}\ndecision-case-file\tallow\t{validation_input_relative}\npattern-catalog\tfixture\t{catalog_relative}\t{candidate_relative}\t{state_relative}\n"
            )
            .as_bytes(),
        );
        let comparison_pattern = json!({
            "type": "file",
            "id": "fixture-ordinary",
            "bucket": "always_allow",
            "case_sensitive": true,
            "pattern_file": ordinary_pattern_relative
        });
        let comparison = fixture.write_json(
            &format!("{prefix}-comparison.json"),
            &json!({
                "catalogs": [{
                    "id": "fixture",
                    "catalog_file": catalog_relative,
                    "candidate_file": candidate_relative,
                    "state_file": state_relative
                }],
                "baseline": {
                    "default": "confirm",
                    "patterns": [comparison_pattern.clone()]
                },
                "candidate": {
                    "default": "confirm",
                    "patterns": [comparison_pattern]
                },
                "cases": [{"type": "file", "input_file": validation_input_relative}]
            }),
        );
        let layer = fixture.write_json(
            &format!("{prefix}-layer.json"),
            &json!({
                "settings_file": candidate_relative,
                "default": "confirm",
                "settled_inputs": [{
                    "type": "file",
                    "id": "fixture-input",
                    "input_file": validation_input_relative,
                    "expected_decision": "allow"
                }]
            }),
        );

        for (name, kind, manifest) in [
            ("suite", "matcher_suite", &suite),
            ("comparison", "comparison", &comparison),
            ("layer", "layer_decision", &layer),
        ] {
            let (manifest_relative, result_relative) = write_evidence(
                fixture,
                root,
                &format!("{prefix}-{name}"),
                kind,
                manifest,
                None,
            );
            results.push(json!({
                "id": format!("{prefix}-{name}"),
                "kind": kind,
                "manifest": manifest_relative,
                "result": result_relative
            }));
            if name == "suite" {
                let (shared_manifest, shared_result) = write_evidence(
                    fixture,
                    root,
                    &format!("{prefix}-{name}-shared"),
                    kind,
                    manifest,
                    None,
                );
                results.push(json!({
                    "id": format!("{prefix}-{name}-shared"),
                    "kind": kind,
                    "manifest": shared_manifest,
                    "result": shared_result
                }));
            }
        }

        let candidate_sha256 = file_sha256(candidate);
        for owner in &owners {
            let members = owner["candidate_members"]
                .as_array()
                .expect("Owner candidate members must be an array");
            let owner_id = owner["id"].as_str().expect("Owner ID must be a string");
            let (kind, manifest) = if members.is_empty() {
                (
                    "candidate_inventory",
                    fixture.write_json(
                        &format!("{prefix}-{owner_id}-zero-owner.json"),
                        &json!({
                            "settings_sha256": candidate_sha256,
                            "inventory_owner": FIXTURE_OWNER
                        }),
                    ),
                )
            } else {
                let entries = members
                    .iter()
                    .map(|member| {
                        let id = member.as_str().expect("Member ID must be a string");
                        let entry = catalog_document["patterns"]
                            .as_array()
                            .expect("Catalog patterns must be an array")
                            .iter()
                            .find(|pattern| pattern["id"] == json!(id))
                            .expect("Owner member must exist in the catalog");
                        json!({
                            "id": id,
                            "bucket": entry["bucket"],
                            "index": entry["source_index"]
                        })
                    })
                    .collect::<Vec<_>>();
                (
                    "owner_audit",
                    fixture.write_json(
                        &format!("{prefix}-{owner_id}-audit.json"),
                        &json!({
                            "settings_sha256": candidate_sha256,
                            "inventory_owner": FIXTURE_OWNER,
                            "entries": entries,
                            "excluded_candidates": [{
                                "bucket": "always_allow",
                                "index": 0
                            }]
                        }),
                    ),
                )
            };
            let (manifest_relative, result_relative) = write_evidence(
                fixture,
                root,
                &format!("{prefix}-{owner_id}-{kind}"),
                kind,
                &manifest,
                Some(candidate),
            );
            results.push(json!({
                "id": format!("{prefix}-{owner_id}"),
                "kind": kind,
                "manifest": manifest_relative,
                "result": result_relative
            }));
        }
    }

    let plan = fixture.write_json(
        &format!("{prefix}-validation.json"),
        &json!({ "results": results }),
    );
    let bundle = fixture.path(&format!("{prefix}-bundle.json"));
    let sealed = run(vec![
        OsString::from("seal"),
        OsString::from("--candidate"),
        candidate.as_os_str().to_owned(),
        OsString::from("--state"),
        state.as_os_str().to_owned(),
        OsString::from("--catalog"),
        catalog.as_os_str().to_owned(),
        OsString::from("--owner-spec"),
        spec_path.as_os_str().to_owned(),
        OsString::from("--validation"),
        plan.as_os_str().to_owned(),
        OsString::from("--output"),
        bundle.as_os_str().to_owned(),
    ]);
    assert_eq!(sealed.status, 0, "seal failed: {}", sealed.stderr);

    bundle
}

/// Attempt to seal a fixture graph, returning the raw result so tests can assert refusals that now
/// happen when the bundle is created rather than when it is promoted.
fn try_seal(
    fixture: &Fixture,
    prefix: &str,
    candidate: &Path,
    state: &Path,
    catalog: &Path,
    patterns: &[Value],
) -> RunResult {
    let spec = owner_spec_value(&state_value(state), patterns);
    let spec_path = fixture.write_json(&format!("{prefix}-try-spec.json"), &spec);
    let plan = fixture.write_json(
        &format!("{prefix}-try-plan.json"),
        &json!({ "results": [] }),
    );
    let bundle = fixture.path(&format!("{prefix}-try-bundle.json"));

    run(vec![
        OsString::from("seal"),
        OsString::from("--candidate"),
        candidate.as_os_str().to_owned(),
        OsString::from("--state"),
        state.as_os_str().to_owned(),
        OsString::from("--catalog"),
        catalog.as_os_str().to_owned(),
        OsString::from("--owner-spec"),
        spec_path.as_os_str().to_owned(),
        OsString::from("--validation"),
        plan.as_os_str().to_owned(),
        OsString::from("--output"),
        bundle.as_os_str().to_owned(),
    ])
}

/// Seal a hand-built catalog so tests can exercise promotion against the bundle surface.
fn seal_hand_catalog(
    fixture: &Fixture,
    prefix: &str,
    candidate: &Path,
    state: &Path,
    catalog: &Path,
    patterns: &[Value],
) -> PathBuf {
    let spec = owner_spec_value(&state_value(state), patterns);
    seal_bundle(fixture, prefix, candidate, state, catalog, &spec)
}

/// Materialize a fixture graph and seal it, returning both the catalog and the sealed bundle.
fn materialize_and_seal(
    fixture: &Fixture,
    prefix: &str,
    candidate: &Path,
    state: &Path,
    patterns: Vec<Value>,
) -> (PathBuf, PathBuf) {
    let selection = fixture.write_json(
        &format!("{prefix}-materialization-selection.json"),
        &materialization_selection(patterns.clone()),
    );
    let output = fixture.path(&format!("{prefix}-materialized"));
    let result = run(materialize_arguments(candidate, state, &selection, &output));
    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stderr.is_empty());
    let catalog = output.join("artifact-catalog.json");
    let spec = owner_spec_value(&state_value(state), &patterns);
    let bundle = seal_bundle(fixture, prefix, candidate, state, &catalog, &spec);

    (catalog, bundle)
}

fn replacement_pattern(id: &str, bucket: &str, index: usize) -> Value {
    json!({
        "id": id,
        "bucket": bucket,
        "index": index,
        "owner_replacement": true
    })
}

fn validation_pattern(id: &str, bucket: &str, index: usize) -> Value {
    json!({
        "id": id,
        "bucket": bucket,
        "index": index,
        "owner_replacement": false
    })
}

fn bucket(label: &str) -> helper::Bucket {
    match label {
        "always_allow" => helper::Bucket::Allow,
        "always_confirm" => helper::Bucket::Confirm,
        "always_deny" => helper::Bucket::Deny,
        _ => panic!("Unsupported fixture bucket"),
    }
}

fn write_bound_catalog(
    fixture: &Fixture,
    prefix: &str,
    candidate: &Path,
    state: &Path,
    patterns: &[Value],
) -> PathBuf {
    let candidate_bytes = fs::read(candidate).expect("Failed to read candidate fixture");
    let candidate_value: Value =
        serde_json::from_slice(&candidate_bytes).expect("Candidate fixture must be valid JSON");
    let state_bytes = fs::read(state).expect("Failed to read state fixture");
    let output = fixture.create_dir(&format!("{prefix}-bound-catalog"));
    let mut catalog_patterns = Vec::with_capacity(patterns.len());

    for (offset, selected) in patterns.iter().enumerate() {
        let id = selected["id"]
            .as_str()
            .expect("Catalog fixture ID must be a string");
        let bucket_label = selected["bucket"]
            .as_str()
            .expect("Catalog fixture bucket must be a string");
        let source_index = selected["index"]
            .as_u64()
            .expect("Catalog fixture index must be an integer") as usize;
        let (pattern_body, case_sensitive) =
            helper::terminal_pattern(&candidate_value, bucket(bucket_label), source_index)
                .expect("Catalog fixture source must be a terminal pattern");
        let pattern_file = format!("pattern-{:03}.regex", offset + 1);
        fs::write(output.join(&pattern_file), pattern_body.as_bytes())
            .expect("Failed to write catalog pattern fixture");
        catalog_patterns.push(json!({
            "id": id,
            "bucket": bucket_label,
            "source_index": source_index,
            "case_sensitive": case_sensitive,
            "sha256": helper::sha256_hex(pattern_body.as_bytes()),
            "pattern_file": pattern_file
        }));
    }

    let catalog = json!({
        "candidate_sha256": helper::sha256_hex(&candidate_bytes),
        "state_sha256": helper::sha256_hex(&state_bytes),
        "patterns": catalog_patterns
    });
    let path = output.join("artifact-catalog.json");
    fs::write(
        &path,
        helper::serialize_pretty_json(&catalog).expect("Catalog fixture must serialize"),
    )
    .expect("Failed to write catalog fixture");
    path
}

fn capture_selected(
    fixture: &Fixture,
    prefix: &str,
    baseline: &Value,
    scopes: Vec<&str>,
    patterns: Vec<Value>,
) -> CaptureArtifacts {
    let settings = fixture.write_pretty_json(&format!("{prefix}-settings.json"), baseline);
    let selection = fixture.write_json(
        &format!("{prefix}-selection.json"),
        &json!({
            "scopes": scopes,
            "patterns": patterns
        }),
    );
    let output = fixture.path(&format!("{prefix}-artifacts"));
    let result = run(capture_arguments(&settings, &selection, &output));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stderr.is_empty());
    CaptureArtifacts {
        candidate: output.join("candidate-settings.json"),
        state: output.join("state.json"),
        output,
        settings,
    }
}

fn promote_owner_candidate(
    fixture: &Fixture,
    prefix: &str,
    baseline: &Value,
    scopes: Vec<&str>,
    captured_patterns: Vec<Value>,
    candidate: &Value,
    catalog_patterns: Vec<Value>,
) -> (RunResult, PathBuf) {
    let captured = capture_selected(fixture, prefix, baseline, scopes, captured_patterns);
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(candidate).expect("Candidate fixture must serialize"),
    )
    .expect("Failed to write candidate fixture");
    let (_catalog, bundle) = materialize_and_seal(
        fixture,
        prefix,
        &captured.candidate,
        &captured.state,
        catalog_patterns,
    );
    let live = fixture.write_pretty_json(&format!("{prefix}-live.json"), baseline);
    let result = run(promote_arguments(&live, &bundle, true));

    (result, live)
}

fn assert_owner_remainder_refused(
    fixture: &Fixture,
    prefix: &str,
    baseline: &Value,
    captured_patterns: Vec<Value>,
    candidate: &Value,
    catalog_patterns: Vec<Value>,
) {
    let (result, live) = promote_owner_candidate(
        fixture,
        prefix,
        baseline,
        vec![ALLOW_SCOPE],
        captured_patterns,
        candidate,
        catalog_patterns,
    );

    assert_eq!(result.status, 1, "{prefix}: {}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(
        result
            .stderr
            .contains("declared owner membership reconciles to")
            || result.stderr.contains("outside-owner remainder for"),
        "{prefix}: {}",
        result.stderr
    );
    let current: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert!(helper::semantic_json_equal(&current, baseline));
    assert!(!result.stderr.contains("private-owner-body"));
}

fn capture_standard(fixture: &Fixture, prefix: &str, baseline: &Value, id: &str) -> CaptureResult {
    let captured = capture_selected(
        fixture,
        prefix,
        baseline,
        vec![ALLOW_SCOPE],
        vec![json!({
            "id": id,
            "bucket": "always_allow",
            "index": 0
        })],
    );
    let state_document = state_value(&captured.state);
    let pattern_file = captured_pattern_file(&captured.output, &state_document);

    CaptureResult {
        candidate: captured.candidate,
        output: captured.output,
        pattern_file,
        settings: captured.settings,
        state: captured.state,
    }
}

fn replace_scope(value: &mut Value, scope: &str, replacement: Value) {
    let tokens = helper::decode_json_pointer(scope).expect("Scope must be a valid pointer");
    helper::replace_pointer_value(value, &tokens, replacement)
        .expect("Scope must exist in fixture settings");
}

fn replace_allow_scope(value: &mut Value, replacement: Value) {
    replace_scope(value, ALLOW_SCOPE, replacement);
}

#[test]
fn documents_all_modes_and_rejects_invalid_arguments() {
    let result = run(vec![OsString::from("--help")]);

    assert_eq!(result.status, 0);
    assert!(result.stderr.is_empty());
    assert!(result.stdout.contains("capture --settings <path>"));
    assert!(result.stdout.contains(
        "materialize --candidate <candidate-path> --state <state-path> --selection <selection-path> --output <directory>\n"
    ));
    assert!(result.stdout.contains("verify --settings <path>"));
    assert!(
        result
            .stdout
            .contains("promote --settings <live-settings-path>")
    );
    assert!(result.stdout.contains("permission-candidate --help"));
    assert!(result.stdout.contains("Capture selection JSON schema"));
    assert!(
        result
            .stdout
            .contains("Materialization selection JSON schema")
    );
    assert!(result.stdout.contains("State JSON schema"));
    assert!(result.stdout.contains("Artifact catalog JSON schema"));
    assert!(result.stdout.contains("--catalog <artifact-catalog-path>"));
    assert!(result.stdout.contains("permission-candidate seal "));
    assert!(result.stdout.contains("permission-candidate preflight "));
    assert!(result.stdout.contains("permission-candidate refresh "));
    assert!(result.stdout.contains("--bundle <bundle-path> --write"));
    assert!(result.stdout.contains("--owner-spec <path>"));
    assert!(
        result
            .stdout
            .contains("Stable owner specification used by `seal` only")
    );
    assert!(
        result
            .stdout
            .contains("`--bundle` and `--write` are mandatory. There is no force option")
    );
    assert!(!result.stdout.contains("`--catalog` and `--write`"));
    assert!(!result.stdout.contains("The strict catalog must bind"));
    assert!(result.stdout.contains(
        "The sealed bundle supplies the candidate, state, catalog, owner specification, and bound validation evidence"
    ));
    assert!(
        result
            .stdout
            .contains("Neither the bundle, a passing preflight, nor `--write` is user approval")
    );
    assert!(result.stdout.contains(
        "reruns the complete preflight in-process immediately before the mutation boundary"
    ));
    assert!(result.stdout.contains("{\"scopes\":[\"/json/pointer\"]"));
    assert!(
        result
            .stdout
            .contains("{\"patterns\":[{\"id\":\"nonempty\"")
    );
    assert!(!result.stdout.contains("\"version\""));
    assert!(!result.stdout.contains("\"owner_replacement\":true"));
    assert!(
        result
            .stdout
            .contains("Ownership is declared by the owner spec, not the selection")
    );
    assert!(result.stdout.contains("`patterns` may be empty"));
    assert!(result.stdout.contains("candidate-settings.json"));
    assert!(result.stdout.contains("does not authenticate itself"));
    assert!(result.stdout.contains(
        "Successful capture, materialization, verification, seal, preflight, refresh, and promotion results are written to standard output"
    ));
    assert!(
        result
            .stdout
            .contains("Refusals and errors are written to standard error")
    );
    assert!(result.stdout.contains("Exit statuses:"));
    assert!(result.stdout.contains(
        "0  Capture, materialization, verification, seal, preflight, refresh, promotion, unchanged promotion, or help succeeded"
    ));
    assert!(result.stdout.contains(
        "1  Current state could not be uniquely reindexed, or candidate authorization, owner coverage, refresh replay, preflight, or guarded promotion was refused"
    ));
    assert!(result.stdout.contains("2  Arguments or data"));

    let misplaced = run(vec![OsString::from("verify"), OsString::from("--help")]);
    assert_eq!(misplaced.status, 2);
    assert!(misplaced.stdout.is_empty());
    assert!(misplaced.stderr.contains("must be used alone"));

    let missing = run(Vec::new());
    assert_eq!(missing.status, 2);
    assert!(missing.stdout.is_empty());
    assert!(missing.stderr.contains(
        "Missing mode. Specify `capture`, `materialize`, `preflight`, `promote`, `refresh`, `seal`, or `verify`."
    ));

    let unknown = run(vec![OsString::from("repair")]);
    assert_eq!(unknown.status, 2);
    assert!(unknown.stderr.contains("Unknown mode"));
}

#[test]
fn rejects_owner_spec_for_materialize_and_catalog_for_promote() {
    let fixture = Fixture::new();

    let materialize = run(vec![
        OsString::from("materialize"),
        OsString::from("--candidate"),
        fixture.path("candidate-settings.json").into_os_string(),
        OsString::from("--state"),
        fixture.path("state.json").into_os_string(),
        OsString::from("--selection"),
        fixture.path("selection.json").into_os_string(),
        OsString::from("--output"),
        fixture.path("materialized").into_os_string(),
        OsString::from("--owner-spec"),
        fixture.path("owner-spec.json").into_os_string(),
    ]);

    assert_eq!(materialize.status, 2);
    assert!(materialize.stdout.is_empty());
    assert!(
        materialize
            .stderr
            .contains("Unknown materialize option `--owner-spec`")
    );

    let promote = run(vec![
        OsString::from("promote"),
        OsString::from("--settings"),
        fixture.path("live.json").into_os_string(),
        OsString::from("--bundle"),
        fixture.path("candidate-bundle.json").into_os_string(),
        OsString::from("--catalog"),
        fixture.path("artifact-catalog.json").into_os_string(),
        OsString::from("--write"),
    ]);

    assert_eq!(promote.status, 2);
    assert!(promote.stdout.is_empty());
    assert!(
        promote
            .stderr
            .contains("Unknown promote option `--catalog`")
    );
}

#[test]
fn rejects_version_and_unknown_selection_and_state_fields() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let invalid_selection = json!({
        "scopes": [ALLOW_SCOPE],
        "patterns": [{
            "id": "alpha",
            "bucket": "always_allow",
            "index": 0
        }],
        "secret-selection-field": true
    });
    let selection_path = fixture.write_json("selection.json", &invalid_selection);
    let output = fixture.path("invalid-selection-artifacts");

    let selection_result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(selection_result.status, 2);
    assert!(
        selection_result
            .stderr
            .contains("does not match the required schema")
    );
    assert!(!selection_result.stderr.contains("secret-selection-field"));
    assert!(!output.exists());

    let mut versioned_selection = selection("alpha", 0);
    versioned_selection["version"] = json!(1);
    fs::write(
        &selection_path,
        serde_json::to_vec(&versioned_selection).expect("Selection JSON must serialize"),
    )
    .expect("Failed to replace selection JSON");
    let version_result = run(capture_arguments(&settings_path, &selection_path, &output));
    assert_eq!(version_result.status, 2);
    assert!(
        version_result
            .stderr
            .contains("does not match the required schema")
    );
    assert!(!output.exists());

    let invalid_bucket = json!({
        "scopes": [ALLOW_SCOPE],
        "patterns": [{
            "id": "alpha",
            "bucket": "secret-pattern-body",
            "index": 0
        }]
    });
    fs::write(
        &selection_path,
        serde_json::to_vec(&invalid_bucket).expect("Selection JSON must serialize"),
    )
    .expect("Failed to replace selection JSON");
    let bucket_result = run(capture_arguments(&settings_path, &selection_path, &output));
    assert_eq!(bucket_result.status, 2);
    assert!(
        bucket_result
            .stderr
            .contains("does not match the required schema")
    );
    assert!(!bucket_result.stderr.contains("secret-pattern-body"));
    assert!(!output.exists());

    let captured = capture_standard(&fixture, "valid", &baseline, "alpha");
    let state = state_value(&captured.state);
    let mut unknown_state = state.clone();
    unknown_state["secret-state-field"] = json!(true);
    fs::write(
        &captured.state,
        serde_json::to_vec(&unknown_state).expect("State JSON must serialize"),
    )
    .expect("Failed to replace state manifest");

    let state_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(state_result.status, 2);
    assert!(
        state_result
            .stderr
            .contains("does not match the required schema")
    );
    assert!(!state_result.stderr.contains("secret-state-field"));

    let mut versioned_state = state;
    versioned_state["version"] = json!(1);
    fs::write(
        &captured.state,
        serde_json::to_vec(&versioned_state).expect("State JSON must serialize"),
    )
    .expect("Failed to replace state manifest");
    let version_result = run(verify_arguments(&captured.settings, &captured.state));
    assert_eq!(version_result.status, 2);
    assert!(
        version_result
            .stderr
            .contains("does not match the required schema")
    );
}

#[test]
fn captures_exact_baseline_and_pattern_bytes_without_newlines() {
    let fixture = Fixture::new();
    let settings_bytes = br#"{ "outside": {"generation":1}, "agent": {"tool_permissions":{"tools":{"terminal":{"always_allow":[{"pattern":"^alpha\\sbeta$","case_sensitive":true}],"always_confirm":[],"always_deny":[]}}}}}"#;
    let settings_path = fixture.write("settings.json", settings_bytes);
    let selection_path = fixture.write_json("selection.json", &selection("alpha", 0));
    let output = fixture.path("artifacts");

    let result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert_eq!(
        fs::read(output.join("baseline-settings.json")).unwrap(),
        settings_bytes
    );
    assert_eq!(
        fs::read(output.join("candidate-settings.json")).unwrap(),
        settings_bytes
    );
    assert!(
        result
            .stdout
            .contains("candidate -> candidate-settings.json")
    );
    let state = state_value(&output.join("state.json"));
    let pattern_file = captured_pattern_file(&output, &state);
    let pattern_bytes = fs::read(pattern_file).expect("Failed to read captured pattern");
    assert_eq!(pattern_bytes, br"^alpha\sbeta$");
    assert_ne!(pattern_bytes.last(), Some(&b'\n'));
    assert_eq!(
        state["baseline_sha256"],
        json!(helper::sha256_hex(settings_bytes))
    );
    assert_eq!(
        state["patterns"][0]["sha256"],
        json!(helper::sha256_hex(&pattern_bytes))
    );
    assert!(!result.stdout.contains(r"^alpha\sbeta$"));
    assert!(result.stderr.is_empty());
}

#[test]
fn refuses_existing_artifacts_before_writing() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selection_path = fixture.write_json("selection.json", &selection("alpha", 0));
    let output = fixture.create_dir("artifacts");
    let baseline_artifact = output.join("baseline-settings.json");
    fs::write(&baseline_artifact, b"sentinel").expect("Failed to write existing artifact");

    let result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("already exists"));
    assert!(
        result
            .stderr
            .contains("Choose an output directory without existing artifacts")
    );
    assert_eq!(fs::read(baseline_artifact).unwrap(), b"sentinel");
    assert!(!output.join("candidate-settings.json").exists());
    assert!(!output.join("state.json").exists());
    assert_eq!(fs::read_dir(output).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn sanitizes_unsafe_ids_and_refuses_symlinked_output_components() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let unsafe_id = "../../séc\nret";
    let selection_path = fixture.write_json("selection.json", &selection(unsafe_id, 0));
    let output = fixture.path("safe-artifacts");

    let result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(result.status, 0, "{}", result.stderr);
    let state = state_value(&output.join("state.json"));
    let relative = state["patterns"][0]["pattern_file"]
        .as_str()
        .expect("Pattern file must be a string");
    assert_eq!(relative, helper::generated_pattern_filename(1, unsafe_id));
    assert_eq!(state["patterns"][0]["id"], json!(unsafe_id));
    assert!(
        Path::new(relative)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    );
    assert!(!relative.contains(".."));
    assert!(output.join(relative).is_file());

    let real_parent = fixture.create_dir("real-parent");
    fs::create_dir(real_parent.join("output")).expect("Failed to create real output");
    let linked_parent = fixture.path("linked-parent");
    symlink(&real_parent, &linked_parent).expect("Failed to create parent symlink");
    let linked_output = linked_parent.join("output");
    let linked_result = run(capture_arguments(
        &settings_path,
        &selection_path,
        &linked_output,
    ));

    assert_eq!(linked_result.status, 2);
    assert!(linked_result.stderr.contains("symbolic link"));
    assert_eq!(fs::read_dir(real_parent.join("output")).unwrap().count(), 0);
}

#[test]
fn computes_standard_sha256_hex() {
    assert_eq!(
        helper::sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn decodes_traverses_and_replaces_rfc6901_pointers() {
    let mut value = json!({
        "a/b": {
            "~key": [
                {"value": 1},
                {"value": 2}
            ],
            "01": "object-key"
        }
    });
    let tokens =
        helper::decode_json_pointer("/a~1b/~0key/1/value").expect("Escaped pointer must decode");

    assert_eq!(
        helper::pointer_value(&value, &tokens).expect("Pointer must resolve"),
        &json!(2)
    );
    helper::replace_pointer_value(&mut value, &tokens, json!(3))
        .expect("Existing pointer value must be replaceable");
    assert_eq!(helper::pointer_value(&value, &tokens).unwrap(), &json!(3));

    let object_index = helper::decode_json_pointer("/a~1b/01").unwrap();
    assert_eq!(
        helper::pointer_value(&value, &object_index).unwrap(),
        &json!("object-key")
    );
    assert!(helper::decode_json_pointer("a/b").is_err());
    assert!(helper::decode_json_pointer("/a~2b").is_err());
    assert!(
        helper::pointer_value(
            &value,
            &["a/b".to_owned(), "~key".to_owned(), "01".to_owned()]
        )
        .is_err()
    );
    assert!(
        helper::pointer_value(
            &value,
            &["a/b".to_owned(), "~key".to_owned(), "-".to_owned()]
        )
        .is_err()
    );
    assert!(
        helper::replace_pointer_value(
            &mut value,
            &["a/b".to_owned(), "missing".to_owned()],
            json!(null),
        )
        .is_err()
    );
    assert!(value["a/b"].get("missing").is_none());
}

#[test]
fn rejects_root_invalid_missing_and_overlapping_scopes() {
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let child_scope = format!("{ALLOW_SCOPE}/0");

    assert!(helper::validate_scopes(&baseline, &[]).is_err());
    assert!(helper::validate_scopes(&baseline, &[String::new()]).is_err());
    assert!(helper::validate_scopes(&baseline, &["outside".to_owned()]).is_err());
    assert!(helper::validate_scopes(&baseline, &["/~2".to_owned()]).is_err());
    assert!(helper::validate_scopes(&baseline, &["/missing".to_owned()]).is_err());
    assert!(
        helper::validate_scopes(&baseline, &[ALLOW_SCOPE.to_owned(), ALLOW_SCOPE.to_owned()],)
            .is_err()
    );
    assert!(helper::validate_scopes(&baseline, &[ALLOW_SCOPE.to_owned(), child_scope]).is_err());
}

#[test]
fn rejects_duplicate_selections_and_patterns_outside_scopes() {
    let fixture = Fixture::new();
    let baseline = settings(
        vec![pattern("^fx alpha$", true), pattern("^fx beta$", true)],
        1,
    );
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let duplicate_ids = json!({
        "scopes": [ALLOW_SCOPE],
        "patterns": [
            {"id": "same", "bucket": "always_allow", "index": 0},
            {"id": "same", "bucket": "always_allow", "index": 1}
        ]
    });
    let selection_path = fixture.write_json("selection.json", &duplicate_ids);
    let output = fixture.path("duplicate-id-artifacts");

    let duplicate_id_result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(duplicate_id_result.status, 2);
    assert!(duplicate_id_result.stderr.contains("IDs must be unique"));
    assert!(!output.exists());

    let duplicate_selections = json!({
        "scopes": [ALLOW_SCOPE],
        "patterns": [
            {"id": "first", "bucket": "always_allow", "index": 0},
            {"id": "second", "bucket": "always_allow", "index": 0}
        ]
    });
    fs::write(
        &selection_path,
        serde_json::to_vec(&duplicate_selections).unwrap(),
    )
    .unwrap();
    let duplicate_selection_output = fixture.path("duplicate-selection-artifacts");
    let duplicate_selection_result = run(capture_arguments(
        &settings_path,
        &selection_path,
        &duplicate_selection_output,
    ));

    assert_eq!(duplicate_selection_result.status, 2);
    assert!(
        duplicate_selection_result
            .stderr
            .contains("bucket/index pairs must be unique")
    );
    assert!(!duplicate_selection_output.exists());

    let outside_selection = json!({
        "scopes": ["/outside"],
        "patterns": [
            {"id": "alpha", "bucket": "always_allow", "index": 0}
        ]
    });
    fs::write(
        &selection_path,
        serde_json::to_vec(&outside_selection).unwrap(),
    )
    .unwrap();
    let outside_output = fixture.path("outside-artifacts");
    let outside_result = run(capture_arguments(
        &settings_path,
        &selection_path,
        &outside_output,
    ));

    assert_eq!(outside_result.status, 2);
    assert!(
        outside_result
            .stderr
            .contains("outside every authorized scope")
    );
    assert!(!outside_output.exists());
}

#[test]
fn shared_bucket_serde_parses_selection_and_state_schemas() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx allow$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selection_path = fixture.write_json(
        "selection.json",
        &json!({
            "scopes": [ALLOW_SCOPE, CONFIRM_SCOPE, DENY_SCOPE],
            "patterns": [
                {"id": "allow", "bucket": "always_allow", "index": 0},
                {"id": "confirm", "bucket": "always_confirm", "index": 0},
                {"id": "deny", "bucket": "always_deny", "index": 0}
            ]
        }),
    );
    let output = fixture.path("artifacts");

    let capture_result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(capture_result.status, 0, "{}", capture_result.stderr);
    let state_path = output.join("state.json");
    let state = state_value(&state_path);
    assert_eq!(state["patterns"][0]["bucket"], json!("always_allow"));
    assert_eq!(state["patterns"][1]["bucket"], json!("always_confirm"));
    assert_eq!(state["patterns"][2]["bucket"], json!("always_deny"));

    let verify_result = run(verify_arguments(&settings_path, &state_path));

    assert_eq!(verify_result.status, 0, "{}", verify_result.stderr);
    assert_eq!(
        verify_result.stdout,
        "Verified 3 patterns: 3 unchanged and 0 moved\n"
    );
}

#[test]
fn indexes_each_relevant_bucket_once_for_many_exact_pattern_selections() {
    const OBJECT_COUNT: usize = 40;
    const SELECTION_COUNT: usize = 20;

    let make_bucket = |label: &str| {
        let mut values = vec![pattern(&format!("^{label}-noise$"), true)];
        values.extend(
            (0..OBJECT_COUNT)
                .map(|index| pattern(&format!("^{label}-{index:03}$"), index.is_multiple_of(2))),
        );
        values
    };
    let mut allow = make_bucket("allow");
    let confirm = make_bucket("confirm");
    let deny = make_bucket("deny");
    allow.push(pattern("^allow-007$", false));

    let selected: Vec<(helper::Bucket, usize, Vec<u8>, bool)> = [
        (helper::Bucket::Allow, "allow"),
        (helper::Bucket::Confirm, "confirm"),
        (helper::Bucket::Deny, "deny"),
    ]
    .into_iter()
    .flat_map(|(bucket, label)| {
        (0..SELECTION_COUNT).map(move |index| {
            (
                bucket,
                index,
                format!("^{label}-{index:03}$").into_bytes(),
                index.is_multiple_of(2),
            )
        })
    })
    .collect();
    let bucket_reads: [Cell<usize>; 3] = std::array::from_fn(|_| Cell::new(0));

    let index = helper::index_current_terminal_patterns(
        selected.iter().map(|(bucket, _, _, _)| *bucket),
        |bucket| {
            let (slot, values) = match bucket {
                helper::Bucket::Allow => (0, allow.as_slice()),
                helper::Bucket::Confirm => (1, confirm.as_slice()),
                helper::Bucket::Deny => (2, deny.as_slice()),
            };
            bucket_reads[slot].set(bucket_reads[slot].get() + 1);
            Some(values)
        },
    );

    for (bucket, source_index, bytes, case_sensitive) in &selected {
        let indexes = index.indexes(*bucket, bytes, *case_sensitive);
        if *bucket == helper::Bucket::Allow && *source_index == 7 {
            assert_eq!(indexes, &[8, 41]);
        } else {
            assert_eq!(indexes, &[*source_index + 1]);
        }
    }
    assert!(
        index
            .indexes(helper::Bucket::Allow, b"^fx allow-007$", true)
            .is_empty()
    );
    assert_eq!(bucket_reads.map(|reads| reads.get()), [1, 1, 1]);
}

#[test]
fn reindexes_patterns_after_bucket_movement() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "moving", &baseline, "alpha");
    let current = settings(
        vec![pattern("^fx new$", true), pattern("^fx alpha$", true)],
        2,
    );
    let current_path = fixture.write_pretty_json("current.json", &current);

    let result = run(verify_arguments(&current_path, &captured.state));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert_eq!(
        result.stdout,
        "alpha -> always_allow[1]\nVerified 1 pattern: 0 unchanged and 1 moved\n"
    );
    assert!(result.stderr.is_empty());
}

#[test]
fn bounds_moved_mappings_and_omits_unchanged_lines() {
    let fixture = Fixture::new();
    let baseline_patterns: Vec<Value> = (0..13)
        .map(|index| pattern(&format!("^private-moved-{index:03}$"), true))
        .collect();
    let baseline = settings(baseline_patterns.clone(), 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selections: Vec<Value> = (0..baseline_patterns.len())
        .map(|index| {
            json!({
                "id": format!("id-{index:03}"),
                "bucket": "always_allow",
                "index": index
            })
        })
        .collect();
    let selection_path = fixture.write_json(
        "selection.json",
        &json!({
            "scopes": [ALLOW_SCOPE],
            "patterns": selections
        }),
    );
    let output = fixture.path("artifacts");
    let capture_result = run(capture_arguments(&settings_path, &selection_path, &output));
    assert_eq!(capture_result.status, 0, "{}", capture_result.stderr);
    let mut current_patterns = baseline_patterns;
    current_patterns[..12].rotate_left(1);
    let current = settings(current_patterns, 2);
    let current_path = fixture.write_pretty_json("current.json", &current);

    let result = run(verify_arguments(&current_path, &output.join("state.json")));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert_eq!(result.stdout.matches(" -> always_allow[").count(), 10);
    assert!(result.stdout.contains("id-000 -> always_allow[11]"));
    assert!(result.stdout.contains("id-009 -> always_allow[8]"));
    assert!(!result.stdout.contains("id-010 ->"));
    assert!(!result.stdout.contains("id-011 ->"));
    assert!(!result.stdout.contains("id-012 ->"));
    assert!(
        result
            .stdout
            .contains("… 2 additional moved mappings omitted")
    );
    assert!(
        result
            .stdout
            .ends_with("Verified 13 patterns: 1 unchanged and 12 moved\n")
    );
    assert!(!result.stdout.contains("private-moved"));
    assert!(result.stderr.is_empty());
}

#[test]
fn refuses_missing_and_duplicate_exact_current_matches() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-pattern$", true)], 1);
    let captured = capture_standard(&fixture, "matches", &baseline, "private");
    let missing = settings(Vec::new(), 1);
    let missing_path = fixture.write_pretty_json("missing.json", &missing);

    let missing_result = run(verify_arguments(&missing_path, &captured.state));

    assert_eq!(missing_result.status, 1);
    assert!(missing_result.stdout.is_empty());
    assert!(
        missing_result
            .stderr
            .contains("Missing: 1. Duplicate matches: 0")
    );
    assert!(
        missing_result
            .stderr
            .contains("private -> always_allow[missing]")
    );
    assert!(!missing_result.stderr.contains("^fx private-pattern$"));

    let duplicate = settings(
        vec![
            pattern("^fx private-pattern$", true),
            pattern("^fx private-pattern$", true),
        ],
        1,
    );
    let duplicate_path = fixture.write_pretty_json("duplicate.json", &duplicate);
    let duplicate_result = run(verify_arguments(&duplicate_path, &captured.state));

    assert_eq!(duplicate_result.status, 1);
    assert!(duplicate_result.stdout.is_empty());
    assert!(
        duplicate_result
            .stderr
            .contains("Missing: 0. Duplicate matches: 1")
    );
    assert!(
        duplicate_result
            .stderr
            .contains("private -> always_allow[duplicate]")
    );
    assert!(!duplicate_result.stderr.contains("^fx private-pattern$"));
}

#[test]
fn bounds_mixed_missing_and_duplicate_failure_details() {
    let fixture = Fixture::new();
    let baseline_patterns: Vec<Value> = (0..24)
        .map(|index| pattern(&format!("^private-failure-{index:03}$"), true))
        .collect();
    let baseline = settings(baseline_patterns.clone(), 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selections: Vec<Value> = (0..baseline_patterns.len())
        .map(|index| {
            json!({
                "id": format!("id-{index:03}"),
                "bucket": "always_allow",
                "index": index
            })
        })
        .collect();
    let selection_path = fixture.write_json(
        "selection.json",
        &json!({
            "scopes": [ALLOW_SCOPE],
            "patterns": selections
        }),
    );
    let output = fixture.path("artifacts");
    let capture_result = run(capture_arguments(&settings_path, &selection_path, &output));
    assert_eq!(capture_result.status, 0, "{}", capture_result.stderr);
    let current_patterns: Vec<Value> = baseline_patterns
        .into_iter()
        .enumerate()
        .filter(|(index, _)| !index.is_multiple_of(2))
        .flat_map(|(_, pattern)| [pattern.clone(), pattern])
        .collect();
    let current = settings(current_patterns, 2);
    let current_path = fixture.write_pretty_json("current.json", &current);

    let result = run(verify_arguments(&current_path, &output.join("state.json")));

    assert_eq!(result.status, 1);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("Missing: 12. Duplicate matches: 12"));
    assert_eq!(result.stderr.matches(" -> always_allow[").count(), 10);
    assert_eq!(result.stderr.matches("[missing]").count(), 5);
    assert_eq!(result.stderr.matches("[duplicate]").count(), 5);
    assert!(
        result
            .stderr
            .contains("… 14 additional reindex failures omitted")
    );
    assert!(!result.stderr.contains("id-010 ->"));
    assert!(!result.stderr.contains("private-failure"));
}

#[test]
fn rejects_baseline_and_pattern_hash_tampering() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-pattern$", true)], 1);
    let captured = capture_standard(&fixture, "hashes", &baseline, "private");
    let baseline_file = captured.output.join("baseline-settings.json");
    let original_baseline = fs::read(&baseline_file).unwrap();
    let mut tampered_baseline = original_baseline.clone();
    tampered_baseline.push(b' ');
    fs::write(&baseline_file, tampered_baseline).unwrap();

    let baseline_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(baseline_result.status, 2);
    assert!(baseline_result.stderr.contains("Baseline artifact SHA-256"));
    assert!(!baseline_result.stderr.contains("^fx private-pattern$"));

    fs::write(&baseline_file, original_baseline).unwrap();
    fs::write(&captured.pattern_file, b"tampered-private-pattern").unwrap();

    let pattern_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(pattern_result.status, 2);
    assert!(pattern_result.stderr.contains("Pattern artifact SHA-256"));
    assert!(!pattern_result.stderr.contains("tampered-private-pattern"));
    assert!(!pattern_result.stderr.contains("^fx private-pattern$"));
}

#[test]
fn rejects_invalid_utf8_and_recorded_baseline_source_mismatches() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "identity", &baseline, "alpha");
    let mut state = state_value(&captured.state);
    let invalid_bytes = [0xff];
    fs::write(&captured.pattern_file, invalid_bytes).unwrap();
    state["patterns"][0]["sha256"] = json!(helper::sha256_hex(&invalid_bytes));
    fs::write(&captured.state, serde_json::to_vec(&state).unwrap()).unwrap();

    let utf8_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(utf8_result.status, 2);
    assert!(utf8_result.stderr.contains("not valid UTF-8"));

    fs::write(&captured.pattern_file, b"^fx alpha$").unwrap();
    state["patterns"][0]["sha256"] = json!(helper::sha256_hex(b"^fx alpha$"));
    state["patterns"][0]["case_sensitive"] = json!(false);
    fs::write(&captured.state, serde_json::to_vec(&state).unwrap()).unwrap();

    let identity_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(identity_result.status, 2);
    assert!(identity_result.stderr.contains("source identity"));
    assert!(!identity_result.stderr.contains("^fx alpha$"));
}

#[test]
fn bounds_metadata_output_and_never_leaks_contents() {
    let fixture = Fixture::new();
    let patterns: Vec<Value> = (0..101)
        .map(|index| pattern(&format!("^private-body-{index}$"), true))
        .collect();
    let baseline = settings(patterns, 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selections: Vec<Value> = (0..101)
        .map(|index| {
            json!({
                "id": format!("id-{index:03}"),
                "bucket": "always_allow",
                "index": index
            })
        })
        .collect();
    let selection_path = fixture.write_json(
        "selection.json",
        &json!({
            "scopes": [ALLOW_SCOPE],
            "patterns": selections
        }),
    );
    let output = fixture.path("artifacts");

    let capture_result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(capture_result.status, 0, "{}", capture_result.stderr);
    assert!(
        capture_result
            .stdout
            .contains("… 1 additional pattern artifacts omitted")
    );
    assert!(!capture_result.stdout.contains("private-body"));
    assert!(!capture_result.stderr.contains("private-body"));

    let verify_result = run(verify_arguments(&settings_path, &output.join("state.json")));

    assert_eq!(verify_result.status, 0, "{}", verify_result.stderr);
    assert_eq!(
        verify_result.stdout,
        "Verified 101 patterns: 101 unchanged and 0 moved\n"
    );
    assert!(!verify_result.stdout.contains("private-body"));
    assert!(!verify_result.stderr.contains("private-body"));
}

#[test]
fn materializes_exact_candidate_pattern_bytes_and_catalog_bindings() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx baseline$", true)], 1);
    let captured = capture_standard(&fixture, "materialize-exact", &baseline, "baseline");
    let mut candidate_value = baseline.clone();
    replace_allow_scope(
        &mut candidate_value,
        json!([
            pattern("ESCAPED_PATTERN_PLACEHOLDER", false),
            pattern(r"^literal\\escape$", true)
        ]),
    );
    let serialized = serde_json::to_string(&candidate_value).expect("Candidate must serialize");
    let candidate_bytes =
        serialized.replace("\"ESCAPED_PATTERN_PLACEHOLDER\"", r#""^line\n\u00e9猫$""#);
    fs::write(&captured.candidate, candidate_bytes.as_bytes()).unwrap();
    let selection_path = fixture.write_json(
        "materialization-selection.json",
        &materialization_selection(vec![
            replacement_pattern("escaped", "always_allow", 0),
            validation_pattern("literal", "always_allow", 1),
        ]),
    );
    let output = fixture.path("materialized");
    let candidate_before = fs::read(&captured.candidate).unwrap();
    let baseline_before = fs::read(captured.output.join("baseline-settings.json")).unwrap();
    let state_before = fs::read(&captured.state).unwrap();

    let result = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &output,
    ));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stderr.is_empty());
    assert!(result.stdout.contains("Materialized 2 patterns"));
    assert!(result.stdout.contains("catalog -> artifact-catalog.json"));
    assert!(!result.stdout.contains("^line"));
    assert!(!result.stdout.contains("literal\\escape"));
    let catalog = catalog_value(&output);
    assert!(catalog.get("version").is_none());
    assert_eq!(
        catalog["candidate_sha256"],
        json!(helper::sha256_hex(&candidate_before))
    );
    assert_eq!(
        catalog["state_sha256"],
        json!(helper::sha256_hex(&state_before))
    );
    assert_eq!(catalog["patterns"][0]["id"], json!("escaped"));
    assert_eq!(catalog["patterns"][0]["bucket"], json!("always_allow"));
    assert_eq!(catalog["patterns"][0]["source_index"], json!(0));
    assert_eq!(catalog["patterns"][0]["case_sensitive"], json!(false));
    assert_eq!(catalog["patterns"][1]["source_index"], json!(1));
    assert_eq!(catalog["patterns"][1]["case_sensitive"], json!(true));
    assert_eq!(catalog["patterns"][0]["owner_replacement"], Value::Null);
    let escaped_bytes = fs::read(materialized_pattern_file(&output, &catalog, 0)).unwrap();
    let literal_bytes = fs::read(materialized_pattern_file(&output, &catalog, 1)).unwrap();
    assert_eq!(escaped_bytes, "^line\né猫$".as_bytes());
    assert_eq!(literal_bytes, br"^literal\\escape$");
    assert_ne!(escaped_bytes.last(), Some(&b'\n'));
    assert_ne!(literal_bytes.last(), Some(&b'\n'));
    assert_eq!(
        catalog["patterns"][0]["sha256"],
        json!(helper::sha256_hex(&escaped_bytes))
    );
    assert_eq!(
        catalog["patterns"][1]["sha256"],
        json!(helper::sha256_hex(&literal_bytes))
    );
    assert_eq!(fs::read(&captured.candidate).unwrap(), candidate_before);
    assert_eq!(
        fs::read(captured.output.join("baseline-settings.json")).unwrap(),
        baseline_before
    );
    assert_eq!(fs::read(&captured.state).unwrap(), state_before);
    assert_eq!(fs::read(&captured.settings).unwrap(), baseline_before);
}

#[test]
fn materializes_a_specific_index_when_candidate_pattern_identities_are_duplicate() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx baseline$", true)], 1);
    let captured = capture_standard(&fixture, "duplicate-identity", &baseline, "baseline");
    let duplicate = pattern("^fx duplicate-decoded-identity$", false);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([duplicate.clone(), duplicate]));
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let selection_path = fixture.write_json(
        "selection.json",
        &materialization_selection(vec![json!({
            "id": "second-duplicate",
            "bucket": "always_allow",
            "index": 1
        })]),
    );
    let output = fixture.path("materialized");

    let result = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &output,
    ));

    assert_eq!(result.status, 0, "{}", result.stderr);
    let catalog = catalog_value(&output);
    assert_eq!(catalog["patterns"].as_array().unwrap().len(), 1);
    assert_eq!(catalog["patterns"][0]["source_index"], json!(1));
    assert_eq!(catalog["patterns"][0]["case_sensitive"], json!(false));
    assert_eq!(
        fs::read(materialized_pattern_file(&output, &catalog, 0)).unwrap(),
        b"^fx duplicate-decoded-identity$"
    );
}

#[test]
fn rejects_invalid_materialization_selection_metadata_before_writing() {
    let fixture = Fixture::new();
    let baseline = settings(
        vec![
            pattern("^fx private-alpha$", true),
            pattern("^fx private-beta$", false),
        ],
        1,
    );
    let captured = capture_standard(&fixture, "selection-errors", &baseline, "alpha");
    let cases = [
        (
            "version-field",
            json!({
                "version": 2,
                "patterns": [{"id": "alpha", "bucket": "always_allow", "index": 0, "owner_replacement": true}]
            }),
            "does not match the required schema",
        ),
        (
            "unknown",
            json!({
                "patterns": [{"id": "alpha", "bucket": "always_allow", "index": 0, "owner_replacement": true}],
                "private-unknown-field": true
            }),
            "does not match the required schema",
        ),
        (
            "duplicate-id",
            materialization_selection(vec![
                json!({"id": "same", "bucket": "always_allow", "index": 0}),
                json!({"id": "same", "bucket": "always_allow", "index": 1}),
            ]),
            "pattern IDs must be unique",
        ),
        (
            "duplicate-locator",
            materialization_selection(vec![
                json!({"id": "first", "bucket": "always_allow", "index": 0}),
                json!({"id": "second", "bucket": "always_allow", "index": 0}),
            ]),
            "bucket/index pairs must be unique",
        ),
        (
            "outside-scope",
            materialization_selection(vec![json!({
                "id": "confirm",
                "bucket": "always_confirm",
                "index": 0
            })]),
            "outside every authorized scope",
        ),
    ];

    for (name, selection, expected) in cases {
        let selection_path = fixture.write_json(&format!("{name}.json"), &selection);
        let output = fixture.path(&format!("{name}-output"));
        let result = run(materialize_arguments(
            &captured.candidate,
            &captured.state,
            &selection_path,
            &output,
        ));

        assert_eq!(result.status, 2, "{name}: {}", result.stderr);
        assert!(result.stdout.is_empty());
        assert!(
            result.stderr.contains(expected),
            "{name}: {}",
            result.stderr
        );
        assert!(!result.stderr.contains("private-alpha"));
        assert!(!result.stderr.contains("private-beta"));
        assert!(!result.stderr.contains("private-unknown-field"));
        assert!(!result.stderr.contains("private-role"));
        assert!(!output.exists());
    }
}

#[test]
fn rejects_missing_and_malformed_selected_candidate_objects_before_writing() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-baseline$", true)], 1);
    let captured = capture_standard(&fixture, "selected-object-errors", &baseline, "baseline");
    let missing_selection = fixture.write_json(
        "missing-selection.json",
        &materialization_selection(vec![json!({
            "id": "missing",
            "bucket": "always_allow",
            "index": 9
        })]),
    );
    let missing_output = fixture.path("missing-output");

    let missing = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &missing_selection,
        &missing_output,
    ));

    assert_eq!(missing.status, 2);
    assert!(missing.stdout.is_empty());
    assert!(missing.stderr.contains("bucket/index does not exist"));
    assert!(!missing.stderr.contains("private-baseline"));
    assert!(!missing_output.exists());

    let mut malformed_candidate = baseline.clone();
    replace_allow_scope(
        &mut malformed_candidate,
        json!([{"pattern": 42, "case_sensitive": "private-invalid-case"}]),
    );
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&malformed_candidate).unwrap(),
    )
    .unwrap();
    let malformed_selection = fixture.write_json(
        "malformed-selection.json",
        &materialization_selection(vec![json!({
            "id": "malformed",
            "bucket": "always_allow",
            "index": 0
        })]),
    );
    let malformed_output = fixture.path("malformed-output");

    let malformed = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &malformed_selection,
        &malformed_output,
    ));

    assert_eq!(malformed.status, 2);
    assert!(malformed.stdout.is_empty());
    assert!(malformed.stderr.contains("string `pattern`"));
    assert!(!malformed.stderr.contains("private-invalid-case"));
    assert!(!malformed_output.exists());
}

#[test]
fn distinguishes_candidate_authorization_refusal_from_malformed_data() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-baseline$", true)], 1);
    let captured = capture_standard(&fixture, "authorization", &baseline, "baseline");
    let selection_path = fixture.write_json(
        "selection.json",
        &materialization_selection(vec![json!({
            "id": "baseline",
            "bucket": "always_allow",
            "index": 0
        })]),
    );
    let mut outside_candidate = baseline.clone();
    outside_candidate["outside"]["generation"] = json!(987654321);
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&outside_candidate).unwrap(),
    )
    .unwrap();
    let refused_output = fixture.path("refused-output");

    let refused = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &refused_output,
    ));

    assert_eq!(refused.status, 1);
    assert!(refused.stdout.is_empty());
    assert!(refused.stderr.contains("Materialization refused"));
    assert!(refused.stderr.contains("outside authorized scopes"));
    assert!(!refused.stderr.contains("987654321"));
    assert!(!refused.stderr.contains("private-baseline"));
    assert!(!refused_output.exists());

    fs::write(&captured.candidate, b"{private-malformed-candidate").unwrap();
    let malformed_output = fixture.path("malformed-output");
    let malformed = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &malformed_output,
    ));

    assert_eq!(malformed.status, 2);
    assert!(malformed.stdout.is_empty());
    assert!(malformed.stderr.contains("JSON syntax is invalid"));
    assert!(!malformed.stderr.contains("private-malformed-candidate"));
    assert!(!malformed_output.exists());
}

#[test]
fn validates_complete_state_before_materializing_candidate_artifacts() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-state-pattern$", true)], 1);
    let captured = capture_standard(&fixture, "materialization-state", &baseline, "state");
    let selection_path = fixture.write_json(
        "selection.json",
        &materialization_selection(vec![json!({
            "id": "state",
            "bucket": "always_allow",
            "index": 0
        })]),
    );
    fs::write(&captured.pattern_file, b"private-tampered-state-artifact").unwrap();
    let tampered_output = fixture.path("tampered-output");

    let tampered = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &tampered_output,
    ));

    assert_eq!(tampered.status, 2);
    assert!(tampered.stdout.is_empty());
    assert!(tampered.stderr.contains("Pattern artifact SHA-256"));
    assert!(!tampered.stderr.contains("private-tampered-state-artifact"));
    assert!(!tampered.stderr.contains("private-state-pattern"));
    assert!(!tampered_output.exists());

    let mut malformed_state = state_value(&captured.state);
    malformed_state["private-unknown-state-field"] = json!(true);
    fs::write(
        &captured.state,
        serde_json::to_vec(&malformed_state).unwrap(),
    )
    .unwrap();
    let malformed_output = fixture.path("malformed-output");
    let malformed = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &malformed_output,
    ));

    assert_eq!(malformed.status, 2);
    assert!(malformed.stdout.is_empty());
    assert!(
        malformed
            .stderr
            .contains("does not match the required schema")
    );
    assert!(!malformed.stderr.contains("private-unknown-state-field"));
    assert!(!malformed_output.exists());
}

#[test]
fn materialization_preflights_every_artifact_and_refuses_overwrites() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-preflight$", true)], 1);
    let captured = capture_standard(&fixture, "materialization-preflight", &baseline, "baseline");
    let selection_path = fixture.write_json(
        "selection.json",
        &materialization_selection(vec![json!({
            "id": "private-id",
            "bucket": "always_allow",
            "index": 0
        })]),
    );
    let output = fixture.create_dir("output");
    let catalog_path = output.join("artifact-catalog.json");
    fs::write(&catalog_path, b"sentinel").unwrap();

    let result = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &output,
    ));

    assert_eq!(result.status, 2);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("already exists"));
    assert!(result.stderr.contains("without existing artifacts"));
    assert!(!result.stderr.contains("private-preflight"));
    assert_eq!(fs::read(&catalog_path).unwrap(), b"sentinel");
    assert!(
        !output
            .join(helper::generated_pattern_filename(1, "private-id"))
            .exists()
    );
    assert_eq!(fs::read_dir(output).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn materialization_sanitizes_ids_and_refuses_symlinked_output_components() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-safe-output$", true)], 1);
    let captured = capture_standard(
        &fixture,
        "materialization-safe-output",
        &baseline,
        "baseline",
    );
    let unsafe_id = "../../séc\nret";
    let selection_path = fixture.write_json(
        "selection.json",
        &materialization_selection(vec![json!({
            "id": unsafe_id,
            "bucket": "always_allow",
            "index": 0
        })]),
    );
    let safe_output = fixture.path("safe-output");

    let safe = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &safe_output,
    ));

    assert_eq!(safe.status, 0, "{}", safe.stderr);
    let catalog = catalog_value(&safe_output);
    let relative = catalog["patterns"][0]["pattern_file"].as_str().unwrap();
    assert_eq!(relative, helper::generated_pattern_filename(1, unsafe_id));
    assert_eq!(catalog["patterns"][0]["id"], json!(unsafe_id));
    assert!(
        Path::new(relative)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    );
    assert!(!relative.contains(".."));

    let real_parent = fixture.create_dir("real-parent");
    fs::create_dir(real_parent.join("output")).unwrap();
    let linked_parent = fixture.path("linked-parent");
    symlink(&real_parent, &linked_parent).unwrap();
    let linked_output = linked_parent.join("output");
    let linked = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &linked_output,
    ));

    assert_eq!(linked.status, 2);
    assert!(linked.stdout.is_empty());
    assert!(linked.stderr.contains("symbolic link"));
    assert!(!linked.stderr.contains("private-safe-output"));
    assert_eq!(fs::read_dir(real_parent.join("output")).unwrap().count(), 0);
}

#[test]
fn rolls_back_created_artifacts_after_a_later_write_failure() {
    let fixture = Fixture::new();
    let output = fixture.create_dir("rollback");
    let artifacts = vec![
        helper::PendingArtifact {
            filename: "first.regex".to_owned(),
            bytes: b"private-first-pattern".to_vec(),
        },
        helper::PendingArtifact {
            filename: "artifact-catalog.json".to_owned(),
            bytes: b"private-catalog".to_vec(),
        },
    ];
    let mut writes = 0;

    let result = helper::commit_artifacts_with_writer(
        &output,
        &artifacts,
        helper::ArtifactOperation::Materialization,
        |path, bytes| {
            writes += 1;
            if writes == 2 {
                return Err("injected write failure".to_owned());
            }
            fs::write(path, bytes).map_err(|error| error.to_string())
        },
    );

    assert_eq!(result, Err("injected write failure".to_owned()));
    assert_eq!(writes, 2);
    assert_eq!(fs::read_dir(output).unwrap().count(), 0);
}

#[test]
fn bounds_materialization_output_and_never_leaks_patterns_settings_arrays_or_hashes() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-baseline$", true)], 1);
    let captured = capture_standard(&fixture, "materialization-output", &baseline, "baseline");
    let candidate_patterns: Vec<Value> = (0..13)
        .map(|index| pattern(&format!("^private-materialized-body-{index:03}$"), true))
        .collect();
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, Value::Array(candidate_patterns));
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let selections: Vec<Value> = (0..13)
        .map(|index| {
            json!({
                "id": format!("id-{index:03}"),
                "bucket": "always_allow",
                "index": index
            })
        })
        .collect();
    let selection_path =
        fixture.write_json("selection.json", &materialization_selection(selections));
    let output = fixture.path("output");

    let result = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection_path,
        &output,
    ));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert_eq!(result.stdout.matches(" -> pattern-").count(), 10);
    assert!(result.stdout.contains("id-000 -> pattern-001-id-000.regex"));
    assert!(result.stdout.contains("id-009 -> pattern-010-id-009.regex"));
    assert!(!result.stdout.contains("id-010 ->"));
    assert!(!result.stdout.contains("id-011 ->"));
    assert!(!result.stdout.contains("id-012 ->"));
    assert!(
        result
            .stdout
            .contains("… 3 additional pattern artifacts omitted")
    );
    assert!(result.stdout.contains("catalog -> artifact-catalog.json"));
    assert!(!result.stdout.contains("private-materialized-body"));
    assert!(!result.stderr.contains("private-materialized-body"));
    assert!(!result.stdout.contains("always_allow\":["));
    assert!(!result.stderr.contains("always_allow\":["));
    let catalog = catalog_value(&output);
    let candidate_hash = catalog["candidate_sha256"].as_str().unwrap();
    let state_hash = catalog["state_sha256"].as_str().unwrap();
    assert!(!result.stdout.contains(candidate_hash));
    assert!(!result.stdout.contains(state_hash));
    assert!(!result.stderr.contains(candidate_hash));
    assert!(!result.stderr.contains(state_hash));
}

#[test]
fn supports_scope_only_capture_verification_materialization_and_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_selected(
        &fixture,
        "scope-only",
        &baseline,
        vec![DEFAULT_SCOPE],
        vec![],
    );
    let state = state_value(&captured.state);
    assert!(state.get("version").is_none());
    assert_eq!(state["patterns"], json!([]));

    let verified = run(verify_arguments(&captured.settings, &captured.state));
    assert_eq!(verified.status, 0, "{}", verified.stderr);
    assert_eq!(
        verified.stdout,
        "Verified 1 authorized scope against the captured baseline\n"
    );

    let mut drifted = baseline.clone();
    replace_scope(&mut drifted, DEFAULT_SCOPE, json!("deny"));
    let drifted_path = fixture.write_pretty_json("scope-only-drifted.json", &drifted);
    let refused = run(verify_arguments(&drifted_path, &captured.state));
    assert_eq!(refused.status, 1);
    assert!(refused.stdout.is_empty());
    assert!(refused.stderr.contains("authorized scope 1 drifted"));
    assert!(!refused.stderr.contains("deny"));

    let mut candidate_value = baseline.clone();
    replace_scope(&mut candidate_value, DEFAULT_SCOPE, json!("allow"));
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let selection = fixture.write_json(
        "scope-only-materialization-selection.json",
        &materialization_selection(vec![]),
    );
    let materialized = fixture.path("scope-only-materialized");
    let materialize_result = run(materialize_arguments(
        &captured.candidate,
        &captured.state,
        &selection,
        &materialized,
    ));
    assert_eq!(
        materialize_result.status, 0,
        "{}",
        materialize_result.stderr
    );
    assert!(
        materialize_result
            .stdout
            .contains("Materialized 0 patterns")
    );
    let catalog = catalog_value(&materialized);
    assert!(catalog.get("version").is_none());
    assert_eq!(catalog["patterns"], json!([]));

    let live = fixture.write_pretty_json("scope-only-live.json", &baseline);
    let bundle = seal_bundle(
        &fixture,
        "scope-only",
        &captured.candidate,
        &captured.state,
        &materialized.join("artifact-catalog.json"),
        &json!({ "owners": [], "overlaps": [] }),
    );
    let promotion = run(promote_arguments(&live, &bundle, true));
    assert_eq!(promotion.status, 0, "{}", promotion.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["default"],
        json!("allow")
    );
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        baseline["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
    );
}

#[test]
fn promotes_catalog_bound_owner_insertion_from_an_empty_capture() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx retained$", true)], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(
        &mut candidate,
        json!([
            pattern("^fx new-allow$", true),
            pattern("^fx retained$", true)
        ]),
    );
    replace_scope(
        &mut candidate,
        CONFIRM_SCOPE,
        json!([
            pattern("^fx new-confirm$", true),
            pattern("^fx confirm$", true)
        ]),
    );
    replace_scope(
        &mut candidate,
        DENY_SCOPE,
        json!([pattern("^fx new-deny$", true), pattern("^fx deny$", true)]),
    );

    let (result, live) = promote_owner_candidate(
        &fixture,
        "insertion-only-owner",
        &baseline,
        vec![ALLOW_SCOPE, CONFIRM_SCOPE, DENY_SCOPE],
        vec![],
        &candidate,
        vec![
            replacement_pattern("new-allow", "always_allow", 0),
            replacement_pattern("new-confirm", "always_confirm", 0),
            replacement_pattern("new-deny", "always_deny", 0),
        ],
    );

    assert_eq!(result.status, 0, "{}", result.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(live).unwrap()).unwrap();
    assert_eq!(promoted, candidate);
}

#[test]
fn refuses_undeclared_remainder_changes_during_insertion_only_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx retained$", true)], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(
        &mut candidate,
        json!([
            pattern("^fx new-owner$", true),
            pattern("^fx private-undeclared$", true),
            pattern("^fx retained$", true)
        ]),
    );
    let expected_live = helper::serialize_pretty_json(&baseline).unwrap();

    let (result, live) = promote_owner_candidate(
        &fixture,
        "insertion-only-remainder",
        &baseline,
        vec![ALLOW_SCOPE],
        vec![],
        &candidate,
        vec![replacement_pattern("new-owner", "always_allow", 0)],
    );

    assert_eq!(result.status, 1, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(
        result
            .stderr
            .contains("declared owner membership reconciles to")
            || result.stderr.contains("outside-owner remainder")
    );
    assert!(!result.stderr.contains("private-undeclared"));
    assert!(!result.stderr.contains("new-owner"));
    assert_eq!(fs::read(live).unwrap(), expected_live);
}

#[test]
fn refuses_terminal_pattern_array_changes_from_an_empty_capture_without_touching_live_bytes() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-baseline$", true)], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(&mut candidate, json!([]));
    let expected_live = helper::serialize_pretty_json(&baseline).unwrap();

    let (result, live) = promote_owner_candidate(
        &fixture,
        "empty-capture-array-change",
        &baseline,
        vec![ALLOW_SCOPE],
        vec![],
        &candidate,
        vec![],
    );

    assert_eq!(result.status, 1, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(
        result
            .stderr
            .contains("declared owner membership reconciles to")
            || result.stderr.contains("outside-owner remainder")
    );
    assert!(!result.stderr.contains("private-baseline"));
    assert_eq!(fs::read(live).unwrap(), expected_live);
}

#[test]
fn rejects_malformed_terminal_pattern_arrays_from_an_empty_capture() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-baseline$", true)], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(&mut candidate, json!({"private-value": true}));
    let expected_live = helper::serialize_pretty_json(&baseline).unwrap();

    let (result, live) = promote_owner_candidate(
        &fixture,
        "empty-capture-malformed-array",
        &baseline,
        vec![ALLOW_SCOPE],
        vec![],
        &candidate,
        vec![],
    );

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(
        result
            .stderr
            .contains("candidate terminal permission bucket `always_allow` must be an array")
    );
    assert!(!result.stderr.contains("private-value"));
    assert!(!result.stderr.contains("private-baseline"));
    assert_eq!(fs::read(live).unwrap(), expected_live);
}

#[test]
fn refuses_undeclared_owner_remainder_addition_removal_edit_and_reorder() {
    let fixture = Fixture::new();
    let baseline = settings(
        vec![
            pattern("^fx private-owner-body$", true),
            pattern("^fx retained-a$", true),
            pattern("^fx retained-b$", false),
        ],
        1,
    );
    let captured = vec![json!({
        "id": "old-owner",
        "bucket": "always_allow",
        "index": 0
    })];

    let mut addition = baseline.clone();
    replace_allow_scope(
        &mut addition,
        json!([
            pattern("^fx replacement$", true),
            pattern("^fx retained-a$", true),
            pattern("^fx retained-b$", false),
            pattern("^fx undeclared-addition$", true)
        ]),
    );
    assert_owner_remainder_refused(
        &fixture,
        "undeclared-addition",
        &baseline,
        captured.clone(),
        &addition,
        vec![
            replacement_pattern("replacement", "always_allow", 0),
            validation_pattern("retained-a", "always_allow", 1),
            validation_pattern("retained-b", "always_allow", 2),
        ],
    );

    let mut removal = baseline.clone();
    replace_allow_scope(
        &mut removal,
        json!([
            pattern("^fx replacement$", true),
            pattern("^fx retained-a$", true)
        ]),
    );
    assert_owner_remainder_refused(
        &fixture,
        "undeclared-removal",
        &baseline,
        captured.clone(),
        &removal,
        vec![
            replacement_pattern("replacement", "always_allow", 0),
            validation_pattern("retained-a", "always_allow", 1),
        ],
    );

    let mut edited = baseline.clone();
    replace_allow_scope(
        &mut edited,
        json!([
            pattern("^fx replacement$", true),
            {
                "pattern": "^fx retained-a$",
                "case_sensitive": true,
                "note": "edited"
            },
            pattern("^fx retained-b$", false)
        ]),
    );
    assert_owner_remainder_refused(
        &fixture,
        "undeclared-edit",
        &baseline,
        captured.clone(),
        &edited,
        vec![
            replacement_pattern("replacement", "always_allow", 0),
            validation_pattern("retained-a", "always_allow", 1),
            validation_pattern("retained-b", "always_allow", 2),
        ],
    );

    let mut reordered = baseline.clone();
    replace_allow_scope(
        &mut reordered,
        json!([
            pattern("^fx replacement$", true),
            pattern("^fx retained-b$", false),
            pattern("^fx retained-a$", true)
        ]),
    );
    assert_owner_remainder_refused(
        &fixture,
        "undeclared-reorder",
        &baseline,
        captured,
        &reordered,
        vec![
            replacement_pattern("replacement", "always_allow", 0),
            validation_pattern("retained-b", "always_allow", 1),
            validation_pattern("retained-a", "always_allow", 2),
        ],
    );
}

#[test]
fn refuses_retained_owner_omitted_from_the_replacement_set() {
    let fixture = Fixture::new();
    let baseline = settings(
        vec![
            pattern("^fx private-owner-body-a$", true),
            pattern("^fx private-owner-body-b$", false),
        ],
        1,
    );
    let mut candidate = baseline.clone();
    replace_allow_scope(
        &mut candidate,
        json!([
            pattern("^fx replacement$", true),
            pattern("^fx private-owner-body-b$", false)
        ]),
    );

    assert_owner_remainder_refused(
        &fixture,
        "omitted-retained-owner",
        &baseline,
        vec![
            json!({"id": "old-a", "bucket": "always_allow", "index": 0}),
            json!({"id": "old-b", "bucket": "always_allow", "index": 1}),
        ],
        &candidate,
        vec![
            replacement_pattern("replacement", "always_allow", 0),
            validation_pattern("retained-owner", "always_allow", 1),
        ],
    );
}

#[test]
fn promotes_complete_one_to_many_replacement_with_validation_only_overlap() {
    let fixture = Fixture::new();
    let overlap = pattern("^fx overlap$", false);
    let baseline = settings(vec![pattern("^fx old-owner$", true), overlap.clone()], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(
        &mut candidate,
        json!([
            pattern("^fx new-owner-a$", true),
            overlap,
            pattern("^fx new-owner-b$", false)
        ]),
    );

    let (result, live) = promote_owner_candidate(
        &fixture,
        "one-to-many",
        &baseline,
        vec![ALLOW_SCOPE],
        vec![json!({"id": "old-owner", "bucket": "always_allow", "index": 0})],
        &candidate,
        vec![
            replacement_pattern("new-owner-a", "always_allow", 0),
            validation_pattern("overlap", "always_allow", 1),
            replacement_pattern("new-owner-b", "always_allow", 2),
        ],
    );

    assert_eq!(result.status, 0, "{}", result.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        candidate["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
    );
}

#[test]
fn promotes_cross_bucket_owner_movement_with_validation_only_overlap() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx old-owner$", true)], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(&mut candidate, json!([]));
    replace_scope(
        &mut candidate,
        CONFIRM_SCOPE,
        json!([
            pattern("^fx confirm$", true),
            pattern("^fx moved-owner$", false)
        ]),
    );

    let (result, live) = promote_owner_candidate(
        &fixture,
        "cross-bucket",
        &baseline,
        vec![ALLOW_SCOPE, CONFIRM_SCOPE],
        vec![json!({"id": "old-owner", "bucket": "always_allow", "index": 0})],
        &candidate,
        vec![
            validation_pattern("confirm-overlap", "always_confirm", 0),
            replacement_pattern("moved-owner", "always_confirm", 1),
        ],
    );

    assert_eq!(result.status, 0, "{}", result.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        json!([])
    );
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_confirm"],
        candidate["agent"]["tool_permissions"]["tools"]["terminal"]["always_confirm"]
    );
}

#[test]
fn promotes_duplicate_decoded_identities_by_exact_source_index() {
    let fixture = Fixture::new();
    let duplicate = pattern("^fx duplicate$", false);
    let baseline = settings(vec![pattern("^fx old-owner$", true), duplicate.clone()], 1);
    let mut candidate = baseline.clone();
    replace_allow_scope(&mut candidate, json!([duplicate.clone(), duplicate]));

    let (result, live) = promote_owner_candidate(
        &fixture,
        "duplicate-source-index",
        &baseline,
        vec![ALLOW_SCOPE],
        vec![json!({"id": "old-owner", "bucket": "always_allow", "index": 0})],
        &candidate,
        vec![
            validation_pattern("retained-duplicate", "always_allow", 0),
            replacement_pattern("replacement-duplicate", "always_allow", 1),
        ],
    );

    assert_eq!(result.status, 0, "{}", result.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        candidate["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
    );
}

#[test]
fn promotes_delete_all_owner_replacement_with_an_empty_catalog() {
    let fixture = Fixture::new();
    let baseline = settings(
        vec![pattern("^fx alpha$", true), pattern("^fx beta$", false)],
        1,
    );
    let captured = capture_selected(
        &fixture,
        "delete-all",
        &baseline,
        vec![ALLOW_SCOPE],
        vec![
            json!({"id": "alpha", "bucket": "always_allow", "index": 0}),
            json!({"id": "beta", "bucket": "always_allow", "index": 1}),
        ],
    );
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([]));
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let (catalog, bundle) = materialize_and_seal(
        &fixture,
        "delete-all",
        &captured.candidate,
        &captured.state,
        vec![],
    );
    assert_eq!(
        catalog_value(catalog.parent().unwrap())["patterns"],
        json!([])
    );
    let live = fixture.write_pretty_json("delete-all-live.json", &baseline);

    let result = run(promote_arguments(&live, &bundle, true));

    assert_eq!(result.status, 0, "{}", result.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        json!([])
    );
}

#[test]
fn requires_exactly_one_bundle_before_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "bundle-option", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let (_, bundle) = materialize_and_seal(
        &fixture,
        "bundle-option",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("alpha", "always_allow", 0)],
    );

    let mut missing = promote_arguments(&live, &bundle, true);
    let position = missing
        .iter()
        .position(|argument| argument == "--bundle")
        .expect("Bundle option must exist");
    missing.drain(position..=position + 1);
    let missing_result = run(missing);
    assert_eq!(missing_result.status, 2);
    assert!(missing_result.stdout.is_empty());
    assert!(
        missing_result
            .stderr
            .contains("Missing required option `--bundle <bundle-path>`")
    );

    let mut duplicate = promote_arguments(&live, &bundle, true);
    duplicate.push(OsString::from("--bundle"));
    duplicate.push(bundle.as_os_str().to_owned());
    let duplicate_result = run(duplicate);
    assert_eq!(duplicate_result.status, 2);
    assert!(duplicate_result.stdout.is_empty());
    assert!(
        duplicate_result
            .stderr
            .contains("Option `--bundle` may be specified only once")
    );
}

#[test]
fn requires_exact_write_guard_before_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "write-guard", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^fx beta$", true)]));
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let original_live = fs::read(&live).unwrap();
    let (_, bundle) = materialize_and_seal(
        &fixture,
        "write-guard",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("beta", "always_allow", 0)],
    );

    let missing = run(promote_arguments(&live, &bundle, false));

    assert_eq!(missing.status, 2);
    assert!(missing.stdout.is_empty());
    assert!(missing.stderr.contains("exact mutation guard `--write`"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    let mut inexact_arguments = promote_arguments(&live, &bundle, false);
    inexact_arguments.push(OsString::from("--write=true"));
    let inexact = run(inexact_arguments);

    assert_eq!(inexact.status, 2);
    assert!(inexact.stderr.contains("Unknown promote option"));
    assert_eq!(fs::read(&live).unwrap(), original_live);
}

#[test]
fn rejects_stale_candidate_and_state_bytes_before_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-alpha$", true)], 1);
    let captured = capture_standard(&fixture, "stale-binding", &baseline, "alpha");
    let live = fixture.write_pretty_json("stale-binding-live.json", &baseline);
    let (_catalog, bundle) = materialize_and_seal(
        &fixture,
        "stale-binding",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("alpha", "always_allow", 0)],
    );
    let original_live = fs::read(&live).unwrap();

    let candidate_value: Value =
        serde_json::from_slice(&fs::read(&captured.candidate).unwrap()).unwrap();
    fs::write(
        &captured.candidate,
        serde_json::to_vec(&candidate_value).expect("Reformatted candidate must serialize"),
    )
    .unwrap();
    let candidate_result = run(promote_arguments(&live, &bundle, true));
    assert_eq!(candidate_result.status, 2);
    assert!(candidate_result.stdout.is_empty());
    assert!(
        candidate_result
            .stderr
            .contains("bound candidate settings does not match its recorded SHA-256")
    );
    assert!(!candidate_result.stderr.contains("private-alpha"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let state_document: Value =
        serde_json::from_slice(&fs::read(&captured.state).unwrap()).unwrap();
    fs::write(
        &captured.state,
        serde_json::to_vec(&state_document).expect("Reformatted state must serialize"),
    )
    .unwrap();
    let state_result = run(promote_arguments(&live, &bundle, true));
    assert_eq!(state_result.status, 2);
    assert!(state_result.stdout.is_empty());
    assert!(
        state_result
            .stderr
            .contains("bound state manifest does not match its recorded SHA-256")
    );
    assert!(!state_result.stderr.contains("private-alpha"));
    assert_eq!(fs::read(&live).unwrap(), original_live);
}

#[test]
fn rejects_cross_candidate_and_invalid_catalog_files_and_artifacts() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-alpha$", true)], 1);
    let captured = capture_standard(&fixture, "catalog-errors", &baseline, "alpha");
    let (catalog, bundle) = materialize_and_seal(
        &fixture,
        "catalog-errors",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("alpha", "always_allow", 0)],
    );
    let live = fixture.write_pretty_json("catalog-errors-live.json", &baseline);
    let original_live = fs::read(&live).unwrap();

    let missing = run(promote_arguments(
        &live,
        &fixture.path("missing-bundle.json"),
        true,
    ));
    assert_eq!(missing.status, 2);
    assert!(missing.stderr.contains("bundle"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    // Catalog schema and binding failures now refuse when the bundle is sealed.
    let mut versioned_catalog = catalog_value(catalog.parent().unwrap());
    versioned_catalog["version"] = json!(2);
    let versioned_catalog_path =
        fixture.write_pretty_json("versioned-catalog.json", &versioned_catalog);
    let versioned_catalog_result = try_seal(
        &fixture,
        "versioned",
        &captured.candidate,
        &captured.state,
        &versioned_catalog_path,
        &[replacement_pattern("alpha", "always_allow", 0)],
    );
    assert_eq!(versioned_catalog_result.status, 2);
    assert!(
        versioned_catalog_result
            .stderr
            .contains("does not match the required schema")
    );
    assert_eq!(fs::read(&live).unwrap(), original_live);

    let mut modified_catalog = catalog_value(catalog.parent().unwrap());
    let private_hash = "0".repeat(64);
    modified_catalog["candidate_sha256"] = json!(private_hash);
    let modified_catalog_path =
        fixture.write_pretty_json("modified-catalog.json", &modified_catalog);
    let modified_catalog_result = try_seal(
        &fixture,
        "modified",
        &captured.candidate,
        &captured.state,
        &modified_catalog_path,
        &[replacement_pattern("alpha", "always_allow", 0)],
    );
    assert_eq!(modified_catalog_result.status, 2);
    assert!(modified_catalog_result.stderr.contains("Candidate SHA-256"));
    assert!(!modified_catalog_result.stderr.contains(&"0".repeat(64)));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    let other_baseline = settings(vec![pattern("^fx private-other$", true)], 1);
    let other = capture_standard(&fixture, "cross-candidate", &other_baseline, "other");
    let cross = try_seal(
        &fixture,
        "cross",
        &other.candidate,
        &captured.state,
        &catalog,
        &[replacement_pattern("alpha", "always_allow", 0)],
    );
    assert_eq!(cross.status, 2);
    assert!(cross.stderr.contains("Candidate SHA-256"));
    assert!(!cross.stderr.contains("private-other"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    let catalog_document = catalog_value(catalog.parent().unwrap());
    let artifact = materialized_pattern_file(catalog.parent().unwrap(), &catalog_document, 0);
    fs::remove_file(&artifact).unwrap();
    let missing_artifact = run(promote_arguments(&live, &bundle, true));
    assert_eq!(missing_artifact.status, 2);
    assert!(missing_artifact.stderr.contains("catalog pattern 1"));
    assert!(!missing_artifact.stderr.contains("private-alpha"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    fs::write(&artifact, b"private-modified-artifact").unwrap();
    let modified = run(promote_arguments(&live, &bundle, true));
    assert_eq!(modified.status, 2);
    assert!(modified.stderr.contains("SHA-256"));
    assert!(!modified.stderr.contains("private-modified-artifact"));
    assert_eq!(fs::read(&live).unwrap(), original_live);
}

#[test]
fn rejects_catalog_source_index_bytes_and_case_setting_mismatches() {
    let fixture = Fixture::new();
    let baseline = settings(
        vec![
            pattern("^fx private-alpha$", true),
            pattern("^fx private-beta$", false),
        ],
        1,
    );
    let captured = capture_standard(&fixture, "source-identity", &baseline, "alpha");
    let (catalog, _bundle) = materialize_and_seal(
        &fixture,
        "source-identity",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("alpha", "always_allow", 0)],
    );
    let original_catalog = catalog_value(catalog.parent().unwrap());
    let artifact = materialized_pattern_file(catalog.parent().unwrap(), &original_catalog, 0);
    let live = fixture.write_pretty_json("source-identity-live.json", &baseline);
    let original_live = fs::read(&live).unwrap();
    let owner = [replacement_pattern("alpha", "always_allow", 0)];

    // Source-identity mismatches now refuse when the bundle is sealed.
    let mut wrong_index = original_catalog.clone();
    wrong_index["patterns"][0]["source_index"] = json!(1);
    let wrong_index_path = catalog.parent().unwrap().join("wrong-index-catalog.json");
    fs::write(
        &wrong_index_path,
        helper::serialize_pretty_json(&wrong_index).unwrap(),
    )
    .unwrap();
    let index_result = try_seal(
        &fixture,
        "wrong-index",
        &captured.candidate,
        &captured.state,
        &wrong_index_path,
        &owner,
    );
    assert_eq!(index_result.status, 2);
    assert!(index_result.stderr.contains("source identity"));
    assert!(!index_result.stderr.contains("private-alpha"));
    assert!(!index_result.stderr.contains("private-beta"));

    let mut wrong_case = original_catalog.clone();
    wrong_case["patterns"][0]["case_sensitive"] = json!(false);
    let wrong_case_path = catalog.parent().unwrap().join("wrong-case-catalog.json");
    fs::write(
        &wrong_case_path,
        helper::serialize_pretty_json(&wrong_case).unwrap(),
    )
    .unwrap();
    let case_result = try_seal(
        &fixture,
        "wrong-case",
        &captured.candidate,
        &captured.state,
        &wrong_case_path,
        &owner,
    );
    assert_eq!(case_result.status, 2);
    assert!(case_result.stderr.contains("source identity"));
    assert!(!case_result.stderr.contains("private-alpha"));

    let cross_candidate_bytes = b"^fx private-cross-candidate-artifact$";
    fs::write(&artifact, cross_candidate_bytes).unwrap();
    let mut wrong_bytes = original_catalog;
    wrong_bytes["patterns"][0]["sha256"] = json!(helper::sha256_hex(cross_candidate_bytes));
    fs::write(
        &catalog,
        helper::serialize_pretty_json(&wrong_bytes).unwrap(),
    )
    .unwrap();
    let bytes_result = try_seal(
        &fixture,
        "wrong-bytes",
        &captured.candidate,
        &captured.state,
        &catalog,
        &owner,
    );
    assert_eq!(bytes_result.status, 2);
    assert!(bytes_result.stderr.contains("source identity"));
    assert!(
        !bytes_result
            .stderr
            .contains("private-cross-candidate-artifact")
    );
    assert_eq!(fs::read(&live).unwrap(), original_live);
}

#[cfg(unix)]
#[test]
fn refuses_symlinked_catalogs_and_artifacts_before_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx private-alpha$", true)], 1);
    let captured = capture_standard(&fixture, "catalog-links", &baseline, "alpha");
    let (catalog, _bundle) = materialize_and_seal(
        &fixture,
        "catalog-links",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("alpha", "always_allow", 0)],
    );
    let live = fixture.write_pretty_json("catalog-links-live.json", &baseline);
    let original_live = fs::read(&live).unwrap();
    let catalog_link = fixture.path("catalog-link.json");
    symlink(&catalog, &catalog_link).unwrap();

    let linked_catalog = try_seal(
        &fixture,
        "catalog-link",
        &captured.candidate,
        &captured.state,
        &catalog_link,
        &[replacement_pattern("alpha", "always_allow", 0)],
    );
    assert_eq!(linked_catalog.status, 2);
    assert!(linked_catalog.stderr.contains("symbolic link"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    let catalog_document = catalog_value(catalog.parent().unwrap());
    let artifact = materialized_pattern_file(catalog.parent().unwrap(), &catalog_document, 0);
    let artifact_target = fixture.write("catalog-artifact-target.regex", b"^fx private-alpha$");
    fs::remove_file(&artifact).unwrap();
    symlink(&artifact_target, &artifact).unwrap();
    let linked_artifact = try_seal(
        &fixture,
        "artifact-link",
        &captured.candidate,
        &captured.state,
        &catalog,
        &[replacement_pattern("alpha", "always_allow", 0)],
    );
    assert_eq!(linked_artifact.status, 2);
    assert!(linked_artifact.stderr.contains("symbolic link"));
    assert!(!linked_artifact.stderr.contains("private-alpha"));
    assert_eq!(fs::read(&live).unwrap(), original_live);
}

#[test]
fn promotes_the_exact_untouched_catalog_accepted_by_matcher_validation() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "matcher-handoff", &baseline, "alpha");
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^fx beta$", true)]));
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).unwrap(),
    )
    .unwrap();
    let (catalog, bundle) = materialize_and_seal(
        &fixture,
        "matcher-handoff",
        &captured.candidate,
        &captured.state,
        vec![replacement_pattern("beta", "always_allow", 0)],
    );
    let relative_catalog = catalog
        .strip_prefix(&fixture.root)
        .unwrap()
        .to_string_lossy();
    let relative_candidate = captured
        .candidate
        .strip_prefix(&fixture.root)
        .unwrap()
        .to_string_lossy();
    let relative_state = captured
        .state
        .strip_prefix(&fixture.root)
        .unwrap()
        .to_string_lossy();
    let suite = fixture.write(
        "matcher-handoff-suite.tsv",
        format!(
            concat!(
                "catalog-pattern\tcandidate\tbeta\n",
                "pattern-case\tbeta\tmatch\tfx beta\n",
                "decision-case\tallow\tfx beta\n",
                "default\tdeny\n",
                "pattern-catalog\tcandidate\t{}\t{}\t{}"
            ),
            relative_catalog, relative_candidate, relative_state
        )
        .as_bytes(),
    );
    let catalog_before = fs::read(&catalog).unwrap();
    let catalog_document = catalog_value(catalog.parent().unwrap());
    let artifact = materialized_pattern_file(catalog.parent().unwrap(), &catalog_document, 0);
    let artifact_before = fs::read(&artifact).unwrap();

    let matcher_result = run_matcher(vec![
        OsString::from("--suite-file"),
        suite.as_os_str().to_owned(),
    ]);
    assert_eq!(matcher_result.status, 0, "{}", matcher_result.stderr);
    assert!(matcher_result.stderr.is_empty());
    assert_eq!(fs::read(&catalog).unwrap(), catalog_before);
    assert_eq!(fs::read(&artifact).unwrap(), artifact_before);

    let live = fixture.write_pretty_json("matcher-handoff-live.json", &baseline);
    let promotion = run(promote_arguments(&live, &bundle, true));
    assert_eq!(promotion.status, 0, "{}", promotion.stderr);
    let promoted: Value = serde_json::from_slice(&fs::read(&live).unwrap()).unwrap();
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        candidate_value["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
    );
}

#[test]
fn promotes_scopes_and_preserves_preexisting_out_of_scope_changes() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "promotion", &baseline, "alpha");
    let mut live_value = baseline.clone();
    live_value["outside"]["generation"] = json!(2);
    live_value["outside"]["concurrent"] = json!("preserved");
    let live = fixture.write_pretty_json("live.json", &live_value);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(
        &mut candidate_value,
        json!([pattern("^fx beta$", true), pattern("^fx gamma$", false)]),
    );
    let candidate = fixture.write_json("candidate.json", &candidate_value);
    let (_catalog, bundle) = materialize_and_seal(
        &fixture,
        "promotion",
        &candidate,
        &captured.state,
        vec![
            replacement_pattern("beta", "always_allow", 0),
            replacement_pattern("gamma", "always_allow", 1),
        ],
    );

    let result = run(promote_arguments(&live, &bundle, true));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stdout.contains("Promoted 1 authorized scope"));
    assert!(!result.stdout.contains("^fx beta$"));
    assert!(!result.stdout.contains("preserved"));
    assert!(result.stderr.is_empty());
    let promoted_bytes = fs::read(&live).unwrap();
    let promoted: Value = serde_json::from_slice(&promoted_bytes).unwrap();
    assert_eq!(promoted["outside"]["generation"], json!(2));
    assert_eq!(promoted["outside"]["concurrent"], json!("preserved"));
    assert_eq!(
        promoted["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"],
        candidate_value["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
    );
    assert_eq!(
        promoted_bytes,
        helper::serialize_pretty_json(&promoted).expect("Promoted JSON must serialize")
    );
    assert!(promoted_bytes.windows(3).any(|window| window == b"\n\t\""));
    assert!(promoted_bytes.ends_with(b"\n"));
    assert!(!promoted_bytes.ends_with(b"\n\n"));
}

#[test]
fn refuses_live_scope_drift_and_candidate_changes_outside_scopes() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "refusal", &baseline, "alpha");
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^fx beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);
    let (_catalog, bundle) = materialize_and_seal(
        &fixture,
        "refusal",
        &candidate,
        &captured.state,
        vec![replacement_pattern("beta", "always_allow", 0)],
    );
    let mut drifted_live = baseline.clone();
    replace_allow_scope(
        &mut drifted_live,
        json!([pattern("^fx concurrent-scope-change$", true)]),
    );
    let live = fixture.write_pretty_json("live.json", &drifted_live);
    let drifted_bytes = fs::read(&live).unwrap();

    let drift_result = run(promote_arguments(&live, &bundle, true));

    assert_eq!(drift_result.status, 1);
    assert!(drift_result.stdout.is_empty());
    assert!(drift_result.stderr.contains("authorized scope 1 drifted"));
    assert!(!drift_result.stderr.contains("concurrent-scope-change"));
    assert_eq!(fs::read(&live).unwrap(), drifted_bytes);

    fs::write(
        &live,
        helper::serialize_pretty_json(&baseline).expect("Baseline must serialize"),
    )
    .unwrap();
    let mut outside_candidate = candidate_value;
    outside_candidate["outside"]["generation"] = json!(99);
    fs::write(
        &candidate,
        helper::serialize_pretty_json(&outside_candidate).unwrap(),
    )
    .unwrap();
    let reset_live = fs::read(&live).unwrap();
    let outside_catalog = write_bound_catalog(
        &fixture,
        "outside-refusal",
        &candidate,
        &captured.state,
        &[replacement_pattern("beta", "always_allow", 0)],
    );

    let outside_bundle = seal_hand_catalog(
        &fixture,
        "outside-refusal",
        &candidate,
        &captured.state,
        &outside_catalog,
        &[replacement_pattern("beta", "always_allow", 0)],
    );
    let outside_result = run(promote_arguments(&live, &outside_bundle, true));

    assert_eq!(outside_result.status, 1);
    assert!(outside_result.stdout.is_empty());
    assert!(outside_result.stderr.contains("outside authorized scopes"));
    assert!(!outside_result.stderr.contains("99"));
    assert_eq!(fs::read(&live).unwrap(), reset_live);
}

#[cfg(unix)]
#[test]
fn leaves_semantic_and_byte_identical_promotion_untouched() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "noop", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let mut reordered = serde_json::Map::new();
    reordered.insert("agent".to_owned(), baseline["agent"].clone());
    reordered.insert("outside".to_owned(), baseline["outside"].clone());
    let candidate_value = Value::Object(reordered);
    assert!(helper::semantic_json_equal(&candidate_value, &baseline));
    let candidate = fixture.write_json("candidate.json", &candidate_value);
    let (_catalog, bundle) = materialize_and_seal(
        &fixture,
        "noop",
        &candidate,
        &captured.state,
        vec![replacement_pattern("alpha", "always_allow", 0)],
    );
    let before = fs::metadata(&live).unwrap();
    let before_bytes = fs::read(&live).unwrap();

    let result = run(promote_arguments(&live, &bundle, true));

    let after = fs::metadata(&live).unwrap();
    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stdout.contains("Live settings unchanged"));
    assert!(result.stderr.is_empty());
    assert_eq!(fs::read(&live).unwrap(), before_bytes);
    assert_eq!(after.ino(), before.ino());
}

#[test]
fn cleans_atomic_siblings_after_handled_failures() {
    let fixture = Fixture::new();
    let directory = fixture.create_dir("atomic");
    let destination = directory.join("settings.json");
    fs::write(&destination, b"original").unwrap();

    let result = helper::atomic_replace_with(&destination, b"replacement", |_| {
        Err(io::Error::other("injected failure"))
    });

    assert!(result.is_err());
    assert_eq!(fs::read(&destination).unwrap(), b"original");
    let entries: Vec<PathBuf> = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(entries, vec![destination]);
}

#[test]
fn best_effort_recheck_refuses_changes_observed_before_rename() {
    let fixture = Fixture::new();
    let directory = fixture.create_dir("atomic-drift");
    let destination = directory.join("settings.json");
    fs::write(&destination, b"original").unwrap();

    let result = helper::atomic_replace_with_best_effort_recheck(
        &destination,
        b"replacement",
        b"original",
        |_| fs::write(&destination, b"concurrent"),
    );

    assert_eq!(result, Err(helper::BestEffortReplaceError::Changed));
    assert_eq!(fs::read(&destination).unwrap(), b"concurrent");
    let entries: Vec<PathBuf> = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(entries, vec![destination]);
}

#[test]
fn preserves_the_uncooperative_writer_race_after_the_best_effort_recheck() {
    let fixture = Fixture::new();
    let directory = fixture.create_dir("atomic-post-recheck-race");
    let destination = directory.join("settings.json");
    fs::write(&destination, b"original").unwrap();

    let result = helper::atomic_replace_with_best_effort_recheck_and_hook(
        &destination,
        b"promoted",
        b"original",
        |_| Ok(()),
        |_| fs::write(&destination, b"uncooperative-writer"),
    );

    assert_eq!(result, Ok(()));
    assert_eq!(fs::read(&destination).unwrap(), b"promoted");
}

#[cfg(unix)]
#[test]
fn preserves_live_permissions_during_atomic_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "permissions", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let mut permissions = fs::metadata(&live).unwrap().permissions();
    permissions.set_mode(0o640);
    fs::set_permissions(&live, permissions).unwrap();
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^fx beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);
    let (_catalog, bundle) = materialize_and_seal(
        &fixture,
        "permissions",
        &candidate,
        &captured.state,
        vec![replacement_pattern("beta", "always_allow", 0)],
    );

    let result = run(promote_arguments(&live, &bundle, true));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert_eq!(
        fs::metadata(&live).unwrap().permissions().mode() & 0o777,
        0o640
    );
}

#[cfg(unix)]
#[test]
fn refuses_symlinked_live_destinations() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^fx alpha$", true)], 1);
    let captured = capture_standard(&fixture, "destination-link", &baseline, "alpha");
    let real_parent = fixture.create_dir("real-live-parent");
    let target = real_parent.join("settings.json");
    fs::write(
        &target,
        helper::serialize_pretty_json(&baseline).expect("Baseline must serialize"),
    )
    .unwrap();
    let linked_parent = fixture.path("linked-live-parent");
    symlink(&real_parent, &linked_parent).expect("Failed to create live parent symlink");
    let linked_live = linked_parent.join("settings.json");
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^fx beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);
    let (_catalog, bundle) = materialize_and_seal(
        &fixture,
        "destination-link",
        &candidate,
        &captured.state,
        vec![replacement_pattern("beta", "always_allow", 0)],
    );
    let original_target = fs::read(&target).unwrap();

    let result = run(promote_arguments(&linked_live, &bundle, true));

    assert_eq!(result.status, 1);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("symbolic link"));
    assert_eq!(fs::read(&target).unwrap(), original_target);
    assert_eq!(fs::read_dir(real_parent).unwrap().count(), 1);
}

fn refresh_arguments(settings: &Path, bundle: &Path, output: &Path) -> Vec<OsString> {
    vec![
        OsString::from("refresh"),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
        OsString::from("--bundle"),
        bundle.as_os_str().to_owned(),
        OsString::from("--output"),
        output.as_os_str().to_owned(),
    ]
}

struct StaleGraph {
    bundle: PathBuf,
    candidate_relative: String,
    catalog_relative: String,
    drifted: Value,
    live: PathBuf,
    owner_spec_relative: String,
    state_relative: String,
}

/// Seal one replacement graph, then drift live settings so every captured position moves.
fn stale_replacement_graph(fixture: &Fixture, prefix: &str) -> StaleGraph {
    let root = fixture.path("");
    // The owner member trails the retained overlap, so drift appended after it moves its index and
    // the reviewed manifest positions can only be resolved through a binding.
    let overlap = pattern("^fx overlap$", false);
    let baseline = settings(vec![overlap.clone(), pattern("^fx old-owner$", true)], 1);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(
        &mut candidate_value,
        json!([overlap.clone(), pattern("^fx new-owner$", true)]),
    );

    let captured = capture_selected(
        fixture,
        prefix,
        &baseline,
        vec![ALLOW_SCOPE],
        vec![json!({"id": "old-owner", "bucket": "always_allow", "index": 1})],
    );
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).expect("Candidate fixture must serialize"),
    )
    .expect("Failed to write candidate fixture");
    let (catalog, bundle) = materialize_and_seal(
        fixture,
        prefix,
        &captured.candidate,
        &captured.state,
        vec![
            validation_pattern("overlap", "always_allow", 0),
            replacement_pattern("new-owner", "always_allow", 1),
        ],
    );

    // An unrelated allow pattern is appended, so the reviewed positions no longer describe reality.
    let drifted = settings(
        vec![
            overlap,
            pattern("^fx old-owner$", true),
            pattern("^fx unrelated$", true),
        ],
        2,
    );
    let live = fixture.write_pretty_json(&format!("{prefix}-live.json"), &drifted);

    StaleGraph {
        candidate_relative: graph_relative(&root, &captured.candidate),
        catalog_relative: graph_relative(&root, &catalog),
        owner_spec_relative: format!("{prefix}-owner-spec.json"),
        state_relative: graph_relative(&root, &captured.state),
        bundle,
        drifted,
        live,
    }
}

/// Re-record fixture evidence for one refreshed plan entry through the artifacts refresh emitted.
fn write_refreshed_evidence(root: &Path, candidate_relative: &str, entry: &Value) {
    let kind = entry["kind"].as_str().expect("Plan kind must be a string");
    let manifest = root.join(
        entry["manifest"]
            .as_str()
            .expect("Plan manifest must be a string"),
    );
    let result_path = root.join(
        entry["result"]
            .as_str()
            .expect("Plan result must be a string"),
    );
    let auxiliary = entry["overlay"].as_str().map(str::to_owned);
    let binds_overlay = matches!(kind, "matcher_suite" | "comparison" | "layer_decision");

    let mut builder =
        patterns::InputClosureBuilder::new(root).expect("Refreshed graph root must be a directory");
    let overlay = binds_overlay
        .then(|| patterns::ResolvedOverlay::load(root).expect("Refreshed overlay must load"));
    let context = patterns::ClosureContext {
        overlay: overlay.as_ref(),
    };
    match kind {
        "matcher_suite" => patterns::resolve_suite_closure(&mut builder, &context, &manifest),
        "comparison" => patterns::resolve_comparison_closure(&mut builder, &context, &manifest),
        "layer_decision" => patterns::resolve_layer_closure(&mut builder, &context, &manifest),
        _ => patterns::resolve_audit_closure(
            &mut builder,
            &manifest,
            &root.join(candidate_relative),
            auxiliary
                .as_ref()
                .map(|relative| root.join(relative))
                .as_deref(),
        ),
    }
    .expect("Refreshed closure must resolve");
    let closure = builder.finish().expect("Refreshed closure must finish");

    let mut bound_inputs = serde_json::Map::new();
    bound_inputs.insert("manifest_sha256".to_owned(), json!(file_sha256(&manifest)));
    if let Some(relative) = &auxiliary {
        bound_inputs.insert(
            "overlay".to_owned(),
            json!({
                "path": relative,
                "sha256": file_sha256(&root.join(relative))
            }),
        );
    }
    bound_inputs.insert(
        "input_closure".to_owned(),
        serde_json::to_value(&closure).expect("Closure must serialize"),
    );

    let result = json!({
        "kind": kind,
        "evaluator": "fixture",
        "outcome": "passed",
        "bound_inputs": Value::Object(bound_inputs),
        "counts": {}
    });
    if let Some(parent) = result_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create refreshed result parent");
    }
    fs::write(
        &result_path,
        serde_json::to_vec(&result).expect("Refreshed result must serialize"),
    )
    .expect("Failed to write refreshed result");
}

fn seal_refreshed_graph(root: &Path, stale: &StaleGraph) -> (Value, PathBuf) {
    let plan: Value = serde_json::from_slice(&fs::read(root.join("validation-plan.json")).unwrap())
        .expect("Validation plan must parse");
    for entry in plan["results"]
        .as_array()
        .expect("Validation plan must declare results")
    {
        write_refreshed_evidence(root, &stale.candidate_relative, entry);
    }

    let bundle = root.join("refreshed-bundle.json");
    let sealed = run(vec![
        OsString::from("seal"),
        OsString::from("--candidate"),
        root.join(&stale.candidate_relative).into_os_string(),
        OsString::from("--state"),
        root.join(&stale.state_relative).into_os_string(),
        OsString::from("--catalog"),
        root.join(&stale.catalog_relative).into_os_string(),
        OsString::from("--owner-spec"),
        root.join(&stale.owner_spec_relative).into_os_string(),
        OsString::from("--validation"),
        root.join("validation-plan.json").into_os_string(),
        OsString::from("--output"),
        bundle.clone().into_os_string(),
    ]);
    assert_eq!(sealed.status, 0, "seal failed: {}", sealed.stderr);

    (plan, bundle)
}

#[test]
fn refreshes_a_stale_graph_into_a_sealable_directory() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "round-trip");
    let root = destination.path("");

    let refreshed = run(refresh_arguments(&stale.live, &stale.bundle, &root));
    assert_eq!(refreshed.status, 0, "{}", refreshed.stderr);
    assert!(refreshed.stdout.contains("unsealed"));

    // Refresh reproduces every reviewed manifest and emits a binding for each audit entry, so the
    // refreshed graph seals without editing any reviewed hash, path, or index by hand.
    let (plan, refreshed_bundle) = seal_refreshed_graph(&root, &stale);
    let entries = plan["results"]
        .as_array()
        .expect("Validation plan must declare results");
    assert!(!entries.is_empty());
    for entry in entries {
        let manifest = entry["manifest"].as_str().unwrap();
        assert!(root.join(manifest).is_file(), "missing manifest {manifest}");
        let auxiliary = entry["overlay"]
            .as_str()
            .unwrap_or_else(|| panic!("entry {} declares no auxiliary artifact", entry["id"]));
        assert!(root.join(auxiliary).is_file(), "missing {auxiliary}");
        match entry["kind"].as_str().unwrap() {
            "owner_audit" | "candidate_inventory" => {
                assert_eq!(auxiliary, format!("{manifest}.binding.json"));
            }
            _ => assert_eq!(auxiliary, "path-overlay.json"),
        }
    }
    assert_eq!(
        fs::read(root.join("round-trip-validation-input.txt")).unwrap(),
        b"fx alpha"
    );
    assert_eq!(
        fs::read(root.join("round-trip-ordinary.regex")).unwrap(),
        b"^fx alpha$"
    );

    let rehearsed = run(vec![
        OsString::from("preflight"),
        OsString::from("--settings"),
        stale.live.as_os_str().to_owned(),
        OsString::from("--bundle"),
        refreshed_bundle.into_os_string(),
    ]);
    assert_eq!(
        rehearsed.status, 0,
        "preflight failed: {}",
        rehearsed.stderr
    );
    let unchanged: Value = serde_json::from_slice(&fs::read(&stale.live).unwrap()).unwrap();
    assert!(helper::semantic_json_equal(&unchanged, &stale.drifted));
}

#[test]
fn refresh_rebinds_moved_audit_positions_without_editing_the_reviewed_manifest() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "rebind");
    let root = destination.path("");

    let refreshed = run(refresh_arguments(&stale.live, &stale.bundle, &root));
    assert_eq!(refreshed.status, 0, "{}", refreshed.stderr);

    let plan: Value =
        serde_json::from_slice(&fs::read(root.join("validation-plan.json")).unwrap()).unwrap();
    let audit = plan["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["kind"] == json!("owner_audit"))
        .expect("The refreshed plan must retain owner-audit evidence");
    let manifest_relative = audit["manifest"].as_str().unwrap();

    let reviewed = fs::read(fixture.path(manifest_relative)).unwrap();
    assert_eq!(
        fs::read(root.join(manifest_relative)).unwrap(),
        reviewed,
        "the reviewed manifest must be reproduced byte-for-byte"
    );

    let binding: Value =
        serde_json::from_slice(&fs::read(root.join(audit["overlay"].as_str().unwrap())).unwrap())
            .unwrap();
    let refreshed_candidate = fs::read(root.join(&stale.candidate_relative)).unwrap();
    assert_eq!(
        binding["settings_sha256"],
        json!(helper::sha256_hex(&refreshed_candidate)),
        "the binding must bind the refreshed candidate"
    );
    let catalog: Value =
        serde_json::from_slice(&fs::read(root.join(&stale.catalog_relative)).unwrap()).unwrap();
    let owner_entry = catalog["patterns"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == json!("new-owner"))
        .expect("The refreshed catalog must retain the owner member");
    let bound = binding["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == json!("new-owner"))
        .expect("The binding must rebind the audited owner member");
    assert_eq!(bound["bucket"], owner_entry["bucket"]);
    assert_eq!(bound["index"], owner_entry["source_index"]);

    let reviewed_manifest: Value = serde_json::from_slice(&reviewed).unwrap();
    let reviewed_entry = reviewed_manifest["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == json!("new-owner"))
        .expect("The reviewed manifest must audit the owner member");
    assert_ne!(
        reviewed_entry["index"], bound["index"],
        "the fixture must move the audited position, or the rebinding proves nothing"
    );
}

#[test]
fn refresh_refuses_an_output_inside_the_reviewed_graph() {
    let fixture = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "nested");
    let nested = fixture.path("nested-refresh-output");

    let result = run(refresh_arguments(&stale.live, &stale.bundle, &nested));

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("outside the reviewed graph root"));
    assert!(!nested.exists());
}

#[test]
fn refresh_rolls_back_every_artifact_when_one_destination_exists() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "rollback");
    let root = destination.path("");
    let occupied = root.join("validation-plan.json");
    fs::write(&occupied, b"occupied").unwrap();

    let result = run(refresh_arguments(&stale.live, &stale.bundle, &root));

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("already exists"));
    assert_eq!(fs::read(&occupied).unwrap(), b"occupied");
    assert_eq!(
        fs::read_dir(&root).unwrap().count(),
        1,
        "a refused refresh must leave no partial graph"
    );
}

#[cfg(unix)]
#[test]
fn refresh_refuses_a_symlinked_artifact_directory() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let escape = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "artifact-link");
    let root = destination.path("");
    let nested = stale
        .state_relative
        .rsplit_once('/')
        .map(|(parent, _)| parent.to_owned())
        .expect("The fixture state manifest must live in a subdirectory");
    symlink(escape.path(""), root.join(&nested)).expect("Failed to create artifact directory link");

    let result = run(refresh_arguments(&stale.live, &stale.bundle, &root));

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("symbolic link"));
    assert_eq!(
        fs::read_dir(escape.path("")).unwrap().count(),
        0,
        "refresh must not write through a symlinked component"
    );
}

#[test]
fn refresh_refuses_parent_directory_aliases_into_the_reviewed_graph() {
    let fixture = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "parent-alias");
    fixture.create_dir("outside");
    let aliased = fixture
        .path("outside")
        .join("..")
        .join("nested-refresh-output");

    let result = run(refresh_arguments(&stale.live, &stale.bundle, &aliased));

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(
        result.stderr.contains("parent-directory"),
        "{}",
        result.stderr
    );
    assert!(!fixture.path("nested-refresh-output").exists());
}

#[cfg(unix)]
#[test]
fn refresh_refuses_a_symlinked_output_ancestor() {
    let fixture = Fixture::new();
    let container = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "output-link");
    let alias = container.path("reviewed-graph-link");
    symlink(fixture.path(""), &alias).expect("Failed to create reviewed graph alias");
    let output = alias.join("nested-refresh-output");

    let result = run(refresh_arguments(&stale.live, &stale.bundle, &output));

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("symbolic link"), "{}", result.stderr);
    assert!(!fixture.path("nested-refresh-output").exists());
}

#[test]
fn refresh_refuses_a_validation_manifest_changed_after_sealing() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "tampered-manifest");
    let bundle: Value = serde_json::from_slice(&fs::read(&stale.bundle).unwrap()).unwrap();
    let manifest = bundle["validation"][0]["manifest"]["path"]
        .as_str()
        .expect("The bundle must bind a validation manifest");
    fs::write(fixture.path(manifest), b"{}").unwrap();

    let result = run(refresh_arguments(
        &stale.live,
        &stale.bundle,
        &destination.path(""),
    ));

    assert_eq!(result.status, 2, "{}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("SHA-256"), "{}", result.stderr);
    assert_eq!(fs::read_dir(destination.path("")).unwrap().count(), 0);
}

#[test]
fn refresh_composes_manifest_bindings_across_repeated_refreshes() {
    let fixture = Fixture::new();
    let first_destination = Fixture::new();
    let second_destination = Fixture::new();
    let stale = stale_replacement_graph(&fixture, "repeat");
    let first_root = first_destination.path("");

    let first = run(refresh_arguments(&stale.live, &stale.bundle, &first_root));
    assert_eq!(first.status, 0, "{}", first.stderr);
    let (_, first_bundle) = seal_refreshed_graph(&first_root, &stale);

    let second_settings = settings(
        vec![
            pattern("^fx overlap$", false),
            pattern("^fx old-owner$", true),
            pattern("^fx unrelated$", true),
            pattern("^fx later$", true),
        ],
        3,
    );
    let second_live = first_destination.write_pretty_json("second-live.json", &second_settings);
    let second_source = StaleGraph {
        bundle: first_bundle,
        candidate_relative: stale.candidate_relative.clone(),
        catalog_relative: stale.catalog_relative.clone(),
        drifted: second_settings,
        live: second_live,
        owner_spec_relative: stale.owner_spec_relative.clone(),
        state_relative: stale.state_relative.clone(),
    };
    let second_root = second_destination.path("");

    let second = run(refresh_arguments(
        &second_source.live,
        &second_source.bundle,
        &second_root,
    ));
    assert_eq!(second.status, 0, "{}", second.stderr);
    let (plan, _) = seal_refreshed_graph(&second_root, &second_source);
    let audit = plan["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["kind"] == json!("owner_audit"))
        .expect("The second refreshed plan must retain owner-audit evidence");
    let binding: Value = serde_json::from_slice(
        &fs::read(second_root.join(audit["overlay"].as_str().unwrap())).unwrap(),
    )
    .unwrap();
    let owner = binding["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == json!("new-owner"))
        .expect("The composed binding must retain the owner entry");
    assert_eq!(owner["index"], json!(3));
}

#[cfg(unix)]
#[test]
fn refresh_rollback_preserves_a_concurrent_replacement() {
    let destination = Fixture::new();
    let root = destination.path("");
    let mut artifacts = BTreeMap::new();
    artifacts.insert("a.txt".to_owned(), b"owned".to_vec());
    artifacts.insert("b.txt".to_owned(), b"later".to_vec());

    let result = helper::commit_refresh_artifacts_with_hook(&root, &artifacts, |index, path| {
        if index == 0 {
            fs::write(path, b"concurrent").unwrap();
            return Err("injected failure".to_owned());
        }
        Ok(())
    });

    assert_eq!(result.unwrap_err(), "injected failure");
    assert_eq!(fs::read(root.join("a.txt")).unwrap(), b"concurrent");
    assert!(!root.join("b.txt").exists());
}

fn allow_sources(settings: &Value) -> Vec<String> {
    settings["agent"]["tool_permissions"]["tools"]["terminal"]["always_allow"]
        .as_array()
        .expect("Refreshed allow patterns must be an array")
        .iter()
        .map(|entry| {
            entry["pattern"]
                .as_str()
                .expect("Refreshed pattern must be a string")
                .to_owned()
        })
        .collect()
}

struct GapGraph {
    bundle: PathBuf,
    candidate_relative: String,
    live: PathBuf,
}

/// Seal one replacement graph whose owner members sit between two retained overlaps, then drift live
/// settings. Refreshed placement can only be resolved through the reviewed gap boundaries, so this
/// fixture exercises the interior-gap branch rather than the start and end sentinels.
fn gap_graph(
    fixture: &Fixture,
    prefix: &str,
    members: &[&str],
    drifted_allow: Vec<Value>,
) -> GapGraph {
    let root = fixture.path("");
    let left = pattern("^fx left$", true);
    let right = pattern("^fx right$", true);
    let baseline = settings(
        vec![left.clone(), pattern("^fx old-owner$", true), right.clone()],
        1,
    );

    let mut candidate_allow = vec![left];
    for member in members {
        candidate_allow.push(pattern(&format!("^fx {member}$"), true));
    }
    candidate_allow.push(right);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, Value::Array(candidate_allow));

    let captured = capture_selected(
        fixture,
        prefix,
        &baseline,
        vec![ALLOW_SCOPE],
        vec![json!({"id": "old-owner", "bucket": "always_allow", "index": 1})],
    );
    fs::write(
        &captured.candidate,
        helper::serialize_pretty_json(&candidate_value).expect("Candidate fixture must serialize"),
    )
    .expect("Failed to write candidate fixture");

    let mut selection = vec![validation_pattern("left", "always_allow", 0)];
    for (offset, member) in members.iter().enumerate() {
        selection.push(replacement_pattern(member, "always_allow", offset + 1));
    }
    selection.push(validation_pattern(
        "right",
        "always_allow",
        members.len() + 1,
    ));

    let (_, bundle) = materialize_and_seal(
        fixture,
        prefix,
        &captured.candidate,
        &captured.state,
        selection,
    );
    let live =
        fixture.write_pretty_json(&format!("{prefix}-live.json"), &settings(drifted_allow, 2));

    GapGraph {
        candidate_relative: graph_relative(&root, &captured.candidate),
        bundle,
        live,
    }
}

#[test]
fn refresh_places_an_owner_member_between_its_relocated_gap_boundaries() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    // Drift inserts unrelated patterns before the left boundary and inside the reviewed gap, so a
    // replay that trusted reviewed indexes would land the member in the wrong place.
    let graph = gap_graph(
        &fixture,
        "interior",
        &["new-owner"],
        vec![
            pattern("^fx extra$", true),
            pattern("^fx left$", true),
            pattern("^fx old-owner$", true),
            pattern("^fx other$", true),
            pattern("^fx right$", true),
        ],
    );
    let root = destination.path("");

    let refreshed = run(refresh_arguments(&graph.live, &graph.bundle, &root));
    assert_eq!(refreshed.status, 0, "{}", refreshed.stderr);

    let candidate: Value =
        serde_json::from_slice(&fs::read(root.join(&graph.candidate_relative)).unwrap()).unwrap();

    assert_eq!(
        allow_sources(&candidate),
        vec![
            "^fx extra$",
            "^fx left$",
            "^fx new-owner$",
            "^fx other$",
            "^fx right$"
        ],
        "the owner member must follow its relocated left boundary"
    );
}

#[test]
fn refresh_keeps_the_reviewed_order_of_members_sharing_one_gap() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let graph = gap_graph(
        &fixture,
        "shared-gap",
        &["new-a", "new-b"],
        vec![
            pattern("^fx left$", true),
            pattern("^fx old-owner$", true),
            pattern("^fx other$", true),
            pattern("^fx right$", true),
        ],
    );
    let root = destination.path("");

    let refreshed = run(refresh_arguments(&graph.live, &graph.bundle, &root));
    assert_eq!(refreshed.status, 0, "{}", refreshed.stderr);

    let candidate: Value =
        serde_json::from_slice(&fs::read(root.join(&graph.candidate_relative)).unwrap()).unwrap();

    assert_eq!(
        allow_sources(&candidate),
        vec![
            "^fx left$",
            "^fx new-a$",
            "^fx new-b$",
            "^fx other$",
            "^fx right$"
        ],
        "members sharing one reviewed gap must retain their reviewed order"
    );
}

#[test]
fn refresh_refuses_a_reviewed_gap_whose_boundaries_reordered() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    // Live settings swap the boundaries, so no placement preserves the reviewed ordering.
    let graph = gap_graph(
        &fixture,
        "reversed",
        &["new-owner"],
        vec![
            pattern("^fx right$", true),
            pattern("^fx old-owner$", true),
            pattern("^fx left$", true),
        ],
    );
    let root = destination.path("");

    let refreshed = run(refresh_arguments(&graph.live, &graph.bundle, &root));

    assert_eq!(refreshed.status, 1, "{}", refreshed.stderr);
    assert!(refreshed.stdout.is_empty());
    assert!(
        refreshed.stderr.contains("reordered across its boundaries"),
        "{}",
        refreshed.stderr
    );
    assert!(!root.join("validation-plan.json").exists());
}

#[test]
fn refresh_reports_outside_owner_remainder_drift() {
    let fixture = Fixture::new();
    let destination = Fixture::new();
    let graph = gap_graph(
        &fixture,
        "drift",
        &["new-owner"],
        vec![
            pattern("^fx left$", true),
            pattern("^fx old-owner$", true),
            pattern("^fx other$", true),
            pattern("^fx right$", true),
        ],
    );
    let root = destination.path("");

    let refreshed = run(refresh_arguments(&graph.live, &graph.bundle, &root));
    assert_eq!(refreshed.status, 0, "{}", refreshed.stderr);

    let report: Value =
        serde_json::from_slice(&fs::read(root.join("refresh-report.json")).unwrap()).unwrap();
    let drift = report["outside_owner_drift"]
        .as_array()
        .expect("The refresh report must record outside-owner drift");

    assert_eq!(drift.len(), 1, "{drift:?}");
    assert_eq!(drift[0], json!("always_allow remainder 2 -> 3"));
}
