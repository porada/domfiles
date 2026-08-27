#[path = "pattern_match.rs"]
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
            "domfiles-pattern-match-{}-{timestamp}-{fixture_id}",
            process::id()
        ));
        fs::create_dir(&root).expect("Failed to create fixture directory");
        let root = fs::canonicalize(root).expect("Failed to canonicalize fixture directory");

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
        "baseline": {
            "default": baseline_default,
            "patterns": baseline_patterns,
        },
        "candidate": {
            "default": candidate_default,
            "patterns": candidate_patterns,
        },
        "catalogs": [],
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
        "type": "file",
        "id": id,
        "bucket": bucket,
        "case_sensitive": case_sensitive,
        "pattern_file": pattern_file,
    })
}

struct CatalogFiles {
    artifact_files: Vec<String>,
    candidate_file: String,
    catalog_file: String,
    state_file: String,
}

struct CatalogPatternFixture<'a> {
    bucket: &'a str,
    case_sensitive: bool,
    contents: &'a [u8],
    id: &'a str,
}

fn write_catalog_fixture(
    fixture: &Fixture,
    directory: &str,
    patterns: &[CatalogPatternFixture<'_>],
) -> CatalogFiles {
    let candidate_file = format!("{directory}/candidate-settings.json");
    let state_file = format!("{directory}/state.json");
    let catalog_file = format!("{directory}/catalog.json");
    let candidate_bytes = format!("candidate bytes for {directory}").into_bytes();
    let state_bytes = format!("state bytes for {directory}").into_bytes();
    fixture.write(&candidate_file, &candidate_bytes);
    fixture.write(&state_file, &state_bytes);

    let mut artifact_files = Vec::with_capacity(patterns.len());
    let catalog_patterns: Vec<_> = patterns
        .iter()
        .enumerate()
        .map(|(index, pattern)| {
            let relative_file = format!("patterns/pattern-{:03}.regex", index + 1);
            let artifact_file = format!("{directory}/{relative_file}");
            fixture.write(&artifact_file, pattern.contents);
            artifact_files.push(artifact_file);

            serde_json::json!({
                "id": pattern.id,
                "bucket": pattern.bucket,
                "source_index": index,
                "case_sensitive": pattern.case_sensitive,
                "sha256": helper::sha256_hex(pattern.contents),
                "pattern_file": relative_file,
            })
        })
        .collect();
    fixture.write_json(
        &catalog_file,
        &serde_json::json!({
            "candidate_sha256": helper::sha256_hex(&candidate_bytes),
            "state_sha256": helper::sha256_hex(&state_bytes),
            "patterns": catalog_patterns,
        }),
    );

    CatalogFiles {
        artifact_files,
        candidate_file,
        catalog_file,
        state_file,
    }
}

fn catalog_definition(id: &str, files: &CatalogFiles) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "catalog_file": files.catalog_file,
        "candidate_file": files.candidate_file,
        "state_file": files.state_file,
    })
}

fn comparison_case_file_with_transition(
    input_file: &str,
    expected_transition: Option<serde_json::Value>,
) -> serde_json::Value {
    let mut case = serde_json::json!({
        "type": "file",
        "input_file": input_file,
    });
    if let Some(expected_transition) = expected_transition {
        case["expected_transition"] = expected_transition;
    }

    case
}

fn comparison_case_inline_with_transition(
    input: &str,
    expected_transition: Option<serde_json::Value>,
) -> serde_json::Value {
    let mut case = serde_json::json!({
        "type": "inline",
        "input": input,
    });
    if let Some(expected_transition) = expected_transition {
        case["expected_transition"] = expected_transition;
    }

    case
}

fn comparison_catalog_pattern(catalog_id: &str, pattern_id: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "catalog",
        "catalog_id": catalog_id,
        "pattern_id": pattern_id,
    })
}

fn comparison_state(
    always_allow: bool,
    always_confirm: bool,
    always_deny: bool,
    final_decision: &str,
) -> serde_json::Value {
    serde_json::json!({
        "always_allow": always_allow,
        "always_confirm": always_confirm,
        "always_deny": always_deny,
        "final_decision": final_decision,
    })
}

fn expected_transition(
    baseline: serde_json::Value,
    candidate: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "baseline": baseline,
        "candidate": candidate,
    })
}

