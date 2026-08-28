#[path = "pattern_match.rs"]
mod helper;

use helper::{Bucket, Parameter, Role, Route, RouteKind};
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const ALLOW_PATTERN: &str = "^https://example\\.com/";
const CONFIRM_PATTERN: &str = "^https://example\\.com/private/";
const DENIED_INPUT: &str = "https://denied.example/secret";
const DENY_PATTERN: &str = "^https://denied\\.example/";
const MATCHING_INPUT: &str = "https://example.com/page";
const NONMATCHING_INPUT: &str = "https://other.example/page";
const PRIVATE_INPUT: &str = "https://example.com/private/notes";

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

        Self { root }
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, contents).expect("Failed to write fixture file");

        path
    }

    fn write_bytes(&self, name: &str, contents: &[u8]) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, contents).expect("Failed to write fixture file");

        path
    }

    fn missing(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn snapshot(&self) -> Vec<(String, Vec<u8>)> {
        let mut entries: Vec<(String, Vec<u8>)> = fs::read_dir(&self.root)
            .expect("Failed to read the fixture directory")
            .map(|entry| {
                let entry = entry.expect("Failed to read a fixture directory entry");
                let contents = fs::read(entry.path()).expect("Failed to read a fixture file");

                (entry.file_name().to_string_lossy().into_owned(), contents)
            })
            .collect();
        entries.sort();

        entries
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn run_arguments<I, T>(arguments: I) -> (u8, String, String)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let status = helper::run(
        arguments.into_iter().map(Into::into),
        &mut stdout,
        &mut stderr,
    );

    (
        status,
        String::from_utf8(stdout).expect("Standard output must be valid UTF-8"),
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
    )
}

fn run_layer(layer_file: &Path, settings: &Path) -> (u8, String, String) {
    run_arguments([
        OsString::from("--layer-file"),
        layer_file.as_os_str().to_owned(),
        OsString::from("--settings"),
        settings.as_os_str().to_owned(),
    ])
}

fn run_comparison(
    baseline: &Path,
    candidate: &Path,
    comparison_file: &Path,
) -> (u8, String, String) {
    run_arguments([
        OsString::from("--baseline-settings"),
        baseline.as_os_str().to_owned(),
        OsString::from("--candidate-settings"),
        candidate.as_os_str().to_owned(),
        OsString::from("--comparison-file"),
        comparison_file.as_os_str().to_owned(),
    ])
}

fn json_string(value: &str) -> String {
    let mut escaped = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            // JSON forbids a raw control character below U+0020 inside a string
            _ if character < '\u{20}' => {
                escaped.push_str(&format!("\\u{:04x}", u32::from(character)));
            }
            _ => escaped.push(character),
        }
    }
    escaped.push('"');

    escaped
}

fn pattern_entry(pattern: &str, case_sensitive: bool) -> String {
    let mut entry = String::from("{\"case_sensitive\":");
    entry.push_str(if case_sensitive { "true" } else { "false" });
    entry.push_str(",\"pattern\":");
    entry.push_str(&json_string(pattern));
    entry.push('}');

    entry
}

fn fetch_object(default: &str, buckets: &[(&str, &[(&str, bool)])]) -> String {
    let mut fields = Vec::new();
    for (bucket, patterns) in buckets {
        let entries = patterns
            .iter()
            .map(|(pattern, case_sensitive)| pattern_entry(pattern, *case_sensitive))
            .collect::<Vec<_>>()
            .join(",");
        fields.push(format!("{}:[{entries}]", json_string(bucket)));
    }
    fields.push(format!("\"default\":{}", json_string(default)));

    let mut object = String::from("{");
    object.push_str(&fields.join(","));
    object.push('}');

    object
}

fn wrap_fetch(fetch: &str) -> String {
    let mut document = String::from("{\"agent\":{\"tool_permissions\":{\"tools\":{\"fetch\":");
    document.push_str(fetch);
    document.push_str("}}}}");

    document
}

fn settings_json(default: &str, buckets: &[(&str, &[(&str, bool)])]) -> String {
    wrap_fetch(&fetch_object(default, buckets))
}

fn allow_settings() -> String {
    settings_json("confirm", &[("always_allow", &[(ALLOW_PATTERN, true)])])
}

fn full_settings() -> String {
    settings_json(
        "confirm",
        &[
            ("always_allow", &[(ALLOW_PATTERN, true)]),
            ("always_confirm", &[(CONFIRM_PATTERN, true)]),
            ("always_deny", &[(DENY_PATTERN, true)]),
        ],
    )
}

fn state(allow: bool, confirm: bool, deny: bool, decision: &str) -> String {
    format!(
        "{{\"always_allow\":{allow},\"always_confirm\":{confirm},\"always_deny\":{deny},\"decision\":\"{decision}\"}}"
    )
}

fn decision_case(expected: &str, input: &str) -> String {
    let mut case = String::from("{\"expected\":");
    case.push_str(expected);
    case.push_str(",\"input\":");
    case.push_str(&json_string(input));
    case.push('}');

    case
}

fn pattern_case(bucket: &str, expected_match: bool, index: usize, input: &str) -> String {
    format!(
        "{{\"bucket\":\"{bucket}\",\"expected_match\":{expected_match},\"index\":{index},\"input\":{}}}",
        json_string(input)
    )
}

fn layer_manifest(decision_cases: &[String], pattern_cases: &[String]) -> String {
    let mut manifest = String::from("{\"decision_cases\":[");
    manifest.push_str(&decision_cases.join(","));
    manifest.push_str("],\"pattern_cases\":[");
    manifest.push_str(&pattern_cases.join(","));
    manifest.push_str("]}");

    manifest
}

fn comparison_case(baseline: &str, candidate: &str, input: &str) -> String {
    let mut case = String::from("{\"baseline\":");
    case.push_str(baseline);
    case.push_str(",\"candidate\":");
    case.push_str(candidate);
    case.push_str(",\"input\":");
    case.push_str(&json_string(input));
    case.push('}');

    case
}

fn comparison_manifest(cases: &[String]) -> String {
    let mut manifest = String::from("{\"cases\":[");
    manifest.push_str(&cases.join(","));
    manifest.push_str("]}");

    manifest
}

fn allow_manifest() -> String {
    layer_manifest(
        &[
            decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
            decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
        ],
        &[
            pattern_case("always_allow", true, 0, MATCHING_INPUT),
            pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
        ],
    )
}

fn full_manifest() -> String {
    layer_manifest(
        &[
            decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
            decision_case(&state(true, true, false, "confirm"), PRIVATE_INPUT),
            decision_case(&state(false, false, true, "deny"), DENIED_INPUT),
            decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
        ],
        &[
            pattern_case("always_allow", true, 0, MATCHING_INPUT),
            pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            pattern_case("always_confirm", true, 0, PRIVATE_INPUT),
            pattern_case("always_confirm", false, 0, MATCHING_INPUT),
            pattern_case("always_deny", true, 0, DENIED_INPUT),
            pattern_case("always_deny", false, 0, MATCHING_INPUT),
        ],
    )
}

/// Fails when a diagnostic exposes a declared or observed bucket flag, decision, or configured
/// result. Bucket labels remain allowed because the contract requires them to locate a pattern
fn assert_declares_no_state(stderr: &str) {
    for disclosure in [
        "\"allow\"",
        "\"confirm\"",
        "\"deny\"",
        "false",
        "observed",
        "true",
    ] {
        assert!(
            !stderr.contains(disclosure),
            "A finding must not disclose `{disclosure}`"
        );
    }
}

