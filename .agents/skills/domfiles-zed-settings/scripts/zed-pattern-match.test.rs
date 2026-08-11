#[path = "zed-pattern-match.rs"]
mod helper;

use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_FIXTURE_ID: AtomicUsize = AtomicUsize::new(0);

struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("write failed"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

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
            "domfiles-zed-pattern-match-{}-{timestamp}-{fixture_id}",
            process::id()
        ));
        fs::create_dir(&root).expect("Failed to create fixture directory");

        Self { root }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn write(&self, name: &str, contents: &[u8]) -> PathBuf {
        let path = self.path(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create fixture parent directory");
        }
        fs::write(&path, contents).expect("Failed to write fixture file");
        path
    }

    fn write_json(&self, name: &str, value: &serde_json::Value) -> PathBuf {
        let contents = serde_json::to_vec(value).expect("Test JSON must serialize");

        self.write(name, &contents)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn comparison_case_file(input_file: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "file",
        "input_file": input_file,
    })
}

fn comparison_case_inline(input: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "inline",
        "input": input,
    })
}

fn comparison_manifest(
    baseline_default: &str,
    baseline_patterns: Vec<serde_json::Value>,
    candidate_default: &str,
    candidate_patterns: Vec<serde_json::Value>,
    cases: Vec<serde_json::Value>,
) -> serde_json::Value {
    serde_json::json!({
        "version": 1,
        "baseline": {
            "default": baseline_default,
            "patterns": baseline_patterns,
        },
        "candidate": {
            "default": candidate_default,
            "patterns": candidate_patterns,
        },
        "cases": cases,
    })
}

fn comparison_pattern(
    id: &str,
    bucket: &str,
    case_sensitive: bool,
    pattern_file: &str,
) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "bucket": bucket,
        "case_sensitive": case_sensitive,
        "pattern_file": pattern_file,
    })
}

fn run_with_cases(
    cases_file: &Path,
    pattern_file: &Path,
    case_sensitive: bool,
) -> (u8, String, String) {
    run_with_source("--cases-file", cases_file, pattern_file, case_sensitive)
}

fn run_with_comparison(comparison_file: &Path) -> (u8, String, String) {
    let arguments = [
        OsString::from("--comparison-file"),
        comparison_file.as_os_str().to_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);

    (
        status,
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    )
}

fn run_with_files(
    input_file: &Path,
    pattern_file: &Path,
    case_sensitive: bool,
) -> (u8, String, String) {
    run_with_source("--input-file", input_file, pattern_file, case_sensitive)
}

fn run_with_suite(suite_file: &Path) -> (u8, String, String) {
    let arguments = [
        OsString::from("--suite-file"),
        suite_file.as_os_str().to_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);

    (
        status,
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    )
}

fn run_with_source(
    source_option: &str,
    source_file: &Path,
    pattern_file: &Path,
    case_sensitive: bool,
) -> (u8, String, String) {
    let mut arguments = Vec::new();
    if case_sensitive {
        arguments.push(OsString::from("--case-sensitive"));
    }
    arguments.extend([
        OsString::from(source_option),
        source_file.as_os_str().to_owned(),
        OsString::from("--pattern-file"),
        pattern_file.as_os_str().to_owned(),
    ]);

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(arguments, &mut stdout, &mut stderr);

    (
        status,
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    )
}

#[test]
fn does_not_match_across_line_break() {
    let regex = helper::compile_pattern("^foo$", true).expect("Test pattern must compile");

    assert!(!regex.is_match("foo\nbar"));
}

#[test]
fn matches_empty_input_with_empty_anchor() {
    let regex = helper::compile_pattern("^$", true).expect("Test pattern must compile");

    assert!(regex.is_match(""));
}

