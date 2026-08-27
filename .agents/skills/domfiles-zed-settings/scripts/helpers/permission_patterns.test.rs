#[allow(dead_code)]
#[path = "permission_patterns.rs"]
mod helper;

use helper::{
    ArtifactCatalog, ArtifactCatalogPattern, BoundedIssues, Bucket, CompiledPattern, Decision,
    MatchState, PatternError, compile_pattern, load_artifact_catalog, load_bound_artifact_catalog,
    parse_artifact_catalog, read_utf8_file, regex_error_summary, sha256_hex,
    validate_artifact_catalog, verify_artifact_catalog_binding,
};
use serde_json::json;
use std::{
    env,
    fmt::Debug,
    fs,
    hash::Hash,
    path::PathBuf,
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::{os::unix::fs::symlink, sync::mpsc, thread, time::Duration};

static NEXT_FIXTURE_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let fixture_id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock must be at or after the Unix epoch")
            .as_nanos();
        let temporary_root = fs::canonicalize(env::temp_dir())
            .expect("Temporary directory must resolve to a real directory");
        let root = temporary_root.join(format!(
            "domfiles-permission-patterns-{}-{timestamp}-{fixture_id}",
            process::id()
        ));
        fs::create_dir(&root).expect("Failed to create fixture directory");

        Self { root }
    }

    fn write(&self, name: &str, contents: &[u8]) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, contents).expect("Failed to write fixture file");
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn assert_value_traits<T: Clone + Copy + Debug + Eq + Hash + Ord>() {}

fn pattern(id: &str, bucket: Bucket, source: &str) -> CompiledPattern {
    CompiledPattern {
        id: id.to_owned(),
        bucket,
        regex: compile_pattern(source, true).expect("Test pattern must compile"),
    }
}

fn catalog(
    candidate_bytes: &[u8],
    state_bytes: &[u8],
    pattern_file: &str,
    pattern_bytes: &[u8],
) -> ArtifactCatalog {
    ArtifactCatalog {
        candidate_sha256: sha256_hex(candidate_bytes),
        state_sha256: sha256_hex(state_bytes),
        patterns: vec![ArtifactCatalogPattern {
            id: "logical-id".to_owned(),
            bucket: Bucket::Allow,
            source_index: 7,
            case_sensitive: false,
            sha256: sha256_hex(pattern_bytes),
            pattern_file: pattern_file.to_owned(),
        }],
    }
}

#[test]
fn reads_exact_utf8_without_adding_or_normalizing_newlines() {
    let fixture = Fixture::new();
    let path = fixture.write("exact", b"first\r\nsecond");

    let contents = read_utf8_file(&path, "test input").expect("Fixture must be valid UTF-8");

    assert_eq!(contents.as_bytes(), b"first\r\nsecond");
    assert!(!contents.ends_with('\n'));
}

#[test]
fn rejects_invalid_utf8_without_echoing_file_contents() {
    let fixture = Fixture::new();
    let path = fixture.write("invalid", b"private-prefix\xffprivate-suffix");

    let error = read_utf8_file(&path, "test input").expect_err("Fixture must be invalid UTF-8");

    assert!(error.contains("Invalid UTF-8 in test input file"));
    assert!(!error.contains("private-prefix"));
    assert!(!error.contains("private-suffix"));
}