fn help_options() -> Vec<String> {
    helper::HELP
        .lines()
        .skip_while(|line| *line != "Options:")
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .next()
                .expect("Each help option line must name an option")
                .to_owned()
        })
        .collect()
}

fn usage_options() -> Vec<Vec<String>> {
    helper::HELP
        .lines()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter(|token| token.starts_with("--"))
                .map(str::to_owned)
                .collect()
        })
        .collect()
}

#[test]
fn prints_help_alone() {
    let (status, stdout, stderr) = run_arguments(["--help"]);

    assert_eq!(status, 0);
    assert_eq!(stdout, helper::HELP);
    assert!(stderr.is_empty(), "Help must leave standard error empty");
}

#[test]
fn documents_every_accepted_option() {
    let accepted = Parameter::ALL
        .map(|parameter| parameter.option().to_owned())
        .to_vec();

    assert_eq!(
        help_options(),
        accepted,
        "The help option list must match the accepted options in alphabetical order"
    );
}

#[test]
fn documents_every_supported_route() {
    let expected = vec![
        route_options(RouteKind::Comparison),
        vec![Parameter::Help.option().to_owned()],
        route_options(RouteKind::Layer),
    ];

    assert_eq!(
        usage_options(),
        expected,
        "Each usage line must name exactly the options its route accepts"
    );
}

fn route_options(route: RouteKind) -> Vec<String> {
    Parameter::ALL
        .into_iter()
        .filter(|parameter| parameter.route() == Some(route))
        .map(|parameter| parameter.option().to_owned())
        .collect()
}

#[test]
fn parses_each_documented_route() {
    let layer = helper::parse_arguments(
        [
            "--layer-file",
            "manifest.json",
            "--settings",
            "settings.json",
        ]
        .map(OsString::from)
        .to_vec(),
    )
    .expect("The layer route must parse");
    assert_eq!(
        layer,
        Route::Layer {
            layer_file: PathBuf::from("manifest.json"),
            settings: PathBuf::from("settings.json"),
        }
    );

    let comparison = helper::parse_arguments(
        [
            "--baseline-settings",
            "baseline.json",
            "--candidate-settings",
            "candidate.json",
            "--comparison-file",
            "comparison.json",
        ]
        .map(OsString::from)
        .to_vec(),
    )
    .expect("The comparison route must parse");
    assert_eq!(
        comparison,
        Route::Comparison {
            baseline_settings: PathBuf::from("baseline.json"),
            candidate_settings: PathBuf::from("candidate.json"),
            comparison_file: PathBuf::from("comparison.json"),
        }
    );

    assert_eq!(
        helper::parse_arguments([OsString::from("--help")]).expect("The help route must parse"),
        Route::Help
    );
}

#[test]
fn rejects_options_from_different_routes() {
    let (status, stdout, stderr) = run_arguments([
        "--layer-file",
        "manifest.json",
        "--comparison-file",
        "c.json",
    ]);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "pattern-match: Options from different routes must not be combined\n"
    );
}

#[test]
fn rejects_an_incomplete_route() {
    let (status, _, stderr) = run_arguments(["--layer-file", "manifest.json"]);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer route requires `--layer-file` and `--settings`\n"
    );

    let (status, _, stderr) = run_arguments(["--baseline-settings", "baseline.json"]);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The comparison route requires `--baseline-settings`, `--candidate-settings`, and `--comparison-file`\n"
    );
}

#[test]
fn rejects_help_combined_with_a_route_option() {
    let (status, stdout, stderr) = run_arguments(["--help", "--settings", "settings.json"]);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "pattern-match: The `--help` option must be used alone\n"
    );
}

#[test]
fn rejects_repeated_options() {
    for option in Parameter::ALL {
        let mut arguments = vec![OsString::from(option.option())];
        if option != Parameter::Help {
            arguments.push(OsString::from("first"));
        }
        arguments.push(OsString::from(option.option()));
        if option != Parameter::Help {
            arguments.push(OsString::from("second"));
        }

        let (status, _, stderr) = run_arguments(arguments);

        assert_eq!(status, 2);
        assert_eq!(
            stderr,
            format!(
                "pattern-match: The `{}` option must not be repeated\n",
                option.option()
            )
        );
    }
}

#[test]
fn rejects_a_missing_option_value() {
    for option in Parameter::ALL {
        if option == Parameter::Help {
            continue;
        }

        let (status, _, stderr) = run_arguments([option.option()]);

        assert_eq!(status, 2);
        assert_eq!(
            stderr,
            format!(
                "pattern-match: The `{}` option requires a value\n",
                option.option()
            )
        );
    }
}

#[test]
fn rejects_positional_arguments() {
    let (status, stdout, stderr) = run_arguments(["/private/settings.json"]);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "pattern-match: Positional arguments are not supported\n"
    );
    assert!(
        !stderr.contains("/private/settings.json"),
        "A positional argument must not be echoed"
    );
}

#[test]
fn classifies_a_non_utf8_argument_by_its_prefix() {
    let option = OsString::from_vec(b"--fo\xffo".to_vec());
    let (status, stdout, stderr) = run_arguments([option]);

    assert_eq!(status, 2);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        "pattern-match: Unknown option. Run `pattern-match --help` for the supported options\n",
        "A malformed option token must not be reported as a positional argument"
    );

    let positional = OsString::from_vec(b"/private/fo\xffo".to_vec());
    let (status, _, stderr) = run_arguments([positional]);

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: Positional arguments are not supported\n"
    );
}

#[test]
fn rejects_an_option_joined_to_its_value() {
    let (status, _, stderr) = run_arguments(["--settings=/private/settings.json"]);

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: An option and its value must be separated by a space\n"
    );
    assert!(
        !stderr.contains("/private/settings.json"),
        "A joined value must not be echoed"
    );
}

#[test]
fn rejects_retired_options() {
    for option in [
        "--artifact-root",
        "--case-sensitive",
        "--cases-file",
        "--graph-root",
        "--input-file",
        "--pattern-file",
        "--result-out",
        "--suite-file",
    ] {
        let (status, _, stderr) = run_arguments([option]);

        assert_eq!(status, 2);
        assert_eq!(
            stderr,
            format!("pattern-match: Unknown option `{option}`\n")
        );
    }
}

#[test]
fn rejects_an_empty_invocation() {
    let (status, _, stderr) = run_arguments(Vec::<OsString>::new());

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: Select one route. Run `pattern-match --help` for the supported routes\n"
    );
}

