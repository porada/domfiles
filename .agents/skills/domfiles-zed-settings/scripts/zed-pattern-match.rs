use regex::{Regex, RegexBuilder};
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const HELP: &str = concat!(
    "Usage:\n",
    "  zed-pattern-match [--case-sensitive] --input-file <path> --pattern-file <path>\n",
    "  zed-pattern-match [--case-sensitive] --cases-file <path> --pattern-file <path>\n",
    "  zed-pattern-match --suite-file <path>\n",
    "\n",
    "Match one UTF-8 input or verify UTF-8 manifests against Zed-compatible regex patterns\n",
    "\n",
    "Options:\n",
    "  --case-sensitive       Use case-sensitive matching\n",
    "  --cases-file <path>    Read LF-delimited `match<TAB><input>` and `no-match<TAB><input>` cases\n",
    "  --help                 Print help\n",
    "  --input-file <path>    Read one complete UTF-8 input from this file\n",
    "  --pattern-file <path>  Read the complete UTF-8 pattern from this file\n",
    "  --suite-file <path>    Verify multiple patterns, pattern cases, and configured-pattern decisions\n",
    "\n",
    "LF-delimited UTF-8 suite manifest with records in any order:\n",
    "  decision-case<TAB>allow|confirm|deny<TAB><input>\n",
    "  decision-case-file<TAB>allow|confirm|deny<TAB><input-file>\n",
    "  default<TAB>allow|confirm|deny\n",
    "  pattern<TAB><id><TAB>always_allow|always_confirm|always_deny<TAB>case-sensitive|case-insensitive<TAB><pattern-file>\n",
    "  pattern-case<TAB><id><TAB>match|no-match<TAB><input>\n",
    "  pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>\n",
    "  Relative pattern and input paths resolve from the suite file’s parent\n",
    "\n",
    "Suite requirements:\n",
    "  Define exactly one default, at least one pattern, and at least one decision case\n",
    "  Define at least one pattern case for every pattern ID\n",
    "  Keep inline inputs single-line. Use file-backed records for multiline inputs\n",
    "  Suite decisions apply configured pattern precedence to one input only\n",
    "  They do not reproduce full Zed permission evaluation\n",
    "\n",
    "Verification output:\n",
    "  Case-manifest success prints one verified-case count\n",
    "  Suite success prints pattern-case, decision-case, and pattern counts\n",
    "  Failure reports at most 10 manifest line numbers without echoing regexes or inputs\n",
    "\n",
    "Exit statuses:\n",
    "  0  Pattern matched, every expectation passed, or help displayed\n",
    "  1  Pattern did not match or an expectation failed\n",
    "  2  Invalid arguments or data, or an I/O failure\n",
);

const MAX_REPORTED_FAILURES: usize = 10;
const STATUS_ERROR: u8 = 2;
const STATUS_MATCH: u8 = 0;
const STATUS_NO_MATCH: u8 = 1;

struct Arguments {
    case_sensitive: bool,
    input: InputMode,
    pattern_file: PathBuf,
}

struct BatchCase {
    expected_match: bool,
    input: String,
    line_number: usize,
}

struct BatchFailure {
    expected_match: bool,
    line_number: usize,
}

struct BatchResult {
    cases_file: PathBuf,
    failures: Vec<BatchFailure>,
    total: usize,
}