#[test]
fn rejects_pattern_that_can_match_invalid_utf8_without_echoing_it() {
    let fixture = Fixture::new();
    let input_file = fixture.write("input", b"foo");
    let invalid_pattern = br"(?-u:\xC3)";
    let pattern_file = fixture.write("pattern", invalid_pattern);

    let (status, stdout, stderr) = run_with_files(&input_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid regex in pattern file"));
    assert!(stderr.contains("invalid UTF-8"));
    assert!(!stderr.contains("(?-u"));
}

#[test]
fn matches_extended_mode_comment_pattern_without_wrapper() {
    let regex = helper::compile_pattern("(?x)foo # comment", true)
        .expect("Unwrapped test pattern must compile");

    assert!(regex.is_match("foo"));
}

#[test]
fn matches_nul_input() {
    let regex = helper::compile_pattern(r"\x00", true).expect("Test pattern must compile");

    assert!(regex.is_match("before\0after"));
}

#[test]
fn matches_case_insensitively_by_default() {
    let fixture = Fixture::new();
    let input_file = fixture.write("input", b"FOO");
    let pattern_file = fixture.write("pattern", b"^foo$");

    let (status, stdout, stderr) = run_with_files(&input_file, &pattern_file, false);

    assert_eq!(status, 0);
    assert!(stdout.is_empty());
    assert!(stderr.is_empty());
}

#[test]
fn rejects_empty_case_manifest() {
    let fixture = Fixture::new();
    let cases_file = fixture.write("cases", b"");
    let pattern_file = fixture.write("pattern", b"^foo$");

    let (status, stdout, stderr) = run_with_cases(&cases_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Case manifest file"));
    assert!(stderr.contains("is empty"));
}

#[test]
fn rejects_input_and_case_manifest_together() {
    let fixture = Fixture::new();
    let cases_file = fixture.write("cases", b"match\tfoo");
    let input_file = fixture.write("input", b"foo");
    let pattern_file = fixture.write("pattern", b"^foo$");
    let arguments = [
        OsString::from("--cases-file"),
        cases_file.as_os_str().to_owned(),
        OsString::from("--input-file"),
        input_file.as_os_str().to_owned(),
        OsString::from("--pattern-file"),
        pattern_file.as_os_str().to_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(arguments, &mut stdout, &mut stderr);
    let stderr = String::from_utf8(stderr).expect("Standard error must be valid UTF-8");

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("are mutually exclusive"));
}

#[test]
fn rejects_invalid_case_manifest_line() {
    let fixture = Fixture::new();
    let cases_file = fixture.write("cases", b"maybe\tfoo");
    let pattern_file = fixture.write("pattern", b"^foo$");

    let (status, stdout, stderr) = run_with_cases(&cases_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid case manifest"));
    assert!(stderr.contains("at line 1"));
    assert!(stderr.contains("match<TAB><input>"));
}

#[test]
fn rejects_carriage_returns_in_case_manifest() {
    let fixture = Fixture::new();
    let cases_file = fixture.write("cases", b"match\tfoo\r\n");
    let pattern_file = fixture.write("pattern", b"^foo$");

    let (status, stdout, stderr) = run_with_cases(&cases_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid case manifest"));
    assert!(stderr.contains("at line 1"));
}

#[test]
fn limits_batch_failure_output_without_inputs() {
    let fixture = Fixture::new();
    let cases = (1..=12)
        .map(|index| format!("match\tprivate-input-{index}"))
        .collect::<Vec<_>>()
        .join("\n");
    let cases_file = fixture.write("cases", cases.as_bytes());
    let pattern_file = fixture.write("pattern", b"^foo$");

    let (status, stdout, stderr) = run_with_cases(&cases_file, &pattern_file, false);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("12 of 12 cases failed"));
    assert!(stderr.contains("Line 1 expected a match"));
    assert!(stderr.contains("Line 10 expected a match"));
    assert!(!stderr.contains("Line 11 expected a match"));
    assert!(stderr.contains("… 2 additional failures omitted"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn verifies_batch_cases() {
    let fixture = Fixture::new();
    let cases_file = fixture.write("cases", b"match\tfoo\nmatch\t\nno-match\tbar");
    let pattern_file = fixture.write("pattern", b"^(?:foo)?$");

    let (status, stdout, stderr) = run_with_cases(&cases_file, &pattern_file, true);

    assert_eq!(status, 0);
    assert_eq!(stdout, "Verified 3 cases\n");
    assert!(stderr.is_empty());
}

#[test]
fn preserves_case_when_case_sensitive() {
    let fixture = Fixture::new();
    let uppercase_input_file = fixture.write("uppercase-input", b"FOO");
    let lowercase_input_file = fixture.write("lowercase-input", b"foo");
    let pattern_file = fixture.write("pattern", b"^foo$");

    let (uppercase_status, uppercase_stdout, uppercase_stderr) =
        run_with_files(&uppercase_input_file, &pattern_file, true);
    let (lowercase_status, lowercase_stdout, lowercase_stderr) =
        run_with_files(&lowercase_input_file, &pattern_file, true);

    assert_eq!(uppercase_status, 1);
    assert!(uppercase_stdout.is_empty());
    assert!(uppercase_stderr.is_empty());
    assert_eq!(lowercase_status, 0);
    assert!(lowercase_stdout.is_empty());
    assert!(lowercase_stderr.is_empty());
}

#[test]
fn returns_error_when_batch_failure_output_cannot_be_written() {
    let fixture = Fixture::new();
    let cases_file = fixture.write("cases", b"match\tbar");
    let pattern_file = fixture.write("pattern", b"^foo$");
    let arguments = [
        OsString::from("--cases-file"),
        cases_file.as_os_str().to_owned(),
        OsString::from("--pattern-file"),
        pattern_file.as_os_str().to_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = FailingWriter;

    let status = helper::run(arguments, &mut stdout, &mut stderr);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
}

#[test]
fn returns_error_for_empty_pattern() {
    let fixture = Fixture::new();
    let input_file = fixture.write("input", b"foo");
    let pattern_file = fixture.write("pattern", b"");

    let (status, stdout, stderr) = run_with_files(&input_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Pattern file"));
    assert!(stderr.contains("is empty"));
}

#[test]
fn returns_error_for_invalid_utf8_input() {
    let fixture = Fixture::new();
    let input_file = fixture.write("input", &[0xff]);
    let pattern_file = fixture.write("pattern", b"foo");

    let (status, stdout, stderr) = run_with_files(&input_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid UTF-8 in input file"));
}

#[test]
fn returns_error_when_required_arguments_are_missing() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(Vec::<OsString>::new(), &mut stdout, &mut stderr);

    let stderr = String::from_utf8(stderr).expect("Standard error must be valid UTF-8");

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("--cases-file"));
    assert!(stderr.contains("--input-file"));
}

#[test]
fn returns_error_when_input_cannot_be_read() {
    let fixture = Fixture::new();
    let input_file = fixture.path("missing-input");
    let pattern_file = fixture.write("pattern", b"foo");

    let (status, stdout, stderr) = run_with_files(&input_file, &pattern_file, false);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Failed to read input file"));
}

#[test]
fn verifies_suite_case_settings_and_permission_precedence() {
    let fixture = Fixture::new();
    fixture.write("allow-pattern", b"^(?:ALLOW|REVIEW|BLOCK)$");
    fixture.write("confirm-pattern", b"^(?:review|block)$");
    fixture.write("deny-pattern", b"^BLOCK$");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "pattern-case\tallow-sensitive\tmatch\tALLOW\n",
            "pattern-case\tallow-sensitive\tno-match\tallow\n",
            "pattern-case\tconfirm-insensitive\tmatch\tREVIEW\n",
            "pattern-case\tdeny-sensitive\tmatch\tBLOCK\n",
            "decision-case\tallow\tALLOW\n",
            "decision-case\tconfirm\tREVIEW\n",
            "decision-case\tdeny\tBLOCK\n",
            "decision-case\tdeny\tother\n",
            "pattern\tallow-sensitive\talways_allow\tcase-sensitive\tallow-pattern\n",
            "pattern\tconfirm-insensitive\talways_confirm\tcase-insensitive\tconfirm-pattern\n",
            "pattern\tdeny-sensitive\talways_deny\tcase-sensitive\tdeny-pattern\n",
            "default\tdeny",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified 4 pattern cases and 4 permission decisions across 3 patterns\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn resolves_suite_pattern_paths_from_manifest_parent() {
    let fixture = Fixture::new();
    fixture.write("manifests/patterns/empty", b"^$");
    let suite_file = fixture.write(
        "manifests/suite",
        concat!(
            "default\tconfirm\n",
            "pattern\tempty\talways_allow\tcase-sensitive\tpatterns/empty\n",
            "pattern-case\tempty\tmatch\t\n",
            "decision-case\tallow\t",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified 1 pattern case and 1 permission decision across 1 pattern\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn verifies_file_backed_multiline_suite_inputs() {
    let fixture = Fixture::new();
    fixture.write("manifests/patterns/multiline", b"(?s)^first\\nsecond$");
    fixture.write("manifests/inputs/multiline", b"first\nsecond");
    let suite_file = fixture.write(
        "manifests/suite",
        concat!(
            "decision-case-file\tallow\tinputs/multiline\n",
            "default\tdeny\n",
            "pattern\tmultiline\talways_allow\tcase-sensitive\tpatterns/multiline\n",
            "pattern-case-file\tmultiline\tmatch\tinputs/multiline",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified 1 pattern case and 1 permission decision across 1 pattern\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn limits_suite_failure_output_without_regexes_or_inputs() {
    let fixture = Fixture::new();
    fixture.write("never-pattern", b"^never$");
    let mut lines = vec![
        "default\tdeny".to_owned(),
        "pattern\tmatcher\talways_allow\tcase-sensitive\tnever-pattern".to_owned(),
    ];
    lines
        .extend((1..=6).map(|index| format!("pattern-case\tmatcher\tmatch\tsecret-input-{index}")));
    lines.extend((7..=12).map(|index| format!("decision-case\tallow\tsecret-input-{index}")));
    let manifest = lines.join("\n");
    let suite_file = fixture.write("suite", manifest.as_bytes());

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("12 of 12 suite expectations failed"));
    assert_eq!(stderr.matches("  Line ").count(), 10);
    assert!(stderr.contains("Line 3 pattern `matcher` expected a match"));
    assert!(stderr.contains("expected permission decision `allow`"));
    assert!(stderr.contains("… 2 additional failures omitted"));
    assert!(!stderr.contains("secret-input"));
    assert!(!stderr.contains("^never$"));
}

#[test]
fn rejects_malformed_suite_lines() {
    let fixture = Fixture::new();
    let manifests = [
        "default\tallow\textra",
        "pattern\t\talways_allow\tcase-sensitive\tpattern",
        "pattern-case\tid\tmatch",
        "pattern-case-file\tid\tmatch\t",
        "decision-case\tallow",
        "decision-case-file\tallow\t",
        "unknown\tprivate-input",
        "default\tallow\r\n",
    ];

    for (index, manifest) in manifests.into_iter().enumerate() {
        let suite_file = fixture.write(&format!("malformed-{index}"), manifest.as_bytes());
        let (status, stdout, stderr) = run_with_suite(&suite_file);

        assert_eq!(status, 2, "Manifest {index} unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(stderr.contains("Invalid suite manifest"));
        assert!(stderr.contains("at line 1"));
        assert!(!stderr.contains("private-input"));
    }
}

#[test]
fn rejects_duplicate_suite_pattern_ids() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "default\tallow\n",
            "pattern\tduplicate\talways_allow\tcase-sensitive\tpattern\n",
            "pattern\tduplicate\talways_deny\tcase-insensitive\tpattern\n",
            "decision-case\tallow\tfoo",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Duplicate pattern id `duplicate`"));
    assert!(stderr.contains("at line 3"));
}

#[test]
fn rejects_duplicate_suite_defaults() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "default\tallow\n",
            "default\tdeny\n",
            "pattern\tmatcher\talways_allow\tcase-sensitive\tpattern\n",
            "decision-case\tallow\tfoo",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Duplicate default"));
    assert!(stderr.contains("at line 2"));
}

#[test]
fn rejects_unknown_suite_pattern_ids_after_parsing_all_records() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "pattern-case\tmissing\tmatch\tprivate-input\n",
            "default\tallow\n",
            "pattern\tknown\talways_allow\tcase-sensitive\tpattern",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Unknown pattern id `missing`"));
    assert!(stderr.contains("at line 1"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn rejects_invalid_suite_regex_without_echoing_its_body() {
    let fixture = Fixture::new();
    fixture.write("invalid-pattern", b"private-regex-body(");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "default\tallow\n",
            "pattern\tinvalid-id\talways_deny\tcase-sensitive\tinvalid-pattern\n",
            "pattern-case\tinvalid-id\tno-match\tprivate-input\n",
            "decision-case\tallow\tprivate-input",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid regex in pattern file"));
    assert!(stderr.contains("suite pattern `invalid-id`"));
    assert!(!stderr.contains("private-regex-body"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn rejects_empty_suite_patterns() {
    let fixture = Fixture::new();
    fixture.write("empty-pattern", b"");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "default\tallow\n",
            "pattern\tempty-id\talways_deny\tcase-sensitive\tempty-pattern\n",
            "pattern-case\tempty-id\tno-match\tprivate-input\n",
            "decision-case\tallow\tprivate-input",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("suite pattern `empty-id` is empty"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn rejects_unreadable_suite_patterns() {
    let fixture = Fixture::new();
    let suite_file = fixture.write(
        "suite",
        concat!(
            "default\tallow\n",
            "pattern\tmissing-id\talways_deny\tcase-sensitive\tmissing-pattern\n",
            "pattern-case\tmissing-id\tno-match\tprivate-input\n",
            "decision-case\tallow\tprivate-input",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Failed to read pattern `missing-id` file"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn rejects_unreadable_suite_input_files() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"^input$");
    let suite_file = fixture.write(
        "suite",
        concat!(
            "default\tallow\n",
            "pattern\tmatcher\talways_allow\tcase-sensitive\tpattern\n",
            "pattern-case\tmatcher\tmatch\tinput\n",
            "decision-case-file\tallow\tmissing-input",
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Failed to read suite input file"));
}

#[test]
fn rejects_suite_manifests_missing_required_records() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let manifests = [
        (
            "missing-patterns",
            "default\tallow\ndecision-case\tallow\tfoo",
            "at least one pattern",
        ),
        (
            "missing-expectations",
            "default\tallow\npattern\tmatcher\talways_allow\tcase-sensitive\tpattern",
            "at least one `decision-case`",
        ),
        (
            "missing-default",
            "pattern\tmatcher\talways_allow\tcase-sensitive\tpattern\ndecision-case\tallow\tfoo",
            "exactly one default",
        ),
    ];

    for (name, manifest, expected_error) in manifests {
        let suite_file = fixture.write(name, manifest.as_bytes());
        let (status, stdout, stderr) = run_with_suite(&suite_file);

        assert_eq!(status, 2, "Manifest {name} unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(stderr.contains(expected_error));
    }
}

#[test]
fn rejects_suite_manifests_without_complete_coverage() {
    let fixture = Fixture::new();
    fixture.write("first-pattern", b"^first$");
    fixture.write("second-pattern", b"^second$");
    let manifests = [
        (
            "missing-decision",
            concat!(
                "default\tallow\n",
                "pattern\tfirst\talways_allow\tcase-sensitive\tfirst-pattern\n",
                "pattern-case\tfirst\tmatch\tfirst",
            ),
            "at least one `decision-case`",
        ),
        (
            "missing-one-pattern-case",
            concat!(
                "default\tallow\n",
                "pattern\tfirst\talways_allow\tcase-sensitive\tfirst-pattern\n",
                "pattern\tsecond\talways_confirm\tcase-sensitive\tsecond-pattern\n",
                "pattern-case\tfirst\tmatch\tfirst\n",
                "decision-case\tallow\tfirst",
            ),
            "pattern `second`",
        ),
        (
            "missing-pattern-cases",
            concat!(
                "default\tallow\n",
                "pattern\tfirst\talways_allow\tcase-sensitive\tfirst-pattern\n",
                "decision-case\tallow\tfirst",
            ),
            "pattern `first`",
        ),
    ];

    for (name, manifest, expected_error) in manifests {
        let suite_file = fixture.write(name, manifest.as_bytes());
        let (status, stdout, stderr) = run_with_suite(&suite_file);

        assert_eq!(status, 2, "Manifest {name} unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(stderr.contains(expected_error));
    }
}

#[test]
fn rejects_suite_mode_with_existing_matching_options() {
    let fixture = Fixture::new();
    let suite_file = fixture.write("suite", b"");
    let option_cases = [
        ("--case-sensitive", None),
        ("--cases-file", Some("cases")),
        ("--input-file", Some("input")),
        ("--pattern-file", Some("pattern")),
    ];

    for (option, value) in option_cases {
        let mut arguments = vec![
            OsString::from("--suite-file"),
            suite_file.as_os_str().to_owned(),
            OsString::from(option),
        ];
        if let Some(value) = value {
            arguments.push(OsString::from(value));
        }
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let status = helper::run(arguments, &mut stdout, &mut stderr);
        let stderr = String::from_utf8(stderr).expect("Standard error must be valid UTF-8");

        assert_eq!(status, 2, "Option {option} unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(stderr.contains("mutually exclusive"));
        assert!(stderr.contains("--suite-file"));
    }
}

#[test]
fn rejects_extra_suite_arguments() {
    let fixture = Fixture::new();
    let suite_file = fixture.write("suite", b"");
    let arguments = [
        OsString::from("--suite-file"),
        suite_file.as_os_str().to_owned(),
        OsString::from("extra"),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(arguments, &mut stdout, &mut stderr);
    let stderr = String::from_utf8(stderr).expect("Standard error must be valid UTF-8");

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Unknown option `extra`"));
}

#[test]
fn rejects_invalid_utf8_suite_manifests() {
    let fixture = Fixture::new();
    let suite_file = fixture.write("suite", &[0xff]);

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid UTF-8 in suite manifest file"));
}

#[test]
fn accepts_equivalent_configured_patterns_over_inline_corpus() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"^foo$");
    fixture.write("candidate-pattern", b"^foo$");
    let manifest = comparison_manifest(
        "deny",
        vec![comparison_pattern(
            "baseline-id",
            "always_allow",
            true,
            "baseline-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate-id",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![comparison_case_inline("foo"), comparison_case_inline("bar")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Representative corpus comparison found equivalent configured pattern behavior across 2 cases with 1 baseline pattern and 1 candidate pattern\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn resolves_relative_comparison_paths_and_reads_file_backed_multiline_input() {
    let fixture = Fixture::new();
    fixture.write("manifests/patterns/baseline", b"(?s)^first\\nsecond$");
    fixture.write("manifests/patterns/candidate", b"(?s)^first\\nsecond$");
    fixture.write("manifests/inputs/multiline", b"first\nsecond");
    let manifest = comparison_manifest(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_confirm",
            true,
            "patterns/baseline",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_confirm",
            true,
            "patterns/candidate",
        )],
        vec![comparison_case_file("inputs/multiline")],
    );
    let comparison_file = fixture.write_json("manifests/comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Representative corpus comparison found equivalent configured pattern behavior across 1 case with 1 baseline pattern and 1 candidate pattern\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn reports_allow_bucket_difference_without_content_leakage() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"^private-inline-input$");
    fixture.write("candidate-pattern", b"^private-never-match$");
    let manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "baseline-pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![comparison_case_inline("private-inline-input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        format!(
            "zed-pattern-match: 1 mismatch across 1 comparison case in `{}`\n  Case 1 differs in: always_allow bucket\n",
            comparison_file.display()
        )
    );
    assert!(!stderr.contains("private-inline-input"));
    assert!(!stderr.contains("private-never-match"));
}

#[test]
fn reports_confirm_and_deny_bucket_differences() {
    let fixture = Fixture::new();
    fixture.write("matching-pattern", b"^input$");
    fixture.write("nonmatching-pattern", b"^other$");
    let manifest = comparison_manifest(
        "deny",
        vec![
            comparison_pattern("confirm", "always_confirm", true, "matching-pattern"),
            comparison_pattern("deny", "always_deny", true, "matching-pattern"),
        ],
        "deny",
        vec![
            comparison_pattern("confirm", "always_confirm", true, "nonmatching-pattern"),
            comparison_pattern("deny", "always_deny", true, "nonmatching-pattern"),
        ],
        vec![comparison_case_inline("input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("1 mismatch across 1 comparison case"));
    assert!(stderr.contains("Case 1 differs in: always_confirm bucket, always_deny bucket"));
    assert!(!stderr.contains("final decision"));
}

#[test]
fn reports_default_only_final_decision_difference() {
    let fixture = Fixture::new();
    fixture.write("nonmatching-pattern", b"^other$");
    let manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "nonmatching-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "nonmatching-pattern",
        )],
        vec![comparison_case_inline("input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Case 1 differs in: final decision"));
    assert!(!stderr.contains("always_allow bucket"));
    assert!(!stderr.contains("always_confirm bucket"));
    assert!(!stderr.contains("always_deny bucket"));
}

#[test]
fn counts_multiple_bucket_and_final_decision_differences_once() {
    let fixture = Fixture::new();
    fixture.write("matching-pattern", b"^input$");
    let manifest = comparison_manifest(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "matching-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_confirm",
            true,
            "matching-pattern",
        )],
        vec![comparison_case_inline("input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("1 mismatch across 1 comparison case"));
    assert_eq!(stderr.matches("  Case ").count(), 1);
    assert!(
        stderr.contains(
            "Case 1 differs in: always_allow bucket, always_confirm bucket, final decision"
        )
    );
}

#[test]
fn applies_comparison_pattern_case_settings() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"^lowercase$");
    let manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            false,
            "pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "pattern",
        )],
        vec![
            comparison_case_inline("LOWERCASE"),
            comparison_case_inline("lowercase"),
        ],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("1 mismatch across 2 comparison cases"));
    assert!(stderr.contains("Case 1 differs in: always_allow bucket"));
    assert!(!stderr.contains("Case 2 differs"));
}

#[test]
fn rejects_comparison_mode_with_every_existing_mode_and_option() {
    let fixture = Fixture::new();
    let comparison_file = fixture.write("comparison.json", b"{}");
    let option_cases = [
        ("--case-sensitive", None),
        ("--cases-file", Some("cases")),
        ("--help", None),
        ("--input-file", Some("input")),
        ("--pattern-file", Some("pattern")),
        ("--suite-file", Some("suite")),
    ];

    for (option, value) in option_cases {
        let mut arguments = vec![
            OsString::from("--comparison-file"),
            comparison_file.as_os_str().to_owned(),
            OsString::from(option),
        ];
        if let Some(value) = value {
            arguments.push(OsString::from(value));
        }
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let status = helper::run(arguments, &mut stdout, &mut stderr);
        let stderr = String::from_utf8(stderr).expect("Standard error must be valid UTF-8");

        assert_eq!(status, 2, "Option {option} unexpectedly succeeded");
        assert!(stdout.is_empty());
        if option == "--help" {
            assert!(stderr.contains("must be used alone"));
        } else {
            assert!(stderr.contains("mutually exclusive"));
            assert!(stderr.contains("--comparison-file"));
        }
    }
}

#[test]
fn rejects_duplicate_comparison_option_and_missing_path() {
    let fixture = Fixture::new();
    let comparison_file = fixture.write("comparison.json", b"{}");
    let argument_sets = [
        vec![OsString::from("--comparison-file")],
        vec![
            OsString::from("--comparison-file"),
            comparison_file.as_os_str().to_owned(),
            OsString::from("--comparison-file"),
            comparison_file.as_os_str().to_owned(),
        ],
    ];

    for arguments in argument_sets {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let status = helper::run(arguments, &mut stdout, &mut stderr);
        let stderr = String::from_utf8(stderr).expect("Standard error must be valid UTF-8");

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains("--comparison-file"));
    }
}

#[test]
fn rejects_comparison_schema_errors_versions_and_unknown_fields() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let valid_pattern = comparison_pattern("id", "always_allow", true, "pattern");
    let cases = [
        (
            "malformed-json",
            b"private-json-content{".to_vec(),
            "Invalid comparison manifest",
        ),
        (
            "wrong-version",
            serde_json::to_vec(&serde_json::json!({
                "version": 2,
                "baseline": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "candidate": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "cases": [comparison_case_inline("input")],
            }))
            .expect("Test JSON must serialize"),
            "Unsupported comparison manifest version 2",
        ),
        (
            "root-unknown",
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "baseline": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "candidate": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "cases": [comparison_case_inline("input")],
                "private_unknown_root": true,
            }))
            .expect("Test JSON must serialize"),
            "version-1 comparison schema",
        ),
        (
            "set-unknown",
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "baseline": {
                    "default": "allow",
                    "patterns": [valid_pattern.clone()],
                    "private_unknown_set": true,
                },
                "candidate": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "cases": [comparison_case_inline("input")],
            }))
            .expect("Test JSON must serialize"),
            "version-1 comparison schema",
        ),
        (
            "pattern-unknown",
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "baseline": {
                    "default": "allow",
                    "patterns": [{
                        "id": "id",
                        "bucket": "always_allow",
                        "case_sensitive": true,
                        "pattern_file": "pattern",
                        "private_unknown_pattern": true,
                    }],
                },
                "candidate": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "cases": [comparison_case_inline("input")],
            }))
            .expect("Test JSON must serialize"),
            "version-1 comparison schema",
        ),
        (
            "case-unknown",
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "baseline": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "candidate": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "cases": [{
                    "type": "inline",
                    "input": "input",
                    "private_unknown_case": true,
                }],
            }))
            .expect("Test JSON must serialize"),
            "version-1 comparison schema",
        ),
        (
            "invalid-default",
            serde_json::to_vec(&serde_json::json!({
                "version": 1,
                "baseline": { "default": "private-default", "patterns": [valid_pattern.clone()] },
                "candidate": { "default": "allow", "patterns": [valid_pattern.clone()] },
                "cases": [comparison_case_inline("input")],
            }))
            .expect("Test JSON must serialize"),
            "version-1 comparison schema",
        ),
    ];

    for (name, contents, expected_error) in cases {
        let comparison_file = fixture.write(name, &contents);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2, "Comparison {name} unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error for {name}: {stderr}"
        );
        assert!(!stderr.contains("private-json-content"));
        assert!(!stderr.contains("private_unknown_root"));
        assert!(!stderr.contains("private_unknown_set"));
        assert!(!stderr.contains("private_unknown_pattern"));
        assert!(!stderr.contains("private_unknown_case"));
        assert!(!stderr.contains("private-default"));
    }
}

#[test]
fn rejects_empty_comparison_sets_cases_ids_and_paths() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let valid = comparison_pattern("id", "always_allow", true, "pattern");
    let cases = [
        (
            "empty-baseline",
            comparison_manifest(
                "allow",
                vec![],
                "allow",
                vec![valid.clone()],
                vec![comparison_case_inline("input")],
            ),
            "at least one baseline pattern",
        ),
        (
            "empty-candidate",
            comparison_manifest(
                "allow",
                vec![valid.clone()],
                "allow",
                vec![],
                vec![comparison_case_inline("input")],
            ),
            "at least one candidate pattern",
        ),
        (
            "empty-cases",
            comparison_manifest(
                "allow",
                vec![valid.clone()],
                "allow",
                vec![valid.clone()],
                vec![],
            ),
            "at least one case",
        ),
        (
            "empty-id",
            comparison_manifest(
                "allow",
                vec![comparison_pattern("", "always_allow", true, "pattern")],
                "allow",
                vec![valid.clone()],
                vec![comparison_case_inline("input")],
            ),
            "empty baseline pattern id",
        ),
        (
            "empty-pattern-file",
            comparison_manifest(
                "allow",
                vec![comparison_pattern("id", "always_allow", true, "")],
                "allow",
                vec![valid.clone()],
                vec![comparison_case_inline("input")],
            ),
            "nonempty `pattern_file`",
        ),
        (
            "empty-input-file",
            comparison_manifest(
                "allow",
                vec![valid.clone()],
                "allow",
                vec![valid.clone()],
                vec![comparison_case_file("")],
            ),
            "nonempty `input_file`",
        ),
    ];

    for (name, manifest, expected_error) in cases {
        let comparison_file = fixture.write_json(name, &manifest);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2, "Comparison {name} unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error for {name}: {stderr}"
        );
    }
}

#[test]
fn rejects_duplicate_comparison_pattern_ids_within_each_set() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let duplicate_patterns = vec![
        comparison_pattern("duplicate", "always_allow", true, "pattern"),
        comparison_pattern("duplicate", "always_deny", false, "pattern"),
    ];

    for set_label in ["baseline", "candidate"] {
        let manifest = if set_label == "baseline" {
            comparison_manifest(
                "allow",
                duplicate_patterns.clone(),
                "allow",
                vec![comparison_pattern(
                    "candidate",
                    "always_allow",
                    true,
                    "pattern",
                )],
                vec![comparison_case_inline("input")],
            )
        } else {
            comparison_manifest(
                "allow",
                vec![comparison_pattern(
                    "baseline",
                    "always_allow",
                    true,
                    "pattern",
                )],
                "allow",
                duplicate_patterns.clone(),
                vec![comparison_case_inline("input")],
            )
        };
        let comparison_file = fixture.write_json(set_label, &manifest);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains("Duplicate pattern id `duplicate`"));
        assert!(stderr.contains(set_label));
    }
}

#[test]
fn rejects_inline_comparison_cr_and_lf_without_content_leakage() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");

    for (name, input, marker) in [
        (
            "carriage-return",
            "secret-cr-marker\rinput",
            "secret-cr-marker",
        ),
        ("line-feed", "secret-lf-marker\ninput", "secret-lf-marker"),
    ] {
        let manifest = comparison_manifest(
            "allow",
            vec![comparison_pattern(
                "baseline",
                "always_allow",
                true,
                "pattern",
            )],
            "allow",
            vec![comparison_pattern(
                "candidate",
                "always_allow",
                true,
                "pattern",
            )],
            vec![comparison_case_inline(input)],
        );
        let comparison_file = fixture.write_json(name, &manifest);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(stderr.contains("must not contain CR or LF"));
        assert!(stderr.contains("file case"));
        assert!(!stderr.contains(marker));
    }
}

#[test]
fn rejects_comparison_pattern_compile_errors_without_content_leakage() {
    let fixture = Fixture::new();
    fixture.write("invalid-pattern", b"private-regex-body(");
    fixture.write("valid-pattern", b"^private-input$");
    let manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            "private-baseline-id",
            "always_allow",
            true,
            "invalid-pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "valid-pattern",
        )],
        vec![comparison_case_inline("private-input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Invalid regex in pattern file"));
    assert!(stderr.contains("baseline comparison pattern `private-baseline-id`"));
    assert!(!stderr.contains("private-regex-body"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn rejects_unreadable_comparison_input_with_error_status() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "pattern",
        )],
        vec![comparison_case_file("missing-input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Failed to read comparison case 1 input file"));
}

#[test]
fn limits_comparison_mismatch_output_and_counts_every_case() {
    let fixture = Fixture::new();
    fixture.write("matching-pattern", b"^private-input-[0-9]+$");
    fixture.write("nonmatching-pattern", b"^other$");
    let cases = (1..=12)
        .map(|index| comparison_case_inline(&format!("private-input-{index}")))
        .collect();
    let manifest = comparison_manifest(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "matching-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "nonmatching-pattern",
        )],
        cases,
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("12 mismatches across 12 comparison cases"));
    assert_eq!(stderr.matches("  Case ").count(), 10);
    assert!(stderr.contains("Case 1 differs in: always_allow bucket, final decision"));
    assert!(stderr.contains("Case 10 differs in: always_allow bucket, final decision"));
    assert!(!stderr.contains("Case 11 differs"));
    assert!(stderr.contains("… 2 additional mismatches omitted"));
    assert!(!stderr.contains("private-input"));
    assert!(!stderr.contains("^other$"));
}