#[test]
fn verifies_a_configured_layer() {
    let fixture = Fixture::new();
    let manifest = fixture.write("layer.json", &full_manifest());
    let settings = fixture.write("settings.json", &full_settings());

    let (status, stdout, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 0);
    assert_eq!(
        stdout,
        "Verified 3 configured patterns, 4 decision cases, and 6 pattern cases\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn applies_deny_confirm_allow_then_default_precedence() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &full_settings());

    for (input, allow, confirm, deny, decision) in [
        (MATCHING_INPUT, true, false, false, "allow"),
        (PRIVATE_INPUT, true, true, false, "confirm"),
        (DENIED_INPUT, false, false, true, "deny"),
        (NONMATCHING_INPUT, false, false, false, "confirm"),
    ] {
        let mut decision_cases = vec![decision_case(&state(allow, confirm, deny, decision), input)];
        decision_cases.extend([
            decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
            decision_case(&state(true, true, false, "confirm"), PRIVATE_INPUT),
            decision_case(&state(false, false, true, "deny"), DENIED_INPUT),
            decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
        ]);
        let manifest = fixture.write(
            "layer.json",
            &layer_manifest(
                &decision_cases,
                &[
                    pattern_case("always_allow", true, 0, MATCHING_INPUT),
                    pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
                    pattern_case("always_confirm", true, 0, PRIVATE_INPUT),
                    pattern_case("always_confirm", false, 0, MATCHING_INPUT),
                    pattern_case("always_deny", true, 0, DENIED_INPUT),
                    pattern_case("always_deny", false, 0, MATCHING_INPUT),
                ],
            ),
        );

        let (status, _, stderr) = run_layer(&manifest, &settings);

        assert_eq!(status, 0, "{input} must resolve to {decision}: {stderr}");
    }
}

#[test]
fn honors_the_configured_case_setting() {
    let fixture = Fixture::new();
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[
                decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
                decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
            ],
            &[
                pattern_case("always_allow", true, 0, MATCHING_INPUT),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            ],
        ),
    );

    let insensitive = fixture.write(
        "insensitive.json",
        &settings_json(
            "confirm",
            &[("always_allow", &[("^https://EXAMPLE\\.com/", false)])],
        ),
    );
    let (status, _, stderr) = run_layer(&manifest, &insensitive);
    assert_eq!(status, 0, "A case-insensitive pattern must match: {stderr}");

    let sensitive = fixture.write(
        "sensitive.json",
        &settings_json(
            "confirm",
            &[("always_allow", &[("^https://EXAMPLE\\.com/", true)])],
        ),
    );
    let (status, _, stderr) = run_layer(&manifest, &sensitive);
    assert_eq!(status, 1);
    assert!(stderr.contains(
        "`pattern_cases[0]` declared expectation disagrees with the configured `always_allow[0]` pattern result"
    ));
}

#[test]
fn reports_layer_findings_from_pattern_cases_to_decision_cases() {
    let fixture = Fixture::new();
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[
                decision_case(&state(true, false, false, "allow"), NONMATCHING_INPUT),
                decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
            ],
            &[
                pattern_case("always_allow", true, 0, MATCHING_INPUT),
                pattern_case("always_allow", false, 0, MATCHING_INPUT),
            ],
        ),
    );
    let settings = fixture.write("settings.json", &allow_settings());

    let (status, stdout, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines[0], "pattern-match: 2 findings");
    assert!(lines[1].contains("`pattern_cases[1]`"));
    assert!(lines[2].contains("`decision_cases[0]`"));
    assert_eq!(lines[3], "pattern-match: 0 findings omitted");
}

#[test]
fn reports_a_decision_case_mismatch_without_disclosing_state() {
    let fixture = Fixture::new();
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[
                decision_case(&state(true, false, false, "allow"), NONMATCHING_INPUT),
                decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
            ],
            &[
                pattern_case("always_allow", true, 0, MATCHING_INPUT),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            ],
        ),
    );
    let settings = fixture.write("settings.json", &allow_settings());

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    assert_eq!(
        stderr,
        concat!(
            "pattern-match: 1 finding\n",
            "  The layer manifest `decision_cases[0]` declared state disagrees with the configured result\n",
            "pattern-match: 0 findings omitted\n",
        )
    );
    assert_declares_no_state(&stderr);
}

#[test]
fn reports_a_pattern_case_mismatch_without_disclosing_polarity() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());

    for (expected_match, input) in [(true, NONMATCHING_INPUT), (false, MATCHING_INPUT)] {
        let manifest = fixture.write(
            "layer.json",
            &layer_manifest(
                &[
                    decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
                    decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
                ],
                &[
                    pattern_case("always_allow", true, 0, MATCHING_INPUT),
                    pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
                    pattern_case("always_allow", expected_match, 0, input),
                ],
            ),
        );

        let (status, stdout, stderr) = run_layer(&manifest, &settings);

        assert_eq!(status, 1);
        assert!(stdout.is_empty());
        assert_eq!(
            stderr,
            concat!(
                "pattern-match: 1 finding\n",
                "  The layer manifest `pattern_cases[2]` declared expectation disagrees with the configured `always_allow[0]` pattern result\n",
                "pattern-match: 0 findings omitted\n",
            ),
            "Both polarities must produce the same value-free finding"
        );
    }
}

#[test]
fn rejects_a_pattern_case_index_outside_the_configured_array() {
    let fixture = Fixture::new();
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[decision_case(
                &state(false, false, false, "confirm"),
                NONMATCHING_INPUT,
            )],
            &[pattern_case("always_allow", true, 1, MATCHING_INPUT)],
        ),
    );
    let settings = fixture.write("settings.json", &allow_settings());

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest `pattern_cases[0]` index is outside the configured `always_allow` array\n"
    );
}

#[test]
fn requires_both_polarities_for_every_configured_pattern() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());

    for (cases, polarity) in [
        (
            vec![pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            "nonmatching",
        ),
        (
            vec![pattern_case("always_allow", false, 0, NONMATCHING_INPUT)],
            "matching",
        ),
    ] {
        let manifest = fixture.write(
            "layer.json",
            &layer_manifest(
                &[
                    decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
                    decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
                ],
                &cases,
            ),
        );

        let (status, _, stderr) = run_layer(&manifest, &settings);

        assert_eq!(status, 2);
        assert_eq!(
            stderr,
            format!(
                "pattern-match: The layer manifest declares no {polarity} case for the configured `always_allow[0]` pattern. A pattern that cannot supply both polarities is unsupported by this workflow\n"
            )
        );
    }
}

#[test]
fn requires_a_deciding_source_for_every_nonempty_bucket() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &full_settings());
    let pattern_cases = [
        pattern_case("always_allow", true, 0, MATCHING_INPUT),
        pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
        pattern_case("always_confirm", true, 0, PRIVATE_INPUT),
        pattern_case("always_confirm", false, 0, MATCHING_INPUT),
        pattern_case("always_deny", true, 0, DENIED_INPUT),
        pattern_case("always_deny", false, 0, MATCHING_INPUT),
    ];
    let sources = [
        (
            "always_allow",
            decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
        ),
        (
            "always_confirm",
            decision_case(&state(true, true, false, "confirm"), PRIVATE_INPUT),
        ),
        (
            "always_deny",
            decision_case(&state(false, false, true, "deny"), DENIED_INPUT),
        ),
    ];

    for (bucket, _) in sources.clone() {
        let mut decision_cases: Vec<String> = sources
            .iter()
            .filter(|(name, _)| *name != bucket)
            .map(|(_, case)| case.clone())
            .collect();
        decision_cases.push(decision_case(
            &state(false, false, false, "confirm"),
            NONMATCHING_INPUT,
        ));
        let manifest = fixture.write(
            "layer.json",
            &layer_manifest(&decision_cases, &pattern_cases),
        );

        let (status, _, stderr) = run_layer(&manifest, &settings);

        assert_eq!(status, 2);
        assert_eq!(
            stderr,
            format!(
                "pattern-match: The layer manifest declares no decision case with `{bucket}` as the deciding source. A fully shadowed bucket is unsupported by the ordinary change workflow\n"
            )
        );
    }
}

#[test]
fn requires_a_deciding_source_for_the_configured_default() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[decision_case(
                &state(true, false, false, "allow"),
                MATCHING_INPUT,
            )],
            &[
                pattern_case("always_allow", true, 0, MATCHING_INPUT),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            ],
        ),
    );

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest declares no decision case with the configured default as the deciding source. An unreachable default is unsupported by the ordinary change workflow\n"
    );
}

