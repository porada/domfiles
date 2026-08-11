#[path = "zed-permission-candidate.rs"]
mod helper;

use serde_json::{Value, json};
use std::{
    cell::Cell,
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
            "domfiles-zed-permission-candidate-{}-{timestamp}-{fixture_id}",
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
    output: PathBuf,
    pattern_file: PathBuf,
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

fn verify_arguments(settings: &Path, state: &Path) -> Vec<OsString> {
    vec![
        OsString::from("verify"),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
        OsString::from("--state"),
        state.as_os_str().to_owned(),
    ]
}

fn promote_arguments(
    settings: &Path,
    candidate: &Path,
    state: &Path,
    write: bool,
) -> Vec<OsString> {
    let mut arguments = vec![
        OsString::from("promote"),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
        OsString::from("--candidate"),
        candidate.as_os_str().to_owned(),
        OsString::from("--state"),
        state.as_os_str().to_owned(),
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
                        "always_allow": allow_patterns,
                        "always_confirm": [
                            pattern("^confirm$", true)
                        ],
                        "always_deny": [
                            pattern("^deny$", true)
                        ]
                    }
                }
            }
        }
    })
}

fn selection(id: &str, index: usize) -> Value {
    json!({
        "version": 1,
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

fn capture_standard(fixture: &Fixture, prefix: &str, baseline: &Value, id: &str) -> CaptureResult {
    let settings = fixture.write_pretty_json(&format!("{prefix}-settings.json"), baseline);
    let selection = fixture.write_json(&format!("{prefix}-selection.json"), &selection(id, 0));
    let output = fixture.path(&format!("{prefix}-artifacts"));
    let result = run(capture_arguments(&settings, &selection, &output));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stderr.is_empty());
    let state = output.join("state.json");
    let state_document = state_value(&state);
    let pattern_file = captured_pattern_file(&output, &state_document);

    CaptureResult {
        output,
        pattern_file,
        settings,
        state,
    }
}

fn replace_allow_scope(value: &mut Value, replacement: Value) {
    let tokens = helper::decode_json_pointer(ALLOW_SCOPE).expect("Scope must be a valid pointer");
    helper::replace_pointer_value(value, &tokens, replacement)
        .expect("Allow scope must exist in fixture settings");
}

#[test]
fn documents_all_modes_and_rejects_invalid_arguments() {
    let result = run(vec![OsString::from("--help")]);

    assert_eq!(result.status, 0);
    assert!(result.stderr.is_empty());
    assert!(result.stdout.contains("capture --settings <path>"));
    assert!(result.stdout.contains("verify --settings <path>"));
    assert!(result.stdout.contains("promote --settings <live>"));
    assert!(result.stdout.contains("Selection JSON schema"));
    assert!(result.stdout.contains("State JSON schema"));
    assert!(result.stdout.contains("candidate-settings.json"));
    assert!(result.stdout.contains("does not authenticate itself"));
    assert!(result.stdout.contains("Exit statuses:"));
    assert!(result.stdout.contains("1  Current state"));
    assert!(result.stdout.contains("2  Arguments or data"));

    let misplaced = run(vec![OsString::from("verify"), OsString::from("--help")]);
    assert_eq!(misplaced.status, 2);
    assert!(misplaced.stdout.is_empty());
    assert!(misplaced.stderr.contains("must be used alone"));

    let missing = run(Vec::new());
    assert_eq!(missing.status, 2);
    assert!(missing.stdout.is_empty());
    assert!(missing.stderr.contains("Missing mode"));

    let unknown = run(vec![OsString::from("repair")]);
    assert_eq!(unknown.status, 2);
    assert!(unknown.stderr.contains("Unknown mode"));
}

#[test]
fn rejects_unknown_selection_and_state_fields() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let invalid_selection = json!({
        "version": 1,
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

    let invalid_bucket = json!({
        "version": 1,
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
    let mut state = state_value(&captured.state);
    state["secret-state-field"] = json!(true);
    fs::write(
        &captured.state,
        serde_json::to_vec(&state).expect("State JSON must serialize"),
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
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selection_path = fixture.write_json("selection.json", &selection("alpha", 0));
    let output = fixture.create_dir("artifacts");
    let baseline_artifact = output.join("baseline-settings.json");
    fs::write(&baseline_artifact, b"sentinel").expect("Failed to write existing artifact");

    let result = run(capture_arguments(&settings_path, &selection_path, &output));

    assert_eq!(result.status, 2);
    assert!(result.stderr.contains("Refusing to overwrite"));
    assert_eq!(fs::read(baseline_artifact).unwrap(), b"sentinel");
    assert!(!output.join("candidate-settings.json").exists());
    assert!(!output.join("state.json").exists());
    assert_eq!(fs::read_dir(output).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn sanitizes_unsafe_ids_and_refuses_symlinked_output_components() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
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
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
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
    let baseline = settings(vec![pattern("^alpha$", true), pattern("^beta$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let duplicate_ids = json!({
        "version": 1,
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
        "version": 1,
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
        "version": 1,
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
    let baseline = settings(vec![pattern("^allow$", true)], 1);
    let settings_path = fixture.write_pretty_json("settings.json", &baseline);
    let selection_path = fixture.write_json(
        "selection.json",
        &json!({
            "version": 1,
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
            .indexes(helper::Bucket::Allow, b"^allow-007$", true)
            .is_empty()
    );
    assert_eq!(bucket_reads.map(|reads| reads.get()), [1, 1, 1]);
}

#[test]
fn reindexes_patterns_after_bucket_movement() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "moving", &baseline, "alpha");
    let current = settings(vec![pattern("^new$", true), pattern("^alpha$", true)], 2);
    let current_path = fixture.write_pretty_json("current.json", &current);

    let result = run(verify_arguments(&current_path, &captured.state));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert_eq!(
        result.stdout,
        "alpha -> always_allow[1]\nVerified 1 patterns: 0 unchanged and 1 moved\n"
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
            "version": 1,
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
    let baseline = settings(vec![pattern("^private-pattern$", true)], 1);
    let captured = capture_standard(&fixture, "matches", &baseline, "private");
    let missing = settings(Vec::new(), 1);
    let missing_path = fixture.write_pretty_json("missing.json", &missing);

    let missing_result = run(verify_arguments(&missing_path, &captured.state));

    assert_eq!(missing_result.status, 1);
    assert!(missing_result.stdout.is_empty());
    assert!(missing_result.stderr.contains("1 missing and 0 duplicate"));
    assert!(
        missing_result
            .stderr
            .contains("private -> always_allow[missing]")
    );
    assert!(!missing_result.stderr.contains("^private-pattern$"));

    let duplicate = settings(
        vec![
            pattern("^private-pattern$", true),
            pattern("^private-pattern$", true),
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
            .contains("0 missing and 1 duplicate")
    );
    assert!(
        duplicate_result
            .stderr
            .contains("private -> always_allow[duplicate]")
    );
    assert!(!duplicate_result.stderr.contains("^private-pattern$"));
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
            "version": 1,
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
    assert!(result.stderr.contains("12 missing and 12 duplicate"));
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
    let baseline = settings(vec![pattern("^private-pattern$", true)], 1);
    let captured = capture_standard(&fixture, "hashes", &baseline, "private");
    let baseline_file = captured.output.join("baseline-settings.json");
    let original_baseline = fs::read(&baseline_file).unwrap();
    let mut tampered_baseline = original_baseline.clone();
    tampered_baseline.push(b' ');
    fs::write(&baseline_file, tampered_baseline).unwrap();

    let baseline_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(baseline_result.status, 2);
    assert!(baseline_result.stderr.contains("Baseline artifact SHA-256"));
    assert!(!baseline_result.stderr.contains("^private-pattern$"));

    fs::write(&baseline_file, original_baseline).unwrap();
    fs::write(&captured.pattern_file, b"tampered-private-pattern").unwrap();

    let pattern_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(pattern_result.status, 2);
    assert!(pattern_result.stderr.contains("Pattern artifact SHA-256"));
    assert!(!pattern_result.stderr.contains("tampered-private-pattern"));
    assert!(!pattern_result.stderr.contains("^private-pattern$"));
}

#[test]
fn rejects_invalid_utf8_and_recorded_baseline_source_mismatches() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "identity", &baseline, "alpha");
    let mut state = state_value(&captured.state);
    let invalid_bytes = [0xff];
    fs::write(&captured.pattern_file, invalid_bytes).unwrap();
    state["patterns"][0]["sha256"] = json!(helper::sha256_hex(&invalid_bytes));
    fs::write(&captured.state, serde_json::to_vec(&state).unwrap()).unwrap();

    let utf8_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(utf8_result.status, 2);
    assert!(utf8_result.stderr.contains("not valid UTF-8"));

    fs::write(&captured.pattern_file, b"^alpha$").unwrap();
    state["patterns"][0]["sha256"] = json!(helper::sha256_hex(b"^alpha$"));
    state["patterns"][0]["case_sensitive"] = json!(false);
    fs::write(&captured.state, serde_json::to_vec(&state).unwrap()).unwrap();

    let identity_result = run(verify_arguments(&captured.settings, &captured.state));

    assert_eq!(identity_result.status, 2);
    assert!(identity_result.stderr.contains("source identity"));
    assert!(!identity_result.stderr.contains("^alpha$"));
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
            "version": 1,
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
fn requires_exact_write_guard_before_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "write-guard", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);
    let original_live = fs::read(&live).unwrap();

    let missing = run(promote_arguments(&live, &candidate, &captured.state, false));

    assert_eq!(missing.status, 2);
    assert!(missing.stdout.is_empty());
    assert!(missing.stderr.contains("exact mutation guard `--write`"));
    assert_eq!(fs::read(&live).unwrap(), original_live);

    let mut inexact_arguments = promote_arguments(&live, &candidate, &captured.state, false);
    inexact_arguments.push(OsString::from("--write=true"));
    let inexact = run(inexact_arguments);

    assert_eq!(inexact.status, 2);
    assert!(inexact.stderr.contains("Unknown promote option"));
    assert_eq!(fs::read(&live).unwrap(), original_live);
}

#[test]
fn promotes_scopes_and_preserves_preexisting_out_of_scope_changes() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "promotion", &baseline, "alpha");
    let mut live_value = baseline.clone();
    live_value["outside"]["generation"] = json!(2);
    live_value["outside"]["concurrent"] = json!("preserved");
    let live = fixture.write_pretty_json("live.json", &live_value);
    let mut candidate_value = baseline.clone();
    replace_allow_scope(
        &mut candidate_value,
        json!([pattern("^beta$", true), pattern("^gamma$", false)]),
    );
    let candidate = fixture.write_json("candidate.json", &candidate_value);

    let result = run(promote_arguments(&live, &candidate, &captured.state, true));

    assert_eq!(result.status, 0, "{}", result.stderr);
    assert!(result.stdout.contains("Promoted 1 authorized scopes"));
    assert!(!result.stdout.contains("^beta$"));
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
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "refusal", &baseline, "alpha");
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);
    let mut drifted_live = baseline.clone();
    replace_allow_scope(
        &mut drifted_live,
        json!([pattern("^concurrent-scope-change$", true)]),
    );
    let live = fixture.write_pretty_json("live.json", &drifted_live);
    let drifted_bytes = fs::read(&live).unwrap();

    let drift_result = run(promote_arguments(&live, &candidate, &captured.state, true));

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

    let outside_result = run(promote_arguments(&live, &candidate, &captured.state, true));

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
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "noop", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let mut reordered = serde_json::Map::new();
    reordered.insert("agent".to_owned(), baseline["agent"].clone());
    reordered.insert("outside".to_owned(), baseline["outside"].clone());
    let candidate_value = Value::Object(reordered);
    assert!(helper::semantic_json_equal(&candidate_value, &baseline));
    let candidate = fixture.write_json("candidate.json", &candidate_value);
    let before = fs::metadata(&live).unwrap();
    let before_bytes = fs::read(&live).unwrap();

    let result = run(promote_arguments(&live, &candidate, &captured.state, true));

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

#[cfg(unix)]
#[test]
fn preserves_live_permissions_during_atomic_promotion() {
    let fixture = Fixture::new();
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
    let captured = capture_standard(&fixture, "permissions", &baseline, "alpha");
    let live = fixture.write_pretty_json("live.json", &baseline);
    let mut permissions = fs::metadata(&live).unwrap().permissions();
    permissions.set_mode(0o640);
    fs::set_permissions(&live, permissions).unwrap();
    let mut candidate_value = baseline.clone();
    replace_allow_scope(&mut candidate_value, json!([pattern("^beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);

    let result = run(promote_arguments(&live, &candidate, &captured.state, true));

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
    let baseline = settings(vec![pattern("^alpha$", true)], 1);
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
    replace_allow_scope(&mut candidate_value, json!([pattern("^beta$", true)]));
    let candidate = fixture.write_pretty_json("candidate.json", &candidate_value);
    let original_target = fs::read(&target).unwrap();

    let result = run(promote_arguments(
        &linked_live,
        &candidate,
        &captured.state,
        true,
    ));

    assert_eq!(result.status, 1);
    assert!(result.stdout.is_empty());
    assert!(result.stderr.contains("symbolic link"));
    assert_eq!(fs::read(&target).unwrap(), original_target);
    assert_eq!(fs::read_dir(real_parent).unwrap().count(), 1);
}
