#[allow(dead_code)]
#[path = "helpers/permission-patterns.rs"]
mod permission_patterns;

#[cfg(test)]
pub(crate) use permission_patterns::sha256_hex;
pub(crate) use permission_patterns::{
    BoundedIssues, Bucket, CompiledPattern, Decision, MatchState, PatternError, compile_pattern,
    read_utf8_file, regex_error_summary,
};
use permission_patterns::{load_artifact_catalog, verify_artifact_catalog_binding};
use serde::{Deserialize, Deserializer, de};
use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const HELP: &str = concat!(
    "Usage:\n",
    "  pattern-match [--case-sensitive] --input-file <path> --pattern-file <path>\n",
    "  pattern-match [--case-sensitive] --cases-file <path> --pattern-file <path>\n",
    "  pattern-match --comparison-file <path>\n",
    "  pattern-match --suite-file <path>\n",
    "  pattern-match --help\n",
    "\n",
    "Match one UTF-8 input, verify case and suite expectations against Zed-compatible regex patterns, or compare configured pattern sets\n",
    "\n",
    "Options:\n",
    "  --case-sensitive          Match case-sensitively (default: case-insensitive)\n",
    "  --cases-file <path>       Read LF-delimited `match<TAB><input>` and `no-match<TAB><input>` cases\n",
    "  --comparison-file <path>  Compare baseline and candidate pattern sets over a representative corpus from a version-1 or version-2 JSON manifest. Mutually exclusive with every other option\n",
    "  --help                    Print help. Must be used alone\n",
    "  --input-file <path>       Read one complete UTF-8 input from this file\n",
    "  --pattern-file <path>     Read the complete UTF-8 pattern from this file\n",
    "  --suite-file <path>       Verify pattern and decision cases from an LF-delimited suite manifest\n",
    "\n",
    "LF-delimited UTF-8 suite manifest with records in any order:\n",
    "  decision-case<TAB>allow|confirm|deny<TAB><input>\n",
    "  decision-case-file<TAB>allow|confirm|deny<TAB><input-file>\n",
    "  default<TAB>allow|confirm|deny\n",
    "  catalog-pattern<TAB><catalog-id><TAB><pattern-id>\n",
    "  pattern<TAB><id><TAB>always_allow|always_confirm|always_deny<TAB>case-sensitive|case-insensitive<TAB><pattern-file>\n",
    "  pattern-catalog<TAB><catalog-id><TAB><catalog-file><TAB><candidate-file><TAB><state-file>\n",
    "  pattern-case<TAB><id><TAB>match|no-match<TAB><input>\n",
    "  pattern-case-file<TAB><id><TAB>match|no-match<TAB><input-file>\n",
    "  Relative suite paths resolve from the suite file’s parent. Catalog artifact paths resolve from the catalog’s parent\n",
    "\n",
    "Suite requirements:\n",
    "  Define exactly one `default` record, at least one ordinary or catalog-backed pattern, and at least one `decision-case` or `decision-case-file` record\n",
    "  Define at least one `pattern-case` or `pattern-case-file` record for every pattern ID\n",
    "  Keep inline inputs single-line. Use file-backed records for multiline inputs\n",
    "  Each decision case applies configured pattern precedence to one input\n",
    "  Suite verification does not reproduce full Zed permission evaluation\n",
    "\n",
    "Version-1 UTF-8 JSON comparison manifest:\n",
    "  Root: {\"version\":1,\"baseline\":<set>,\"candidate\":<set>,\"cases\":[<case>,...]}\n",
    "  Set: {\"default\":\"allow|confirm|deny\",\"patterns\":[<pattern>,...]}\n",
    "  Pattern: {\"id\":\"...\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"case_sensitive\":true|false,\"pattern_file\":\"path\"}\n",
    "  Inline case: {\"type\":\"inline\",\"input\":\"single line\"}\n",
    "  File case: {\"type\":\"file\",\"input_file\":\"path\"}\n",
    "\n",
    "Version-2 UTF-8 JSON comparison manifest:\n",
    "  Root adds strict `catalogs` declarations and uses explicitly tagged `file` or `catalog` patterns\n",
    "  File pattern: {\"type\":\"file\",\"id\":\"...\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"case_sensitive\":true|false,\"pattern_file\":\"path\"}\n",
    "  Catalog pattern: {\"type\":\"catalog\",\"catalog_id\":\"...\",\"pattern_id\":\"...\"}\n",
    "  Catalog: {\"id\":\"...\",\"catalog_file\":\"path\",\"candidate_file\":\"path\",\"state_file\":\"path\"}\n",
    "  Cases may add a complete `expected_transition` with baseline and candidate bucket booleans plus `final_decision`\n",
    "  Relative manifest paths resolve from the comparison file’s parent. Catalog artifact paths resolve from the catalog’s parent\n",
    "\n",
    "Comparison requirements:\n",
    "  Define at least one pattern in each set and at least one case\n",
    "  Keep pattern IDs nonempty and unique within each set\n",
    "  Keep inline inputs single-line. Use file cases for multiline inputs\n",
    "  For each case, comparison checks whether each bucket matched and compares the configured final decision\n",
    "  Cases without `expected_transition` require complete equivalence. Declared transitions require both complete observed states\n",
    "  Declared final decisions must agree with deny, confirm, allow, then default precedence. No-op transitions are invalid\n",
    "  Comparison covers only the representative corpus\n",
    "  It does not reproduce full Zed permission evaluation or establish formal language equivalence\n",
    "\n",
    "Output:\n",
    "  Single-input matching writes no output\n",
    "  Successful case manifest verification writes the verified case count to standard output\n",
    "  Successful comparison writes corpus case, baseline pattern, and candidate pattern counts to standard output\n",
    "  Successful suite verification writes pattern case, decision case, and pattern counts to standard output\n",
    "  Expectation failures and comparison mismatches write at most 10 details to standard error without echoing regexes or inputs\n",
    "  Argument, data, and I/O errors write a diagnostic to standard error\n",
    "\n",
    "Exit statuses:\n",
    "  0  Pattern matched, every expectation passed, comparison found no mismatches, or help displayed\n",
    "  1  Pattern did not match, an expectation failed, or comparison found mismatches\n",
    "  2  Invalid arguments or data, or an I/O failure\n",
);