struct CompiledSuitePattern {
    bucket: PatternBucket,
    id: String,
    regex: Regex,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PermissionDecision {
    Allow,
    Confirm,
    Deny,
}

#[derive(Clone, Copy)]
enum PatternBucket {
    Allow,
    Confirm,
    Deny,
}

struct SuiteFailure {
    expectation: SuiteFailureExpectation,
    line_number: usize,
}

enum SuiteFailureExpectation {
    Decision(PermissionDecision),
    Pattern {
        expected_match: bool,
        pattern_id: String,
    },
}

struct SuiteManifest {
    default: PermissionDecision,
    expectations: Vec<SuiteExpectation>,
    patterns: Vec<SuitePatternDefinition>,
}

enum SuiteExpectation {
    DecisionCase {
        expected: PermissionDecision,
        input: String,
        line_number: usize,
    },
    PatternCase {
        expected_match: bool,
        input: String,
        line_number: usize,
        pattern_id: String,
    },
}

struct SuitePatternDefinition {
    bucket: PatternBucket,
    case_sensitive: bool,
    id: String,
    pattern_file: PathBuf,
}

struct SuiteResult {
    decision_cases: usize,
    failures: Vec<SuiteFailure>,
    pattern_cases: usize,
    pattern_count: usize,
    suite_file: PathBuf,
}

enum Evaluation {
    Batch(BatchResult),
    Single(bool),
}

enum InputMode {
    Batch(PathBuf),
    Single(PathBuf),
}

enum ParsedArguments {
    Help,
    Run(Arguments),
    Suite(PathBuf),
}

#[derive(Debug)]
pub(crate) enum PatternError {
    Empty,
    Invalid(regex::Error),
}

fn parse_arguments<I>(arguments: I) -> Result<ParsedArguments, String>
where
    I: IntoIterator<Item = OsString>,
{
    let arguments: Vec<OsString> = arguments.into_iter().collect();

    if arguments.len() == 1 && arguments[0].as_os_str() == OsStr::new("--help") {
        return Ok(ParsedArguments::Help);
    }

    let mut arguments = arguments.into_iter();
    let mut case_sensitive = false;
    let mut cases_file = None;
    let mut input_file = None;
    let mut pattern_file = None;
    let mut suite_file = None;

    while let Some(argument) = arguments.next() {
        let Some(option) = argument.to_str() else {
            return Err("Option names must be valid UTF-8".to_owned());
        };

        match option {
            "--cases-file" => {
                if cases_file.is_some() {
                    return Err("Option `--cases-file` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--cases-file` requires a path".to_owned());
                };
                cases_file = Some(PathBuf::from(path));
            }
            "--case-sensitive" => {
                if case_sensitive {
                    return Err("Option `--case-sensitive` may be specified only once".to_owned());
                }

                case_sensitive = true;
            }
            "--help" => {
                return Err("Option `--help` must be used alone".to_owned());
            }
            "--input-file" => {
                if input_file.is_some() {
                    return Err("Option `--input-file` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--input-file` requires a path".to_owned());
                };
                input_file = Some(PathBuf::from(path));
            }
            "--pattern-file" => {
                if pattern_file.is_some() {
                    return Err("Option `--pattern-file` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--pattern-file` requires a path".to_owned());
                };
                pattern_file = Some(PathBuf::from(path));
            }
            "--suite-file" => {
                if suite_file.is_some() {
                    return Err("Option `--suite-file` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--suite-file` requires a path".to_owned());
                };
                suite_file = Some(PathBuf::from(path));
            }
            _ => {
                return Err(format!(
                    "Unknown option `{option}`. Run `zed-pattern-match --help` for usage"
                ));
            }
        }
    }

    if let Some(suite_file) = suite_file {
        if case_sensitive || cases_file.is_some() || input_file.is_some() || pattern_file.is_some()
        {
            return Err(
                "Option `--suite-file` is mutually exclusive with `--case-sensitive`, `--cases-file`, `--input-file`, and `--pattern-file`"
                    .to_owned(),
            );
        }

        return Ok(ParsedArguments::Suite(suite_file));
    }

    let input = match (cases_file, input_file) {
        (Some(_), Some(_)) => {
            return Err(
                "Options `--cases-file` and `--input-file` are mutually exclusive".to_owned(),
            );
        }
        (Some(path), None) => InputMode::Batch(path),
        (None, Some(path)) => InputMode::Single(path),
        (None, None) => {
            return Err(
                "Missing required option `--cases-file <path>` or `--input-file <path>`".to_owned(),
            );
        }
    };
    let pattern_file =
        pattern_file.ok_or_else(|| "Missing required option `--pattern-file <path>`".to_owned())?;

    Ok(ParsedArguments::Run(Arguments {
        case_sensitive,
        input,
        pattern_file,
    }))
}

fn read_utf8_file(path: &Path, description: &str) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "Failed to read {description} file `{}`:\n\n{error}",
            path.display()
        )
    })?;

    String::from_utf8(bytes).map_err(|error| {
        format!(
            "Invalid UTF-8 in {description} file `{}`:\n\n{error}",
            path.display()
        )
    })
}