#[test]
fn parses_validates_and_loads_exact_bound_artifact_catalog_bytes() {
    let fixture = Fixture::new();
    let candidate_bytes = br#"{"candidate":true}"#;
    let state_bytes = br#"{"state":true}"#;
    let pattern_bytes = "^café\\n猫\r\n$".as_bytes();
    let pattern_path = fixture.write("pattern.regex", pattern_bytes);
    let catalog_document = catalog(candidate_bytes, state_bytes, "pattern.regex", pattern_bytes);
    let catalog_bytes = serde_json::to_vec(&catalog_document).expect("Catalog must serialize");
    let catalog_path = fixture.write("artifact-catalog.json", &catalog_bytes);

    let parsed = parse_artifact_catalog(&catalog_bytes).expect("Catalog must parse");
    validate_artifact_catalog(&parsed).expect("Catalog must validate");
    verify_artifact_catalog_binding(&parsed, candidate_bytes, state_bytes)
        .expect("Catalog source binding must validate");
    let loaded = load_bound_artifact_catalog(&catalog_path, candidate_bytes, state_bytes)
        .expect("Bound catalog artifacts must load");

    assert_eq!(loaded.document, catalog_document);
    assert_eq!(loaded.patterns.len(), 1);
    assert_eq!(loaded.patterns[0].definition.source_index, 7);
    assert!(!loaded.patterns[0].definition.case_sensitive);
    assert_eq!(loaded.patterns[0].pattern.as_bytes(), pattern_bytes);
    assert_eq!(fs::read(pattern_path).unwrap(), pattern_bytes);
    assert_ne!(loaded.patterns[0].pattern.as_bytes().last(), Some(&b'\n'));
}

#[test]
fn validates_source_binding_before_catalog_artifacts() {
    let fixture = Fixture::new();
    let candidate_bytes = b"candidate";
    let state_bytes = b"state";
    let pattern_bytes = b"private-pattern-body";
    let document = catalog(candidate_bytes, state_bytes, "missing.regex", pattern_bytes);
    let catalog_path = fixture.write(
        "artifact-catalog.json",
        &serde_json::to_vec(&document).expect("Catalog must serialize"),
    );

    let binding_error = load_bound_artifact_catalog(&catalog_path, b"stale", state_bytes)
        .expect_err("Stale candidate bytes must fail before artifact loading");
    assert!(binding_error.contains("Candidate SHA-256"));
    assert!(!binding_error.contains("missing.regex"));
    assert!(!binding_error.contains("private-pattern-body"));

    let artifact_error = load_bound_artifact_catalog(&catalog_path, candidate_bytes, state_bytes)
        .expect_err("Missing artifacts must fail after binding validation");
    assert!(artifact_error.contains("catalog pattern 1"));
    assert!(!artifact_error.contains("Candidate SHA-256"));
    assert!(!artifact_error.contains("private-pattern-body"));
}

#[test]
fn accepts_empty_artifact_catalogs() {
    let candidate_bytes = b"candidate";
    let state_bytes = b"state";
    let catalog = ArtifactCatalog {
        candidate_sha256: sha256_hex(candidate_bytes),
        state_sha256: sha256_hex(state_bytes),
        patterns: vec![],
    };
    let bytes = serde_json::to_vec(&catalog).expect("Catalog must serialize");

    let parsed = parse_artifact_catalog(&bytes).expect("Empty catalog must parse");
    verify_artifact_catalog_binding(&parsed, candidate_bytes, state_bytes)
        .expect("Empty catalog source binding must validate");

    assert!(parsed.patterns.is_empty());
}

#[test]
fn rejects_artifact_catalog_version_fields_as_unknown() {
    for version in [1, 2] {
        let mut document =
            serde_json::to_value(catalog(b"candidate", b"state", "pattern.regex", b"pattern"))
                .expect("Catalog must convert to JSON");
        document["version"] = json!(version);

        let error = parse_artifact_catalog(&serde_json::to_vec(&document).unwrap())
            .expect_err("Artifact catalog version fields must be rejected");

        assert!(error.contains("does not match the required schema"));
    }
}

#[test]
fn rejects_aggregate_ownership_metadata_on_an_artifact_catalog_pattern() {
    // Ownership belongs to the owner spec, so a catalog carrying the former aggregate field is
    // rejected rather than silently accepted
    let mut document =
        serde_json::to_value(catalog(b"candidate", b"state", "pattern.regex", b"pattern"))
            .expect("Catalog must convert to JSON");
    document["patterns"][0]
        .as_object_mut()
        .expect("Catalog pattern must be an object")
        .insert(
            "owner_replacement".to_owned(),
            json!("private-owner-replacement"),
        );

    let error = parse_artifact_catalog(&serde_json::to_vec(&document).unwrap())
        .expect_err("Aggregate ownership metadata must be rejected");

    assert!(error.contains("does not match the required schema"));
    assert!(!error.contains("private-owner-replacement"));
}

