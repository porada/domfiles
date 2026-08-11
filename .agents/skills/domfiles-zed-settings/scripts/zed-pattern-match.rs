#[allow(dead_code)]
#[path = "helpers/permission-patterns.rs"]
mod permission_patterns;

pub(crate) use permission_patterns::{
    BoundedIssues, Bucket, CompiledPattern, Decision, MatchState, PatternError, compile_pattern,
    read_utf8_file, regex_error_summary,
};
use serde::{Deserialize, Deserializer, de};
use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    io::{self, Write},
    path::{Path, PathBuf},
};

const HELP: &str = concat!(
    "Usage:\n",
    "  zed-pattern-match [--case-sensitive] --input-file <path> --pattern-file <path>\n",
    "  zed-pattern-match [--case-sensitive] --cases-file <path> --pattern-file <path>\n",
    "  zed-pattern-match --comparison-file <path>\n",
    "  zed-pattern-match --suite-file <path>\n",
    "\n",
    "Match one UTF-8 input or verify UTF-8 manifests against Zed-compatible regex patterns\n",
    "\n",
    "Options:\n",
    "  --case-sensitive          Use case-sensitive matching\n",
    "  --cases-file <path>       Read LF-delimited `match<TAB><input>` and `no-match<TAB><input>` cases\n",
    "  --comparison-file <path>  Compare configured pattern sets over a representative JSON corpus. Mutually exclusive with every other option\n",
    "  --help                    Print help\n",
    "  --input-file <path>       Read one complete UTF-8 input from this file\n",
    "  --pattern-file <path>     Read the complete UTF-8 pattern from this file\n",
    "  --suite-file <path>       Verify multiple patterns, pattern cases, and configured-pattern decisions\n",
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
    "Version-1 UTF-8 JSON comparison manifest:\n",
    "  Root: {\"version\":1,\"baseline\":<set>,\"candidate\":<set>,\"cases\":[<case>,...]}\n",
    "  Set: {\"default\":\"allow|confirm|deny\",\"patterns\":[<pattern>,...]}\n",
    "  Pattern: {\"id\":\"...\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"case_sensitive\":true|false,\"pattern_file\":\"path\"}\n",
    "  Inline case: {\"type\":\"inline\",\"input\":\"single line\"}\n",
    "  File case: {\"type\":\"file\",\"input_file\":\"path\"}\n",
    "  Relative pattern and input paths resolve from the comparison file’s parent\n",
    "\n",
    "Comparison requirements:\n",
    "  Define at least one pattern in each set and at least one case\n",
    "  Keep pattern IDs nonempty and unique within each set\n",
    "  Keep inline inputs single-line. Use file cases for multiline inputs\n",
    "  Comparison checks each bucket’s matched state and the configured final decision\n",
    "  This is configured pattern comparison over a representative corpus only\n",
    "  It is not full Zed permission evaluation or formal language equivalence\n",
    "\n",
    "Verification output:\n",
    "  Case-manifest success prints one verified-case count\n",
    "  Comparison success prints case and baseline/candidate pattern counts\n",
    "  Suite success prints pattern-case, decision-case, and pattern counts\n",
    "  Failure reports at most 10 manifest positions without echoing regexes or inputs\n",
    "\n",
    "Exit statuses:\n",
    "  0  Pattern matched, every expectation passed, comparison was equivalent, or help displayed\n",
    "  1  Pattern did not match, an expectation failed, or comparison found mismatches\n",
    "  2  Invalid arguments or data, or an I/O failure\n",
);