fn comparison_manifest_with_catalogs(
    baseline_default: &str,
    baseline_patterns: Vec<serde_json::Value>,
    candidate_default: &str,
    candidate_patterns: Vec<serde_json::Value>,
    catalogs: Vec<serde_json::Value>,
    cases: Vec<serde_json::Value>,
) -> serde_json::Value {
    serde_json::json!({
        "baseline": {
            "default": baseline_default,
            "patterns": baseline_patterns,
        },
        "candidate": {
            "default": candidate_default,
            "patterns": candidate_patterns,
        },
        "catalogs": catalogs,
        "cases": cases,
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

fn run_with_layer(layer_file: &Path) -> (u8, String, String) {
    let arguments = [
        OsString::from("--layer-file"),
        layer_file.as_os_str().to_owned(),
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

fn run_with_layer_evidence(
    layer_file: &Path,
    graph_root: &Path,
    result_out: &Path,
) -> (u8, String, String) {
    let arguments = [
        OsString::from("--layer-file"),
        layer_file.as_os_str().to_owned(),
        OsString::from("--graph-root"),
        graph_root.as_os_str().to_owned(),
        OsString::from("--result-out"),
        result_out.as_os_str().to_owned(),
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
    assert!(stderr.contains("Failed to compile regex from pattern file"));
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
    assert!(stderr.contains("Case manifest"));
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
        "Verified 4 pattern cases and 4 decision cases across 3 patterns\n"
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
        "Verified 1 pattern case and 1 decision case across 1 pattern\n"
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
        "Verified 1 pattern case and 1 decision case across 1 pattern\n"
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
    assert!(stderr.contains("expected configured decision `allow`"));
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
    assert!(stderr.contains("Duplicate pattern ID `duplicate`"));
    assert!(stderr.contains("at line 3"));
}

#[test]
fn bounds_suite_pattern_ids_in_diagnostics() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"^never$");
    let long_id = format!("{}private-tail", "x".repeat(80));
    let duplicate_suite = fixture.write(
        "duplicate-long-id-suite",
        format!(
            concat!(
                "default\tallow\n",
                "pattern\t{}\talways_allow\tcase-sensitive\tpattern\n",
                "pattern\t{}\talways_deny\tcase-sensitive\tpattern\n",
                "decision-case\tallow\tinput"
            ),
            long_id, long_id
        )
        .as_bytes(),
    );
    let (status, stdout, stderr) = run_with_suite(&duplicate_suite);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("`{}…`", "x".repeat(80))));
    assert!(!stderr.contains("private-tail"));

    let failing_suite = fixture.write(
        "failing-long-id-suite",
        format!(
            concat!(
                "default\tdeny\n",
                "pattern\t{}\talways_allow\tcase-sensitive\tpattern\n",
                "pattern-case\t{}\tmatch\tinput\n",
                "decision-case\tdeny\tinput"
            ),
            long_id, long_id
        )
        .as_bytes(),
    );
    let (status, stdout, stderr) = run_with_suite(&failing_suite);
    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("`{}…`", "x".repeat(80))));
    assert!(!stderr.contains("private-tail"));

    let unreadable_suite = fixture.write(
        "unreadable-long-id-suite",
        format!(
            concat!(
                "default\tallow\n",
                "pattern\t{}\talways_allow\tcase-sensitive\tmissing-pattern\n",
                "pattern-case\t{}\tmatch\tinput\n",
                "decision-case\tallow\tinput"
            ),
            long_id, long_id
        )
        .as_bytes(),
    );
    let (status, stdout, stderr) = run_with_suite(&unreadable_suite);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("`{}…`", "x".repeat(80))));
    assert!(!stderr.contains("private-tail"));

    fixture.write("invalid-long-id-pattern", b"private-regex-body(");
    let invalid_suite = fixture.write(
        "invalid-long-id-suite",
        format!(
            concat!(
                "default\tallow\n",
                "pattern\t{}\talways_allow\tcase-sensitive\tinvalid-long-id-pattern\n",
                "pattern-case\t{}\tmatch\tinput\n",
                "decision-case\tallow\tinput"
            ),
            long_id, long_id
        )
        .as_bytes(),
    );
    let (status, stdout, stderr) = run_with_suite(&invalid_suite);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("`{}…`", "x".repeat(80))));
    assert!(!stderr.contains("private-tail"));
    assert!(!stderr.contains("private-regex-body"));
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
    assert!(stderr.contains("Duplicate `default` record"));
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
    assert!(stderr.contains("Unknown pattern ID `missing`"));
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
    assert!(stderr.contains("Failed to compile regex from pattern file"));
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
            "at least one `decision-case` or `decision-case-file` record",
        ),
        (
            "missing-default",
            "pattern\tmatcher\talways_allow\tcase-sensitive\tpattern\ndecision-case\tallow\tfoo",
            "exactly one `default` record",
        ),
    ];

    for (name, manifest, expected_error) in manifests {
        let suite_file = fixture.write(name, manifest.as_bytes());
        let (status, stdout, stderr) = run_with_suite(&suite_file);

        assert_eq!(status, 2, "Manifest `{name}` unexpectedly succeeded");
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
            "at least one `decision-case` or `decision-case-file` record",
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

        assert_eq!(status, 2, "Manifest `{name}` unexpectedly succeeded");
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

        assert_eq!(status, 2, "Option `{option}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(stderr.contains("mutually exclusive"));
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
fn accepts_unversioned_equivalence_comparison_manifest() {
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
        "Verified a representative comparison corpus with 2 equivalence cases, 0 matched transitions, 1 baseline pattern, and 1 candidate pattern\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn rejects_legacy_untagged_comparison_manifests() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"^input$");
    let legacy = serde_json::json!({
        "baseline": {
            "default": "deny",
            "patterns": [{
                "id": "baseline",
                "bucket": "always_allow",
                "case_sensitive": true,
                "pattern_file": "pattern",
            }],
        },
        "candidate": {
            "default": "deny",
            "patterns": [{
                "id": "candidate",
                "bucket": "always_allow",
                "case_sensitive": true,
                "pattern_file": "pattern",
            }],
        },
        "cases": [comparison_case_inline("input")],
    });
    let comparison_file = fixture.write_json("legacy-comparison.json", &legacy);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("does not match the required comparison schema"));
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
        "Verified a representative comparison corpus with 1 equivalence case, 0 matched transitions, 1 baseline pattern, and 1 candidate pattern\n"
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
            "pattern-match: 1 mismatch across 1 comparison case in `{}`\n  Case 1 baseline/candidate differ in `always_allow` bucket\n",
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
    assert!(stderr.contains(
        "Case 1 baseline/candidate differ in `always_confirm` bucket, `always_deny` bucket"
    ));
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
    assert!(stderr.contains("Case 1 baseline/candidate differ in final decision"));
    assert!(!stderr.contains("`always_allow` bucket"));
    assert!(!stderr.contains("`always_confirm` bucket"));
    assert!(!stderr.contains("`always_deny` bucket"));
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
    assert!(stderr.contains(
        "Case 1 baseline/candidate differ in `always_allow` bucket, `always_confirm` bucket, final decision"
    ));
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
    assert!(stderr.contains("Case 1 baseline/candidate differ in `always_allow` bucket"));
    assert!(!stderr.contains("Case 2 baseline/candidate differ"));
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

        assert_eq!(status, 2, "Option `{option}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        if option == "--help" {
            assert!(stderr.contains("must be used alone"));
        } else {
            assert!(stderr.contains("mutually exclusive"));
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
fn rejects_comparison_schema_errors_version_fields_and_unknown_fields() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let valid_pattern = comparison_pattern("id", "always_allow", true, "pattern");
    let valid = comparison_manifest(
        "allow",
        vec![valid_pattern.clone()],
        "allow",
        vec![valid_pattern],
        vec![comparison_case_inline("input")],
    );
    let mut version_one = valid.clone();
    version_one["version"] = serde_json::json!(1);
    let mut version_two = valid.clone();
    version_two["version"] = serde_json::json!(2);
    let mut root_unknown = valid.clone();
    root_unknown["private_unknown_root"] = serde_json::json!(true);
    let mut set_unknown = valid.clone();
    set_unknown["baseline"]["private_unknown_set"] = serde_json::json!(true);
    let mut pattern_unknown = valid.clone();
    pattern_unknown["baseline"]["patterns"][0]["private_unknown_pattern"] = serde_json::json!(true);
    let mut case_unknown = valid.clone();
    case_unknown["cases"][0]["private_unknown_case"] = serde_json::json!(true);
    let mut invalid_default = valid;
    invalid_default["baseline"]["default"] = serde_json::json!("private-default");
    let cases = [
        (
            "malformed-json",
            b"private-json-content{".to_vec(),
            "Invalid comparison manifest",
        ),
        (
            "version-one",
            serde_json::to_vec(&version_one).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
        (
            "version-two",
            serde_json::to_vec(&version_two).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
        (
            "root-unknown",
            serde_json::to_vec(&root_unknown).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
        (
            "set-unknown",
            serde_json::to_vec(&set_unknown).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
        (
            "pattern-unknown",
            serde_json::to_vec(&pattern_unknown).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
        (
            "case-unknown",
            serde_json::to_vec(&case_unknown).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
        (
            "invalid-default",
            serde_json::to_vec(&invalid_default).expect("Test JSON must serialize"),
            "required comparison schema",
        ),
    ];

    for (name, contents, expected_error) in cases {
        let comparison_file = fixture.write(name, &contents);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2, "Comparison `{name}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error for `{name}`: {stderr}"
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
fn rejects_empty_comparison_cases_ids_and_paths() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let valid = comparison_pattern("id", "always_allow", true, "pattern");
    let cases = [
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
            "empty baseline pattern ID",
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

        assert_eq!(status, 2, "Comparison `{name}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error for `{name}`: {stderr}"
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
        assert!(stderr.contains("Duplicate pattern ID `duplicate`"));
        assert!(stderr.contains(set_label));
    }
}

#[test]
fn bounds_comparison_pattern_ids_in_diagnostics() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"foo");
    let long_id = format!("{}private-tail", "x".repeat(80));
    let duplicate = vec![
        comparison_pattern(&long_id, "always_allow", true, "pattern"),
        comparison_pattern(&long_id, "always_deny", false, "pattern"),
    ];
    let manifest = comparison_manifest(
        "allow",
        duplicate,
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "pattern",
        )],
        vec![comparison_case_inline("input")],
    );
    let comparison_file = fixture.write_json("long-id-comparison", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("`{}…`", "x".repeat(80))));
    assert!(!stderr.contains("private-tail"));

    fixture.write("invalid-long-id-pattern", b"private-regex-body(");
    let invalid_manifest = comparison_manifest(
        "allow",
        vec![comparison_pattern(
            &long_id,
            "always_allow",
            true,
            "invalid-long-id-pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "pattern",
        )],
        vec![comparison_case_inline("input")],
    );
    let invalid_file = fixture.write_json("invalid-long-id-comparison", &invalid_manifest);
    let (status, stdout, stderr) = run_with_comparison(&invalid_file);
    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains(&format!("`{}…`", "x".repeat(80))));
    assert!(!stderr.contains("private-tail"));
    assert!(!stderr.contains("private-regex-body"));
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
    assert!(stderr.contains("Failed to compile regex from pattern file"));
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
    assert!(
        stderr
            .contains("Case 1 baseline/candidate differ in `always_allow` bucket, final decision")
    );
    assert!(
        stderr
            .contains("Case 10 baseline/candidate differ in `always_allow` bucket, final decision")
    );
    assert!(!stderr.contains("Case 11 baseline/candidate differ"));
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
fn accepts_confirm_to_allow_transition_with_equivalence_case() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"^change$");
    fixture.write("candidate-pattern", b"^change$");
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_confirm",
            true,
            "baseline-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![],
        vec![
            comparison_case_inline_with_transition(
                "change",
                Some(expected_transition(
                    comparison_state(false, true, false, "confirm"),
                    comparison_state(true, false, false, "allow"),
                )),
            ),
            comparison_case_inline_with_transition("unchanged", None),
        ],
    );
    let comparison_file = fixture.write_json("transition-comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified a representative comparison corpus with 1 equivalence case, 1 matched transition, 1 baseline pattern, and 1 candidate pattern\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn accepts_one_to_zero_transition_using_the_candidate_default() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"^change$");
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_confirm",
            true,
            "baseline-pattern",
        )],
        "allow",
        vec![],
        vec![],
        vec![comparison_case_inline_with_transition(
            "change",
            Some(expected_transition(
                comparison_state(false, true, false, "confirm"),
                comparison_state(false, false, false, "allow"),
            )),
        )],
    );
    let comparison_file = fixture.write_json("one-to-zero.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("1 matched transition"));
    assert!(stdout.contains("1 baseline pattern, and 0 candidate patterns"));
    assert!(stderr.is_empty());
}

#[test]
fn accepts_zero_to_one_transition_using_the_baseline_default() {
    let fixture = Fixture::new();
    fixture.write("candidate-pattern", b"^change$");
    let manifest = comparison_manifest_with_catalogs(
        "confirm",
        vec![],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![],
        vec![comparison_case_inline_with_transition(
            "change",
            Some(expected_transition(
                comparison_state(false, false, false, "confirm"),
                comparison_state(true, false, false, "allow"),
            )),
        )],
    );
    let comparison_file = fixture.write_json("zero-to-one.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("1 matched transition"));
    assert!(stdout.contains("0 baseline patterns, and 1 candidate pattern"));
    assert!(stderr.is_empty());
}

#[test]
fn accepts_file_backed_transition() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"(?s)^first\\nsecond$");
    fixture.write("candidate-pattern", b"(?s)^first\\nsecond$");
    fixture.write("multiline-input", b"first\nsecond");
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_confirm",
            true,
            "baseline-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![],
        vec![comparison_case_file_with_transition(
            "multiline-input",
            Some(expected_transition(
                comparison_state(false, true, false, "confirm"),
                comparison_state(true, false, false, "allow"),
            )),
        )],
    );
    let comparison_file = fixture.write_json("file-transition.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("1 matched transition"));
    assert!(stderr.is_empty());
}