#[test]
fn rejects_malformed_duplicate_and_unbound_artifact_catalogs_without_leaking_bytes() {
    let candidate_bytes = b"private-candidate-body";
    let state_bytes = b"private-state-body";
    let pattern_bytes = b"private-pattern-body";
    let valid = catalog(candidate_bytes, state_bytes, "pattern.regex", pattern_bytes);
    let mut unknown = serde_json::to_value(&valid).expect("Catalog must convert to JSON");
    unknown["private-unknown-field"] = json!(true);
    let unknown_error = parse_artifact_catalog(&serde_json::to_vec(&unknown).unwrap())
        .expect_err("Unknown catalog fields must be rejected");
    assert!(unknown_error.contains("does not match the required schema"));
    assert!(!unknown_error.contains("private-unknown-field"));

    let mut duplicate_ids = valid.clone();
    let mut duplicate = duplicate_ids.patterns[0].clone();
    duplicate.source_index = 8;
    duplicate.pattern_file = "second.regex".to_owned();
    duplicate_ids.patterns.push(duplicate);
    assert!(
        validate_artifact_catalog(&duplicate_ids)
            .expect_err("Duplicate IDs must be rejected")
            .contains("IDs must be unique")
    );

    let mut duplicate_sources = valid.clone();
    let mut duplicate = duplicate_sources.patterns[0].clone();
    duplicate.id = "second".to_owned();
    duplicate.pattern_file = "second.regex".to_owned();
    duplicate_sources.patterns.push(duplicate);
    assert!(
        validate_artifact_catalog(&duplicate_sources)
            .expect_err("Duplicate source locators must be rejected")
            .contains("bucket/source-index pairs must be unique")
    );

    let binding_error = verify_artifact_catalog_binding(&valid, b"changed", state_bytes)
        .expect_err("Changed candidate bytes must be rejected");
    assert!(binding_error.contains("Candidate SHA-256"));
    assert!(!binding_error.contains("private-candidate-body"));
    let state_error = verify_artifact_catalog_binding(&valid, candidate_bytes, b"changed")
        .expect_err("Changed state bytes must be rejected");
    assert!(state_error.contains("State SHA-256"));
    assert!(!state_error.contains("private-state-body"));
}

#[test]
fn rejects_tampered_invalid_utf8_and_unsafe_catalog_artifacts_without_leaking_bytes() {
    let fixture = Fixture::new();
    let candidate_bytes = b"candidate";
    let state_bytes = b"state";
    let original = b"private-original-pattern";
    let catalog_document = catalog(candidate_bytes, state_bytes, "pattern.regex", original);
    let catalog_path = fixture.write(
        "artifact-catalog.json",
        &serde_json::to_vec(&catalog_document).unwrap(),
    );
    fixture.write("pattern.regex", b"private-tampered-pattern");

    let hash_error =
        load_artifact_catalog(&catalog_path).expect_err("Tampered pattern bytes must be rejected");
    assert!(hash_error.contains("SHA-256"));
    assert!(!hash_error.contains("private-tampered-pattern"));
    assert!(!hash_error.contains("private-original-pattern"));

    let invalid_bytes = [0xff];
    fixture.write("pattern.regex", &invalid_bytes);
    let invalid_catalog = catalog(
        candidate_bytes,
        state_bytes,
        "pattern.regex",
        &invalid_bytes,
    );
    fs::write(&catalog_path, serde_json::to_vec(&invalid_catalog).unwrap()).unwrap();
    let utf8_error = load_artifact_catalog(&catalog_path)
        .expect_err("Invalid UTF-8 pattern bytes must be rejected");
    assert!(utf8_error.contains("not valid UTF-8"));

    let traversal = catalog(candidate_bytes, state_bytes, "../pattern.regex", original);
    assert!(
        validate_artifact_catalog(&traversal)
            .expect_err("Traversal must be rejected")
            .contains("must not contain root, parent, or current-directory components")
    );
}

