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
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn run_with_cases(
    cases_file: &Path,
    pattern_file: &Path,
    case_sensitive: bool,
) -> (u8, String, String) {
    run_with_source("--cases-file", cases_file, pattern_file, case_sensitive)
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
fn returns_success_for_help() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(vec![OsString::from("--help")], &mut stdout, &mut stderr);

    assert_eq!(status, 0);
    let stdout = String::from_utf8(stdout).expect("Standard output must be valid UTF-8");

    assert!(stdout.starts_with("Usage:\n  zed-pattern-match"));
    assert!(stdout.contains("--cases-file"));
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
    assert!(stderr.is_empty());
}