#[test]
fn returns_error_when_comparison_output_cannot_be_written() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"^input$");
    fixture.write("candidate-pattern", b"^other$");
    let manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "baseline-pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![comparison_case_inline("input")],
    );
    let comparison_file = fixture.write_json("comparison.json", &manifest);
    let arguments = [
        OsString::from("--comparison-file"),
        comparison_file.as_os_str().to_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = FailingWriter;

    let status = helper::run(arguments, &mut stdout, &mut stderr);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
}

#[test]
fn returns_success_for_help() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(vec![OsString::from("--help")], &mut stdout, &mut stderr);

    assert_eq!(status, 0);
    let stdout = String::from_utf8(stdout).expect("Standard output must be valid UTF-8");

    assert!(stdout.starts_with("Usage:\n  zed-pattern-match"));
    assert!(stdout.contains("--cases-file"));
    assert!(stdout.contains("--comparison-file <path>"));
    assert!(stdout.contains("--suite-file"));
    assert!(stdout.contains("configured-pattern decisions"));
    assert!(stdout.contains("default<TAB>allow|confirm|deny"));
    assert!(stdout.contains("pattern-case<TAB><id><TAB>match|no-match<TAB><input>"));
    assert!(stdout.contains("pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>"));
    assert!(stdout.contains("decision-case<TAB>allow|confirm|deny<TAB><input>"));
    assert!(stdout.contains("decision-case-file<TAB>allow|confirm|deny<TAB><input-file>"));
    assert!(stdout.contains("Suite requirements:"));
    assert!(stdout.contains("at least one decision case"));
    assert!(stdout.contains("at least one pattern case for every pattern ID"));
    assert!(stdout.contains("file-backed records for multiline inputs"));
    assert!(stdout.contains("do not reproduce full Zed permission evaluation"));
    assert!(stdout.contains("Version-1 UTF-8 JSON comparison manifest:"));
    assert!(stdout.contains("representative corpus only"));
    assert!(stdout.contains("not full Zed permission evaluation or formal language equivalence"));
    assert!(
        stdout.contains("checks each bucket’s matched state and the configured final decision")
    );
    assert!(stdout.contains("Mutually exclusive with every other option"));
    assert!(stderr.is_empty());
}