#[test]
fn accepts_declared_bucket_only_drift_masked_by_deny_precedence() {
    let fixture = Fixture::new();
    fixture.write("deny-pattern", b"^input$");
    fixture.write("allow-pattern", b"^input$");
    fixture.write("never-pattern", b"^never$");
    let manifest = comparison_manifest_with_catalogs(
        "confirm",
        vec![
            comparison_pattern("baseline-allow", "always_allow", true, "allow-pattern"),
            comparison_pattern("baseline-deny", "always_deny", true, "deny-pattern"),
        ],
        "confirm",
        vec![
            comparison_pattern("candidate-allow", "always_allow", true, "never-pattern"),
            comparison_pattern("candidate-deny", "always_deny", true, "deny-pattern"),
        ],
        vec![],
        vec![comparison_case_inline_with_transition(
            "input",
            Some(expected_transition(
                comparison_state(true, false, true, "deny"),
                comparison_state(false, false, true, "deny"),
            )),
        )],
    );
    let comparison_file = fixture.write_json("bucket-only.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("1 matched transition"));
    assert!(stderr.is_empty());
}

#[test]
fn accepts_default_only_final_decision_transition() {
    let fixture = Fixture::new();
    fixture.write("never-pattern", b"^never$");
    let manifest = comparison_manifest_with_catalogs(
        "confirm",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "never-pattern",
        )],
        "allow",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "never-pattern",
        )],
        vec![],
        vec![comparison_case_inline_with_transition(
            "input",
            Some(expected_transition(
                comparison_state(false, false, false, "confirm"),
                comparison_state(false, false, false, "allow"),
            )),
        )],
    );
    let comparison_file = fixture.write_json("default-only.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("1 matched transition"));
    assert!(stderr.is_empty());
}

#[test]
fn rejects_incomplete_contradictory_noop_and_unknown_transitions() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"^input$");
    let valid_pattern = comparison_pattern("pattern", "always_allow", true, "pattern");
    let valid_baseline = comparison_state(false, false, false, "deny");
    let valid_candidate = comparison_state(true, false, false, "allow");
    let cases = [
        (
            "incomplete",
            serde_json::json!({
                "baseline": {
                    "always_allow": false,
                    "always_confirm": false,
                    "final_decision": "deny",
                },
                "candidate": valid_candidate.clone(),
            }),
            "required comparison schema",
        ),
        (
            "contradictory",
            expected_transition(
                comparison_state(false, false, false, "allow"),
                valid_candidate.clone(),
            ),
            "contradicts configured precedence",
        ),
        (
            "noop",
            expected_transition(valid_baseline.clone(), valid_baseline.clone()),
            "is a no-op",
        ),
        (
            "unknown-field",
            serde_json::json!({
                "baseline": valid_baseline,
                "candidate": valid_candidate,
                "private_transition_body": true,
            }),
            "required comparison schema",
        ),
    ];

    for (name, transition, expected_error) in cases {
        let manifest = comparison_manifest_with_catalogs(
            "deny",
            vec![valid_pattern.clone()],
            "deny",
            vec![valid_pattern.clone()],
            vec![],
            vec![comparison_case_inline_with_transition(
                "private-input",
                Some(transition),
            )],
        );
        let comparison_file = fixture.write_json(name, &manifest);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2, "Transition `{name}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error: {stderr}"
        );
        assert!(!stderr.contains("private-input"));
        assert!(!stderr.contains("private_transition_body"));
    }
}