#[cfg(unix)]
#[test]
fn refuses_symlinked_catalog_artifact_paths() {
    let fixture = Fixture::new();
    let pattern_bytes = b"private-symlink-target";
    let target = fixture.write("target.regex", pattern_bytes);
    let link = fixture.root.join("pattern.regex");
    symlink(&target, &link).expect("Failed to create pattern symlink");
    let catalog_document = catalog(b"candidate", b"state", "pattern.regex", pattern_bytes);
    let catalog_path = fixture.write(
        "artifact-catalog.json",
        &serde_json::to_vec(&catalog_document).unwrap(),
    );

    let error = load_artifact_catalog(&catalog_path)
        .expect_err("Symlinked pattern artifacts must be rejected");

    assert!(error.contains("symbolic link"));
    assert!(!error.contains("private-symlink-target"));
}

#[cfg(unix)]
#[test]
fn rejects_fifo_catalogs_without_blocking() {
    let fixture = Fixture::new();
    let fifo_path = fixture.root.join("artifact-catalog.json");
    let status = process::Command::new("mkfifo")
        .arg(&fifo_path)
        .status()
        .expect("Failed to invoke `mkfifo`");
    assert!(status.success(), "Failed to create fixture FIFO");
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        sender
            .send(load_artifact_catalog(&fifo_path))
            .expect("Failed to send special-file validation result");
    });
    let result = receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("Special catalog validation must not block while opening a FIFO");
    handle
        .join()
        .expect("Special-file validation thread must finish");
    let error = result.expect_err("FIFO catalogs must be rejected as non-regular files");

    assert!(error.contains("must be a regular file"));
    assert!(!error.contains("Failed to open"));
}

#[test]
fn classifies_empty_and_invalid_patterns_without_echoing_the_invalid_pattern_body() {
    assert!(matches!(
        compile_pattern("", false),
        Err(PatternError::Empty)
    ));

    let error = match compile_pattern("private-regex-body(", true) {
        Err(PatternError::Invalid(error)) => error,
        _ => panic!("Compiling an invalid test pattern must return `PatternError::Invalid`"),
    };
    let summary = regex_error_summary(&error);

    assert!(summary.contains("unclosed group"));
    assert_eq!(summary.lines().count(), 1);
    assert!(!summary.contains("private-regex-body"));
}

#[test]
fn honors_case_sensitivity_and_preserves_extended_mode_without_wrapping() {
    let case_insensitive =
        compile_pattern("^lowercase$", false).expect("Case-insensitive pattern must compile");
    let case_sensitive =
        compile_pattern("^lowercase$", true).expect("Case-sensitive pattern must compile");
    let extended = compile_pattern("(?x)word # comment", true)
        .expect("Unwrapped extended-mode pattern must compile");

    assert!(case_insensitive.is_match("LOWERCASE"));
    assert!(!case_sensitive.is_match("LOWERCASE"));
    assert!(case_sensitive.is_match("lowercase"));
    assert!(extended.is_match("word"));
}

#[test]
fn parses_and_labels_every_bucket() {
    assert_value_traits::<Bucket>();

    for (label, bucket) in [
        ("always_allow", Bucket::Allow),
        ("always_confirm", Bucket::Confirm),
        ("always_deny", Bucket::Deny),
    ] {
        assert_eq!(Bucket::parse(label), Some(bucket));
        assert_eq!(bucket.label(), label);
    }

    assert_eq!(Bucket::parse("allow"), None);
    assert_eq!(Bucket::parse("ALWAYS_ALLOW"), None);
}

#[test]
fn parses_and_labels_decisions_and_selects_the_most_restrictive_for_every_pair() {
    assert_value_traits::<Decision>();

    for (label, decision) in [
        ("allow", Decision::Allow),
        ("confirm", Decision::Confirm),
        ("deny", Decision::Deny),
    ] {
        assert_eq!(Decision::parse(label), Some(decision));
        assert_eq!(decision.label(), label);
    }

    assert_eq!(Decision::parse("always_allow"), None);
    assert_eq!(Decision::parse("ALLOW"), None);

    for (left, right, expected) in [
        (Decision::Allow, Decision::Allow, Decision::Allow),
        (Decision::Allow, Decision::Confirm, Decision::Confirm),
        (Decision::Allow, Decision::Deny, Decision::Deny),
        (Decision::Confirm, Decision::Allow, Decision::Confirm),
        (Decision::Confirm, Decision::Confirm, Decision::Confirm),
        (Decision::Confirm, Decision::Deny, Decision::Deny),
        (Decision::Deny, Decision::Allow, Decision::Deny),
        (Decision::Deny, Decision::Confirm, Decision::Deny),
        (Decision::Deny, Decision::Deny, Decision::Deny),
    ] {
        assert_eq!(left.most_restrictive(right), expected);
    }
}