const COMPARISON_VERSION_1: u64 = 1;
const COMPARISON_VERSION_2: u64 = 2;
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
struct ComparisonVersion {
    version: u64,
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

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogDefinition {
    id: String,
    catalog_file: String,
    candidate_file: String,
    state_file: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ComparisonManifestV2 {
    baseline: ComparisonPatternSetV2,
    candidate: ComparisonPatternSetV2,
    cases: Vec<ComparisonCaseV2>,
    catalogs: Vec<CatalogDefinition>,
    version: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ComparisonPatternSetV2 {
    #[serde(deserialize_with = "deserialize_decision")]
    default: Decision,
    patterns: Vec<ComparisonPatternV2>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case", tag = "type")]
enum ComparisonPatternV2 {
    Catalog {
        catalog_id: String,
        pattern_id: String,
    },
    File {
        bucket: Bucket,
        case_sensitive: bool,
        id: String,
        pattern_file: String,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case", tag = "type")]
enum ComparisonCaseV2 {
    File {
        input_file: String,
        #[serde(default, deserialize_with = "deserialize_expected_transition")]
        expected_transition: Option<ExpectedTransition>,
    },
    Inline {
        input: String,
        #[serde(default, deserialize_with = "deserialize_expected_transition")]
        expected_transition: Option<ExpectedTransition>,
    },
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct ExpectedTransition {
    baseline: DeclaredComparisonState,
    candidate: DeclaredComparisonState,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct DeclaredComparisonState {
    always_allow: bool,
    always_confirm: bool,
    always_deny: bool,
    #[serde(deserialize_with = "deserialize_decision")]
    final_decision: Decision,
}

impl DeclaredComparisonState {
    fn observed(state: MatchState, default: Decision) -> Self {
        Self {
            always_allow: state.matched(Bucket::Allow),
            always_confirm: state.matched(Bucket::Confirm),
            always_deny: state.matched(Bucket::Deny),
            final_decision: state.decision(default),
        }
    }

    fn recomputed_decision(self, default: Decision) -> Decision {
        MatchState {
            allow: self.always_allow,
            confirm: self.always_confirm,
            deny: self.always_deny,
        }
        .decision(default)
    }
}

#[derive(Clone, Default)]
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

    fn from_declared(observed: DeclaredComparisonState, expected: DeclaredComparisonState) -> Self {
        Self {
            allow_bucket: observed.always_allow != expected.always_allow,
            confirm_bucket: observed.always_confirm != expected.always_confirm,
            deny_bucket: observed.always_deny != expected.always_deny,
            final_decision: observed.final_decision != expected.final_decision,
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

enum ComparisonMismatchV2 {
    Equivalence {
        case_position: usize,
        differences: ComparisonDifferences,
    },
    Transition {
        baseline_differences: ComparisonDifferences,
        candidate_differences: ComparisonDifferences,
        case_position: usize,
    },
}

struct ComparisonResultV2 {
    baseline_pattern_count: usize,
    candidate_pattern_count: usize,
    case_count: usize,
    comparison_file: PathBuf,
    equivalence_case_count: usize,
    mismatches: BoundedIssues<ComparisonMismatchV2>,
    transition_case_count: usize,
}

struct PatternDefinition {
    bucket: Bucket,
    case_sensitive: bool,
    id: String,
    reported_id: String,
    source: PatternSource,
}

enum PatternSource {
    Catalog(String),
    File(PathBuf),
}

struct LoadedCatalogBinding {
    patterns: HashMap<String, CatalogPattern>,
}

struct CatalogPattern {
    bucket: Bucket,
    case_sensitive: bool,
    pattern: String,
}

enum SuitePatternDeclaration {
    Catalog {
        catalog_id: String,
        line_number: usize,
        pattern_id: String,
    },
    File {
        bucket: Bucket,
        case_sensitive: bool,
        id: String,
        line_number: usize,
        pattern_file: PathBuf,
    },
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
    pattern_report_ids: HashMap<String, String>,
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

enum ComparisonEvaluation {
    Version1(ComparisonResult),
    Version2(ComparisonResultV2),
}

enum ParsedComparisonManifest {
    Version1(ComparisonManifestDefinition),
    Version2(ComparisonManifestV2),
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
                    "Unknown option `{option}`. Run `pattern-match --help` for usage"
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
                "Missing required option. Specify either `--cases-file` or `--input-file`"
                    .to_owned(),
            );
        }
    };
    let pattern_file =
        pattern_file.ok_or_else(|| "Missing required option `--pattern-file`".to_owned())?;

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
        return Err(format!("Case manifest `{}` is empty", path.display()));
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

fn deserialize_expected_transition<'de, D>(
    deserializer: D,
) -> Result<Option<ExpectedTransition>, D::Error>
where
    D: Deserializer<'de>,
{
    ExpectedTransition::deserialize(deserializer).map(Some)
}

fn invalid_suite_line(path: &Path, line_number: usize) -> String {
    format!(
        "Invalid suite manifest `{}` at line {line_number}. Expected a record format shown by `pattern-match --help`",
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

fn display_id(id: &str) -> String {
    let mut output = String::new();
    let mut characters = id.chars();

    for _ in 0..80 {
        let Some(character) = characters.next() else {
            return output;
        };
        if character.is_control() {
            output.push('?');
        } else {
            output.push(character);
        }
    }
    if characters.next().is_some() {
        output.push('…');
    }

    output
}

fn read_catalog_source(
    manifest_file: &Path,
    catalog: &CatalogDefinition,
) -> Result<LoadedCatalogBinding, String> {
    let catalog_file = resolve_manifest_path(manifest_file, &catalog.catalog_file);
    let candidate_file = resolve_manifest_path(manifest_file, &catalog.candidate_file);
    let state_file = resolve_manifest_path(manifest_file, &catalog.state_file);
    let candidate_bytes = fs::read(&candidate_file).map_err(|error| {
        format!(
            "Failed to read candidate file for pattern catalog `{}` at `{}`:\n\n{error}",
            display_id(&catalog.id),
            candidate_file.display()
        )
    })?;
    let state_bytes = fs::read(&state_file).map_err(|error| {
        format!(
            "Failed to read state file for pattern catalog `{}` at `{}`:\n\n{error}",
            display_id(&catalog.id),
            state_file.display()
        )
    })?;
    let loaded = load_artifact_catalog(&catalog_file).map_err(|error| {
        format!(
            "Invalid pattern catalog `{}` at `{}`. {error}",
            display_id(&catalog.id),
            catalog_file.display()
        )
    })?;
    verify_artifact_catalog_binding(&loaded.document, &candidate_bytes, &state_bytes).map_err(
        |error| {
            format!(
                "Invalid pattern catalog `{}` source binding. {error}",
                display_id(&catalog.id)
            )
        },
    )?;
    let patterns = loaded
        .patterns
        .into_iter()
        .map(|pattern| {
            (
                pattern.definition.id,
                CatalogPattern {
                    bucket: pattern.definition.bucket,
                    case_sensitive: pattern.definition.case_sensitive,
                    pattern: pattern.pattern,
                },
            )
        })
        .collect();

    Ok(LoadedCatalogBinding { patterns })
}

fn load_catalogs(
    manifest_file: &Path,
    definitions: Vec<CatalogDefinition>,
    owner: &str,
) -> Result<HashMap<String, LoadedCatalogBinding>, String> {
    validate_catalog_definitions(&definitions, owner)?;

    let mut catalogs = HashMap::with_capacity(definitions.len());
    for definition in definitions {
        let loaded = read_catalog_source(manifest_file, &definition)?;
        catalogs.insert(definition.id, loaded);
    }

    Ok(catalogs)
}

fn catalog_pattern_definition(
    catalogs: &HashMap<String, LoadedCatalogBinding>,
    catalog_id: &str,
    pattern_id: &str,
    owner: &str,
) -> Result<PatternDefinition, String> {
    let catalog = catalogs.get(catalog_id).ok_or_else(|| {
        format!(
            "Unknown pattern catalog ID `{}` in {owner}",
            display_id(catalog_id)
        )
    })?;
    let pattern = catalog.patterns.get(pattern_id).ok_or_else(|| {
        format!(
            "Unknown pattern ID `{}` in pattern catalog `{}` for {owner}",
            display_id(pattern_id),
            display_id(catalog_id)
        )
    })?;

    Ok(PatternDefinition {
        bucket: pattern.bucket,
        case_sensitive: pattern.case_sensitive,
        id: pattern_id.to_owned(),
        reported_id: display_id(pattern_id),
        source: PatternSource::Catalog(pattern.pattern.clone()),
    })
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
    let mut catalog_definitions = Vec::new();
    let mut catalog_ids = HashSet::new();
    let mut declarations = Vec::new();
    let mut declared_pattern_ids = HashSet::new();
    let mut default = None;
    let mut expectations = Vec::new();

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
                        "Duplicate `default` record in suite manifest `{}` at line {line_number}",
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
                if !declared_pattern_ids.insert(fields[1].to_owned()) {
                    return Err(format!(
                        "Duplicate pattern ID `{}` in suite manifest `{}` at line {line_number}",
                        fields[1],
                        path.display()
                    ));
                }

                declarations.push(SuitePatternDeclaration::File {
                    bucket,
                    case_sensitive,
                    id: fields[1].to_owned(),
                    line_number,
                    pattern_file: resolve_manifest_path(path, fields[4]),
                });
            }
            Some("pattern-catalog") => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 5
                    || fields[1].is_empty()
                    || fields[2].is_empty()
                    || fields[3].is_empty()
                    || fields[4].is_empty()
                {
                    return Err(invalid_suite_line(path, line_number));
                }
                if !catalog_ids.insert(fields[1].to_owned()) {
                    return Err(format!(
                        "Duplicate pattern catalog ID `{}` in suite manifest `{}` at line {line_number}",
                        display_id(fields[1]),
                        path.display()
                    ));
                }
                catalog_definitions.push(CatalogDefinition {
                    id: fields[1].to_owned(),
                    catalog_file: fields[2].to_owned(),
                    candidate_file: fields[3].to_owned(),
                    state_file: fields[4].to_owned(),
                });
            }
            Some("catalog-pattern") => {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() != 3 || fields[1].is_empty() || fields[2].is_empty() {
                    return Err(invalid_suite_line(path, line_number));
                }
                if !declared_pattern_ids.insert(fields[2].to_owned()) {
                    return Err(format!(
                        "Duplicate pattern ID `{}` in suite manifest `{}` at line {line_number}",
                        display_id(fields[2]),
                        path.display()
                    ));
                }
                declarations.push(SuitePatternDeclaration::Catalog {
                    catalog_id: fields[1].to_owned(),
                    line_number,
                    pattern_id: fields[2].to_owned(),
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
            "Suite manifest `{}` must define exactly one `default` record",
            path.display()
        )
    })?;
    if declarations.is_empty() {
        return Err(format!(
            "Suite manifest `{}` must define at least one pattern",
            path.display()
        ));
    }
    let owner = format!("suite manifest `{}`", path.display());
    let catalogs = load_catalogs(path, catalog_definitions, &owner)?;
    let mut patterns = Vec::with_capacity(declarations.len());
    let mut pattern_indices = HashMap::with_capacity(declarations.len());
    for declaration in declarations {
        let (line_number, pattern) = match declaration {
            SuitePatternDeclaration::Catalog {
                catalog_id,
                line_number,
                pattern_id,
            } => (
                line_number,
                catalog_pattern_definition(&catalogs, &catalog_id, &pattern_id, &owner)?,
            ),
            SuitePatternDeclaration::File {
                bucket,
                case_sensitive,
                id,
                line_number,
                pattern_file,
            } => (
                line_number,
                PatternDefinition {
                    bucket,
                    case_sensitive,
                    reported_id: id.clone(),
                    id,
                    source: PatternSource::File(pattern_file),
                },
            ),
        };
        if pattern_indices
            .insert(pattern.id.clone(), patterns.len())
            .is_some()
        {
            return Err(format!(
                "Duplicate pattern ID `{}` in suite manifest `{}` at line {line_number}",
                pattern.reported_id,
                path.display()
            ));
        }
        patterns.push(pattern);
    }
    let pattern_report_ids = patterns
        .iter()
        .map(|pattern| (pattern.id.clone(), pattern.reported_id.clone()))
        .collect();
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
                "Unknown pattern ID `{pattern_id}` in suite manifest `{}` at line {line_number}",
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
            "Suite manifest `{}` must include at least one `decision-case` or `decision-case-file` record",
            path.display()
        ));
    }

    for (pattern, has_pattern_case) in patterns.iter().zip(pattern_case_coverage) {
        if !has_pattern_case {
            return Err(format!(
                "Suite manifest `{}` must include at least one `pattern-case` or `pattern-case-file` record for pattern `{}`",
                path.display(),
                pattern.reported_id
            ));
        }
    }

    Ok(SuiteManifest {
        default,
        expectations,
        pattern_indices,
        pattern_report_ids,
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
                "Comparison manifest `{}` contains an empty {label} pattern ID",
                comparison_file.display()
            ));
        }
        if pattern.pattern_file.is_empty() {
            return Err(format!(
                "Comparison pattern `{}` in the {label} set of comparison manifest `{}` must define a nonempty `pattern_file`",
                pattern.id,
                comparison_file.display()
            ));
        }
        if !ids.insert(pattern.id.as_str()) {
            return Err(format!(
                "Duplicate pattern ID `{}` in the {label} set of comparison manifest `{}`",
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
    if definition.version != COMPARISON_VERSION_1 {
        return Err(format!(
            "Unsupported comparison manifest version in `{}`. Expected `{COMPARISON_VERSION_1}`, received `{}`",
            comparison_file.display(),
            definition.version
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

fn validate_comparison_pattern_set_v2(
    comparison_file: &Path,
    label: &str,
    definition: &ComparisonPatternSetV2,
) -> Result<(), String> {
    if definition.patterns.is_empty() {
        return Err(format!(
            "Comparison manifest `{}` must define at least one {label} pattern",
            comparison_file.display()
        ));
    }

    let mut ids = HashSet::with_capacity(definition.patterns.len());
    for pattern in &definition.patterns {
        let id = match pattern {
            ComparisonPatternV2::Catalog {
                catalog_id,
                pattern_id,
            } => {
                if catalog_id.is_empty() || pattern_id.is_empty() {
                    return Err(format!(
                        "Comparison manifest `{}` contains a {label} catalog-backed pattern with an empty catalog or pattern ID",
                        comparison_file.display()
                    ));
                }
                pattern_id
            }
            ComparisonPatternV2::File {
                id, pattern_file, ..
            } => {
                if id.is_empty() {
                    return Err(format!(
                        "Comparison manifest `{}` contains an empty {label} pattern ID",
                        comparison_file.display()
                    ));
                }
                if pattern_file.is_empty() {
                    return Err(format!(
                        "Comparison pattern `{}` in the {label} set of comparison manifest `{}` must define a nonempty `pattern_file`",
                        display_id(id),
                        comparison_file.display()
                    ));
                }
                id
            }
        };
        if !ids.insert(id.as_str()) {
            return Err(format!(
                "Duplicate pattern ID `{}` in the {label} set of comparison manifest `{}`",
                display_id(id),
                comparison_file.display()
            ));
        }
    }

    Ok(())
}

fn validate_catalog_definitions(
    definitions: &[CatalogDefinition],
    owner: &str,
) -> Result<(), String> {
    let mut ids = HashSet::with_capacity(definitions.len());
    for definition in definitions {
        if definition.id.is_empty()
            || definition.catalog_file.is_empty()
            || definition.candidate_file.is_empty()
            || definition.state_file.is_empty()
        {
            return Err(format!(
                "{owner} contains a pattern catalog declaration with an empty ID or path"
            ));
        }
        if !ids.insert(definition.id.as_str()) {
            return Err(format!(
                "Duplicate pattern catalog ID `{}` in {owner}",
                display_id(&definition.id)
            ));
        }
    }

    Ok(())
}

fn validate_declared_transition(
    comparison_file: &Path,
    case_position: usize,
    transition: ExpectedTransition,
    baseline_default: Decision,
    candidate_default: Decision,
) -> Result<(), String> {
    for (side, state, default) in [
        ("baseline", transition.baseline, baseline_default),
        ("candidate", transition.candidate, candidate_default),
    ] {
        if state.recomputed_decision(default) != state.final_decision {
            return Err(format!(
                "Expected transition in comparison case {case_position} of `{}` declares a {side} final decision that contradicts configured precedence",
                comparison_file.display()
            ));
        }
    }
    if transition.baseline == transition.candidate {
        return Err(format!(
            "Expected transition in comparison case {case_position} of `{}` is a no-op",
            comparison_file.display()
        ));
    }

    Ok(())
}

fn validate_comparison_manifest_v2(
    comparison_file: &Path,
    definition: &ComparisonManifestV2,
) -> Result<(), String> {
    if definition.version != COMPARISON_VERSION_2 {
        return Err(format!(
            "Unsupported comparison manifest version in `{}`. Expected `{COMPARISON_VERSION_2}`, received `{}`",
            comparison_file.display(),
            definition.version
        ));
    }

    validate_comparison_pattern_set_v2(comparison_file, "baseline", &definition.baseline)?;
    validate_comparison_pattern_set_v2(comparison_file, "candidate", &definition.candidate)?;
    let owner = format!("comparison manifest `{}`", comparison_file.display());
    validate_catalog_definitions(&definition.catalogs, &owner)?;

    if definition.cases.is_empty() {
        return Err(format!(
            "Comparison manifest `{}` must define at least one case",
            comparison_file.display()
        ));
    }

    for (index, case) in definition.cases.iter().enumerate() {
        let case_position = index + 1;
        let (input_file, input, transition) = match case {
            ComparisonCaseV2::File {
                input_file,
                expected_transition,
            } => (Some(input_file), None, *expected_transition),
            ComparisonCaseV2::Inline {
                input,
                expected_transition,
            } => (None, Some(input), *expected_transition),
        };
        if input_file.is_some_and(|path| path.is_empty()) {
            return Err(format!(
                "File comparison case {case_position} in `{}` must define a nonempty `input_file`",
                comparison_file.display()
            ));
        }
        if input.is_some_and(|value| value.contains('\r') || value.contains('\n')) {
            return Err(format!(
                "Inline comparison case {case_position} in `{}` must not contain CR or LF. Use a file case for multiline input",
                comparison_file.display()
            ));
        }
        if let Some(transition) = transition {
            validate_declared_transition(
                comparison_file,
                case_position,
                transition,
                definition.baseline.default,
                definition.candidate.default,
            )?;
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
            reported_id: definition.id.clone(),
            id: definition.id,
            source: PatternSource::File(resolve_manifest_path(
                comparison_file,
                &definition.pattern_file,
            )),
        })
        .collect()
}

fn resolve_pattern_definitions_v2(
    comparison_file: &Path,
    definitions: Vec<ComparisonPatternV2>,
    catalogs: &HashMap<String, LoadedCatalogBinding>,
    owner: &str,
) -> Result<Vec<PatternDefinition>, String> {
    definitions
        .into_iter()
        .map(|definition| match definition {
            ComparisonPatternV2::Catalog {
                catalog_id,
                pattern_id,
            } => catalog_pattern_definition(catalogs, &catalog_id, &pattern_id, owner),
            ComparisonPatternV2::File {
                bucket,
                case_sensitive,
                id,
                pattern_file,
            } => Ok(PatternDefinition {
                bucket,
                case_sensitive,
                reported_id: display_id(&id),
                id,
                source: PatternSource::File(resolve_manifest_path(comparison_file, &pattern_file)),
            }),
        })
        .collect()
}

fn compile_patterns(
    definitions: Vec<PatternDefinition>,
    collection: PatternCollection,
) -> Result<Vec<CompiledPattern>, String> {
    let mut patterns = Vec::with_capacity(definitions.len());

    for definition in definitions {
        let description = collection.read_description(&definition.reported_id);
        let (pattern, source_label) = match definition.source {
            PatternSource::Catalog(pattern) => (pattern, None),
            PatternSource::File(pattern_file) => {
                let pattern = read_utf8_file(&pattern_file, &description)?;
                (pattern, Some(pattern_file))
            }
        };
        let regex = match compile_pattern(&pattern, definition.case_sensitive) {
            Ok(regex) => regex,
            Err(PatternError::Empty) => {
                return Err(match source_label {
                    Some(pattern_file) => format!(
                        "Pattern file `{}` for {} `{}` is empty",
                        pattern_file.display(),
                        collection.owner_label(),
                        definition.reported_id
                    ),
                    None => format!(
                        "Catalog-backed {} `{}` is empty",
                        collection.owner_label(),
                        definition.reported_id
                    ),
                });
            }
            Err(PatternError::Invalid(error)) => {
                let summary = regex_error_summary(&error);

                return Err(match source_label {
                    Some(pattern_file) => format!(
                        "Failed to compile regex from pattern file `{}` for {} `{}`:\n\n{summary}",
                        pattern_file.display(),
                        collection.owner_label(),
                        definition.reported_id
                    ),
                    None => format!(
                        "Failed to compile regex from catalog-backed {} `{}`:\n\n{summary}",
                        collection.owner_label(),
                        definition.reported_id
                    ),
                });
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
        serde_json::error::Category::Io => "Failed to read JSON input",
        serde_json::error::Category::Syntax => "JSON syntax is invalid",
    };

    format!(
        "{summary} at line {} column {}",
        error.line(),
        error.column()
    )
}

fn comparison_json_error_v2(error: &serde_json::Error) -> String {
    let summary = match error.classify() {
        serde_json::error::Category::Data => {
            "JSON data does not match the version-2 comparison schema"
        }
        serde_json::error::Category::Eof => "JSON input ended before the manifest was complete",
        serde_json::error::Category::Io => "Failed to read JSON input",
        serde_json::error::Category::Syntax => "JSON syntax is invalid",
    };

    format!(
        "{summary} at line {} column {}",
        error.line(),
        error.column()
    )
}

fn invalid_comparison_json(
    comparison_file: &Path,
    error: &serde_json::Error,
    version: Option<u64>,
) -> String {
    let summary = if version == Some(COMPARISON_VERSION_2) {
        comparison_json_error_v2(error)
    } else {
        comparison_json_error(error)
    };

    format!(
        "Invalid comparison manifest `{}`. {summary}",
        comparison_file.display()
    )
}

fn parse_comparison_manifest(
    comparison_file: &Path,
    manifest: &str,
) -> Result<ParsedComparisonManifest, String> {
    let version: ComparisonVersion = serde_json::from_str(manifest)
        .map_err(|error| invalid_comparison_json(comparison_file, &error, None))?;

    match version.version {
        COMPARISON_VERSION_1 => {
            let definition: ComparisonManifestDefinition =
                serde_json::from_str(manifest).map_err(|error| {
                    invalid_comparison_json(comparison_file, &error, Some(COMPARISON_VERSION_1))
                })?;
            validate_comparison_manifest(comparison_file, &definition)?;
            Ok(ParsedComparisonManifest::Version1(definition))
        }
        COMPARISON_VERSION_2 => {
            let definition: ComparisonManifestV2 =
                serde_json::from_str(manifest).map_err(|error| {
                    invalid_comparison_json(comparison_file, &error, Some(COMPARISON_VERSION_2))
                })?;
            validate_comparison_manifest_v2(comparison_file, &definition)?;
            Ok(ParsedComparisonManifest::Version2(definition))
        }
        received => Err(format!(
            "Unsupported comparison manifest version in `{}`. Expected `1` or `2`, received `{received}`",
            comparison_file.display()
        )),
    }
}

fn evaluate_comparison_v1(
    comparison_file: &Path,
    definition: ComparisonManifestDefinition,
) -> Result<ComparisonResult, String> {
    let ComparisonManifestDefinition {
        baseline,
        candidate,
        cases,
        version: _,
    } = definition;
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

fn read_comparison_input_v2(
    comparison_file: &Path,
    case_position: usize,
    case: ComparisonCaseV2,
) -> Result<(String, Option<ExpectedTransition>), String> {
    match case {
        ComparisonCaseV2::File {
            input_file,
            expected_transition,
        } => {
            let input_file = resolve_manifest_path(comparison_file, &input_file);
            let description = format!("comparison case {case_position} input");
            let input = read_utf8_file(&input_file, &description)?;

            Ok((input, expected_transition))
        }
        ComparisonCaseV2::Inline {
            input,
            expected_transition,
        } => Ok((input, expected_transition)),
    }
}

fn evaluate_comparison_v2(
    comparison_file: &Path,
    definition: ComparisonManifestV2,
) -> Result<ComparisonResultV2, String> {
    let ComparisonManifestV2 {
        baseline,
        candidate,
        cases,
        catalogs,
        version: _,
    } = definition;
    let baseline_default = baseline.default;
    let candidate_default = candidate.default;
    let owner = format!("comparison manifest `{}`", comparison_file.display());
    let catalogs = load_catalogs(comparison_file, catalogs, &owner)?;
    let baseline_patterns = compile_patterns(
        resolve_pattern_definitions_v2(comparison_file, baseline.patterns, &catalogs, &owner)?,
        PatternCollection::BaselineComparison,
    )?;
    let candidate_patterns = compile_patterns(
        resolve_pattern_definitions_v2(comparison_file, candidate.patterns, &catalogs, &owner)?,
        PatternCollection::CandidateComparison,
    )?;
    let baseline_pattern_count = baseline_patterns.len();
    let candidate_pattern_count = candidate_patterns.len();
    let case_count = cases.len();
    let mut equivalence_case_count = 0;
    let mut mismatches = BoundedIssues::new(MAX_REPORTED_FAILURES);
    let mut transition_case_count = 0;

    for (index, case) in cases.into_iter().enumerate() {
        let case_position = index + 1;
        let (input, expected_transition) =
            read_comparison_input_v2(comparison_file, case_position, case)?;
        let baseline_state = MatchState::evaluate(&input, &baseline_patterns);
        let candidate_state = MatchState::evaluate(&input, &candidate_patterns);

        if let Some(expected) = expected_transition {
            transition_case_count += 1;
            let baseline_observed =
                DeclaredComparisonState::observed(baseline_state, baseline_default);
            let candidate_observed =
                DeclaredComparisonState::observed(candidate_state, candidate_default);
            let baseline_differences =
                ComparisonDifferences::from_declared(baseline_observed, expected.baseline);
            let candidate_differences =
                ComparisonDifferences::from_declared(candidate_observed, expected.candidate);
            if !baseline_differences.is_empty() || !candidate_differences.is_empty() {
                mismatches.push(ComparisonMismatchV2::Transition {
                    baseline_differences,
                    candidate_differences,
                    case_position,
                });
            }
        } else {
            equivalence_case_count += 1;
            let differences = ComparisonDifferences::between(
                baseline_state,
                baseline_default,
                candidate_state,
                candidate_default,
            );
            if !differences.is_empty() {
                mismatches.push(ComparisonMismatchV2::Equivalence {
                    case_position,
                    differences,
                });
            }
        }
    }

    Ok(ComparisonResultV2 {
        baseline_pattern_count,
        candidate_pattern_count,
        case_count,
        comparison_file: comparison_file.to_owned(),
        equivalence_case_count,
        mismatches,
        transition_case_count,
    })
}

fn evaluate_comparison(comparison_file: &Path) -> Result<ComparisonEvaluation, String> {
    let manifest = read_utf8_file(comparison_file, "comparison manifest")?;

    match parse_comparison_manifest(comparison_file, &manifest)? {
        ParsedComparisonManifest::Version1(definition) => {
            evaluate_comparison_v1(comparison_file, definition).map(ComparisonEvaluation::Version1)
        }
        ParsedComparisonManifest::Version2(definition) => {
            evaluate_comparison_v2(comparison_file, definition).map(ComparisonEvaluation::Version2)
        }
    }
}

fn evaluate_suite(suite_file: &Path) -> Result<SuiteResult, String> {
    let manifest = read_utf8_file(suite_file, "suite manifest")?;
    let SuiteManifest {
        default,
        expectations,
        pattern_indices,
        pattern_report_ids,
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
                    let reported_pattern_id = pattern_report_ids[&pattern_id].clone();
                    failures.push(SuiteFailure {
                        expectation: SuiteFailureExpectation::Pattern {
                            expected_match,
                            pattern_id: reported_pattern_id,
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
                "Failed to compile regex from pattern file `{}`:\n\n{summary}",
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
    if count == 1 {
        "decision case"
    } else {
        "decision cases"
    }
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
        "pattern-match: {} of {} {} failed in `{}`",
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
        "pattern-match: {} of {total} suite expectations failed in `{}`",
        result.failures.total_count(),
        result.suite_file.display()
    )?;
    for failure in result.failures.issues() {
        match &failure.expectation {
            SuiteFailureExpectation::Decision(expected) => {
                writeln!(
                    stderr,
                    "  Line {} expected configured decision `{}`",
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
        labels.push("`always_allow` bucket");
    }
    if differences.confirm_bucket {
        labels.push("`always_confirm` bucket");
    }
    if differences.deny_bucket {
        labels.push("`always_deny` bucket");
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
        "pattern-match: {} {} across {} comparison {} in `{}`",
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
            "  Case {} differs in {dimensions}",
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

fn report_comparison_mismatches_v2(
    stderr: &mut dyn Write,
    result: &ComparisonResultV2,
) -> io::Result<()> {
    writeln!(
        stderr,
        "pattern-match: {} {} across {} version-2 comparison {} in `{}`",
        result.mismatches.total_count(),
        mismatch_label(result.mismatches.total_count()),
        result.case_count,
        case_label(result.case_count),
        result.comparison_file.display()
    )?;
    for mismatch in result.mismatches.issues() {
        match mismatch {
            ComparisonMismatchV2::Equivalence {
                case_position,
                differences,
            } => {
                let dimensions = comparison_dimension_labels(differences).join(", ");
                writeln!(
                    stderr,
                    "  Case {case_position} baseline/candidate differ in {dimensions}"
                )?;
            }
            ComparisonMismatchV2::Transition {
                baseline_differences,
                candidate_differences,
                case_position,
            } => {
                let baseline = comparison_dimension_labels(baseline_differences).join(", ");
                let candidate = comparison_dimension_labels(candidate_differences).join(", ");
                match (baseline.is_empty(), candidate.is_empty()) {
                    (false, false) => writeln!(
                        stderr,
                        "  Case {case_position} baseline differs in {baseline}, and candidate differs in {candidate}"
                    )?,
                    (false, true) => writeln!(
                        stderr,
                        "  Case {case_position} baseline differs in {baseline}"
                    )?,
                    (true, false) => writeln!(
                        stderr,
                        "  Case {case_position} candidate differs in {candidate}"
                    )?,
                    (true, true) => unreachable!("Stored transition mismatches have differences"),
                }
            }
        }
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
    let _ = writeln!(stderr, "pattern-match: {message}");
}

fn report_equivalent_comparison(
    stdout: &mut dyn Write,
    result: &ComparisonResult,
) -> Result<(), String> {
    writeln!(
        stdout,
        "Baseline and candidate configured pattern behavior matched across a representative corpus of {} {} with {} baseline {} and {} candidate {}",
        result.case_count,
        case_label(result.case_count),
        result.baseline_pattern_count,
        pattern_label(result.baseline_pattern_count),
        result.candidate_pattern_count,
        pattern_label(result.candidate_pattern_count)
    )
    .map_err(|error| format!("Failed to write comparison result to standard output:\n\n{error}"))
}

fn report_verified_comparison_v2(
    stdout: &mut dyn Write,
    result: &ComparisonResultV2,
) -> Result<(), String> {
    writeln!(
        stdout,
        "Verified a representative version-2 comparison corpus with {} equivalence {}, {} matched {}, {} baseline {}, and {} candidate {}",
        result.equivalence_case_count,
        case_label(result.equivalence_case_count),
        result.transition_case_count,
        if result.transition_case_count == 1 {
            "transition"
        } else {
            "transitions"
        },
        result.baseline_pattern_count,
        pattern_label(result.baseline_pattern_count),
        result.candidate_pattern_count,
        pattern_label(result.candidate_pattern_count)
    )
    .map_err(|error| format!("Failed to write comparison result to standard output:\n\n{error}"))
}

fn report_verified_cases(stdout: &mut dyn Write, total: usize) -> Result<(), String> {
    writeln!(stdout, "Verified {total} {}", case_label(total)).map_err(|error| {
        format!("Failed to write case verification result to standard output:\n\n{error}")
    })
}

fn report_verified_suite(stdout: &mut dyn Write, result: &SuiteResult) -> Result<(), String> {
    writeln!(
        stdout,
        "Verified {} pattern {} and {} {} across {} {}",
        result.pattern_cases,
        case_label(result.pattern_cases),
        result.decision_cases,
        decision_label(result.decision_cases),
        result.pattern_count,
        pattern_label(result.pattern_count)
    )
    .map_err(|error| format!("Failed to write suite result to standard output:\n\n{error}"))
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
            Ok(ComparisonEvaluation::Version1(result)) if result.mismatches.total_count() == 0 => {
                if let Err(error) = report_equivalent_comparison(stdout, &result) {
                    report_error(stderr, &error);
                    return STATUS_ERROR;
                }

                STATUS_MATCH
            }
            Ok(ComparisonEvaluation::Version1(result)) => {
                if report_comparison_mismatches(stderr, &result).is_err() {
                    return STATUS_ERROR;
                }

                STATUS_NO_MATCH
            }
            Ok(ComparisonEvaluation::Version2(result)) if result.mismatches.total_count() == 0 => {
                if let Err(error) = report_verified_comparison_v2(stdout, &result) {
                    report_error(stderr, &error);
                    return STATUS_ERROR;
                }

                STATUS_MATCH
            }
            Ok(ComparisonEvaluation::Version2(result)) => {
                if report_comparison_mismatches_v2(stderr, &result).is_err() {
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
                report_error(
                    stderr,
                    &format!("Failed to write help to standard output:\n\n{error}"),
                );
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