#[test]
fn accepts_a_confirm_witness_that_also_matches_the_allow_bucket() {
    let fixture = Fixture::new();
    let manifest = fixture.write("layer.json", &full_manifest());
    let settings = fixture.write("settings.json", &full_settings());

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(
        status, 0,
        "A confirm witness may also match the allow bucket: {stderr}"
    );
}

#[test]
fn rejects_a_declared_state_that_breaks_precedence() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[decision_case(
                &state(true, false, false, "confirm"),
                MATCHING_INPUT,
            )],
            &[
                pattern_case("always_allow", true, 0, MATCHING_INPUT),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            ],
        ),
    );

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest `decision_cases[0]` expected state declares a decision that does not follow deny, confirm, allow, then default precedence\n"
    );
}

#[test]
fn verifies_a_comparison_transition() {
    let fixture = Fixture::new();
    let baseline = fixture.write("baseline.json", &settings_json("confirm", &[]));
    let candidate = fixture.write("candidate.json", &allow_settings());
    let comparison = fixture.write(
        "comparison.json",
        &comparison_manifest(&[
            comparison_case(
                &state(false, false, false, "confirm"),
                &state(true, false, false, "allow"),
                MATCHING_INPUT,
            ),
            comparison_case(
                &state(false, false, false, "confirm"),
                &state(false, false, false, "confirm"),
                NONMATCHING_INPUT,
            ),
        ]),
    );

    let (status, stdout, stderr) = run_comparison(&baseline, &candidate, &comparison);

    assert_eq!(status, 0, "{stderr}");
    assert_eq!(
        stdout,
        "Verified 0 baseline patterns, 1 candidate pattern, and 2 comparison cases\n"
    );
    assert!(stderr.is_empty());
}

#[test]
fn reports_a_comparison_mismatch_for_each_side_without_disclosing_state() {
    let fixture = Fixture::new();
    let baseline = fixture.write("baseline.json", &allow_settings());
    let candidate = fixture.write("candidate.json", &allow_settings());
    let comparison = fixture.write(
        "comparison.json",
        &comparison_manifest(&[comparison_case(
            &state(false, false, false, "confirm"),
            &state(false, false, false, "confirm"),
            MATCHING_INPUT,
        )]),
    );

    let (status, stdout, stderr) = run_comparison(&baseline, &candidate, &comparison);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        concat!(
            "pattern-match: 2 findings\n",
            "  The comparison manifest `cases[0]` declared baseline state disagrees with the configured result\n",
            "  The comparison manifest `cases[0]` declared candidate state disagrees with the configured result\n",
            "pattern-match: 0 findings omitted\n",
        )
    );
    assert_declares_no_state(&stderr);
}

#[test]
fn reports_only_the_disagreeing_comparison_side() {
    let fixture = Fixture::new();
    let baseline = fixture.write("baseline.json", &settings_json("confirm", &[]));
    let candidate = fixture.write("candidate.json", &allow_settings());
    let comparison = fixture.write(
        "comparison.json",
        &comparison_manifest(&[comparison_case(
            &state(false, false, false, "confirm"),
            &state(false, false, false, "confirm"),
            MATCHING_INPUT,
        )]),
    );

    let (status, _, stderr) = run_comparison(&baseline, &candidate, &comparison);

    assert_eq!(status, 1);
    assert_eq!(
        stderr,
        concat!(
            "pattern-match: 1 finding\n",
            "  The comparison manifest `cases[0]` declared candidate state disagrees with the configured result\n",
            "pattern-match: 0 findings omitted\n",
        )
    );
    assert_declares_no_state(&stderr);
}

#[test]
fn rejects_a_comparison_state_that_breaks_precedence() {
    let fixture = Fixture::new();
    let baseline = fixture.write("baseline.json", &allow_settings());
    let candidate = fixture.write("candidate.json", &allow_settings());

    for (side, case) in [
        (
            "baseline",
            comparison_case(
                &state(false, false, false, "allow"),
                &state(true, false, false, "allow"),
                MATCHING_INPUT,
            ),
        ),
        (
            "candidate",
            comparison_case(
                &state(true, false, false, "allow"),
                &state(false, false, false, "allow"),
                MATCHING_INPUT,
            ),
        ),
    ] {
        let comparison = fixture.write("comparison.json", &comparison_manifest(&[case]));

        let (status, _, stderr) = run_comparison(&baseline, &candidate, &comparison);

        assert_eq!(status, 2);
        assert_eq!(
            stderr,
            format!(
                "pattern-match: The comparison manifest `cases[0]` {side} state declares a decision that does not follow deny, confirm, allow, then default precedence\n"
            )
        );
    }
}

#[test]
fn reports_baseline_configuration_findings_before_candidate_findings() {
    let fixture = Fixture::new();
    let baseline = fixture.write(
        "baseline.json",
        &settings_json("confirm", &[("always_allow", &[("(", true)])]),
    );
    let candidate = fixture.write(
        "candidate.json",
        &settings_json("confirm", &[("always_confirm", &[("[", true)])]),
    );
    let comparison = fixture.write(
        "comparison.json",
        &comparison_manifest(&[comparison_case(
            &state(false, false, false, "confirm"),
            &state(false, false, false, "confirm"),
            MATCHING_INPUT,
        )]),
    );

    let (status, stdout, stderr) = run_comparison(&baseline, &candidate, &comparison);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines[0], "pattern-match: 2 findings");
    assert_eq!(
        lines[1],
        "  The baseline settings `always_allow[0]` pattern is not valid regex syntax"
    );
    assert_eq!(
        lines[2],
        "  The candidate settings `always_confirm[0]` pattern is not valid regex syntax"
    );
}

#[test]
fn reports_an_overlong_pattern_without_compiling_it() {
    let fixture = Fixture::new();
    let overlong = format!("^a{}", "b".repeat(1_000));
    let settings = fixture.write(
        "settings.json",
        &settings_json("confirm", &[("always_allow", &[(overlong.as_str(), true)])]),
    );
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, stdout, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    assert!(stdout.is_empty());
    assert_eq!(
        stderr,
        concat!(
            "pattern-match: 1 finding\n",
            "  The settings `always_allow[0]` pattern exceeds the 1,000-scalar reviewability bound\n",
            "pattern-match: 0 findings omitted\n",
        )
    );
}

#[test]
fn accepts_a_pattern_at_the_scalar_bound() {
    let fixture = Fixture::new();
    let bounded = format!("^{}", "b".repeat(999));
    let settings = fixture.write(
        "settings.json",
        &settings_json("confirm", &[("always_allow", &[(bounded.as_str(), true)])]),
    );
    let manifest = fixture.write(
        "layer.json",
        &layer_manifest(
            &[
                decision_case(&state(true, false, false, "allow"), &"b".repeat(999)),
                decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
            ],
            &[
                pattern_case("always_allow", true, 0, &"b".repeat(999)),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            ],
        ),
    );

    let (status, stdout, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 0, "{stderr}");
    assert_eq!(
        stdout, "Verified 1 configured pattern, 2 decision cases, and 2 pattern cases\n",
        "A pattern of exactly 1,000 scalars must compile and evaluate"
    );
    assert!(stderr.is_empty());
}