#[test]
fn resolves_every_match_state_with_precedence_and_default_fallback() {
    for allow in [false, true] {
        for confirm in [false, true] {
            for deny in [false, true] {
                let state = MatchState {
                    allow,
                    confirm,
                    deny,
                };

                for default in [Decision::Allow, Decision::Confirm, Decision::Deny] {
                    let expected = if deny {
                        Decision::Deny
                    } else if confirm {
                        Decision::Confirm
                    } else if allow {
                        Decision::Allow
                    } else {
                        default
                    };

                    assert_eq!(state.decision(default), expected);
                }
            }
        }
    }
}

#[test]
fn evaluates_and_exposes_matches_for_every_bucket() {
    let patterns = [
        pattern("allow", Bucket::Allow, "^(?:all|allow-confirm)$"),
        pattern("confirm", Bucket::Confirm, "^(?:all|allow-confirm)$"),
        pattern("deny", Bucket::Deny, "^all$"),
    ];

    let partial = MatchState::evaluate("allow-confirm", &patterns);
    let all = MatchState::evaluate("all", &patterns);
    let none = MatchState::evaluate("none", &patterns);

    assert_eq!(patterns[0].id, "allow");
    assert!(partial.matched(Bucket::Allow));
    assert!(partial.matched(Bucket::Confirm));
    assert!(!partial.matched(Bucket::Deny));
    assert_eq!(partial.decision(Decision::Deny), Decision::Confirm);

    assert!(all.matched(Bucket::Allow));
    assert!(all.matched(Bucket::Confirm));
    assert!(all.matched(Bucket::Deny));
    assert_eq!(all.decision(Decision::Allow), Decision::Deny);

    assert!(!none.matched(Bucket::Allow));
    assert!(!none.matched(Bucket::Confirm));
    assert!(!none.matched(Bucket::Deny));
    assert_eq!(none.decision(Decision::Confirm), Decision::Confirm);
}