fn invalid_case_line(path: &Path, line_number: usize) -> String {
    format!(
        "Invalid case manifest `{}` at line {line_number}. Expected `match<TAB><input>` or `no-match<TAB><input>`",
        path.display()
    )
}

fn parse_case_manifest(path: &Path, manifest: &str) -> Result<Vec<BatchCase>, String> {
    if manifest.is_empty() {
        return Err(format!("Case manifest file `{}` is empty", path.display()));
    }

    manifest
        .split_terminator('\n')
        .enumerate()
        .map(|(index, line)| {
            let line_number = index + 1;
            if line.contains('\r') {
                return Err(invalid_case_line(path, line_number));
            }

            let Some((expectation, input)) = line.split_once('\t') else {
                return Err(invalid_case_line(path, line_number));
            };
            let expected_match = match expectation {
                "match" => true,
                "no-match" => false,
                _ => return Err(invalid_case_line(path, line_number)),
            };

            Ok(BatchCase {
                expected_match,
                input: input.to_owned(),
                line_number,
            })
        })
        .collect()
}

impl PatternBucket {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "always_allow" => Some(Self::Allow),
            "always_confirm" => Some(Self::Confirm),
            "always_deny" => Some(Self::Deny),
            _ => None,
        }
    }
}

impl PermissionDecision {
    fn label(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Confirm => "confirm",
            Self::Deny => "deny",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "allow" => Some(Self::Allow),
            "confirm" => Some(Self::Confirm),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
}

fn invalid_suite_line(path: &Path, line_number: usize) -> String {
    format!(
        "Invalid suite manifest `{}` at line {line_number}. Expected a documented suite record",
        path.display()
    )
}

fn resolve_suite_path(suite_file: &Path, referenced_file: &str) -> PathBuf {
    let referenced_file = Path::new(referenced_file);
    if referenced_file.is_absolute() {
        referenced_file.to_owned()
    } else {
        suite_file
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .join(referenced_file)
    }
}

fn suite_expected_match(value: &str) -> Option<bool> {
    match value {
        "match" => Some(true),
        "no-match" => Some(false),
        _ => None,
    }
}

fn read_suite_input(suite_file: &Path, input_file: &str) -> Result<String, String> {
    let input_file = resolve_suite_path(suite_file, input_file);

    read_utf8_file(&input_file, "suite input")
}

fn parse_suite_manifest(path: &Path, manifest: &str) -> Result<SuiteManifest, String> {
    let mut default = None;
    let mut expectations = Vec::new();
    let mut patterns: Vec<SuitePatternDefinition> = Vec::new();

    for (index, line) in manifest.split_terminator('\n').enumerate() {
        let line_number = index + 1;
        if line.contains('\r') {
            return Err(invalid_suite_line(path, line_number));
        }

        let record_type = line.split_once('\t').map(|(record_type, _)| record_type);
        match record_type {
            Some("default") => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 2 {
                    return Err(invalid_suite_line(path, line_number));
                }
                let Some(decision) = PermissionDecision::parse(fields[1]) else {
                    return Err(invalid_suite_line(path, line_number));
                };
                if default.is_some() {
                    return Err(format!(
                        "Duplicate default in suite manifest `{}` at line {line_number}",
                        path.display()
                    ));
                }
                default = Some(decision);
            }
            Some("pattern") => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 5 || fields[1].is_empty() || fields[4].is_empty() {
                    return Err(invalid_suite_line(path, line_number));
                }
                let Some(bucket) = PatternBucket::parse(fields[2]) else {
                    return Err(invalid_suite_line(path, line_number));
                };
                let case_sensitive = match fields[3] {
                    "case-insensitive" => false,
                    "case-sensitive" => true,
                    _ => return Err(invalid_suite_line(path, line_number)),
                };
                if patterns.iter().any(|pattern| pattern.id == fields[1]) {
                    return Err(format!(
                        "Duplicate pattern id `{}` in suite manifest `{}` at line {line_number}",
                        fields[1],
                        path.display()
                    ));
                }

                patterns.push(SuitePatternDefinition {
                    bucket,
                    case_sensitive,
                    id: fields[1].to_owned(),
                    pattern_file: resolve_suite_path(path, fields[4]),
                });
            }
            Some("pattern-case") | Some("pattern-case-file") => {
                let fields: Vec<&str> = line.splitn(4, '\t').collect();
                if fields.len() != 4 || fields[1].is_empty() {
                    return Err(invalid_suite_line(path, line_number));
                }
                let Some(expected_match) = suite_expected_match(fields[2]) else {
                    return Err(invalid_suite_line(path, line_number));
                };
                let input = if record_type == Some("pattern-case-file") {
                    if fields[3].is_empty() {
                        return Err(invalid_suite_line(path, line_number));
                    }
                    read_suite_input(path, fields[3])?
                } else {
                    fields[3].to_owned()
                };

                expectations.push(SuiteExpectation::PatternCase {
                    expected_match,
                    input,
                    line_number,
                    pattern_id: fields[1].to_owned(),
                });
            }
            Some("decision-case") | Some("decision-case-file") => {
                let fields: Vec<&str> = line.splitn(3, '\t').collect();
                if fields.len() != 3 {
                    return Err(invalid_suite_line(path, line_number));
                }
                let Some(expected) = PermissionDecision::parse(fields[1]) else {
                    return Err(invalid_suite_line(path, line_number));
                };
                let input = if record_type == Some("decision-case-file") {
                    if fields[2].is_empty() {
                        return Err(invalid_suite_line(path, line_number));
                    }
                    read_suite_input(path, fields[2])?
                } else {
                    fields[2].to_owned()
                };

                expectations.push(SuiteExpectation::DecisionCase {
                    expected,
                    input,
                    line_number,
                });
            }
            _ => return Err(invalid_suite_line(path, line_number)),
        }
    }

    let default = default.ok_or_else(|| {
        format!(
            "Suite manifest `{}` must define exactly one default",
            path.display()
        )
    })?;
    if patterns.is_empty() {
        return Err(format!(
            "Suite manifest `{}` must define at least one pattern",
            path.display()
        ));
    }
    for expectation in &expectations {
        let SuiteExpectation::PatternCase {
            line_number,
            pattern_id,
            ..
        } = expectation
        else {
            continue;
        };
        if !patterns.iter().any(|pattern| pattern.id == *pattern_id) {
            return Err(format!(
                "Unknown pattern id `{pattern_id}` in suite manifest `{}` at line {line_number}",
                path.display()
            ));
        }
    }

    if !expectations
        .iter()
        .any(|expectation| matches!(expectation, SuiteExpectation::DecisionCase { .. }))
    {
        return Err(format!(
            "Suite manifest `{}` must include at least one `decision-case`",
            path.display()
        ));
    }

    for pattern in &patterns {
        let has_pattern_case = expectations.iter().any(|expectation| {
            matches!(
                expectation,
                SuiteExpectation::PatternCase { pattern_id, .. } if pattern_id == &pattern.id
            )
        });
        if !has_pattern_case {
            return Err(format!(
                "Suite manifest `{}` must include at least one `pattern-case` for pattern `{}`",
                path.display(),
                pattern.id
            ));
        }
    }

    Ok(SuiteManifest {
        default,
        expectations,
        patterns,
    })
}