#[test]
fn counts_pattern_length_in_unicode_scalars() {
    let fixture = Fixture::new();
    let multibyte = format!("^{}", "é".repeat(1_000));
    let settings = fixture.write(
        "settings.json",
        &settings_json(
            "confirm",
            &[("always_allow", &[(multibyte.as_str(), true)])],
        ),
    );
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    assert!(stderr.contains("exceeds the 1,000-scalar reviewability bound"));
}

#[test]
fn reports_an_invalid_regex_without_echoing_it() {
    let fixture = Fixture::new();
    let settings = fixture.write(
        "settings.json",
        &settings_json("confirm", &[("always_allow", &[("(?P<", true)])]),
    );
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    assert!(stderr.contains("The settings `always_allow[0]` pattern is not valid regex syntax"));
    assert!(
        !stderr.contains("(?P<"),
        "A compilation finding must not echo the pattern"
    );
}

#[test]
fn skips_expectation_evaluation_when_a_pattern_fails() {
    let fixture = Fixture::new();
    let settings = fixture.write(
        "settings.json",
        &settings_json("confirm", &[("always_allow", &[("(", true)])]),
    );
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    assert_eq!(
        stderr,
        concat!(
            "pattern-match: 1 finding\n",
            "  The settings `always_allow[0]` pattern is not valid regex syntax\n",
            "pattern-match: 0 findings omitted\n",
        ),
        "A configuration finding must replace every manifest expectation finding"
    );
}

#[test]
fn compiles_each_valid_configured_pattern_once() {
    let document = helper::parse_json(
        &settings_json(
            "confirm",
            &[
                (
                    "always_allow",
                    &[(ALLOW_PATTERN, true), (DENY_PATTERN, false)],
                ),
                ("always_confirm", &[(CONFIRM_PATTERN, true)]),
            ],
        ),
        Role::Settings,
    )
    .expect("The settings document must parse");
    let layer = helper::project_fetch_layer(&document, Role::Settings)
        .expect("The fetch layer must project");
    let compiled = helper::compile_fetch_layer(&layer, Role::Settings)
        .expect("Every configured pattern must compile");

    assert_eq!(layer.total(), 3);
    for (bucket, expected) in [(Bucket::Allow, 2), (Bucket::Confirm, 1), (Bucket::Deny, 0)] {
        assert_eq!(
            compiled.patterns(bucket).len(),
            expected,
            "`{}` must compile one regex per configured pattern",
            bucket.label()
        );
        assert_eq!(layer.patterns(bucket).len(), expected);
    }
}

#[test]
fn orders_configuration_findings_by_bucket_then_index() {
    let overlong = "b".repeat(1_001);
    let document = helper::parse_json(
        &settings_json(
            "confirm",
            &[
                ("always_allow", &[("(", true), ("[", true)]),
                ("always_confirm", &[("*", true), (overlong.as_str(), true)]),
                ("always_deny", &[("(?P<", true)]),
            ],
        ),
        Role::Settings,
    )
    .expect("The settings document must parse");
    let layer = helper::project_fetch_layer(&document, Role::Settings)
        .expect("The fetch layer must project");

    let findings = helper::compile_fetch_layer(&layer, Role::Settings)
        .err()
        .expect("Every configured pattern must produce a finding");

    assert_eq!(
        findings,
        vec![
            "The settings `always_allow[0]` pattern is not valid regex syntax".to_owned(),
            "The settings `always_allow[1]` pattern is not valid regex syntax".to_owned(),
            "The settings `always_confirm[0]` pattern is not valid regex syntax".to_owned(),
            "The settings `always_confirm[1]` pattern exceeds the 1,000-scalar reviewability bound"
                .to_owned(),
            "The settings `always_deny[0]` pattern is not valid regex syntax".to_owned(),
        ],
        "Findings must follow `always_allow`, `always_confirm`, then `always_deny` order and ascending index"
    );
}

#[test]
fn omits_an_overlong_pattern_from_the_compiled_layer() {
    let overlong = "b".repeat(1_001);
    let document = helper::parse_json(
        &settings_json(
            "confirm",
            &[(
                "always_allow",
                &[(ALLOW_PATTERN, true), (overlong.as_str(), true)],
            )],
        ),
        Role::Settings,
    )
    .expect("The settings document must parse");
    let layer = helper::project_fetch_layer(&document, Role::Settings)
        .expect("The fetch layer must project");

    let findings = helper::compile_fetch_layer(&layer, Role::Settings)
        .err()
        .expect("An overlong pattern must be a configuration finding");

    assert_eq!(
        findings,
        vec![
            "The settings `always_allow[1]` pattern exceeds the 1,000-scalar reviewability bound"
                .to_owned()
        ]
    );
}

#[test]
fn ignores_fields_outside_the_selected_fetch_object() {
    let fixture = Fixture::new();
    let mut document = String::from(
        "{\"theme\":\"dark\",\"agent\":{\"always_allow_tool_actions\":true,\"tool_permissions\":{\"default\":\"allow\",\"tools\":{\"terminal\":{\"unrelated\":1},\"fetch\":",
    );
    document.push_str(&fetch_object(
        "confirm",
        &[("always_allow", &[(ALLOW_PATTERN, true)])],
    ));
    document.push_str("}}}}");
    let settings = fixture.write("settings.json", &document);
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, stdout, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 0, "{stderr}");
    assert_eq!(
        stdout,
        "Verified 1 configured pattern, 2 decision cases, and 2 pattern cases\n"
    );
}

#[test]
fn treats_an_absent_bucket_array_as_empty() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, stdout, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 0, "{stderr}");
    assert_eq!(
        stdout, "Verified 1 configured pattern, 2 decision cases, and 2 pattern cases\n",
        "Absent `always_confirm` and `always_deny` arrays must contribute no pattern and require no deciding-source witness"
    );
}

#[test]
fn rejects_duplicate_object_keys() {
    let fixture = Fixture::new();
    let manifest = fixture.write("layer.json", &allow_manifest());
    let settings = fixture.write("settings.json", &allow_settings());

    let duplicated_settings = fixture.write(
        "duplicate-settings.json",
        &wrap_fetch("{\"always_allow\":[],\"always_allow\":[],\"default\":\"confirm\"}"),
    );
    let (status, _, stderr) = run_layer(&manifest, &duplicated_settings);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The settings JSON contains a duplicate object key\n"
    );

    let duplicated_manifest = fixture.write(
        "duplicate-layer.json",
        "{\"decision_cases\":[],\"pattern_cases\":[],\"pattern_cases\":[]}",
    );
    let (status, _, stderr) = run_layer(&duplicated_manifest, &settings);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest JSON contains a duplicate object key\n"
    );
}

#[test]
fn rejects_a_duplicate_key_nested_outside_the_fetch_object() {
    let fixture = Fixture::new();
    let manifest = fixture.write("layer.json", &allow_manifest());
    let settings = fixture.write(
        "settings.json",
        "{\"editor\":{\"tab\":1,\"tab\":2},\"agent\":{\"tool_permissions\":{\"tools\":{\"fetch\":{\"default\":\"confirm\"}}}}}",
    );

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The settings JSON contains a duplicate object key\n"
    );
}

