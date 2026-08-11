#[path = "permission-patterns.rs"]
mod helper;

use helper::{
    BoundedIssues, Bucket, CompiledPattern, Decision, MatchState, PatternError, compile_pattern,
    read_utf8_file, regex_error_summary,
};
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
        let root = env::temp_dir().join(format!(
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
fn rejects_empty_and_invalid_patterns_without_echoing_pattern_bodies() {
    assert!(matches!(
        compile_pattern("", false),
        Err(PatternError::Empty)
    ));

    let error = match compile_pattern("private-regex-body(", true) {
        Err(PatternError::Invalid(error)) => error,
        _ => panic!("Invalid test pattern must return PatternError::Invalid"),
    };
    let summary = regex_error_summary(&error);

    assert!(summary.contains("unclosed group"));
    assert_eq!(summary.lines().count(), 1);
    assert!(!summary.contains("private-regex-body"));
}

#[test]
fn applies_case_settings_and_preserves_unwrapped_regex_semantics() {
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
fn parses_labels_and_orders_every_decision_pair() {
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
fn resolves_every_match_precedence_combination_before_the_default() {
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
fn stores_only_the_first_bounded_issues_and_tracks_counts() {
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
fn tracks_all_issues_as_omitted_at_zero_capacity() {
    let mut issues = BoundedIssues::new(0);

    issues.push(NonCloneIssue("first"));
    issues.push(NonCloneIssue("second"));

    assert!(issues.issues().is_empty());
    assert_eq!(issues.total_count(), 2);
    assert_eq!(issues.omitted_count(), 2);
}