fn regex_error_summary(error: &regex::Error) -> String {
    let message = error.to_string();

    message
        .lines()
        .rev()
        .find_map(|line| line.trim().strip_prefix("error: "))
        .unwrap_or("Regex compilation failed")
        .to_owned()
}

pub(crate) fn compile_pattern(pattern: &str, case_sensitive: bool) -> Result<Regex, PatternError> {
    if pattern.is_empty() {
        return Err(PatternError::Empty);
    }

    RegexBuilder::new(pattern)
        .case_insensitive(!case_sensitive)
        .build()
        .map_err(PatternError::Invalid)
}

fn compile_suite_patterns(
    definitions: Vec<SuitePatternDefinition>,
) -> Result<Vec<CompiledSuitePattern>, String> {
    let mut patterns = Vec::with_capacity(definitions.len());

    for definition in definitions {
        let description = format!("pattern `{}`", definition.id);
        let pattern = read_utf8_file(&definition.pattern_file, &description)?;
        let regex = match compile_pattern(&pattern, definition.case_sensitive) {
            Ok(regex) => regex,
            Err(PatternError::Empty) => {
                return Err(format!(
                    "Pattern file `{}` for suite pattern `{}` is empty",
                    definition.pattern_file.display(),
                    definition.id
                ));
            }
            Err(PatternError::Invalid(error)) => {
                let summary = regex_error_summary(&error);

                return Err(format!(
                    "Invalid regex in pattern file `{}` for suite pattern `{}`: {summary}",
                    definition.pattern_file.display(),
                    definition.id
                ));
            }
        };

        patterns.push(CompiledSuitePattern {
            bucket: definition.bucket,
            id: definition.id,
            regex,
        });
    }

    Ok(patterns)
}