#[test]
fn rejects_unknown_fields() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let manifest = fixture.write("layer.json", &allow_manifest());

    for (name, document, expected) in [
        (
            "fetch.json",
            wrap_fetch("{\"default\":\"confirm\",\"always_ask\":[]}"),
            "The settings fetch object permits only `always_allow`, `always_confirm`, `always_deny`, and `default`",
        ),
        (
            "pattern.json",
            wrap_fetch(
                "{\"default\":\"confirm\",\"always_allow\":[{\"case_sensitive\":true,\"pattern\":\"^a\",\"note\":\"x\"}]}",
            ),
            "The settings `always_allow[0]` entry permits only `case_sensitive` and `pattern`",
        ),
    ] {
        let path = fixture.write(name, &document);

        let (status, _, stderr) = run_layer(&manifest, &path);

        assert_eq!(status, 2);
        assert_eq!(stderr, format!("pattern-match: {expected}\n"));
    }

    for (name, document, expected) in [
        (
            "root.json",
            "{\"decision_cases\":[],\"pattern_cases\":[],\"notes\":[]}".to_owned(),
            "The layer manifest root permits only `decision_cases` and `pattern_cases`",
        ),
        (
            "case.json",
            layer_manifest(
                &[
                    "{\"expected\":{\"always_allow\":false,\"always_confirm\":false,\"always_deny\":false,\"decision\":\"confirm\"},\"input\":\"x\",\"id\":\"a\"}".to_owned(),
                ],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `decision_cases[0]` permits only `expected` and `input`",
        ),
    ] {
        let path = fixture.write(name, &document);

        let (status, _, stderr) = run_layer(&path, &settings);

        assert_eq!(status, 2);
        assert_eq!(stderr, format!("pattern-match: {expected}\n"));
    }
}

#[test]
fn rejects_a_malformed_settings_projection() {
    let fixture = Fixture::new();
    let manifest = fixture.write("layer.json", &allow_manifest());

    for (name, document, expected) in [
        (
            "root.json",
            "[]".to_owned(),
            "The settings JSON root must be an object rather than an array",
        ),
        (
            "missing.json",
            "{\"agent\":{\"tool_permissions\":{\"tools\":{}}}}".to_owned(),
            "The settings `agent.tool_permissions.tools.fetch` path is missing `fetch`",
        ),
        (
            "nonobject.json",
            "{\"agent\":{\"tool_permissions\":{\"tools\":{\"fetch\":[]}}}}".to_owned(),
            "The settings `fetch` value must be an object rather than an array",
        ),
        (
            "default.json",
            wrap_fetch("{\"always_allow\":[]}"),
            "The settings fetch object requires a `default` value of `allow`, `confirm`, or `deny`",
        ),
        (
            "null-default.json",
            wrap_fetch("{\"default\":null}"),
            "The settings fetch object requires a `default` value of `allow`, `confirm`, or `deny`",
        ),
        (
            "bucket.json",
            wrap_fetch("{\"default\":\"confirm\",\"always_deny\":{}}"),
            "The settings `always_deny` value must be an array rather than an object",
        ),
        (
            "entry.json",
            wrap_fetch("{\"default\":\"confirm\",\"always_allow\":[\"^a\"]}"),
            "The settings `always_allow[0]` entry must be an object rather than a string",
        ),
        (
            "case-sensitive.json",
            wrap_fetch("{\"default\":\"confirm\",\"always_allow\":[{\"pattern\":\"^a\"}]}"),
            "The settings `always_allow[0]` entry requires a Boolean `case_sensitive` value",
        ),
        (
            "null-case-sensitive.json",
            wrap_fetch(
                "{\"default\":\"confirm\",\"always_allow\":[{\"case_sensitive\":null,\"pattern\":\"^a\"}]}",
            ),
            "The settings `always_allow[0]` entry requires a Boolean `case_sensitive` value",
        ),
        (
            "pattern.json",
            wrap_fetch("{\"default\":\"confirm\",\"always_allow\":[{\"case_sensitive\":true}]}"),
            "The settings `always_allow[0]` entry requires a string `pattern` value",
        ),
        (
            "pattern-type.json",
            wrap_fetch(
                "{\"default\":\"confirm\",\"always_allow\":[{\"case_sensitive\":true,\"pattern\":7}]}",
            ),
            "The settings `always_allow[0]` entry requires a string `pattern` value",
        ),
    ] {
        let path = fixture.write(name, &document);

        let (status, stdout, stderr) = run_layer(&manifest, &path);

        assert_eq!(status, 2, "{name} must be contract-invalid");
        assert!(stdout.is_empty());
        assert_eq!(stderr, format!("pattern-match: {expected}\n"));
    }
}