struct NonCloneIssue(&'static str);

#[test]
fn stores_the_first_issues_up_to_the_limit_and_tracks_total_and_omitted_counts() {
    let mut issues = BoundedIssues::new(2);

    issues.push(NonCloneIssue("first"));
    issues.push(NonCloneIssue("second"));
    issues.push(NonCloneIssue("third"));
    issues.push(NonCloneIssue("fourth"));

    let stored: Vec<&str> = issues.issues().iter().map(|issue| issue.0).collect();

    assert_eq!(stored, ["first", "second"]);
    assert_eq!(issues.total_count(), 4);
    assert_eq!(issues.omitted_count(), 2);
}

#[test]
fn tracks_all_issues_as_omitted_when_the_limit_is_zero() {
    let mut issues = BoundedIssues::new(0);

    issues.push(NonCloneIssue("first"));
    issues.push(NonCloneIssue("second"));

    assert!(issues.issues().is_empty());
    assert_eq!(issues.total_count(), 2);
    assert_eq!(issues.omitted_count(), 2);
}

fn suite_closure_paths(root: &std::path::Path, manifest: &std::path::Path) -> Vec<String> {
    let mut builder =
        helper::InputClosureBuilder::new(root).expect("Graph root must be a directory");
    let context = helper::ClosureContext { overlay: None };
    helper::resolve_suite_closure(&mut builder, &context, manifest)
        .expect("Suite closure must resolve");

    builder
        .finish()
        .expect("Suite closure must finish")
        .records
        .into_iter()
        .map(|record| record.path)
        .collect()
}

#[test]
fn records_suite_inputs_whose_paths_contain_the_field_separator() {
    let fixture = Fixture::new();
    // The suite evaluator reads a case input with a bounded `splitn`, so a tab inside the path stays
    // part of the path. The closure must record the same file rather than dropping the record
    let input = fixture.write("case\tone.txt", b"fx alpha");
    let pattern_file = fixture.write("pattern.regex", b"^fx alpha$");
    let manifest = fixture.write(
        "suite.tsv",
        concat!(
            "default\tconfirm\n",
            "pattern\tp1\talways_allow\tcase-sensitive\tpattern.regex\n",
            "pattern-case-file\tp1\tmatch\tcase\tone.txt\n",
            "decision-case-file\tallow\tcase\tone.txt\n"
        )
        .as_bytes(),
    );
    assert!(input.is_file() && pattern_file.is_file());

    let recorded = suite_closure_paths(&fixture.root, &manifest);

    assert!(
        recorded.contains(&"case\tone.txt".to_owned()),
        "{recorded:?}"
    );
    assert!(
        recorded.contains(&"pattern.regex".to_owned()),
        "{recorded:?}"
    );
    assert!(recorded.contains(&"suite.tsv".to_owned()), "{recorded:?}");
}

#[test]
fn refuses_a_suite_manifest_record_the_closure_cannot_represent() {
    let fixture = Fixture::new();
    let manifest = fixture.write(
        "suite.tsv",
        b"default\tconfirm\nunsupported-record\tp1\tsomewhere.txt\n",
    );

    let mut builder =
        helper::InputClosureBuilder::new(&fixture.root).expect("Graph root must be a directory");
    let context = helper::ClosureContext { overlay: None };
    let error = helper::resolve_suite_closure(&mut builder, &context, &manifest)
        .expect_err("An unrepresentable record must refuse");

    assert!(error.contains("unsupported record type"), "{error}");
}

fn transformation(
    prefix: &str,
    baseline_middle: &str,
    candidate_middle: &str,
    suffix: &str,
) -> helper::VisibilityTransformation {
    helper::VisibilityTransformation {
        prefix: prefix.to_owned(),
        baseline_middle: baseline_middle.to_owned(),
        candidate_middle: candidate_middle.to_owned(),
        suffix: suffix.to_owned(),
    }
}

#[test]
fn a_visibility_rewrite_accepts_a_reordered_literal_alternation() {
    let transformation = transformation("^git ", "(?:add|rm)", "(?:rm|add)", "$");

    helper::verify_visibility_transformation(
        &transformation,
        "^git (?:add|rm)$",
        "^git (?:rm|add)$",
    )
    .expect("An equal literal expansion must be accepted");
}

#[test]
fn a_visibility_rewrite_accepts_an_equivalent_optional_group() {
    let transformation = transformation("^p ", "a?b", "(?:a)?b", "$");

    helper::verify_visibility_transformation(&transformation, "^p a?b$", "^p (?:a)?b$")
        .expect("Optional literals and optional groups must expand alike");
}

#[test]
fn a_visibility_rewrite_refuses_a_different_literal_expansion() {
    let transformation = transformation("^git ", "(?:add|rm)", "(?:add|mv)", "$");

    let error = helper::verify_visibility_transformation(
        &transformation,
        "^git (?:add|rm)$",
        "^git (?:add|mv)$",
    )
    .expect_err("A changed literal set must refuse");

    assert!(
        error.contains("expand to different literal sets"),
        "{error}"
    );
}

#[test]
fn a_visibility_rewrite_refuses_bare_alternation_in_a_middle() {
    // `^x a|b$` is not equivalent to `^x (?:a|b)$`, so an ungrouped `|` must never expand
    let transformation = transformation("^x ", "(?:a|b)", "a|b", "$");

    let error = helper::verify_visibility_transformation(&transformation, "^x (?:a|b)$", "^x a|b$")
        .expect_err("Bare alternation must refuse");

    assert!(error.contains("accepts only literals"), "{error}");
}

#[test]
fn a_visibility_rewrite_refuses_a_suffix_that_quantifies_the_middle() {
    // `ab*` quantifies only `b`, while `(?:ab)*` quantifies the whole middle
    let transformation = transformation("", "ab", "(?:ab)", "*");

    let error = helper::verify_visibility_transformation(&transformation, "ab*", "(?:ab)*")
        .expect_err("A quantifier bound to the middle must refuse");

    assert!(error.contains("quantifier that would bind"), "{error}");
}

#[test]
fn a_visibility_rewrite_refuses_a_boundary_inside_a_character_class() {
    let transformation = transformation("[a", "b", "(?:b)", "]c");

    let error = helper::verify_visibility_transformation(&transformation, "[ab]c", "[a(?:b)]c")
        .expect_err("A split inside a character class must refuse");

    assert!(error.contains("supported split point"), "{error}");
}

#[test]
fn a_visibility_rewrite_refuses_a_boundary_inside_an_escape_pair() {
    let transformation = transformation("a\\", "d", "(?:d)", "b");

    let error = helper::verify_visibility_transformation(&transformation, "a\\db", "a\\(?:d)b")
        .expect_err("A split between an escape and its escapee must refuse");

    assert!(error.contains("supported split point"), "{error}");
}

#[test]
fn a_visibility_rewrite_refuses_affixes_that_do_not_reconstruct_a_member() {
    let transformation = transformation("^x ", "a", "(?:a)", "$");

    let error = helper::verify_visibility_transformation(&transformation, "^y a$", "^x (?:a)$")
        .expect_err("Affixes that do not rebuild the baseline must refuse");

    assert!(error.contains("reconstruct the baseline member"), "{error}");
}

#[test]
fn a_visibility_rewrite_refuses_a_nested_group() {
    let transformation = transformation("", "(?:a(?:b))", "(?:ab)", "");

    let error = helper::verify_visibility_transformation(&transformation, "(?:a(?:b))", "(?:ab)")
        .expect_err("A nested group must refuse rather than be approximated");

    assert!(error.contains("Nested transformation groups"), "{error}");
}

#[test]
fn a_visibility_rewrite_refuses_an_unbounded_literal_expansion() {
    let middle = "(?:a|b)".repeat(9);
    let transformation = transformation("", &middle, &middle, "");

    let error = helper::verify_visibility_transformation(&transformation, &middle, &middle)
        .expect_err("An expansion beyond the bound must refuse");

    assert!(error.contains("more than"), "{error}");
}

#[test]
fn leading_assignments_stay_transparent_to_role_inference() {
    for witness in [
        "rg --no-config pattern",
        "RIPGREP_CONFIG_PATH=/dev/null rg --no-config pattern",
        "A=1 B=2 C=3 rg --no-config pattern",
    ] {
        let inferred =
            helper::infer_owner_role(witness, &[]).expect("Witness must resolve an owner");

        assert_eq!(inferred.owner, "rg", "{witness}");
        assert_eq!(
            inferred.role,
            helper::Role::Direct,
            "Assignment prefixes must not change the inferred role: {witness}"
        );
    }
}

#[test]
fn recognized_wrappers_keep_the_wrapped_role_behind_assignments() {
    for witness in [
        "nohup rg --no-config pattern",
        "A=1 nohup rg --no-config pattern",
        "xargs rg --no-config pattern",
        "A=1 xargs rg --no-config pattern",
    ] {
        let inferred =
            helper::infer_owner_role(witness, &[]).expect("Witness must resolve an owner");

        assert_eq!(inferred.owner, "rg", "{witness}");
        assert_eq!(
            inferred.role,
            helper::Role::Wrapped,
            "A recognized wrapper must keep the wrapped role: {witness}"
        );
    }
}

#[test]
fn witness_owner_inference_reports_the_shared_role() {
    let direct = helper::infer_witness_owner("A=1 rg --no-config pattern")
        .expect("Witness must resolve an owner");
    assert_eq!(direct.owner, "rg");
    assert_eq!(direct.inventory_owner, "rg");
    assert_eq!(direct.role, helper::Role::Direct);

    let wrapped = helper::infer_witness_owner("A=1 nohup rg --no-config pattern")
        .expect("Witness must resolve an owner");
    assert_eq!(wrapped.owner, "rg");
    assert_eq!(wrapped.role, helper::Role::Wrapped);
}