fn permission_decision(
    input: &str,
    patterns: &[CompiledSuitePattern],
    default: PermissionDecision,
) -> PermissionDecision {
    let mut matched_allow = false;
    let mut matched_confirm = false;

    for pattern in patterns {
        if !pattern.regex.is_match(input) {
            continue;
        }

        match pattern.bucket {
            PatternBucket::Allow => matched_allow = true,
            PatternBucket::Confirm => matched_confirm = true,
            PatternBucket::Deny => return PermissionDecision::Deny,
        }
    }

    if matched_confirm {
        PermissionDecision::Confirm
    } else if matched_allow {
        PermissionDecision::Allow
    } else {
        default
    }
}

fn evaluate_suite(suite_file: &Path) -> Result<SuiteResult, String> {
    let manifest = read_utf8_file(suite_file, "suite manifest")?;
    let SuiteManifest {
        default,
        expectations,
        patterns,
    } = parse_suite_manifest(suite_file, &manifest)?;
    let patterns = compile_suite_patterns(patterns)?;
    let pattern_count = patterns.len();
    let mut decision_cases = 0;
    let mut failures = Vec::new();
    let mut pattern_cases = 0;

    for expectation in expectations {
        match expectation {
            SuiteExpectation::DecisionCase {
                expected,
                input,
                line_number,
            } => {
                decision_cases += 1;
                if permission_decision(&input, &patterns, default) != expected {
                    failures.push(SuiteFailure {
                        expectation: SuiteFailureExpectation::Decision(expected),
                        line_number,
                    });
                }
            }
            SuiteExpectation::PatternCase {
                expected_match,
                input,
                line_number,
                pattern_id,
            } => {
                pattern_cases += 1;
                let Some(pattern) = patterns
                    .iter()
                    .find(|pattern| pattern.id == pattern_id.as_str())
                else {
                    return Err(format!(
                        "Unknown pattern id `{pattern_id}` in suite manifest `{}` at line {line_number}",
                        suite_file.display()
                    ));
                };
                if pattern.regex.is_match(&input) != expected_match {
                    failures.push(SuiteFailure {
                        expectation: SuiteFailureExpectation::Pattern {
                            expected_match,
                            pattern_id,
                        },
                        line_number,
                    });
                }
            }
        }
    }

    Ok(SuiteResult {
        decision_cases,
        failures,
        pattern_cases,
        pattern_count,
        suite_file: suite_file.to_owned(),
    })
}