#[test]
fn rejects_a_malformed_layer_manifest() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let valid_state = state(false, false, false, "confirm");

    for (name, document, expected) in [
        (
            "root.json",
            "[]".to_owned(),
            "The layer manifest JSON root must be an object rather than an array",
        ),
        (
            "empty.json",
            "{\"decision_cases\":[],\"pattern_cases\":[]}".to_owned(),
            "The layer manifest requires a nonempty `decision_cases` array",
        ),
        (
            "empty-pattern-cases.json",
            layer_manifest(
                &[decision_case(
                    &state(false, false, false, "confirm"),
                    NONMATCHING_INPUT,
                )],
                &[],
            ),
            "The layer manifest requires a nonempty `pattern_cases` array",
        ),
        (
            "expected.json",
            layer_manifest(
                &["{\"input\":\"https://example.com/\"}".to_owned()],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `decision_cases[0]` requires an `expected` state",
        ),
        (
            "decision.json",
            layer_manifest(
                &[decision_case(
                    &state(false, false, false, "prompt"),
                    MATCHING_INPUT,
                )],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `decision_cases[0]` `expected` state requires a `decision` value of `allow`, `confirm`, or `deny`",
        ),
        (
            "flag.json",
            layer_manifest(
                &[
                    "{\"expected\":{\"always_allow\":\"yes\",\"always_confirm\":false,\"always_deny\":false,\"decision\":\"confirm\"},\"input\":\"https://example.com/\"}".to_owned(),
                ],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `decision_cases[0]` `expected` state requires a Boolean `always_allow` value",
        ),
        (
            "input.json",
            layer_manifest(
                &[format!("{{\"expected\":{valid_state},\"input\":7}}")],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `decision_cases[0]` requires a string `input` value",
        ),
        (
            "multiline.json",
            layer_manifest(
                &[decision_case(&valid_state, "https://example.com/\nhttps://b/")],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `decision_cases[0]` `input` value must not contain a line break",
        ),
        (
            "bucket.json",
            layer_manifest(
                &[decision_case(&valid_state, MATCHING_INPUT)],
                &[pattern_case("always_prompt", true, 0, MATCHING_INPUT)],
            ),
            "The layer manifest `pattern_cases[0]` requires a `bucket` value of `always_allow`, `always_confirm`, or `always_deny`",
        ),
        (
            "index.json",
            layer_manifest(
                &[decision_case(&valid_state, MATCHING_INPUT)],
                &[
                    "{\"bucket\":\"always_allow\",\"expected_match\":true,\"index\":1.5,\"input\":\"https://example.com/\"}".to_owned(),
                ],
            ),
            "The layer manifest `pattern_cases[0]` requires a nonnegative integer `index` value",
        ),
        (
            "negative-index.json",
            layer_manifest(
                &[decision_case(&valid_state, MATCHING_INPUT)],
                &[
                    "{\"bucket\":\"always_allow\",\"expected_match\":true,\"index\":-1,\"input\":\"https://example.com/\"}".to_owned(),
                ],
            ),
            "The layer manifest `pattern_cases[0]` requires a nonnegative integer `index` value",
        ),
        (
            "expected-match.json",
            layer_manifest(
                &[decision_case(&valid_state, MATCHING_INPUT)],
                &[
                    "{\"bucket\":\"always_allow\",\"expected_match\":null,\"index\":0,\"input\":\"https://example.com/\"}".to_owned(),
                ],
            ),
            "The layer manifest `pattern_cases[0]` requires a Boolean `expected_match` value",
        ),
    ] {
        let path = fixture.write(name, &document);

        let (status, stdout, stderr) = run_layer(&path, &settings);

        assert_eq!(status, 2, "{name} must be contract-invalid");
        assert!(stdout.is_empty());
        assert_eq!(stderr, format!("pattern-match: {expected}\n"));
    }
}

#[test]
fn rejects_a_malformed_comparison_manifest() {
    let fixture = Fixture::new();
    let baseline = fixture.write("baseline.json", &allow_settings());
    let candidate = fixture.write("candidate.json", &allow_settings());
    let valid_state = state(true, false, false, "allow");

    for (name, document, expected) in [
        (
            "empty.json",
            "{\"cases\":[]}".to_owned(),
            "The comparison manifest requires a nonempty `cases` array",
        ),
        (
            "baseline.json",
            format!(
                "{{\"cases\":[{{\"candidate\":{valid_state},\"input\":\"https://example.com/\"}}]}}"
            ),
            "The comparison manifest `cases[0]` requires a `baseline` state",
        ),
        (
            "candidate.json",
            format!(
                "{{\"cases\":[{{\"baseline\":{valid_state},\"input\":\"https://example.com/\"}}]}}"
            ),
            "The comparison manifest `cases[0]` requires a `candidate` state",
        ),
        (
            "state.json",
            format!(
                "{{\"cases\":[{{\"baseline\":{valid_state},\"candidate\":[],\"input\":\"https://example.com/\"}}]}}"
            ),
            "The comparison manifest `cases[0]` `candidate` state must be an object rather than an array",
        ),
    ] {
        let path = fixture.write(name, &document);

        let (status, stdout, stderr) = run_comparison(&baseline, &candidate, &path);

        assert_eq!(status, 2, "{name} must be contract-invalid");
        assert!(stdout.is_empty());
        assert_eq!(stderr, format!("pattern-match: {expected}\n"));
    }
}

#[test]
fn rejects_every_line_break_scalar_in_a_manifest_input() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let valid_state = state(false, false, false, "confirm");

    for scalar in [
        '\u{a}', '\u{b}', '\u{c}', '\u{d}', '\u{85}', '\u{2028}', '\u{2029}',
    ] {
        let input = format!("https://example.com/{scalar}page");
        let manifest = fixture.write(
            "layer.json",
            &layer_manifest(
                &[decision_case(&valid_state, &input)],
                &[pattern_case("always_allow", true, 0, MATCHING_INPUT)],
            ),
        );

        let (status, stdout, stderr) = run_layer(&manifest, &settings);

        assert_eq!(
            status,
            2,
            "U+{:04X} must be contract-invalid input",
            u32::from(scalar)
        );
        assert!(stdout.is_empty());
        assert_eq!(
            stderr,
            "pattern-match: The layer manifest `decision_cases[0]` `input` value must not contain a line break\n"
        );
        assert!(
            !stderr.trim_end_matches('\n').contains(scalar),
            "A rejected input must not be echoed"
        );
    }
}

#[test]
fn applies_the_line_break_rule_to_every_manifest_input() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let separated = format!("https://example.com/{}page", '\u{2028}');

    let pattern_case_manifest = fixture.write(
        "pattern-case.json",
        &layer_manifest(
            &[decision_case(
                &state(false, false, false, "confirm"),
                NONMATCHING_INPUT,
            )],
            &[pattern_case("always_allow", true, 0, &separated)],
        ),
    );
    let (status, _, stderr) = run_layer(&pattern_case_manifest, &settings);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest `pattern_cases[0]` `input` value must not contain a line break\n"
    );

    let comparison = fixture.write(
        "comparison.json",
        &comparison_manifest(&[comparison_case(
            &state(true, false, false, "allow"),
            &state(true, false, false, "allow"),
            &separated,
        )]),
    );
    let (status, _, stderr) = run_comparison(&settings, &settings, &comparison);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The comparison manifest `cases[0]` `input` value must not contain a line break\n"
    );
}

#[test]
fn rejects_unreadable_and_undecodable_files() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &allow_settings());
    let manifest = fixture.write("layer.json", &allow_manifest());

    let (status, _, stderr) = run_layer(&fixture.missing("absent.json"), &settings);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest file does not exist\n"
    );

    let (status, _, stderr) = run_layer(&manifest, &fixture.root);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The settings file must be a regular file\n"
    );

    let invalid = fixture.write_bytes("invalid.json", &[0x7b, 0xff, 0x7d]);
    let (status, _, stderr) = run_layer(&manifest, &invalid);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The settings file is not valid UTF-8\n"
    );

    let malformed = fixture.write("malformed.json", "{\"agent\":");
    let (status, _, stderr) = run_layer(&manifest, &malformed);
    assert_eq!(status, 2);
    assert!(
        stderr.starts_with("pattern-match: The settings JSON ends before a complete value at ")
    );
}

#[test]
fn selects_the_first_failing_phase() {
    let fixture = Fixture::new();
    let valid_settings = fixture.write("settings.json", &allow_settings());
    let valid_manifest = fixture.write("layer.json", &allow_manifest());
    let absent = fixture.missing("absent.json");
    let unparsable = fixture.write("unparsable.json", "{");
    let undecodable = fixture.write_bytes("undecodable.json", &[0xff]);

    // Readability precedes UTF-8 decoding, which precedes JSON parsing
    let (_, _, stderr) = run_layer(&absent, &undecodable);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest file does not exist\n"
    );
    let (_, _, stderr) = run_layer(&undecodable, &unparsable);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest file is not valid UTF-8\n"
    );

    // Manifest structure precedes settings projection, which precedes cross-file validation
    let unprojectable = fixture.write("unprojectable.json", &wrap_fetch("{}"));
    let empty_manifest =
        fixture.write("empty.json", "{\"decision_cases\":[],\"pattern_cases\":[]}");
    let (_, _, stderr) = run_layer(&empty_manifest, &unprojectable);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest requires a nonempty `decision_cases` array\n"
    );
    let unreferenced = fixture.write(
        "unreferenced.json",
        &layer_manifest(
            &[decision_case(
                &state(false, false, false, "confirm"),
                NONMATCHING_INPUT,
            )],
            &[pattern_case("always_allow", true, 9, MATCHING_INPUT)],
        ),
    );
    let (_, _, stderr) = run_layer(&unreferenced, &unprojectable);
    assert_eq!(
        stderr,
        "pattern-match: The settings fetch object requires a `default` value of `allow`, `confirm`, or `deny`\n"
    );
    let (_, _, stderr) = run_layer(&unreferenced, &valid_settings);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest `pattern_cases[0]` index is outside the configured `always_allow` array\n"
    );

    // A cross-file failure precedes any configuration or expectation finding
    let broken_settings = fixture.write(
        "broken.json",
        &settings_json("confirm", &[("always_allow", &[("(", true)])]),
    );
    let (status, _, stderr) = run_layer(&unreferenced, &broken_settings);
    assert_eq!(status, 2);
    assert_eq!(
        stderr,
        "pattern-match: The layer manifest `pattern_cases[0]` index is outside the configured `always_allow` array\n"
    );

    let (status, _, _) = run_layer(&valid_manifest, &valid_settings);
    assert_eq!(status, 0);
}