const COMPARISON_VERSION: u64 = 1;
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
    failures: BoundedIssues<BatchFailure>,
    total: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ComparisonManifestDefinition {
    baseline: ComparisonPatternSetDefinition,
    candidate: ComparisonPatternSetDefinition,
    cases: Vec<ComparisonCaseDefinition>,
    version: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ComparisonPatternSetDefinition {
    #[serde(deserialize_with = "deserialize_decision")]
    default: Decision,
    patterns: Vec<ComparisonPatternDefinition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ComparisonPatternDefinition {
    bucket: Bucket,
    case_sensitive: bool,
    id: String,
    pattern_file: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case", tag = "type")]
enum ComparisonCaseDefinition {
    File { input_file: String },
    Inline { input: String },
}

#[derive(Default)]
struct ComparisonDifferences {
    allow_bucket: bool,
    confirm_bucket: bool,
    deny_bucket: bool,
    final_decision: bool,
}

impl ComparisonDifferences {
    fn between(
        baseline: MatchState,
        baseline_default: Decision,
        candidate: MatchState,
        candidate_default: Decision,
    ) -> Self {
        Self {
            allow_bucket: baseline.matched(Bucket::Allow) != candidate.matched(Bucket::Allow),
            confirm_bucket: baseline.matched(Bucket::Confirm) != candidate.matched(Bucket::Confirm),
            deny_bucket: baseline.matched(Bucket::Deny) != candidate.matched(Bucket::Deny),
            final_decision: baseline.decision(baseline_default)
                != candidate.decision(candidate_default),
        }
    }

    fn is_empty(&self) -> bool {
        !self.allow_bucket && !self.confirm_bucket && !self.deny_bucket && !self.final_decision
    }
}

struct ComparisonMismatch {
    case_position: usize,
    differences: ComparisonDifferences,
}

struct ComparisonResult {
    baseline_pattern_count: usize,
    candidate_pattern_count: usize,
    case_count: usize,
    comparison_file: PathBuf,
    mismatches: BoundedIssues<ComparisonMismatch>,
}

struct PatternDefinition {
    bucket: Bucket,
    case_sensitive: bool,
    id: String,
    pattern_file: PathBuf,
}

#[derive(Clone, Copy)]
enum PatternCollection {
    BaselineComparison,
    CandidateComparison,
    Suite,
}

impl PatternCollection {
    fn owner_label(self) -> &'static str {
        match self {
            Self::BaselineComparison => "baseline comparison pattern",
            Self::CandidateComparison => "candidate comparison pattern",
            Self::Suite => "suite pattern",
        }
    }

    fn read_description(self, id: &str) -> String {
        match self {
            Self::BaselineComparison => format!("baseline comparison pattern `{id}`"),
            Self::CandidateComparison => format!("candidate comparison pattern `{id}`"),
            Self::Suite => format!("pattern `{id}`"),
        }
    }
}

struct SuiteFailure {
    expectation: SuiteFailureExpectation,
    line_number: usize,
}

enum SuiteFailureExpectation {
    Decision(Decision),
    Pattern {
        expected_match: bool,
        pattern_id: String,
    },
}

struct SuiteManifest {
    default: Decision,
    expectations: Vec<SuiteExpectation>,
    pattern_indices: HashMap<String, usize>,
    patterns: Vec<PatternDefinition>,
}

enum SuiteExpectation {
    DecisionCase {
        expected: Decision,
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

struct SuiteResult {
    decision_cases: usize,
    failures: BoundedIssues<SuiteFailure>,
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
    Comparison(PathBuf),
    Help,
    Run(Arguments),
    Suite(PathBuf),
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
    let mut comparison_file = None;
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
            "--comparison-file" => {
                if comparison_file.is_some() {
                    return Err("Option `--comparison-file` may be specified only once".to_owned());
                }

                let Some(path) = arguments.next() else {
                    return Err("Option `--comparison-file` requires a path".to_owned());
                };
                comparison_file = Some(PathBuf::from(path));
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

    if let Some(comparison_file) = comparison_file {
        if case_sensitive
            || cases_file.is_some()
            || input_file.is_some()
            || pattern_file.is_some()
            || suite_file.is_some()
        {
            return Err(
                "Option `--comparison-file` is mutually exclusive with `--case-sensitive`, `--cases-file`, `--input-file`, `--pattern-file`, and `--suite-file`"
                    .to_owned(),
            );
        }

        return Ok(ParsedArguments::Comparison(comparison_file));
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

fn deserialize_decision<'de, D>(deserializer: D) -> Result<Decision, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    Decision::parse(&value)
        .ok_or_else(|| de::Error::unknown_variant(&value, &["allow", "confirm", "deny"]))
}

fn invalid_suite_line(path: &Path, line_number: usize) -> String {
    format!(
        "Invalid suite manifest `{}` at line {line_number}. Expected a documented suite record",
        path.display()
    )
}

fn resolve_manifest_path(manifest_file: &Path, referenced_file: &str) -> PathBuf {
    let referenced_file = Path::new(referenced_file);
    if referenced_file.is_absolute() {
        referenced_file.to_owned()
    } else {
        manifest_file
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
    let input_file = resolve_manifest_path(suite_file, input_file);

    read_utf8_file(&input_file, "suite input")
}

fn parse_suite_manifest(path: &Path, manifest: &str) -> Result<SuiteManifest, String> {
    let mut default = None;
    let mut expectations = Vec::new();
    let mut pattern_indices = HashMap::new();
    let mut patterns: Vec<PatternDefinition> = Vec::new();

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
                let Some(decision) = Decision::parse(fields[1]) else {
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
                let Some(bucket) = Bucket::parse(fields[2]) else {
                    return Err(invalid_suite_line(path, line_number));
                };
                let case_sensitive = match fields[3] {
                    "case-insensitive" => false,
                    "case-sensitive" => true,
                    _ => return Err(invalid_suite_line(path, line_number)),
                };
                if pattern_indices.contains_key(fields[1]) {
                    return Err(format!(
                        "Duplicate pattern id `{}` in suite manifest `{}` at line {line_number}",
                        fields[1],
                        path.display()
                    ));
                }

                let id = fields[1].to_owned();
                pattern_indices.insert(id.clone(), patterns.len());
                patterns.push(PatternDefinition {
                    bucket,
                    case_sensitive,
                    id,
                    pattern_file: resolve_manifest_path(path, fields[4]),
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
                let Some(expected) = Decision::parse(fields[1]) else {
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
    let mut pattern_case_coverage = vec![false; patterns.len()];
    for expectation in &expectations {
        let SuiteExpectation::PatternCase {
            line_number,
            pattern_id,
            ..
        } = expectation
        else {
            continue;
        };
        let Some(pattern_index) = pattern_indices.get(pattern_id) else {
            return Err(format!(
                "Unknown pattern id `{pattern_id}` in suite manifest `{}` at line {line_number}",
                path.display()
            ));
        };
        pattern_case_coverage[*pattern_index] = true;
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

    for (pattern, has_pattern_case) in patterns.iter().zip(pattern_case_coverage) {
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
        pattern_indices,
        patterns,
    })
}

fn validate_comparison_pattern_set(
    comparison_file: &Path,
    label: &str,
    definition: &ComparisonPatternSetDefinition,
) -> Result<(), String> {
    if definition.patterns.is_empty() {
        return Err(format!(
            "Comparison manifest `{}` must define at least one {label} pattern",
            comparison_file.display()
        ));
    }

    let mut ids = HashSet::with_capacity(definition.patterns.len());
    for pattern in &definition.patterns {
        if pattern.id.is_empty() {
            return Err(format!(
                "Comparison manifest `{}` contains an empty {label} pattern id",
                comparison_file.display()
            ));
        }
        if pattern.pattern_file.is_empty() {
            return Err(format!(
                "Comparison pattern `{}` in the {label} set must define a nonempty `pattern_file`",
                pattern.id
            ));
        }
        if !ids.insert(pattern.id.as_str()) {
            return Err(format!(
                "Duplicate pattern id `{}` in the {label} set of comparison manifest `{}`",
                pattern.id,
                comparison_file.display()
            ));
        }
    }

    Ok(())
}

fn validate_comparison_manifest(
    comparison_file: &Path,
    definition: &ComparisonManifestDefinition,
) -> Result<(), String> {
    if definition.version != COMPARISON_VERSION {
        return Err(format!(
            "Unsupported comparison manifest version {} in `{}`. Expected version {COMPARISON_VERSION}",
            definition.version,
            comparison_file.display()
        ));
    }

    validate_comparison_pattern_set(comparison_file, "baseline", &definition.baseline)?;
    validate_comparison_pattern_set(comparison_file, "candidate", &definition.candidate)?;

    if definition.cases.is_empty() {
        return Err(format!(
            "Comparison manifest `{}` must define at least one case",
            comparison_file.display()
        ));
    }

    for (index, case) in definition.cases.iter().enumerate() {
        let case_position = index + 1;
        match case {
            ComparisonCaseDefinition::File { input_file } if input_file.is_empty() => {
                return Err(format!(
                    "File comparison case {case_position} in `{}` must define a nonempty `input_file`",
                    comparison_file.display()
                ));
            }
            ComparisonCaseDefinition::Inline { input }
                if input.contains('\r') || input.contains('\n') =>
            {
                return Err(format!(
                    "Inline comparison case {case_position} in `{}` must not contain CR or LF. Use a file case for multiline input",
                    comparison_file.display()
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

fn resolve_pattern_definitions(
    comparison_file: &Path,
    definitions: Vec<ComparisonPatternDefinition>,
) -> Vec<PatternDefinition> {
    definitions
        .into_iter()
        .map(|definition| PatternDefinition {
            bucket: definition.bucket,
            case_sensitive: definition.case_sensitive,
            id: definition.id,
            pattern_file: resolve_manifest_path(comparison_file, &definition.pattern_file),
        })
        .collect()
}

fn compile_patterns(
    definitions: Vec<PatternDefinition>,
    collection: PatternCollection,
) -> Result<Vec<CompiledPattern>, String> {
    let mut patterns = Vec::with_capacity(definitions.len());

    for definition in definitions {
        let description = collection.read_description(&definition.id);
        let pattern = read_utf8_file(&definition.pattern_file, &description)?;
        let regex = match compile_pattern(&pattern, definition.case_sensitive) {
            Ok(regex) => regex,
            Err(PatternError::Empty) => {
                return Err(format!(
                    "Pattern file `{}` for {} `{}` is empty",
                    definition.pattern_file.display(),
                    collection.owner_label(),
                    definition.id
                ));
            }
            Err(PatternError::Invalid(error)) => {
                let summary = regex_error_summary(&error);

                return Err(format!(
                    "Invalid regex in pattern file `{}` for {} `{}`: {summary}",
                    definition.pattern_file.display(),
                    collection.owner_label(),
                    definition.id
                ));
            }
        };

        patterns.push(CompiledPattern {
            bucket: definition.bucket,
            id: definition.id,
            regex,
        });
    }

    Ok(patterns)
}

fn read_comparison_input(
    comparison_file: &Path,
    case_position: usize,
    case: ComparisonCaseDefinition,
) -> Result<String, String> {
    match case {
        ComparisonCaseDefinition::File { input_file } => {
            let input_file = resolve_manifest_path(comparison_file, &input_file);
            let description = format!("comparison case {case_position} input");

            read_utf8_file(&input_file, &description)
        }
        ComparisonCaseDefinition::Inline { input } => Ok(input),
    }
}

fn comparison_json_error(error: &serde_json::Error) -> String {
    let summary = match error.classify() {
        serde_json::error::Category::Data => {
            "JSON data does not match the version-1 comparison schema"
        }
        serde_json::error::Category::Eof => "JSON input ended before the manifest was complete",
        serde_json::error::Category::Io => "JSON input could not be read",
        serde_json::error::Category::Syntax => "JSON syntax is invalid",
    };

    format!(
        "{summary} at line {} column {}",
        error.line(),
        error.column()
    )
}

fn parse_comparison_manifest(
    comparison_file: &Path,
    manifest: &str,
) -> Result<ComparisonManifestDefinition, String> {
    let definition: ComparisonManifestDefinition =
        serde_json::from_str(manifest).map_err(|error| {
            let summary = comparison_json_error(&error);

            format!(
                "Invalid comparison manifest `{}`: {summary}",
                comparison_file.display()
            )
        })?;
    validate_comparison_manifest(comparison_file, &definition)?;

    Ok(definition)
}

fn evaluate_comparison(comparison_file: &Path) -> Result<ComparisonResult, String> {
    let manifest = read_utf8_file(comparison_file, "comparison manifest")?;
    let ComparisonManifestDefinition {
        baseline,
        candidate,
        cases,
        version: _,
    } = parse_comparison_manifest(comparison_file, &manifest)?;
    let baseline_default = baseline.default;
    let candidate_default = candidate.default;
    let baseline_patterns = compile_patterns(
        resolve_pattern_definitions(comparison_file, baseline.patterns),
        PatternCollection::BaselineComparison,
    )?;
    let candidate_patterns = compile_patterns(
        resolve_pattern_definitions(comparison_file, candidate.patterns),
        PatternCollection::CandidateComparison,
    )?;
    let baseline_pattern_count = baseline_patterns.len();
    let candidate_pattern_count = candidate_patterns.len();
    let case_count = cases.len();
    let mut mismatches = BoundedIssues::new(MAX_REPORTED_FAILURES);

    for (index, case) in cases.into_iter().enumerate() {
        let case_position = index + 1;
        let input = read_comparison_input(comparison_file, case_position, case)?;
        let baseline_state = MatchState::evaluate(&input, &baseline_patterns);
        let candidate_state = MatchState::evaluate(&input, &candidate_patterns);
        let differences = ComparisonDifferences::between(
            baseline_state,
            baseline_default,
            candidate_state,
            candidate_default,
        );
        if !differences.is_empty() {
            mismatches.push(ComparisonMismatch {
                case_position,
                differences,
            });
        }
    }

    Ok(ComparisonResult {
        baseline_pattern_count,
        candidate_pattern_count,
        case_count,
        comparison_file: comparison_file.to_owned(),
        mismatches,
    })
}

fn evaluate_suite(suite_file: &Path) -> Result<SuiteResult, String> {
    let manifest = read_utf8_file(suite_file, "suite manifest")?;
    let SuiteManifest {
        default,
        expectations,
        pattern_indices,
        patterns,
    } = parse_suite_manifest(suite_file, &manifest)?;
    let patterns = compile_patterns(patterns, PatternCollection::Suite)?;
    let pattern_count = patterns.len();
    let mut decision_cases = 0;
    let mut failures = BoundedIssues::new(MAX_REPORTED_FAILURES);
    let mut pattern_cases = 0;

    for expectation in expectations {
        match expectation {
            SuiteExpectation::DecisionCase {
                expected,
                input,
                line_number,
            } => {
                decision_cases += 1;
                let state = MatchState::evaluate(&input, &patterns);
                if state.decision(default) != expected {
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
                let pattern_index = pattern_indices[&pattern_id];
                if patterns[pattern_index].regex.is_match(&input) != expected_match {
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
            let mut failures = BoundedIssues::new(MAX_REPORTED_FAILURES);
            for case in cases {
                if regex.is_match(&case.input) != case.expected_match {
                    failures.push(BatchFailure {
                        expected_match: case.expected_match,
                        line_number: case.line_number,
                    });
                }
            }

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

fn mismatch_label(count: usize) -> &'static str {
    if count == 1 { "mismatch" } else { "mismatches" }
}

fn pattern_label(count: usize) -> &'static str {
    if count == 1 { "pattern" } else { "patterns" }
}

fn report_batch_failures(stderr: &mut dyn Write, result: &BatchResult) -> io::Result<()> {
    writeln!(
        stderr,
        "zed-pattern-match: {} of {} {} failed in `{}`",
        result.failures.total_count(),
        result.total,
        case_label(result.total),
        result.cases_file.display()
    )?;
    for failure in result.failures.issues() {
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

    let omitted = result.failures.omitted_count();
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
        result.failures.total_count(),
        result.suite_file.display()
    )?;
    for failure in result.failures.issues() {
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

    let omitted = result.failures.omitted_count();
    if omitted > 0 {
        writeln!(
            stderr,
            "  … {omitted} additional {} omitted",
            failure_label(omitted)
        )?;
    }

    Ok(())
}

fn comparison_dimension_labels(differences: &ComparisonDifferences) -> Vec<&'static str> {
    let mut labels = Vec::with_capacity(4);
    if differences.allow_bucket {
        labels.push("always_allow bucket");
    }
    if differences.confirm_bucket {
        labels.push("always_confirm bucket");
    }
    if differences.deny_bucket {
        labels.push("always_deny bucket");
    }
    if differences.final_decision {
        labels.push("final decision");
    }

    labels
}

fn report_comparison_mismatches(
    stderr: &mut dyn Write,
    result: &ComparisonResult,
) -> io::Result<()> {
    writeln!(
        stderr,
        "zed-pattern-match: {} {} across {} comparison {} in `{}`",
        result.mismatches.total_count(),
        mismatch_label(result.mismatches.total_count()),
        result.case_count,
        case_label(result.case_count),
        result.comparison_file.display()
    )?;
    for mismatch in result.mismatches.issues() {
        let dimensions = comparison_dimension_labels(&mismatch.differences).join(", ");
        writeln!(
            stderr,
            "  Case {} differs in: {dimensions}",
            mismatch.case_position
        )?;
    }

    let omitted = result.mismatches.omitted_count();
    if omitted > 0 {
        writeln!(
            stderr,
            "  … {omitted} additional {} omitted",
            mismatch_label(omitted)
        )?;
    }

    Ok(())
}

fn report_error(stderr: &mut dyn Write, message: &str) {
    let _ = writeln!(stderr, "zed-pattern-match: {message}");
}

fn report_equivalent_comparison(
    stdout: &mut dyn Write,
    result: &ComparisonResult,
) -> Result<(), String> {
    writeln!(
        stdout,
        "Representative corpus comparison found equivalent configured pattern behavior across {} {} with {} baseline {} and {} candidate {}",
        result.case_count,
        case_label(result.case_count),
        result.baseline_pattern_count,
        pattern_label(result.baseline_pattern_count),
        result.candidate_pattern_count,
        pattern_label(result.candidate_pattern_count)
    )
    .map_err(|error| format!("Failed to write comparison result:\n\n{error}"))
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
        ParsedArguments::Comparison(comparison_file) => match evaluate_comparison(&comparison_file)
        {
            Ok(result) if result.mismatches.total_count() == 0 => {
                if let Err(error) = report_equivalent_comparison(stdout, &result) {
                    report_error(stderr, &error);
                    return STATUS_ERROR;
                }

                STATUS_MATCH
            }
            Ok(result) => {
                if report_comparison_mismatches(stderr, &result).is_err() {
                    return STATUS_ERROR;
                }

                STATUS_NO_MATCH
            }
            Err(error) => {
                report_error(stderr, &error);
                STATUS_ERROR
            }
        },
        ParsedArguments::Help => {
            if let Err(error) = stdout.write_all(HELP.as_bytes()) {
                report_error(stderr, &format!("Failed to write help:\n\n{error}"));
                return STATUS_ERROR;
            }

            STATUS_MATCH
        }
        ParsedArguments::Run(arguments) => match evaluate(&arguments) {
            Ok(Evaluation::Batch(result)) if result.failures.total_count() == 0 => {
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
            Ok(result) if result.failures.total_count() == 0 => {
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