fn evaluate(arguments: &Arguments) -> Result<Evaluation, String> {
    let pattern = read_utf8_file(&arguments.pattern_file, "pattern")?;
    let regex = match compile_pattern(&pattern, arguments.case_sensitive) {
        Ok(regex) => regex,
        Err(PatternError::Empty) => {
            return Err(format!(
                "Pattern file `{}` is empty",
                arguments.pattern_file.display()
            ));
        }
        Err(PatternError::Invalid(error)) => {
            let summary = regex_error_summary(&error);

            return Err(format!(
                "Invalid regex in pattern file `{}`: {summary}",
                arguments.pattern_file.display()
            ));
        }
    };

    match &arguments.input {
        InputMode::Batch(cases_file) => {
            let manifest = read_utf8_file(cases_file, "case manifest")?;
            let cases = parse_case_manifest(cases_file, &manifest)?;
            let total = cases.len();
            let failures = cases
                .into_iter()
                .filter_map(|case| {
                    (regex.is_match(&case.input) != case.expected_match).then_some(BatchFailure {
                        expected_match: case.expected_match,
                        line_number: case.line_number,
                    })
                })
                .collect();

            Ok(Evaluation::Batch(BatchResult {
                cases_file: cases_file.clone(),
                failures,
                total,
            }))
        }
        InputMode::Single(input_file) => {
            let input = read_utf8_file(input_file, "input")?;

            Ok(Evaluation::Single(regex.is_match(&input)))
        }
    }
}

fn case_label(count: usize) -> &'static str {
    if count == 1 { "case" } else { "cases" }
}

fn decision_label(count: usize) -> &'static str {
    if count == 1 { "decision" } else { "decisions" }
}

fn failure_label(count: usize) -> &'static str {
    if count == 1 { "failure" } else { "failures" }
}

fn pattern_label(count: usize) -> &'static str {
    if count == 1 { "pattern" } else { "patterns" }
}

fn report_batch_failures(stderr: &mut dyn Write, result: &BatchResult) -> io::Result<()> {
    writeln!(
        stderr,
        "zed-pattern-match: {} of {} {} failed in `{}`",
        result.failures.len(),
        result.total,
        case_label(result.total),
        result.cases_file.display()
    )?;
    for failure in result.failures.iter().take(MAX_REPORTED_FAILURES) {
        let expectation = if failure.expected_match {
            "a match"
        } else {
            "no match"
        };
        writeln!(
            stderr,
            "  Line {} expected {expectation}",
            failure.line_number
        )?;
    }

    let omitted = result.failures.len().saturating_sub(MAX_REPORTED_FAILURES);
    if omitted > 0 {
        writeln!(
            stderr,
            "  … {omitted} additional {} omitted",
            failure_label(omitted)
        )?;
    }

    Ok(())
}