#[test]
fn reports_baseline_and_candidate_transition_mismatches_by_side() {
    let fixture = Fixture::new();
    fixture.write("baseline-pattern", b"^input$");
    fixture.write("candidate-pattern", b"^input$");
    let baseline_observed = comparison_state(false, true, false, "confirm");
    let candidate_observed = comparison_state(true, false, false, "allow");
    let no_match = comparison_state(false, false, false, "deny");
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_confirm",
            true,
            "baseline-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "candidate-pattern",
        )],
        vec![],
        vec![
            comparison_case_inline_with_transition(
                "input",
                Some(expected_transition(
                    no_match.clone(),
                    candidate_observed.clone(),
                )),
            ),
            comparison_case_inline_with_transition(
                "input",
                Some(expected_transition(baseline_observed, no_match)),
            ),
        ],
    );
    let comparison_file = fixture.write_json("side-mismatches.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("2 mismatches across 2 comparison cases"));
    assert!(stderr.contains("Case 1 baseline differs in `always_confirm` bucket, final decision"));
    assert!(stderr.contains("Case 2 candidate differs in `always_allow` bucket, final decision"));
    assert!(!stderr.contains("^input$"));
}

#[test]
fn rejects_undeclared_bucket_and_decision_drift() {
    let fixture = Fixture::new();
    fixture.write("pattern", b"^private-input$");
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_confirm",
            true,
            "pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_allow",
            true,
            "pattern",
        )],
        vec![],
        vec![comparison_case_inline_with_transition(
            "private-input",
            None,
        )],
    );
    let comparison_file = fixture.write_json("undeclared.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("Case 1 baseline/candidate differ in"));
    assert!(stderr.contains("`always_allow` bucket"));
    assert!(stderr.contains("`always_confirm` bucket"));
    assert!(stderr.contains("final decision"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn limits_transition_mismatch_output_without_inputs_or_patterns() {
    let fixture = Fixture::new();
    fixture.write("never-pattern", b"^private-never-pattern$");
    let cases = (1..=12)
        .map(|index| {
            comparison_case_inline_with_transition(
                &format!("private-input-{index}"),
                Some(expected_transition(
                    comparison_state(true, false, false, "allow"),
                    comparison_state(false, true, false, "confirm"),
                )),
            )
        })
        .collect();
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![comparison_pattern(
            "baseline",
            "always_allow",
            true,
            "never-pattern",
        )],
        "deny",
        vec![comparison_pattern(
            "candidate",
            "always_confirm",
            true,
            "never-pattern",
        )],
        vec![],
        cases,
    );
    let comparison_file = fixture.write_json("bounded-transition.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("12 mismatches across 12 comparison cases"));
    assert_eq!(stderr.matches("  Case ").count(), 10);
    assert!(stderr.contains("… 2 additional mismatches omitted"));
    assert!(!stderr.contains("Case 11 "));
    assert!(!stderr.contains("private-input"));
    assert!(!stderr.contains("private-never-pattern"));
}

#[test]
fn verifies_mixed_suite_patterns_with_catalog_metadata() {
    let fixture = Fixture::new();
    fixture.write("ordinary-deny", b"^BLOCK$");
    let catalog = write_catalog_fixture(
        &fixture,
        "suite-catalog",
        &[CatalogPatternFixture {
            bucket: "always_allow",
            case_sensitive: false,
            contents: b"^lowercase$",
            id: "catalog-allow",
        }],
    );
    let suite_file = fixture.write(
        "catalog-suite",
        format!(
            concat!(
                "catalog-pattern\tcandidate\tcatalog-allow\n",
                "pattern-case\tcatalog-allow\tmatch\tLOWERCASE\n",
                "pattern\tordinary-deny\talways_deny\tcase-sensitive\tordinary-deny\n",
                "pattern-case\tordinary-deny\tmatch\tBLOCK\n",
                "decision-case\tallow\tLOWERCASE\n",
                "decision-case\tdeny\tBLOCK\n",
                "default\tconfirm\n",
                "pattern-catalog\tcandidate\t{}\t{}\t{}"
            ),
            catalog.catalog_file, catalog.candidate_file, catalog.state_file
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified 2 pattern cases and 2 decision cases across 2 patterns\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn loads_empty_catalogs_without_counting_declarations_as_suite_patterns() {
    let fixture = Fixture::new();
    fixture.write("ordinary", b"^input$");
    let catalog = write_catalog_fixture(&fixture, "empty-suite-catalog", &[]);
    let catalog_record = format!(
        "pattern-catalog\tempty\t{}\t{}\t{}",
        catalog.catalog_file, catalog.candidate_file, catalog.state_file
    );
    let suite_file = fixture.write(
        "empty-catalog-suite",
        format!(
            concat!(
                "default\tdeny\n",
                "pattern\tordinary\talways_allow\tcase-sensitive\tordinary\n",
                "pattern-case\tordinary\tmatch\tinput\n",
                "decision-case\tallow\tinput\n",
                "{}"
            ),
            catalog_record
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified 1 pattern case and 1 decision case across 1 pattern\n"
    );
    assert!(stderr.is_empty());

    let declaration_only_suite = fixture.write(
        "empty-catalog-declaration-only-suite",
        format!("default\tdeny\ndecision-case\tdeny\tprivate-input\n{catalog_record}").as_bytes(),
    );
    let (status, stdout, stderr) = run_with_suite(&declaration_only_suite);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("must define at least one pattern"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn loads_empty_catalogs_without_adding_comparison_patterns() {
    let fixture = Fixture::new();
    fixture.write("ordinary", b"^input$");
    let catalog = write_catalog_fixture(&fixture, "empty-comparison-catalog", &[]);
    let ordinary = comparison_pattern("ordinary", "always_allow", true, "ordinary");
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![ordinary.clone()],
        "deny",
        vec![ordinary.clone()],
        vec![catalog_definition("empty", &catalog)],
        vec![comparison_case_inline_with_transition("input", None)],
    );
    let comparison_file = fixture.write_json("empty-catalog-comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("1 baseline pattern, and 1 candidate pattern"));
    assert!(stderr.is_empty());

    let declaration_only = comparison_manifest_with_catalogs(
        "deny",
        vec![],
        "deny",
        vec![ordinary],
        vec![catalog_definition("empty", &catalog)],
        vec![comparison_case_inline_with_transition(
            "private-input",
            None,
        )],
    );
    let declaration_only_file = fixture.write_json(
        "empty-catalog-declaration-only-comparison.json",
        &declaration_only,
    );
    let (status, stdout, stderr) = run_with_comparison(&declaration_only_file);

    assert_eq!(status, 0);
    assert!(stdout.contains("0 baseline patterns, and 1 candidate pattern"));
    assert!(stderr.is_empty());
    assert!(!stdout.contains("private-input"));
}

#[test]
fn verifies_mixed_catalog_and_file_patterns_in_comparison() {
    let fixture = Fixture::new();
    fixture.write("baseline-change", b"^change$");
    fixture.write("shared", b"^shared$");
    let catalog = write_catalog_fixture(
        &fixture,
        "comparison-catalog",
        &[CatalogPatternFixture {
            bucket: "always_allow",
            case_sensitive: false,
            contents: b"^change$",
            id: "candidate-change",
        }],
    );
    let manifest = comparison_manifest_with_catalogs(
        "deny",
        vec![
            comparison_pattern(
                "baseline-change",
                "always_confirm",
                false,
                "baseline-change",
            ),
            comparison_pattern("baseline-shared", "always_allow", true, "shared"),
        ],
        "deny",
        vec![
            comparison_catalog_pattern("candidate", "candidate-change"),
            comparison_pattern("candidate-shared", "always_allow", true, "shared"),
        ],
        vec![catalog_definition("candidate", &catalog)],
        vec![
            comparison_case_inline_with_transition(
                "CHANGE",
                Some(expected_transition(
                    comparison_state(false, true, false, "confirm"),
                    comparison_state(true, false, false, "allow"),
                )),
            ),
            comparison_case_inline_with_transition("shared", None),
        ],
    );
    let comparison_file = fixture.write_json("mixed-catalog-comparison.json", &manifest);

    let (status, stdout, stderr) = run_with_comparison(&comparison_file);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified a representative comparison corpus with 1 equivalence case, 1 matched transition, 2 baseline patterns, and 2 candidate patterns\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn rejects_unknown_duplicate_and_conflicting_comparison_catalog_ids() {
    let fixture = Fixture::new();
    fixture.write("ordinary", b"^input$");
    let catalog = write_catalog_fixture(
        &fixture,
        "id-catalog",
        &[CatalogPatternFixture {
            bucket: "always_allow",
            case_sensitive: true,
            contents: b"^input$",
            id: "catalog-pattern",
        }],
    );
    let ordinary = comparison_pattern("ordinary", "always_allow", true, "ordinary");
    let cases = [
        (
            "duplicate-catalog",
            vec![ordinary.clone()],
            vec![
                catalog_definition("duplicate", &catalog),
                catalog_definition("duplicate", &catalog),
            ],
            "Duplicate pattern catalog ID `duplicate`",
        ),
        (
            "unknown-catalog",
            vec![comparison_catalog_pattern("missing", "catalog-pattern")],
            vec![],
            "Unknown pattern catalog ID `missing`",
        ),
        (
            "unknown-pattern",
            vec![comparison_catalog_pattern("known", "missing-pattern")],
            vec![catalog_definition("known", &catalog)],
            "Unknown pattern ID `missing-pattern`",
        ),
        (
            "duplicate-pattern",
            vec![
                comparison_pattern("catalog-pattern", "always_allow", true, "ordinary"),
                comparison_catalog_pattern("known", "catalog-pattern"),
            ],
            vec![catalog_definition("known", &catalog)],
            "Duplicate pattern ID `catalog-pattern`",
        ),
    ];

    for (name, baseline_patterns, catalogs, expected_error) in cases {
        let manifest = comparison_manifest_with_catalogs(
            "deny",
            baseline_patterns,
            "deny",
            vec![ordinary.clone()],
            catalogs,
            vec![comparison_case_inline_with_transition(
                "private-input",
                None,
            )],
        );
        let comparison_file = fixture.write_json(name, &manifest);
        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2, "Comparison `{name}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error: {stderr}"
        );
        assert!(!stderr.contains("private-input"));
    }
}

#[test]
fn rejects_unknown_duplicate_and_conflicting_suite_catalog_ids() {
    let fixture = Fixture::new();
    fixture.write("ordinary", b"^input$");
    let catalog = write_catalog_fixture(
        &fixture,
        "suite-id-catalog",
        &[CatalogPatternFixture {
            bucket: "always_allow",
            case_sensitive: true,
            contents: b"^input$",
            id: "catalog-pattern",
        }],
    );
    let declarations = format!(
        "pattern-catalog\tknown\t{}\t{}\t{}",
        catalog.catalog_file, catalog.candidate_file, catalog.state_file
    );
    let manifests = [
        (
            "suite-unknown-catalog",
            concat!(
                "default\tdeny\n",
                "catalog-pattern\tmissing\tcatalog-pattern\n",
                "pattern-case\tcatalog-pattern\tmatch\tinput\n",
                "decision-case\tallow\tinput"
            )
            .to_owned(),
            "Unknown pattern catalog ID `missing`",
        ),
        (
            "suite-unknown-pattern",
            format!(
                concat!(
                    "default\tdeny\n",
                    "catalog-pattern\tknown\tmissing-pattern\n",
                    "pattern-case\tmissing-pattern\tmatch\tinput\n",
                    "decision-case\tallow\tinput\n",
                    "{}"
                ),
                declarations
            ),
            "Unknown pattern ID `missing-pattern`",
        ),
        (
            "suite-duplicate-catalog",
            format!(
                concat!(
                    "default\tdeny\n",
                    "catalog-pattern\tknown\tcatalog-pattern\n",
                    "pattern-case\tcatalog-pattern\tmatch\tinput\n",
                    "decision-case\tallow\tinput\n",
                    "{}\n",
                    "{}"
                ),
                declarations, declarations
            ),
            "Duplicate pattern catalog ID `known`",
        ),
        (
            "suite-duplicate-pattern",
            format!(
                concat!(
                    "default\tdeny\n",
                    "pattern\tcatalog-pattern\talways_allow\tcase-sensitive\tordinary\n",
                    "catalog-pattern\tknown\tcatalog-pattern\n",
                    "decision-case\tallow\tinput\n",
                    "{}"
                ),
                declarations
            ),
            "Duplicate pattern ID `catalog-pattern`",
        ),
    ];

    for (name, manifest, expected_error) in manifests {
        let suite_file = fixture.write(name, manifest.as_bytes());
        let (status, stdout, stderr) = run_with_suite(&suite_file);

        assert_eq!(status, 2, "Suite `{name}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error: {stderr}"
        );
    }
}

#[test]
fn rejects_stale_catalog_candidate_and_state_bytes() {
    let fixture = Fixture::new();

    for (index, source, expected_error) in [
        (1, "candidate", "Candidate SHA-256"),
        (2, "state", "State SHA-256"),
    ] {
        let directory = format!("stale-{index}");
        let catalog = write_catalog_fixture(
            &fixture,
            &directory,
            &[CatalogPatternFixture {
                bucket: "always_allow",
                case_sensitive: true,
                contents: b"^private-pattern$",
                id: "catalog-pattern",
            }],
        );
        let stale_file = if source == "candidate" {
            &catalog.candidate_file
        } else {
            &catalog.state_file
        };
        fs::write(fixture.path(stale_file), b"private-stale-source")
            .expect("Failed to stale catalog source");
        let suite_file = fixture.write(
            &format!("stale-suite-{index}"),
            format!(
                concat!(
                    "default\tdeny\n",
                    "catalog-pattern\tknown\tcatalog-pattern\n",
                    "pattern-case\tcatalog-pattern\tmatch\tprivate-input\n",
                    "decision-case\tallow\tprivate-input\n",
                    "pattern-catalog\tknown\t{}\t{}\t{}"
                ),
                catalog.catalog_file, catalog.candidate_file, catalog.state_file
            )
            .as_bytes(),
        );

        let (status, stdout, stderr) = run_with_suite(&suite_file);

        assert_eq!(status, 2);
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error: {stderr}"
        );
        assert!(!stderr.contains("private-stale-source"));
        assert!(!stderr.contains("private-pattern"));
        assert!(!stderr.contains("private-input"));
    }
}

#[test]
fn rejects_tampered_newline_modified_missing_malformed_and_invalid_utf8_catalog_artifacts() {
    let fixture = Fixture::new();

    for (name, mutation, expected_error) in [
        ("tampered", "tamper", "SHA-256 does not match"),
        ("newline", "newline", "SHA-256 does not match"),
        ("missing", "missing", "Failed to open catalog pattern"),
        ("malformed", "malformed", "Artifact catalog JSON"),
        ("invalid-utf8", "invalid-utf8", "not valid UTF-8"),
    ] {
        let contents: &[u8] = if mutation == "invalid-utf8" {
            &[0xff]
        } else {
            b"^private-pattern$"
        };
        let catalog = write_catalog_fixture(
            &fixture,
            name,
            &[CatalogPatternFixture {
                bucket: "always_allow",
                case_sensitive: true,
                contents,
                id: "catalog-pattern",
            }],
        );
        match mutation {
            "tamper" => fs::write(
                fixture.path(&catalog.artifact_files[0]),
                b"^private-tampered-pattern$",
            )
            .expect("Failed to tamper with artifact"),
            "newline" => fs::write(
                fixture.path(&catalog.artifact_files[0]),
                b"^private-pattern$\n",
            )
            .expect("Failed to add artifact newline"),
            "missing" => fs::remove_file(fixture.path(&catalog.artifact_files[0]))
                .expect("Failed to remove artifact"),
            "malformed" => fs::write(fixture.path(&catalog.catalog_file), b"private-json{")
                .expect("Failed to corrupt catalog"),
            "invalid-utf8" => {}
            _ => unreachable!(),
        }
        let manifest = comparison_manifest_with_catalogs(
            "deny",
            vec![comparison_catalog_pattern("known", "catalog-pattern")],
            "deny",
            vec![comparison_catalog_pattern("known", "catalog-pattern")],
            vec![catalog_definition("known", &catalog)],
            vec![comparison_case_inline_with_transition(
                "private-input",
                None,
            )],
        );
        let comparison_file = fixture.write_json(&format!("{name}-comparison.json"), &manifest);

        let (status, stdout, stderr) = run_with_comparison(&comparison_file);

        assert_eq!(status, 2, "Artifact case `{name}` unexpectedly succeeded");
        assert!(stdout.is_empty());
        assert!(
            stderr.contains(expected_error),
            "Unexpected error: {stderr}"
        );
        assert!(!stderr.contains("private-pattern"));
        assert!(!stderr.contains("private-tampered-pattern"));
        assert!(!stderr.contains("private-input"));
        assert!(!stderr.contains("private-json"));
    }
}

#[test]
fn rejects_catalog_backed_compile_errors_without_body_leakage() {
    let fixture = Fixture::new();
    let catalog = write_catalog_fixture(
        &fixture,
        "compile-catalog",
        &[CatalogPatternFixture {
            bucket: "always_allow",
            case_sensitive: true,
            contents: b"private-catalog-regex(",
            id: "catalog-invalid",
        }],
    );
    let suite_file = fixture.write(
        "compile-catalog-suite",
        format!(
            concat!(
                "default\tdeny\n",
                "catalog-pattern\tknown\tcatalog-invalid\n",
                "pattern-case\tcatalog-invalid\tno-match\tprivate-input\n",
                "decision-case\tdeny\tprivate-input\n",
                "pattern-catalog\tknown\t{}\t{}\t{}"
            ),
            catalog.catalog_file, catalog.candidate_file, catalog.state_file
        )
        .as_bytes(),
    );

    let (status, stdout, stderr) = run_with_suite(&suite_file);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("catalog-backed suite pattern `catalog-invalid`"));
    assert!(!stderr.contains("private-catalog-regex"));
    assert!(!stderr.contains("private-input"));
}

#[test]
fn evaluates_terminal_and_fetch_configured_layers() {
    let fixture = Fixture::new();
    let settings = fixture.write_json(
        "layer-settings.json",
        &serde_json::json!({
            "agent": {
                "tool_permissions": {
                    "tools": {
                        "fetch": {
                            "default": "confirm",
                            "always_allow": [{
                                "pattern": "^https://host/(?:approved|other)$",
                                "case_sensitive": true
                            }],
                            "always_confirm": [{
                                "pattern": "^https://host/other$",
                                "case_sensitive": true
                            }]
                        },
                        "terminal": {
                            "default": "deny",
                            "always_allow": [{
                                "pattern": "^(?:allowed|shared)$",
                                "case_sensitive": true
                            }],
                            "always_confirm": [{
                                "pattern": "^shared$",
                                "case_sensitive": true
                            }],
                            "always_deny": [{
                                "pattern": "^denied$",
                                "case_sensitive": true
                            }]
                        }
                    }
                }
            }
        }),
    );
    let settings_file = settings.file_name().unwrap().to_str().unwrap();
    let terminal = fixture.write_json(
        "terminal-layer.json",
        &serde_json::json!({
            "settings_file": settings_file,
            "tool": "terminal",
            "pattern_cases": [
                {"type": "inline", "id": "allow-pattern", "bucket": "always_allow", "index": 0, "input": "allowed", "expected_match": true},
                {"type": "inline", "id": "confirm-pattern", "bucket": "always_confirm", "index": 0, "input": "shared", "expected_match": true},
                {"type": "inline", "id": "deny-pattern", "bucket": "always_deny", "index": 0, "input": "denied", "expected_match": true}
            ],
            "settled_inputs": [
                {"type": "inline", "id": "allowed", "input": "allowed", "expected_decision": "allow"},
                {"type": "inline", "id": "shared", "input": "shared", "expected_decision": "confirm"},
                {"type": "inline", "id": "denied", "input": "denied", "expected_decision": "deny"},
                {"type": "inline", "id": "defaulted", "input": "unmatched", "expected_decision": "deny"}
            ],
            "aggregate_cases": [{"id": "terminal-aggregate", "inputs": ["allowed", "shared"], "expected_decision": "confirm"}]
        }),
    );
    let fetch = fixture.write_json(
        "fetch-layer.json",
        &serde_json::json!({
            "settings_file": settings_file,
            "tool": "fetch",
            "pattern_cases": [
                {"type": "inline", "id": "fetch-allow", "bucket": "always_allow", "index": 0, "input": "https://host/approved", "expected_match": true},
                {"type": "inline", "id": "fetch-confirm", "bucket": "always_confirm", "index": 0, "input": "https://host/other", "expected_match": true}
            ],
            "settled_inputs": [
                {"type": "inline", "id": "approved", "input": "https://host/approved", "expected_decision": "allow"},
                {"type": "inline", "id": "other", "input": "https://host/other", "expected_decision": "confirm"},
                {"type": "inline", "id": "fetch-default", "input": "http://host/approved", "expected_decision": "confirm"}
            ],
            "aggregate_cases": [{"id": "fetch-aggregate", "inputs": ["approved", "fetch-default"], "expected_decision": "confirm"}]
        }),
    );

    let (terminal_status, terminal_stdout, terminal_stderr) = run_with_layer(&terminal);
    assert_eq!(terminal_status, 0, "{terminal_stderr}");
    assert!(terminal_stdout.contains(
        "Verified 3 pattern cases, 4 settled cases, and 1 aggregate case across 3 configured patterns"
    ));
    let (fetch_status, fetch_stdout, fetch_stderr) = run_with_layer(&fetch);
    assert_eq!(fetch_status, 0, "{fetch_stderr}");
    assert!(fetch_stdout.contains(
        "Verified 2 pattern cases, 3 settled cases, and 1 aggregate case across 2 configured patterns"
    ));
}

#[test]
fn requires_tool_and_rejects_legacy_layer_defaults() {
    let fixture = Fixture::new();
    fixture.write_json(
        "settings.json",
        &serde_json::json!({
            "agent": {"tool_permissions": {"tools": {"terminal": {
                "default": "confirm",
                "always_allow": [],
                "always_confirm": [],
                "always_deny": []
            }}}}
        }),
    );
    let valid = serde_json::json!({
        "settings_file": "settings.json",
        "tool": "terminal",
        "pattern_cases": [],
        "settled_inputs": [{"type": "inline", "id": "default", "input": "input", "expected_decision": "confirm"}]
    });
    let mut missing_tool = valid.clone();
    missing_tool.as_object_mut().unwrap().remove("tool");
    let mut legacy_default = valid.clone();
    legacy_default["default"] = serde_json::json!("confirm");
    let mut invalid_tool = valid;
    invalid_tool["tool"] = serde_json::json!("browser");

    for (name, manifest) in [
        ("missing-tool", missing_tool),
        ("legacy-default", legacy_default),
        ("invalid-tool", invalid_tool),
    ] {
        let path = fixture.write_json(&format!("{name}.json"), &manifest);
        let (status, stdout, stderr) = run_with_layer(&path);
        assert_eq!(status, 2, "{name}: {stderr}");
        assert!(stdout.is_empty());
        assert!(stderr.contains("required schema"), "{name}: {stderr}");
    }
}

#[test]
fn validates_only_the_selected_tool_and_rejects_malformed_fetch_values() {
    let fixture = Fixture::new();
    let valid_fetch = serde_json::json!({
        "default": "confirm",
        "always_allow": [],
        "always_confirm": []
    });
    let settings = fixture.write_json(
        "selected-tool-settings.json",
        &serde_json::json!({
            "agent": {"tool_permissions": {"tools": {
                "fetch": valid_fetch,
                "terminal": "private-malformed-terminal"
            }}}
        }),
    );
    let manifest = serde_json::json!({
        "settings_file": settings.file_name().unwrap().to_str().unwrap(),
        "tool": "fetch",
        "pattern_cases": [],
        "settled_inputs": [{"type": "inline", "id": "default", "input": "https://example.com/", "expected_decision": "confirm"}]
    });
    let manifest_path = fixture.write_json("selected-fetch.json", &manifest);
    let (status, _, stderr) = run_with_layer(&manifest_path);
    assert_eq!(status, 0, "{stderr}");

    for (name, malformed_fetch) in [
        ("missing-default", serde_json::json!({"always_allow": []})),
        (
            "malformed-bucket",
            serde_json::json!({"default": "confirm", "always_allow": "private-malformed-fetch"}),
        ),
        (
            "malformed-pattern",
            serde_json::json!({"default": "confirm", "always_allow": ["private-malformed-pattern"]}),
        ),
        (
            "malformed-pattern-object",
            serde_json::json!({"default": "confirm", "always_allow": [{"pattern": 7, "case_sensitive": true}]}),
        ),
    ] {
        fixture.write_json(
            "selected-tool-settings.json",
            &serde_json::json!({
                "agent": {"tool_permissions": {"tools": {"fetch": malformed_fetch}}}
            }),
        );
        let (status, stdout, stderr) = run_with_layer(&manifest_path);
        assert_eq!(status, 2, "{name}: {stderr}");
        assert!(stdout.is_empty());
        assert!(!stderr.contains("private-malformed-fetch"));
        assert!(!stderr.contains("private-malformed-pattern"));
    }
}

#[test]
fn requires_complete_layer_pattern_case_coverage() {
    let fixture = Fixture::new();
    fixture.write_json(
        "coverage-settings.json",
        &serde_json::json!({
            "agent": {"tool_permissions": {"tools": {"fetch": {
                "default": "confirm",
                "always_allow": [{"pattern": "^https://example\\.com/$", "case_sensitive": true}]
            }}}}
        }),
    );
    let base = serde_json::json!({
        "settings_file": "coverage-settings.json",
        "tool": "fetch",
        "pattern_cases": [],
        "settled_inputs": [{"type": "inline", "id": "settled", "input": "https://example.com/", "expected_decision": "allow"}]
    });
    let missing = fixture.write_json("missing-coverage.json", &base);
    let (status, _, stderr) = run_with_layer(&missing);
    assert_eq!(status, 2);
    assert!(stderr.contains("must cover configured pattern `always_allow[0]`"));

    let mut unknown = base.clone();
    unknown["pattern_cases"] = serde_json::json!([{
        "type": "inline", "id": "unknown", "bucket": "always_allow", "index": 1,
        "input": "https://example.com/", "expected_match": true
    }]);
    let unknown = fixture.write_json("unknown-position.json", &unknown);
    let (status, _, stderr) = run_with_layer(&unknown);
    assert_eq!(status, 2);
    assert!(stderr.contains("unknown configured pattern `always_allow[1]`"));

    let mut multiline = base;
    multiline["pattern_cases"] = serde_json::json!([{
        "type": "inline", "id": "multiline", "bucket": "always_allow", "index": 0,
        "input": "private-layer-input\ncontinued", "expected_match": false
    }]);
    let multiline = fixture.write_json("multiline-pattern-case.json", &multiline);
    let (status, _, stderr) = run_with_layer(&multiline);
    assert_eq!(status, 2);
    assert!(stderr.contains("pattern-case inputs must be single-line"));
    assert!(!stderr.contains("private-layer-input"));
}

#[test]
fn binds_file_backed_layer_pattern_cases_to_evidence() {
    let fixture = Fixture::new();
    let settings = fixture.write_json(
        "evidence-settings.json",
        &serde_json::json!({
            "agent": {"tool_permissions": {"tools": {"fetch": {
                "default": "confirm",
                "always_allow": [{"pattern": "^https://example\\.com/$", "case_sensitive": true}]
            }}}}
        }),
    );
    fixture.write("pattern-case-input.txt", b"https://example.com/");
    fixture.write("settled-input.txt", b"https://example.com/");
    let manifest = fixture.write_json(
        "evidence-layer.json",
        &serde_json::json!({
            "settings_file": settings.file_name().unwrap().to_str().unwrap(),
            "tool": "fetch",
            "pattern_cases": [{
                "type": "file", "id": "pattern-case", "bucket": "always_allow", "index": 0,
                "input_file": "pattern-case-input.txt", "expected_match": true
            }],
            "settled_inputs": [{
                "type": "file", "id": "settled", "input_file": "settled-input.txt",
                "expected_decision": "allow"
            }]
        }),
    );
    let result_path = fixture.path("layer-result.json");

    let (status, stdout, stderr) = run_with_layer_evidence(&manifest, &fixture.root, &result_path);
    assert_eq!(status, 0, "{stderr}");
    assert!(stdout.contains("Verified 1 pattern case, 1 settled case"));
    let result: serde_json::Value =
        serde_json::from_slice(&fs::read(&result_path).unwrap()).unwrap();
    assert_eq!(result["kind"], serde_json::json!("layer_decision"));
    assert_eq!(result["counts"]["pattern_cases"], serde_json::json!(1));
    assert_eq!(
        result["bound_inputs"]["settings_sha256"],
        serde_json::json!(helper::sha256_hex(&fs::read(&settings).unwrap()))
    );
    let input_paths = result["bound_inputs"]["input_closure"]["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|record| record["role"] == serde_json::json!("input_file"))
        .map(|record| record["path"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(input_paths, ["pattern-case-input.txt", "settled-input.txt"]);
}

#[test]
fn bounds_layer_failure_details_without_input_or_pattern_bodies() {
    let fixture = Fixture::new();
    fixture.write_json(
        "bounded-settings.json",
        &serde_json::json!({
            "agent": {"tool_permissions": {"tools": {"fetch": {
                "default": "confirm",
                "always_allow": [{"pattern": "^private-layer-input$", "case_sensitive": true}]
            }}}}
        }),
    );
    let cases = (0..11)
        .map(|index| {
            serde_json::json!({
                "type": "inline",
                "id": format!("case-{index}"),
                "bucket": "always_allow",
                "index": 0,
                "input": "private-layer-input",
                "expected_match": false
            })
        })
        .collect::<Vec<_>>();
    let manifest = fixture.write_json(
        "bounded-layer.json",
        &serde_json::json!({
            "settings_file": "bounded-settings.json",
            "tool": "fetch",
            "pattern_cases": cases,
            "settled_inputs": [{
                "type": "inline", "id": "decision", "input": "private-layer-input",
                "expected_decision": "confirm"
            }]
        }),
    );

    let (status, stdout, stderr) = run_with_layer(&manifest);
    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert_eq!(stderr.matches("  case-").count(), 10);
    assert!(stderr.contains("… 2 additional case details omitted"));
    assert!(!stderr.contains("private-layer-input"));
    assert!(!stderr.contains("^private-layer-input$"));
}

#[test]
fn returns_success_for_help() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let status = helper::run(vec![OsString::from("--help")], &mut stdout, &mut stderr);

    assert_eq!(status, 0);
    let stdout = String::from_utf8(stdout).expect("Standard output must be valid UTF-8");

    assert!(stdout.starts_with("Usage:\n  pattern-match"));
    assert!(stdout.contains("--cases-file"));
    assert!(stdout.contains("--comparison-file <path>"));
    assert!(stdout.contains("--suite-file"));
    assert!(stdout.contains("configured pattern precedence"));
    assert!(stdout.contains("default<TAB>allow|confirm|deny"));
    assert!(stdout.contains("pattern-case<TAB><id><TAB>match|no-match<TAB><input>"));
    assert!(stdout.contains("pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>"));
    assert!(stdout.contains("decision-case<TAB>allow|confirm|deny<TAB><input>"));
    assert!(stdout.contains("decision-case-file<TAB>allow|confirm|deny<TAB><input-file>"));
    assert!(stdout.contains("Suite requirements:"));
    assert!(stdout.contains("strict artifact catalog schema"));
    assert!(stdout.contains("Ownership lives in the owner spec, not the catalog"));
    assert!(stdout.contains("Catalog declarations do not count as patterns"));
    assert!(stdout.contains("at least one `decision-case` or `decision-case-file` record"));
    assert!(stdout.contains(
        "at least one `pattern-case` or `pattern-case-file` record for every pattern ID"
    ));
    assert!(stdout.contains("file-backed records for multiline inputs"));
    assert!(stdout.contains("does not reproduce full Zed permission evaluation"));
    assert!(stdout.contains("Strict UTF-8 JSON comparison manifest"));
    assert!(stdout.contains("Root requires `catalogs`, which may be empty"));
    assert!(stdout.contains("Either pattern set may be empty"));
    assert!(stdout.contains(
        "An empty set has no bucket matches and resolves each case from its configured default"
    ));
    assert!(stdout.contains("Define at least one case"));
    assert!(stdout.contains("Comparison covers only the representative corpus"));
    assert!(stdout.contains(
        "does not reproduce full Zed permission evaluation or establish formal language equivalence"
    ));
    assert!(stdout.contains(
        "comparison checks whether each bucket matched and compares the configured final decision"
    ));
    assert!(
        stdout.contains("--layer-file <path>       Evaluate configured-pattern-layer decisions")
    );
    assert!(stdout.contains("\"tool\":\"fetch|terminal\""));
    assert!(
        stdout.contains("Every configured pattern requires at least one independent pattern case")
    );
    assert!(stdout.contains("reads the selected tool’s configured default"));
    assert!(!stdout.contains("\"default\":\"allow|confirm|deny\",\"raw_provenance\""));
    assert!(stderr.is_empty());
}

#[test]
fn comparison_help_documents_the_exact_case_schema_the_parser_accepts() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(vec![OsString::from("--help")], &mut stdout, &mut stderr);
    assert_eq!(status, 0);
    let help = String::from_utf8(stdout).expect("Standard output must be valid UTF-8");

    for shape in [
        "Inline case: {\"type\":\"inline\",\"input\":\"...\",\"expected_transition\":<transition>}",
        "File case: {\"type\":\"file\",\"input_file\":\"path\",\"expected_transition\":<transition>}",
        "Transition: {\"baseline\":<state>,\"candidate\":<state>}",
        "State: {\"always_allow\":true|false,\"always_confirm\":true|false,\"always_deny\":true|false,\"final_decision\":\"allow|confirm|deny\"}",
        "Cases carry no `id`",
    ] {
        assert!(help.contains(shape), "Help must document `{shape}`");
    }

    // The documented inline shape, including a complete transition, must parse
    let fixture = Fixture::new();
    let pattern_file = fixture.write("comparison-help-pattern.txt", b"^fx run$");
    let candidate_patterns = vec![serde_json::json!({
        "type": "file",
        "id": "fx",
        "bucket": "always_allow",
        "case_sensitive": true,
        "pattern_file": pattern_file.to_string_lossy(),
    })];
    let documented = comparison_manifest(
        "confirm",
        vec![],
        "confirm",
        candidate_patterns.clone(),
        vec![comparison_case_inline_with_transition(
            "fx run",
            Some(expected_transition(
                comparison_state(false, false, false, "confirm"),
                comparison_state(true, false, false, "allow"),
            )),
        )],
    );
    let accepted = fixture.write_json("comparison-help-accepted.json", &documented);
    let (status, _, stderr) = run_with_comparison(&accepted);
    assert_eq!(status, 0, "{stderr}");

    // `id` is documented as absent, so the strict parser must reject it
    let mut with_id = documented;
    with_id["cases"][0]["id"] = serde_json::json!("case-1");
    let rejected = fixture.write_json("comparison-help-rejected.json", &with_id);
    let (status, _, stderr) = run_with_comparison(&rejected);
    assert_eq!(status, 2, "A case `id` must be rejected");
    assert!(stderr.contains("comparison schema"), "{stderr}");
}