#[test]
fn reads_comparison_files_in_the_documented_option_order() {
    let fixture = Fixture::new();
    let valid = fixture.write("settings.json", &allow_settings());
    let absent = fixture.missing("absent.json");

    let (_, _, stderr) = run_comparison(&absent, &absent, &absent);
    assert_eq!(
        stderr,
        "pattern-match: The baseline settings file does not exist\n"
    );

    let (_, _, stderr) = run_comparison(&valid, &absent, &absent);
    assert_eq!(
        stderr,
        "pattern-match: The candidate settings file does not exist\n"
    );

    let (_, _, stderr) = run_comparison(&valid, &valid, &absent);
    assert_eq!(
        stderr,
        "pattern-match: The comparison manifest file does not exist\n"
    );
}

#[test]
fn omits_selected_paths_inputs_and_patterns_from_diagnostics() {
    let fixture = Fixture::new();
    let secret_input = "https://example.com/private/token-abc123";
    let secret_pattern = "^https://example\\.com/private/token-abc123";
    let settings = fixture.write(
        "secret-settings.json",
        &settings_json("confirm", &[("always_allow", &[(secret_pattern, true)])]),
    );
    let manifest = fixture.write(
        "secret-layer.json",
        &layer_manifest(
            &[
                decision_case(&state(false, false, false, "confirm"), secret_input),
                decision_case(&state(true, false, false, "allow"), secret_input),
                decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
            ],
            &[
                pattern_case("always_allow", true, 0, secret_input),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
            ],
        ),
    );

    let (status, _, stderr) = run_layer(&manifest, &settings);

    assert_eq!(status, 1);
    for secret in [
        secret_input,
        secret_pattern,
        "token-abc123",
        "secret-settings.json",
        "secret-layer.json",
        &fixture.root.to_string_lossy(),
    ] {
        assert!(
            !stderr.contains(secret),
            "Standard error must not disclose `{secret}`"
        );
    }
    assert_declares_no_state(&stderr);
    assert!(stderr.contains("`decision_cases[0]`"));
}

#[test]
fn omits_manifest_and_settings_values_from_every_mismatch_diagnostic() {
    let fixture = Fixture::new();
    let settings = fixture.write("settings.json", &full_settings());
    let layer = fixture.write(
        "layer.json",
        &layer_manifest(
            &[
                decision_case(&state(true, false, false, "allow"), MATCHING_INPUT),
                decision_case(&state(true, true, false, "confirm"), PRIVATE_INPUT),
                decision_case(&state(false, false, true, "deny"), DENIED_INPUT),
                decision_case(&state(false, false, false, "confirm"), NONMATCHING_INPUT),
                decision_case(&state(false, false, true, "deny"), NONMATCHING_INPUT),
            ],
            &[
                pattern_case("always_allow", true, 0, MATCHING_INPUT),
                pattern_case("always_allow", false, 0, NONMATCHING_INPUT),
                pattern_case("always_confirm", true, 0, PRIVATE_INPUT),
                pattern_case("always_confirm", false, 0, MATCHING_INPUT),
                pattern_case("always_deny", true, 0, DENIED_INPUT),
                pattern_case("always_deny", false, 0, MATCHING_INPUT),
                pattern_case("always_deny", true, 0, MATCHING_INPUT),
            ],
        ),
    );

    let (status, _, stderr) = run_layer(&layer, &settings);

    assert_eq!(status, 1);
    assert_eq!(
        stderr,
        concat!(
            "pattern-match: 2 findings\n",
            "  The layer manifest `pattern_cases[6]` declared expectation disagrees with the configured `always_deny[0]` pattern result\n",
            "  The layer manifest `decision_cases[4]` declared state disagrees with the configured result\n",
            "pattern-match: 0 findings omitted\n",
        ),
        "Aggregation and pattern-case-then-decision-case ordering must survive the value-free wording"
    );
    assert_declares_no_state(&stderr);

    let comparison = fixture.write(
        "comparison.json",
        &comparison_manifest(&[comparison_case(
            &state(false, false, true, "deny"),
            &state(false, false, true, "deny"),
            MATCHING_INPUT,
        )]),
    );

    let (status, _, stderr) = run_comparison(&settings, &settings, &comparison);

    assert_eq!(status, 1);
    assert_declares_no_state(&stderr);
    for value in [MATCHING_INPUT, ALLOW_PATTERN, DENY_PATTERN] {
        assert!(
            !stderr.contains(value),
            "Standard error must not disclose `{value}`"
        );
    }
}

#[test]
fn bounds_reported_findings_and_reports_the_omitted_count() {
    let findings: Vec<String> = (0..250).map(|index| format!("Finding {index}")).collect();

    let rendered = helper::render_findings(&findings);
    let lines: Vec<&str> = rendered.lines().collect();

    assert_eq!(lines[0], "pattern-match: 250 findings");
    assert_eq!(lines.len(), 102);
    assert_eq!(lines[1], "  Finding 0");
    assert_eq!(lines[100], "  Finding 99");
    assert_eq!(lines[101], "pattern-match: 150 findings omitted");
    assert!(rendered.len() <= 64 * 1024);
}

#[test]
fn bounds_each_finding_to_512_bytes_at_a_scalar_boundary() {
    let finding = format!("x{}", "€".repeat(300));

    let rendered = helper::render_findings(std::slice::from_ref(&finding));
    let detail = rendered.lines().nth(1).expect("The finding must render");

    assert!(detail.len() <= 512);
    assert!(
        detail.len() > 508,
        "Truncation must keep the largest whole-scalar prefix"
    );
    assert!(format!("  {finding}").starts_with(detail));
}

#[test]
fn bounds_standard_error_to_64_kibibytes() {
    let findings: Vec<String> = (0..250)
        .map(|index| format!("{index} {}", "x".repeat(900)))
        .collect();

    let rendered = helper::render_findings(&findings);

    assert!(rendered.len() <= 64 * 1024);
    assert!(rendered.starts_with("pattern-match: 250 findings\n"));
    assert!(rendered.ends_with("pattern-match: 150 findings omitted\n"));
    for line in rendered.lines() {
        assert!(line.len() <= 512, "Every rendered line must fit the bound");
    }
}

#[test]
fn bounds_one_diagnostic_to_512_bytes() {
    let (status, _, stderr) = run_arguments([format!("--{}", "a".repeat(600))]);

    assert_eq!(status, 2);
    assert_eq!(stderr.lines().count(), 1);
    assert!(stderr.trim_end().len() <= 512);
}

#[test]
fn writes_no_files_and_leaves_inputs_unchanged() {
    let fixture = Fixture::new();
    fixture.write("layer.json", &full_manifest());
    fixture.write("settings.json", &full_settings());
    let before = fixture.snapshot();

    let (status, _, stderr) = run_layer(
        &fixture.root.join("layer.json"),
        &fixture.root.join("settings.json"),
    );

    assert_eq!(status, 0, "{stderr}");
    assert_eq!(
        fixture.snapshot(),
        before,
        "A run must not create, remove, or change any file"
    );
}

#[test]
fn reports_a_standard_output_write_failure() {
    let mut stderr = Vec::new();

    let status = helper::run([OsString::from("--help")], &mut FailingWriter, &mut stderr);

    assert_eq!(status, 2);
    assert_eq!(
        String::from_utf8(stderr).expect("Standard error must be valid UTF-8"),
        "pattern-match: Failed to write to standard output\n"
    );
}