fn report_suite_failures(stderr: &mut dyn Write, result: &SuiteResult) -> io::Result<()> {
    let total = result.pattern_cases + result.decision_cases;
    writeln!(
        stderr,
        "zed-pattern-match: {} of {total} suite expectations failed in `{}`",
        result.failures.len(),
        result.suite_file.display()
    )?;
    for failure in result.failures.iter().take(MAX_REPORTED_FAILURES) {
        match &failure.expectation {
            SuiteFailureExpectation::Decision(expected) => {
                writeln!(
                    stderr,
                    "  Line {} expected permission decision `{}`",
                    failure.line_number,
                    expected.label()
                )?;
            }
            SuiteFailureExpectation::Pattern {
                expected_match,
                pattern_id,
            } => {
                let expectation = if *expected_match {
                    "a match"
                } else {
                    "no match"
                };
                writeln!(
                    stderr,
                    "  Line {} pattern `{pattern_id}` expected {expectation}",
                    failure.line_number
                )?;
            }
        }
    }

    let omitted = result.failures.len().saturating_sub(MAX_REPORTED_FAILURES);
    if omitted > 0 {
        writeln!(
            stderr,
            "  … {omitted} additional {} omitted",
            failure_label(omitted)
        )?;
    }

    Ok(())
}

fn report_error(stderr: &mut dyn Write, message: &str) {
    let _ = writeln!(stderr, "zed-pattern-match: {message}");
}

fn report_verified_cases(stdout: &mut dyn Write, total: usize) -> Result<(), String> {
    writeln!(stdout, "Verified {total} {}", case_label(total))
        .map_err(|error| format!("Failed to write batch result:\n\n{error}"))
}

fn report_verified_suite(stdout: &mut dyn Write, result: &SuiteResult) -> Result<(), String> {
    writeln!(
        stdout,
        "Verified {} pattern {} and {} permission {} across {} {}",
        result.pattern_cases,
        case_label(result.pattern_cases),
        result.decision_cases,
        decision_label(result.decision_cases),
        result.pattern_count,
        pattern_label(result.pattern_count)
    )
    .map_err(|error| format!("Failed to write suite result:\n\n{error}"))
}

pub(crate) fn run<I>(arguments: I, stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8
where
    I: IntoIterator<Item = OsString>,
{
    let parsed_arguments = match parse_arguments(arguments) {
        Ok(parsed_arguments) => parsed_arguments,
        Err(error) => {
            report_error(stderr, &error);
            return STATUS_ERROR;
        }
    };

    match parsed_arguments {
        ParsedArguments::Help => {
            if let Err(error) = stdout.write_all(HELP.as_bytes()) {
                report_error(stderr, &format!("Failed to write help:\n\n{error}"));
                return STATUS_ERROR;
            }

            STATUS_MATCH
        }
        ParsedArguments::Run(arguments) => match evaluate(&arguments) {
            Ok(Evaluation::Batch(result)) if result.failures.is_empty() => {
                if let Err(error) = report_verified_cases(stdout, result.total) {
                    report_error(stderr, &error);
                    return STATUS_ERROR;
                }

                STATUS_MATCH
            }
            Ok(Evaluation::Batch(result)) => {
                if report_batch_failures(stderr, &result).is_err() {
                    return STATUS_ERROR;
                }

                STATUS_NO_MATCH
            }
            Ok(Evaluation::Single(true)) => STATUS_MATCH,
            Ok(Evaluation::Single(false)) => STATUS_NO_MATCH,
            Err(error) => {
                report_error(stderr, &error);
                STATUS_ERROR
            }
        },
        ParsedArguments::Suite(suite_file) => match evaluate_suite(&suite_file) {
            Ok(result) if result.failures.is_empty() => {
                if let Err(error) = report_verified_suite(stdout, &result) {
                    report_error(stderr, &error);
                    return STATUS_ERROR;
                }

                STATUS_MATCH
            }
            Ok(result) => {
                if report_suite_failures(stderr, &result).is_err() {
                    return STATUS_ERROR;
                }

                STATUS_NO_MATCH
            }
            Err(error) => {
                report_error(stderr, &error);
                STATUS_ERROR
            }
        },
    }
}

#[cfg(not(test))]
fn main() -> std::process::ExitCode {
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    std::process::ExitCode::from(run(std::env::args_os().skip(1), &mut stdout, &mut stderr))
}
