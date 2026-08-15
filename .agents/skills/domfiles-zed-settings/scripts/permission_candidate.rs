#[allow(dead_code)]
#[path = "helpers/permission_patterns.rs"]
pub(crate) mod permission_patterns;

use permission_patterns::{
    ArtifactCatalog, ArtifactCatalogPattern, BoundArtifact, BoundEntryPosition, BoundPosition,
    BoundedIssues, Bundle, ClosureContext, InputClosureBuilder, LoadedArtifactCatalog,
    ManifestBinding, OUTCOME_PASSED, OwnerOperationKind, OwnerSpec, PATH_OVERLAY_FILE, PathOverlay,
    PatternError, PositionRemap, ResolvedOverlay, ResultKind, StateDocument, StatePattern,
    SupplementalSide, TerminalPosition, ValidationEntry, ValidationPlan, ValidationPlanEntry,
    ValidationResult, compile_pattern, infer_witness_owner, is_valid_sha256,
    load_bound_artifact_catalog, owner_source_matcher, parse_strict_json, read_utf8_file,
    regex_error_summary, relative_within_root, resolve_audit_closure, resolve_comparison_closure,
    resolve_layer_closure, resolve_suite_closure, serialize_pretty_json_bytes,
    terminal_pattern_at as snapshot_pattern_at, validate_artifact_catalog, validate_owner_spec,
    verify_input_closure, verify_visibility_transformation,
};
pub(crate) use permission_patterns::{Bucket, sha256_hex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env,
    ffi::{OsStr, OsString},
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Component, Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

const ARTIFACT_CATALOG_FILE: &str = "artifact-catalog.json";
const BASELINE_FILE: &str = "baseline-settings.json";
const CANDIDATE_FILE: &str = "candidate-settings.json";
const MAX_REPORTED_ITEMS: usize = 100;
const MAX_REPORTED_MATERIALIZED_ITEMS: usize = 10;
const MAX_REPORTED_VERIFY_ITEMS: usize = 10;
const STATE_FILE: &str = "state.json";
const STATUS_ERROR: u8 = 2;
const STATUS_REFUSED: u8 = 1;
const STATUS_SUCCESS: u8 = 0;

static NEXT_TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

const HELP: &str = concat!(
    "Usage:\n",
    "  permission-candidate capture --settings <path> --selection <selection-path> --output <directory>\n",
    "  permission-candidate materialize --candidate <candidate-path> --state <state-path> --selection <selection-path> --output <directory>\n",
    "  permission-candidate verify --settings <path> --state <state-path>\n",
    "  permission-candidate seal --candidate <candidate-path> --state <state-path> --catalog <artifact-catalog-path> --owner-spec <owner-spec-path> --validation <validation-path> --output <bundle-path>\n",
    "  permission-candidate preflight --settings <live-settings-path> --bundle <bundle-path>\n",
    "  permission-candidate refresh --settings <settings-path> --bundle <bundle-path> --output <directory>\n",
    "  permission-candidate promote --settings <live-settings-path> --bundle <bundle-path> --write\n",
    "  permission-candidate --help\n",
    "\n",
    "Capture exact Zed permission candidates, materialize authorized terminal patterns, seal a reviewed graph with its evidence, rehearse promotion, refresh a stale graph, and promote bundle-bound scopes\n",
    "\n",
    "Modes:\n",
    "  capture      Read exact settings bytes, validate authorized scopes and selected terminal pattern objects, and create capture artifacts\n",
    "  materialize  Validate a captured state and authorized candidate, then create exact selected pattern artifacts and a bound catalog\n",
    "  verify       Validate every state artifact, then check captured scopes or uniquely locate captured terminal patterns.\n",
    "               It establishes source identity only, never promotion readiness\n",
    "  seal         Bind the reviewed graph and its fresh evidence into one bundle, deriving owner coverage mechanically\n",
    "  preflight    Run every promotion check read-only. The rehearsal expires as soon as live settings can change\n",
    "  refresh      Rebuild a stale reviewed graph against current settings into a new unsealed directory\n",
    "  promote      Rerun the complete preflight in-process, then atomically replace live settings\n",
    "\n",
    "Options:\n",
    "  --bundle <path>             Sealed bundle consumed by `preflight`, `promote`, and `refresh`\n",
    "  --candidate <path>          Candidate JSON object used by `materialize` or `seal`\n",
    "  --catalog <path>            Artifact catalog used by `seal`\n",
    "  --help                      Print help. Must be used alone\n",
    "  --output <path>             Artifact directory for `capture`, `materialize`, and `refresh`, or the bundle file for `seal`\n",
    "  --owner-spec <path>         Stable owner specification used by `seal` only\n",
    "  --selection <path>          Capture or materialization selection JSON selected by the mode\n",
    "  --settings <path>           Baseline or current settings for `capture`, `verify`, and `refresh`, or the live destination for `preflight` and `promote`\n",
    "  --state <path>              State manifest used by `materialize`, `verify`, and `seal`\n",
    "  --validation <path>         Validation manifest listing the fresh evidence `seal` binds\n",
    "  --write                     Required exact mutation guard for `promote`\n",
    "\n",
    "Capture selection JSON schema (unknown fields are rejected):\n",
    "  {\"scopes\":[\"/json/pointer\"],\"patterns\":[{\"id\":\"nonempty\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"index\":0}]}\n",
    "  `scopes` must be nonempty. `patterns` may be empty. Pattern IDs and bucket/index selections must be unique\n",
    "  Scopes must be existing, non-root RFC 6901 pointers with no duplicates or parent/child overlap\n",
    "  Every selected pattern object must lie within an authorized scope and contain string `pattern` and boolean `case_sensitive` fields\n",
    "\n",
    "Materialization selection JSON schema (unknown fields are rejected):\n",
    "  {\"patterns\":[{\"id\":\"nonempty\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"index\":0}]}\n",
    "  `patterns` may be empty. Pattern IDs and bucket/index selections must be unique\n",
    "  Each bucket/index is a transient locator into the exact candidate bound by the catalog\n",
    "  Ownership is declared by the owner spec, not the selection. A selection carrying `owner_replacement` is rejected\n",
    "\n",
    "State JSON schema (unknown fields are rejected):\n",
    "  {\"baseline_file\":\"relative path\",\"baseline_sha256\":\"64 lowercase hex characters\",\"scopes\":[\"/json/pointer\"],\"patterns\":[{\"id\":\"nonempty\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"source_index\":0,\"case_sensitive\":true,\"sha256\":\"64 lowercase hex characters\",\"pattern_file\":\"relative path\"}]}\n",
    "  `patterns` may be empty. A patternless state can promote scope-only changes or catalog-bound insertion-only owners\n",
    "  Relative baseline and pattern paths resolve from the state manifest’s parent\n",
    "  The manifest records hashes but does not authenticate itself\n",
    "\n",
    "Artifact catalog JSON schema (unknown fields are rejected):\n",
    "  {\"candidate_sha256\":\"64 lowercase hex characters\",\"state_sha256\":\"64 lowercase hex characters\",\"patterns\":[{\"id\":\"nonempty\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"source_index\":0,\"case_sensitive\":true,\"sha256\":\"64 lowercase hex characters\",\"pattern_file\":\"relative path\"}]}\n",
    "  `patterns` may be empty. Relative pattern paths resolve from the catalog’s parent\n",
    "  Candidate, state, and artifact hashes provide integrity and freshness but not authenticity\n",
    "\n",
    "Capture contract:\n",
    "  Settings must parse as a JSON object and are retained byte-for-byte as immutable `baseline-settings.json`\n",
    "  An editable byte-identical `candidate-settings.json` is created beside the baseline\n",
    "  Pattern files contain exact decoded UTF-8 pattern bytes with no added newline\n",
    "  Generated pattern names use a sequence and sanitized ID. Raw IDs never become paths\n",
    "  The output directory’s parent must exist. The output directory itself may already exist\n",
    "  Symlink traversal, non-directory output paths, and existing artifact paths are refused\n",
    "\n",
    "Materialize contract:\n",
    "  Every state artifact and recorded hash is validated before candidate authorization\n",
    "  Candidate values outside authorized scopes must equal the captured baseline\n",
    "  Every selected object must lie under an authorized scope and provide string `pattern` and boolean `case_sensitive` fields\n",
    "  Every selected candidate regex is compiled with the Zed-compatible engine before it can become promotable\n",
    "  Pattern files contain exact decoded candidate UTF-8 bytes with no added newline or reserialization\n",
    "  Create-new writes use complete preflight, symlink refusal, safe generated filenames, rollback, and overwrite refusal\n",
    "  Candidate, baseline, state, and live settings remain untouched\n",
    "\n",
    "Verify contract:\n",
    "  Baseline and pattern hashes, JSON structure, scopes, UTF-8, and recorded baseline source identities are validated before reindexing\n",
    "  Missing or duplicate exact current matches are refused\n",
    "  A state without terminal patterns instead requires every current authorized scope to equal its captured baseline scope\n",
    "  Successful output contains at most 10 moved `id -> bucket[index]` metadata lines, an omission summary when needed, and aggregate counts\n",
    "  Missing or duplicate refusal reports at most 10 exceptional IDs and counts every failure\n",
    "\n",
    "Promote contract:\n",
    "  `--bundle` and `--write` are mandatory. There is no force option\n",
    "  The sealed bundle supplies the candidate, state, catalog, owner specification, and bound validation evidence\n",
    "  Neither the bundle, a passing preflight, nor `--write` is user approval to promote\n",
    "  Promotion reruns the complete preflight in-process immediately before the mutation boundary\n",
    "  The bundle must bind the exact candidate and state bytes, every artifact, and every candidate source identity\n",
    "  Every state pattern and every catalog entry must be claimed exactly once by an owner operation or a declared overlap\n",
    "  Per-owner accounting is independent, and ordered remainder equality plus per-bucket count reconciliation stop one owner cancelling another\n",
    "  An empty insertion catalog instead requires all terminal pattern arrays to remain semantically unchanged\n",
    "  Live values at every authorized scope must equal the captured baseline values\n",
    "  Candidate changes outside authorized scopes are refused, and absent parents are never created\n",
    "  Candidate scope values are merged into the live object read for promotion, preserving its out-of-scope values\n",
    "  Changed output uses tabs, preserves object insertion order, and ends with exactly one newline\n",
    "  Byte-identical output leaves live settings untouched\n",
    "  Changed output is written to a create-new same-directory sibling, assigned live permissions, synced, and atomically renamed\n",
    "  Promotion rechecks live bytes immediately before rename on a best-effort basis. A writer can still race after that check\n",
    "  The live destination and every traversed component must not be a symlink\n",
    "\n",
    "Output:\n",
    "  Successful capture, materialization, verification, seal, preflight, refresh, and promotion results are written to standard output\n",
    "  Help is written to standard output\n",
    "  Materialization reports aggregate counts, the catalog path, and at most 10 `id -> artifact` metadata lines\n",
    "  Refusals and errors are written to standard error\n",
    "  Output never includes candidate or state hashes, pattern bodies, settings contents, complete arrays, or unbounded IDs\n",
    "\n",
    "Exit statuses:\n",
    "  0  Capture, materialization, verification, seal, preflight, refresh, promotion, unchanged promotion, or help succeeded\n",
    "  1  Current state could not be uniquely reindexed, or candidate authorization, owner coverage, refresh replay, preflight, or guarded promotion was refused\n",
    "  2  Arguments or data were invalid, or an I/O operation failed\n",
);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SelectionDocument {
    scopes: Vec<String>,
    patterns: Vec<SelectionPattern>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SelectionPattern {
    id: String,
    bucket: Bucket,
    index: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MaterializationSelectionDocument {
    patterns: Vec<MaterializationSelectionPattern>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MaterializationSelectionPattern {
    id: String,
    bucket: Bucket,
    index: usize,
}

struct CaptureArguments {
    settings: PathBuf,
    selection: PathBuf,
    output: PathBuf,
}

struct MaterializeArguments {
    candidate: PathBuf,
    state: PathBuf,
    selection: PathBuf,
    output: PathBuf,
}

struct VerifyArguments {
    settings: PathBuf,
    state: PathBuf,
}

struct PreflightArguments {
    bundle: PathBuf,
    settings: PathBuf,
}

struct SealArguments {
    candidate: PathBuf,
    catalog: PathBuf,
    output: PathBuf,
    owner_spec: PathBuf,
    state: PathBuf,
    validation: PathBuf,
}

struct RefreshArguments {
    bundle: PathBuf,
    output: PathBuf,
    settings: PathBuf,
}

struct PromoteArguments {
    bundle: PathBuf,
    settings: PathBuf,
}

enum Operation {
    Capture(CaptureArguments),
    Materialize(MaterializeArguments),
    Preflight(PreflightArguments),
    Promote(PromoteArguments),
    Refresh(RefreshArguments),
    Seal(SealArguments),
    Verify(VerifyArguments),
}

enum ParsedArguments {
    Help,
    Run(Operation),
}

#[derive(Debug)]
enum AppError {
    Invalid(String),
    Refused(String),
}

#[derive(Debug)]
pub(crate) enum PathInspectionError {
    Io(String),
    Symlink(PathBuf),
}

impl fmt::Display for PathInspectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(message) => formatter.write_str(message),
            Self::Symlink(path) => {
                write!(
                    formatter,
                    "Path traverses symbolic link `{}`",
                    path.display()
                )
            }
        }
    }
}

impl AppError {
    fn status(&self) -> u8 {
        match self {
            Self::Invalid(_) => STATUS_ERROR,
            Self::Refused(_) => STATUS_REFUSED,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::Invalid(message) | Self::Refused(message) => message,
        }
    }
}

struct CapturedPattern {
    bytes: Vec<u8>,
    state: StatePattern,
}

struct MaterializedPattern {
    bytes: Vec<u8>,
    catalog: ArtifactCatalogPattern,
}

pub(crate) struct PendingArtifact {
    pub(crate) filename: String,
    pub(crate) bytes: Vec<u8>,
}

#[derive(Clone, Copy)]
pub(crate) enum ArtifactOperation {
    Capture,
    Materialization,
}

impl ArtifactOperation {
    fn label(self) -> &'static str {
        match self {
            Self::Capture => "Capture",
            Self::Materialization => "Materialization",
        }
    }

    fn lowercase_label(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Materialization => "materialization",
        }
    }
}

struct LoadedPattern {
    id: String,
    bucket: Bucket,
    source_index: usize,
    case_sensitive: bool,
    bytes: Vec<u8>,
}

struct ValidatedState {
    baseline: Value,
    bytes: Vec<u8>,
    document: StateDocument,
    patterns: Vec<LoadedPattern>,
    scopes: Vec<Vec<String>>,
}

struct TemporarySibling {
    path: PathBuf,
    remove_on_drop: bool,
}

impl TemporarySibling {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            remove_on_drop: true,
        }
    }

    fn preserve(mut self) {
        self.remove_on_drop = false;
    }
}

impl Drop for TemporarySibling {
    fn drop(&mut self) {
        if self.remove_on_drop {
            let _ = fs::remove_file(&self.path);
        }
    }
}

struct CreatedDirectories {
    paths: Vec<PathBuf>,
    remove_on_drop: bool,
}

impl CreatedDirectories {
    fn new() -> Self {
        Self {
            paths: Vec::new(),
            remove_on_drop: true,
        }
    }

    fn record(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    fn preserve(&mut self) {
        self.remove_on_drop = false;
    }
}

impl Drop for CreatedDirectories {
    fn drop(&mut self) {
        if self.remove_on_drop {
            for path in self.paths.iter().rev() {
                let _ = fs::remove_dir(path);
            }
        }
    }
}

struct CreatedRefreshArtifact {
    path: PathBuf,
    remove_on_drop: bool,
    sha256: String,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
}

impl CreatedRefreshArtifact {
    fn new(path: PathBuf, file: &File, bytes: &[u8]) -> Result<Self, AppError> {
        let metadata = file.metadata().map_err(|error| {
            invalid(format!(
                "Failed to inspect created refresh artifact `{}`:\n\n{error}",
                path.display()
            ))
        })?;

        Ok(Self {
            path,
            remove_on_drop: true,
            sha256: sha256_hex(bytes),
            #[cfg(unix)]
            device: metadata.dev(),
            #[cfg(unix)]
            inode: metadata.ino(),
        })
    }

    fn preserve(&mut self) {
        self.remove_on_drop = false;
    }
}

impl Drop for CreatedRefreshArtifact {
    fn drop(&mut self) {
        if !self.remove_on_drop {
            return;
        }

        #[cfg(unix)]
        {
            let Ok(metadata) = fs::symlink_metadata(&self.path) else {
                return;
            };
            if metadata.file_type().is_symlink()
                || metadata.dev() != self.device
                || metadata.ino() != self.inode
            {
                return;
            }
        }

        let Ok(bytes) = fs::read(&self.path) else {
            return;
        };
        if sha256_hex(&bytes) != self.sha256 {
            return;
        }

        #[cfg(unix)]
        {
            let Ok(metadata) = fs::symlink_metadata(&self.path) else {
                return;
            };
            if metadata.file_type().is_symlink()
                || metadata.dev() != self.device
                || metadata.ino() != self.inode
            {
                return;
            }
        }

        let _ = fs::remove_file(&self.path);
    }
}

fn invalid(message: impl Into<String>) -> AppError {
    AppError::Invalid(message.into())
}

fn refused(message: impl Into<String>) -> AppError {
    AppError::Refused(message.into())
}

fn parse_arguments<I>(arguments: I) -> Result<ParsedArguments, AppError>
where
    I: IntoIterator<Item = OsString>,
{
    let arguments: Vec<OsString> = arguments.into_iter().collect();

    if arguments.len() == 1 && arguments[0].as_os_str() == OsStr::new("--help") {
        return Ok(ParsedArguments::Help);
    }
    if arguments
        .iter()
        .any(|argument| argument.as_os_str() == OsStr::new("--help"))
    {
        return Err(invalid("Option `--help` must be used alone"));
    }

    let Some(mode) = arguments.first() else {
        return Err(invalid(
            "Missing mode. Specify `capture`, `materialize`, `preflight`, `promote`, `refresh`, `seal`, or `verify`. Run `permission-candidate --help` for usage",
        ));
    };
    let Some(mode) = mode.to_str() else {
        return Err(invalid("Mode must be valid UTF-8"));
    };
    let options = &arguments[1..];

    match mode {
        "capture" => parse_capture_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Capture(arguments))),
        "materialize" => parse_materialize_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Materialize(arguments))),
        "verify" => parse_verify_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Verify(arguments))),
        "preflight" => parse_preflight_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Preflight(arguments))),
        "promote" => parse_promote_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Promote(arguments))),
        "refresh" => parse_refresh_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Refresh(arguments))),
        "seal" => parse_seal_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Seal(arguments))),
        _ => Err(invalid(format!(
            "Unknown mode `{mode}`. Run `permission-candidate --help` for usage"
        ))),
    }
}

fn option_name(argument: &OsStr) -> Result<&str, AppError> {
    argument
        .to_str()
        .ok_or_else(|| invalid("Option names must be valid UTF-8"))
}

fn take_path(options: &[OsString], index: &mut usize, option: &str) -> Result<PathBuf, AppError> {
    *index += 1;
    let Some(path) = options.get(*index) else {
        return Err(invalid(format!("Option `{option}` requires a path")));
    };

    Ok(PathBuf::from(path))
}

fn set_once(slot: &mut Option<PathBuf>, value: PathBuf, option: &str) -> Result<(), AppError> {
    if slot.replace(value).is_some() {
        return Err(invalid(format!(
            "Option `{option}` may be specified only once"
        )));
    }

    Ok(())
}

fn parse_capture_arguments(options: &[OsString]) -> Result<CaptureArguments, AppError> {
    let mut output = None;
    let mut selection = None;
    let mut settings = None;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--output" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut output, path, option)?;
            }
            "--selection" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut selection, path, option)?;
            }
            "--settings" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut settings, path, option)?;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown capture option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    Ok(CaptureArguments {
        settings: settings.ok_or_else(|| invalid("Missing required option `--settings <path>`"))?,
        selection: selection
            .ok_or_else(|| invalid("Missing required option `--selection <path>`"))?,
        output: output.ok_or_else(|| invalid("Missing required option `--output <directory>`"))?,
    })
}

fn parse_materialize_arguments(options: &[OsString]) -> Result<MaterializeArguments, AppError> {
    let mut candidate = None;
    let mut output = None;
    let mut selection = None;
    let mut state = None;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--candidate" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut candidate, path, option)?;
            }
            "--output" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut output, path, option)?;
            }
            "--selection" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut selection, path, option)?;
            }
            "--state" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut state, path, option)?;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown materialize option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    Ok(MaterializeArguments {
        candidate: candidate
            .ok_or_else(|| invalid("Missing required option `--candidate <candidate-path>`"))?,
        state: state.ok_or_else(|| invalid("Missing required option `--state <state-path>`"))?,
        selection: selection
            .ok_or_else(|| invalid("Missing required option `--selection <selection-path>`"))?,
        output: output.ok_or_else(|| invalid("Missing required option `--output <directory>`"))?,
    })
}

fn parse_verify_arguments(options: &[OsString]) -> Result<VerifyArguments, AppError> {
    let mut settings = None;
    let mut state = None;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--settings" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut settings, path, option)?;
            }
            "--state" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut state, path, option)?;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown verify option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    Ok(VerifyArguments {
        settings: settings.ok_or_else(|| invalid("Missing required option `--settings <path>`"))?,
        state: state.ok_or_else(|| invalid("Missing required option `--state <path>`"))?,
    })
}

fn parse_promote_arguments(options: &[OsString]) -> Result<PromoteArguments, AppError> {
    let mut bundle = None;
    let mut settings = None;
    let mut write = false;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--bundle" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut bundle, path, option)?;
            }
            "--settings" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut settings, path, option)?;
            }
            "--write" => {
                if write {
                    return Err(invalid("Option `--write` may be specified only once"));
                }
                write = true;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown promote option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    if !write {
        return Err(invalid(
            "Promotion requires the exact mutation guard `--write`",
        ));
    }

    Ok(PromoteArguments {
        settings: settings
            .ok_or_else(|| invalid("Missing required option `--settings <live-settings-path>`"))?,
        bundle: bundle
            .ok_or_else(|| invalid("Missing required option `--bundle <bundle-path>`"))?,
    })
}

fn parse_preflight_arguments(options: &[OsString]) -> Result<PreflightArguments, AppError> {
    let mut bundle = None;
    let mut settings = None;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--bundle" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut bundle, path, option)?;
            }
            "--settings" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut settings, path, option)?;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown preflight option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    Ok(PreflightArguments {
        settings: settings
            .ok_or_else(|| invalid("Missing required option `--settings <live-settings-path>`"))?,
        bundle: bundle
            .ok_or_else(|| invalid("Missing required option `--bundle <bundle-path>`"))?,
    })
}

fn parse_seal_arguments(options: &[OsString]) -> Result<SealArguments, AppError> {
    let mut candidate = None;
    let mut catalog = None;
    let mut output = None;
    let mut owner_spec = None;
    let mut state = None;
    let mut validation = None;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--candidate" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut candidate, path, option)?;
            }
            "--catalog" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut catalog, path, option)?;
            }
            "--output" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut output, path, option)?;
            }
            "--owner-spec" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut owner_spec, path, option)?;
            }
            "--state" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut state, path, option)?;
            }
            "--validation" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut validation, path, option)?;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown seal option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    Ok(SealArguments {
        candidate: candidate
            .ok_or_else(|| invalid("Missing required option `--candidate <candidate-path>`"))?,
        catalog: catalog.ok_or_else(|| {
            invalid("Missing required option `--catalog <artifact-catalog-path>`")
        })?,
        output: output
            .ok_or_else(|| invalid("Missing required option `--output <bundle-path>`"))?,
        owner_spec: owner_spec
            .ok_or_else(|| invalid("Missing required option `--owner-spec <owner-spec-path>`"))?,
        state: state.ok_or_else(|| invalid("Missing required option `--state <state-path>`"))?,
        validation: validation
            .ok_or_else(|| invalid("Missing required option `--validation <validation-path>`"))?,
    })
}

fn parse_refresh_arguments(options: &[OsString]) -> Result<RefreshArguments, AppError> {
    let mut bundle = None;
    let mut output = None;
    let mut settings = None;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--bundle" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut bundle, path, option)?;
            }
            "--output" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut output, path, option)?;
            }
            "--settings" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut settings, path, option)?;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown refresh option `{option}`. Run `permission-candidate --help` for usage"
                )));
            }
        }
        index += 1;
    }

    Ok(RefreshArguments {
        settings: settings
            .ok_or_else(|| invalid("Missing required option `--settings <settings-path>`"))?,
        bundle: bundle
            .ok_or_else(|| invalid("Missing required option `--bundle <bundle-path>`"))?,
        output: output.ok_or_else(|| invalid("Missing required option `--output <directory>`"))?,
    })
}

fn read_bytes(path: &Path, description: &str) -> Result<Vec<u8>, AppError> {
    fs::read(path).map_err(|error| {
        invalid(format!(
            "Failed to read {description} `{}`:\n\n{error}",
            path.display()
        ))
    })
}

fn invalid_json(description: &str, path: &Path, error: serde_json::Error) -> AppError {
    let summary = match error.classify() {
        serde_json::error::Category::Data => "JSON data does not match the required schema",
        serde_json::error::Category::Eof => "JSON input ends before a complete value",
        serde_json::error::Category::Io => "Failed to read JSON input",
        serde_json::error::Category::Syntax => "JSON syntax is invalid",
    };

    invalid(format!(
        "Invalid {description} `{}` at line {}, column {}. {summary}",
        path.display(),
        error.line(),
        error.column()
    ))
}

fn parse_json_object(bytes: &[u8], description: &str, path: &Path) -> Result<Value, AppError> {
    let value: Value =
        serde_json::from_slice(bytes).map_err(|error| invalid_json(description, path, error))?;
    if !value.is_object() {
        return Err(invalid(format!(
            "The {description} `{}` must contain a JSON object",
            path.display()
        )));
    }

    Ok(value)
}

fn read_json_object(path: &Path, description: &str) -> Result<(Vec<u8>, Value), AppError> {
    let bytes = read_bytes(path, description)?;
    let value = parse_json_object(&bytes, description, path)?;

    Ok((bytes, value))
}

fn decode_utf8(
    bytes: Vec<u8>,
    path: &Path,
    description: &str,
) -> Result<(Vec<u8>, String), AppError> {
    let contents = String::from_utf8(bytes).map_err(|error| {
        invalid(format!(
            "Invalid UTF-8 in {description} file `{}`:\n\n{error}",
            path.display()
        ))
    })?;
    let bytes = contents.as_bytes().to_vec();

    Ok((bytes, contents))
}

fn read_utf8_json_object_with_bytes(
    path: &Path,
    description: &str,
) -> Result<(Vec<u8>, Value), AppError> {
    let bytes = read_bytes(path, description)?;
    let (bytes, contents) = decode_utf8(bytes, path, description)?;
    let value = parse_json_object(contents.as_bytes(), description, path)?;

    Ok((bytes, value))
}

fn read_utf8_json_object(path: &Path, description: &str) -> Result<Value, AppError> {
    read_utf8_json_object_with_bytes(path, description).map(|(_, value)| value)
}

fn decode_reference_token(token: &str) -> Result<String, String> {
    let mut decoded = String::with_capacity(token.len());
    let mut characters = token.chars();

    while let Some(character) = characters.next() {
        if character != '~' {
            decoded.push(character);
            continue;
        }

        match characters.next() {
            Some('0') => decoded.push('~'),
            Some('1') => decoded.push('/'),
            _ => return Err("JSON Pointer contains a malformed `~` escape".to_owned()),
        }
    }

    Ok(decoded)
}

pub(crate) fn decode_json_pointer(pointer: &str) -> Result<Vec<String>, String> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    let Some(tokens) = pointer.strip_prefix('/') else {
        return Err("JSON Pointer must begin with `/`".to_owned());
    };

    tokens.split('/').map(decode_reference_token).collect()
}

fn parse_array_index(token: &str) -> Result<usize, String> {
    if token == "0" {
        return Ok(0);
    }
    if token.is_empty()
        || token.starts_with('0')
        || !token.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err("JSON Pointer array indexes must be canonical nonnegative integers".to_owned());
    }

    token
        .parse::<usize>()
        .map_err(|_| "JSON Pointer array index is too large".to_owned())
}

pub(crate) fn pointer_value<'a>(value: &'a Value, tokens: &[String]) -> Result<&'a Value, String> {
    let mut current = value;

    for token in tokens {
        current = match current {
            Value::Object(object) => object
                .get(token)
                .ok_or_else(|| "JSON Pointer refers to a missing object member".to_owned())?,
            Value::Array(array) => {
                let index = parse_array_index(token)?;
                array
                    .get(index)
                    .ok_or_else(|| "JSON Pointer array index is out of range".to_owned())?
            }
            _ => {
                return Err("JSON Pointer attempts to traverse a scalar or null value".to_owned());
            }
        };
    }

    Ok(current)
}

fn pointer_value_mut<'a>(value: &'a mut Value, tokens: &[String]) -> Result<&'a mut Value, String> {
    let mut current = value;

    for token in tokens {
        current = match current {
            Value::Object(object) => object
                .get_mut(token)
                .ok_or_else(|| "JSON Pointer refers to a missing object member".to_owned())?,
            Value::Array(array) => {
                let index = parse_array_index(token)?;
                array
                    .get_mut(index)
                    .ok_or_else(|| "JSON Pointer array index is out of range".to_owned())?
            }
            _ => {
                return Err("JSON Pointer attempts to traverse a scalar or null value".to_owned());
            }
        };
    }

    Ok(current)
}

pub(crate) fn replace_pointer_value(
    value: &mut Value,
    tokens: &[String],
    replacement: Value,
) -> Result<(), String> {
    let destination = pointer_value_mut(value, tokens)?;
    *destination = replacement;

    Ok(())
}

fn paths_overlap(first: &[String], second: &[String]) -> bool {
    first.starts_with(second) || second.starts_with(first)
}

pub(crate) fn validate_scopes(
    settings: &Value,
    scopes: &[String],
) -> Result<Vec<Vec<String>>, String> {
    if scopes.is_empty() {
        return Err("At least one authorized scope is required".to_owned());
    }

    let mut decoded = Vec::with_capacity(scopes.len());
    for scope in scopes {
        if scope.is_empty() {
            return Err("The document-root JSON Pointer is not an authorized scope".to_owned());
        }
        if !scope.starts_with('/') {
            return Err("Every authorized scope must begin with `/`".to_owned());
        }
        let tokens = decode_json_pointer(scope)?;
        pointer_value(settings, &tokens)
            .map_err(|_| "An authorized scope does not exist in settings".to_owned())?;
        if decoded
            .iter()
            .any(|existing: &Vec<String>| paths_overlap(existing, &tokens))
        {
            return Err(
                "Authorized scopes must not be duplicated or overlap as parent and child"
                    .to_owned(),
            );
        }
        decoded.push(tokens);
    }

    Ok(decoded)
}

fn pattern_pointer(bucket: Bucket, index: usize) -> Vec<String> {
    vec![
        "agent".to_owned(),
        "tool_permissions".to_owned(),
        "tools".to_owned(),
        "terminal".to_owned(),
        bucket.label().to_owned(),
        index.to_string(),
    ]
}

fn pattern_is_authorized(pattern: &[String], scopes: &[Vec<String>]) -> bool {
    scopes.iter().any(|scope| pattern.starts_with(scope))
}

pub(crate) fn terminal_pattern(
    settings: &Value,
    bucket: Bucket,
    index: usize,
) -> Result<(&str, bool), String> {
    let tokens = pattern_pointer(bucket, index);
    let value = pointer_value(settings, &tokens)
        .map_err(|_| "The selected terminal pattern bucket/index does not exist".to_owned())?;
    let object = value
        .as_object()
        .ok_or_else(|| "The selected terminal pattern must be a JSON object".to_owned())?;
    let pattern = object
        .get("pattern")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            "The selected terminal pattern must contain a string `pattern`".to_owned()
        })?;
    let case_sensitive = object
        .get("case_sensitive")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            "The selected terminal pattern must contain a boolean `case_sensitive`".to_owned()
        })?;

    Ok((pattern, case_sensitive))
}

fn sanitized_id(id: &str) -> String {
    let mut sanitized = String::new();
    let mut previous_separator = false;

    for character in id.chars() {
        if sanitized.len() >= 48 {
            break;
        }
        if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
            sanitized.push(character);
            previous_separator = false;
        } else if !previous_separator && !sanitized.is_empty() {
            sanitized.push('-');
            previous_separator = true;
        }
    }

    while sanitized.ends_with('-') {
        sanitized.pop();
    }
    if sanitized.is_empty() {
        sanitized.push_str("pattern");
    }

    sanitized
}

pub(crate) fn generated_pattern_filename(sequence: usize, id: &str) -> String {
    format!("pattern-{sequence:03}-{}.regex", sanitized_id(id))
}

fn validate_generated_filename(filename: &str) -> Result<(), AppError> {
    let path = Path::new(filename);
    let mut components = path.components();
    if filename.is_empty()
        || !matches!(components.next(), Some(Component::Normal(_)))
        || components.next().is_some()
    {
        return Err(invalid("Generated artifact filename is unsafe"));
    }

    Ok(())
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

fn path_as_absolute(path: &Path) -> Result<PathBuf, PathInspectionError> {
    if path.is_absolute() {
        Ok(path.to_owned())
    } else {
        env::current_dir()
            .map(|directory| directory.join(path))
            .map_err(|error| {
                PathInspectionError::Io(format!(
                    "Failed to resolve the current directory:\n\n{error}"
                ))
            })
    }
}

fn normalized_absolute_path(path: &Path) -> Result<PathBuf, AppError> {
    let absolute = path_as_absolute(path).map_err(|error| invalid(error.to_string()))?;
    let mut normalized = PathBuf::new();

    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(invalid(
                    "The refresh output path must not contain parent-directory components",
                ));
            }
            Component::Normal(part) => normalized.push(part),
        }
    }

    Ok(normalized)
}

fn validate_refresh_directory(path: &Path, metadata: &fs::Metadata) -> Result<(), AppError> {
    if metadata.file_type().is_symlink() {
        return Err(invalid(format!(
            "Refresh output path `{}` must not traverse a symbolic link",
            path.display()
        )));
    }
    if !metadata.is_dir() {
        return Err(invalid(format!(
            "Refresh output path `{}` must be a directory",
            path.display()
        )));
    }

    Ok(())
}

/// Inspect the existing output prefix without following symlinks and project any missing suffix from
/// its canonical parent. The projected path is safe for graph-root separation checks.
fn inspect_refresh_output_path(path: &Path) -> Result<PathBuf, AppError> {
    let mut current = PathBuf::new();
    let mut last_existing = PathBuf::new();
    let mut missing = Vec::new();
    let mut found_missing = false;

    for component in path.components() {
        current.push(component.as_os_str());
        if found_missing {
            missing.push(component.as_os_str().to_owned());
            continue;
        }
        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                validate_refresh_directory(&current, &metadata)?;
                last_existing = current.clone();
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                found_missing = true;
                missing.push(component.as_os_str().to_owned());
            }
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to inspect refresh output path `{}`:\n\n{error}",
                    current.display()
                )));
            }
        }
    }

    let mut projected = fs::canonicalize(&last_existing).map_err(|error| {
        invalid(format!(
            "Failed to resolve refresh output path `{}`:\n\n{error}",
            last_existing.display()
        ))
    })?;
    for component in missing {
        projected.push(component);
    }

    Ok(projected)
}

fn create_refresh_output_path(path: &Path) -> Result<CreatedDirectories, AppError> {
    let mut created = CreatedDirectories::new();
    let mut current = PathBuf::new();

    for component in path.components() {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) => validate_refresh_directory(&current, &metadata)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                match fs::create_dir(&current) {
                    Ok(()) => created.record(current.clone()),
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current).map_err(|error| {
                            invalid(format!(
                                "Failed to inspect raced refresh output path `{}`:\n\n{error}",
                                current.display()
                            ))
                        })?;
                        validate_refresh_directory(&current, &metadata)?;
                    }
                    Err(error) => {
                        return Err(invalid(format!(
                            "Failed to create refresh output directory `{}`:\n\n{error}",
                            current.display()
                        )));
                    }
                }
            }
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to inspect refresh output path `{}`:\n\n{error}",
                    current.display()
                )));
            }
        }
    }

    Ok(created)
}

pub(crate) fn ensure_no_symlink_components(path: &Path) -> Result<(), PathInspectionError> {
    let absolute = path_as_absolute(path)?;
    let mut current = PathBuf::new();

    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
            Component::RootDir => current.push(component.as_os_str()),
            Component::CurDir => continue,
            Component::ParentDir => current.push(component.as_os_str()),
            Component::Normal(part) => current.push(part),
        }

        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            PathInspectionError::Io(format!(
                "Failed to inspect path component `{}`:\n\n{error}",
                current.display()
            ))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(PathInspectionError::Symlink(current));
        }
    }

    Ok(())
}

fn prepare_output_directory(output: &Path, operation: ArtifactOperation) -> Result<bool, AppError> {
    match fs::symlink_metadata(output) {
        Ok(metadata) => {
            ensure_no_symlink_components(output).map_err(|error| invalid(error.to_string()))?;
            if !metadata.is_dir() {
                return Err(invalid(format!(
                    "{} output `{}` must be a directory",
                    operation.label(),
                    output.display()
                )));
            }
            Ok(false)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let parent = output.parent().unwrap_or_else(|| Path::new("."));
            ensure_no_symlink_components(parent).map_err(|error| invalid(error.to_string()))?;
            fs::create_dir(output).map_err(|error| {
                invalid(format!(
                    "Failed to create {} output directory `{}`:\n\n{error}",
                    operation.lowercase_label(),
                    output.display()
                ))
            })?;
            ensure_no_symlink_components(output).map_err(|error| invalid(error.to_string()))?;
            Ok(true)
        }
        Err(error) => Err(invalid(format!(
            "Failed to inspect {} output `{}`:\n\n{error}",
            operation.lowercase_label(),
            output.display()
        ))),
    }
}

fn preflight_artifacts(
    output: &Path,
    filenames: &[String],
    operation: ArtifactOperation,
) -> Result<(), AppError> {
    let mut unique = HashSet::new();
    for filename in filenames {
        validate_generated_filename(filename)?;
        if !unique.insert(filename) {
            return Err(invalid("Generated artifact filenames are not unique"));
        }
        let path = output.join(filename);
        match fs::symlink_metadata(&path) {
            Ok(_) => {
                return Err(invalid(format!(
                    "{} artifact `{}` already exists. Choose an output directory without existing artifacts",
                    operation.label(),
                    path.display()
                )));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to inspect {} artifact `{}`:\n\n{error}",
                    operation.lowercase_label(),
                    path.display()
                )));
            }
        }
    }

    Ok(())
}

fn write_new_file(path: &Path, bytes: &[u8], operation: ArtifactOperation) -> Result<(), AppError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            invalid(format!(
                "Failed to create {} artifact `{}`:\n\n{error}",
                operation.lowercase_label(),
                path.display()
            ))
        })?;

    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(invalid(format!(
            "Failed to write {} artifact `{}`:\n\n{error}",
            operation.lowercase_label(),
            path.display()
        )));
    }

    Ok(())
}

pub(crate) fn commit_artifacts_with_writer<F>(
    output: &Path,
    artifacts: &[PendingArtifact],
    operation: ArtifactOperation,
    mut write_artifact: F,
) -> Result<(), String>
where
    F: FnMut(&Path, &[u8]) -> Result<(), String>,
{
    let output_created =
        prepare_output_directory(output, operation).map_err(|error| error.message().to_owned())?;
    let filenames: Vec<String> = artifacts
        .iter()
        .map(|artifact| artifact.filename.clone())
        .collect();
    if let Err(error) = preflight_artifacts(output, &filenames, operation) {
        if output_created {
            let _ = fs::remove_dir(output);
        }
        return Err(error.message().to_owned());
    }

    let mut created = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let path = output.join(&artifact.filename);
        if let Err(error) = write_artifact(&path, &artifact.bytes) {
            for created_path in created.iter().rev() {
                let _ = fs::remove_file(created_path);
            }
            if output_created {
                let _ = fs::remove_dir(output);
            }
            return Err(error);
        }
        created.push(path);
    }

    Ok(())
}

fn commit_artifacts(
    output: &Path,
    artifacts: &[PendingArtifact],
    operation: ArtifactOperation,
) -> Result<(), AppError> {
    commit_artifacts_with_writer(output, artifacts, operation, |path, bytes| {
        write_new_file(path, bytes, operation).map_err(|error| error.message().to_owned())
    })
    .map_err(invalid)
}

fn add_refresh_artifact(
    artifacts: &mut BTreeMap<String, Vec<u8>>,
    relative: String,
    bytes: Vec<u8>,
) -> Result<(), AppError> {
    permission_patterns::validate_safe_relative_path(&relative).map_err(invalid)?;
    match artifacts.get(&relative) {
        Some(existing) if *existing == bytes => Ok(()),
        Some(_) => Err(invalid(format!(
            "Refresh artifact `{relative}` is declared with conflicting contents"
        ))),
        None => {
            artifacts.insert(relative, bytes);
            Ok(())
        }
    }
}

/// Create each missing directory component beneath the validated output, recording what this refresh
/// created so a partial failure rolls back, and refuse a symlinked component inside the output.
fn create_refresh_parent(
    output: &Path,
    relative: &Path,
    created: &mut CreatedDirectories,
) -> Result<(), AppError> {
    let directories = relative.components().count().saturating_sub(1);
    let mut current = output.to_owned();

    for component in relative.components().take(directories) {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) => validate_refresh_directory(&current, &metadata)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                match fs::create_dir(&current) {
                    Ok(()) => created.record(current.clone()),
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current).map_err(|error| {
                            invalid(format!(
                                "Failed to inspect raced refresh artifact path `{}`:\n\n{error}",
                                current.display()
                            ))
                        })?;
                        validate_refresh_directory(&current, &metadata)?;
                    }
                    Err(error) => {
                        return Err(invalid(format!(
                            "Failed to create refreshed directory `{}`:\n\n{error}",
                            current.display()
                        )));
                    }
                }
            }
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to inspect refreshed directory `{}`:\n\n{error}",
                    current.display()
                )));
            }
        }
    }

    Ok(())
}

fn write_refresh_file(path: &Path, bytes: &[u8]) -> Result<CreatedRefreshArtifact, AppError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            invalid(format!(
                "Failed to create refresh artifact `{}`:\n\n{error}",
                path.display()
            ))
        })?;
    let artifact = CreatedRefreshArtifact::new(path.to_owned(), &file, bytes)?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| {
            invalid(format!(
                "Failed to write refresh artifact `{}`:\n\n{error}",
                path.display()
            ))
        })?;

    Ok(artifact)
}

/// Commit every refreshed artifact as one unit. Refresh writes a nested graph, so it preflights its
/// own destinations instead of reusing the flat generated-filename route.
pub(crate) fn commit_refresh_artifacts_with_hook<F>(
    output: &Path,
    artifacts: &BTreeMap<String, Vec<u8>>,
    mut after_write: F,
) -> Result<(), String>
where
    F: FnMut(usize, &Path) -> Result<(), String>,
{
    let mut created_directories =
        create_refresh_output_path(output).map_err(|error| error.message().to_owned())?;
    let mut resolved = Vec::with_capacity(artifacts.len());

    for (relative, bytes) in artifacts {
        let safe = permission_patterns::validate_safe_relative_path(relative)?;
        let path = output.join(&safe);
        if let Some(parent) = path.parent() {
            inspect_refresh_output_path(parent).map_err(|error| error.message().to_owned())?;
        }
        match fs::symlink_metadata(&path) {
            Ok(_) => {
                return Err(format!(
                    "Refresh artifact `{}` already exists. Choose an output directory without existing artifacts",
                    path.display()
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "Failed to inspect refresh artifact `{}`:\n\n{error}",
                    path.display()
                ));
            }
        }
        resolved.push((safe, path, bytes.as_slice()));
    }

    let mut created_files = Vec::with_capacity(resolved.len());
    for (index, (safe, path, bytes)) in resolved.iter().enumerate() {
        let outcome = create_refresh_parent(output, safe, &mut created_directories)
            .and_then(|()| write_refresh_file(path, bytes));
        let artifact = match outcome {
            Ok(artifact) => artifact,
            Err(error) => {
                drop(created_files);
                drop(created_directories);
                return Err(error.message().to_owned());
            }
        };
        created_files.push(artifact);
        if let Err(error) = after_write(index, path) {
            drop(created_files);
            drop(created_directories);
            return Err(error);
        }
    }

    for artifact in &mut created_files {
        artifact.preserve();
    }
    created_directories.preserve();

    Ok(())
}

fn commit_refresh_artifacts(
    output: &Path,
    artifacts: &BTreeMap<String, Vec<u8>>,
) -> Result<(), AppError> {
    commit_refresh_artifacts_with_hook(output, artifacts, |_, _| Ok(())).map_err(invalid)
}

pub(crate) fn serialize_pretty_json(value: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    value
        .serialize(&mut serializer)
        .map_err(|error| format!("Failed to serialize JSON:\n\n{error}"))?;
    bytes.push(b'\n');

    Ok(bytes)
}

fn serialize_state(state: &StateDocument) -> Result<Vec<u8>, AppError> {
    let value = serde_json::to_value(state)
        .map_err(|error| invalid(format!("Failed to serialize state manifest:\n\n{error}")))?;
    serialize_pretty_json(&value).map_err(invalid)
}

fn serialize_artifact_catalog(catalog: &ArtifactCatalog) -> Result<Vec<u8>, AppError> {
    let value = serde_json::to_value(catalog)
        .map_err(|error| invalid(format!("Failed to serialize artifact catalog:\n\n{error}")))?;
    serialize_pretty_json(&value).map_err(invalid)
}

fn capture(arguments: &CaptureArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let (settings_bytes, settings) = read_json_object(&arguments.settings, "settings")?;
    let selection_contents =
        read_utf8_file(&arguments.selection, "selection JSON").map_err(invalid)?;
    let selection: SelectionDocument = serde_json::from_str(&selection_contents)
        .map_err(|error| invalid_json("selection JSON", &arguments.selection, error))?;
    let scopes = validate_scopes(&settings, &selection.scopes).map_err(invalid)?;
    let mut ids = HashSet::new();
    let mut selections = HashSet::new();
    let mut captured = Vec::with_capacity(selection.patterns.len());

    for (offset, selected) in selection.patterns.iter().enumerate() {
        if selected.id.is_empty() {
            return Err(invalid("Selected pattern IDs must be nonempty"));
        }
        if !ids.insert(selected.id.as_str()) {
            return Err(invalid("Selected pattern IDs must be unique"));
        }
        if !selections.insert((selected.bucket, selected.index)) {
            return Err(invalid(
                "Selected terminal bucket/index pairs must be unique",
            ));
        }

        let pointer = pattern_pointer(selected.bucket, selected.index);
        if !pattern_is_authorized(&pointer, &scopes) {
            return Err(invalid(
                "A selected terminal pattern object lies outside every authorized scope",
            ));
        }
        let (pattern, case_sensitive) =
            terminal_pattern(&settings, selected.bucket, selected.index).map_err(invalid)?;
        let bytes = pattern.as_bytes().to_vec();
        let pattern_file = generated_pattern_filename(offset + 1, &selected.id);
        validate_generated_filename(&pattern_file)?;
        captured.push(CapturedPattern {
            state: StatePattern {
                id: selected.id.clone(),
                bucket: selected.bucket,
                source_index: selected.index,
                case_sensitive,
                sha256: sha256_hex(&bytes),
                pattern_file,
            },
            bytes,
        });
    }

    let state = StateDocument {
        baseline_file: BASELINE_FILE.to_owned(),
        baseline_sha256: sha256_hex(&settings_bytes),
        scopes: selection.scopes,
        patterns: captured
            .iter()
            .map(|pattern| pattern.state.clone())
            .collect(),
    };
    let state_bytes = serialize_state(&state)?;
    let mut artifacts = Vec::with_capacity(captured.len() + 3);
    artifacts.push(PendingArtifact {
        filename: BASELINE_FILE.to_owned(),
        bytes: settings_bytes.clone(),
    });
    artifacts.push(PendingArtifact {
        filename: CANDIDATE_FILE.to_owned(),
        bytes: settings_bytes,
    });
    artifacts.extend(captured.iter().map(|pattern| PendingArtifact {
        filename: pattern.state.pattern_file.clone(),
        bytes: pattern.bytes.clone(),
    }));
    artifacts.push(PendingArtifact {
        filename: STATE_FILE.to_owned(),
        bytes: state_bytes,
    });
    commit_artifacts(&arguments.output, &artifacts, ArtifactOperation::Capture)?;

    writeln!(
        stdout,
        "Captured {} {} in `{}`",
        captured.len(),
        if captured.len() == 1 {
            "pattern"
        } else {
            "patterns"
        },
        arguments.output.display()
    )
    .map_err(|error| {
        invalid(format!(
            "Failed to write capture result to standard output:\n\n{error}"
        ))
    })?;
    writeln!(stdout, "  baseline -> {BASELINE_FILE}").map_err(|error| {
        invalid(format!(
            "Failed to write capture result to standard output:\n\n{error}"
        ))
    })?;
    writeln!(stdout, "  candidate -> {CANDIDATE_FILE}").map_err(|error| {
        invalid(format!(
            "Failed to write capture result to standard output:\n\n{error}"
        ))
    })?;
    for pattern in captured.iter().take(MAX_REPORTED_ITEMS) {
        writeln!(
            stdout,
            "  {} -> {}",
            display_id(&pattern.state.id),
            pattern.state.pattern_file
        )
        .map_err(|error| {
            invalid(format!(
                "Failed to write capture result to standard output:\n\n{error}"
            ))
        })?;
    }
    let omitted = captured.len().saturating_sub(MAX_REPORTED_ITEMS);
    if omitted > 0 {
        writeln!(stdout, "  … {omitted} additional pattern artifacts omitted").map_err(
            |error| {
                invalid(format!(
                    "Failed to write capture result to standard output:\n\n{error}"
                ))
            },
        )?;
    }
    writeln!(stdout, "  state -> {STATE_FILE}").map_err(|error| {
        invalid(format!(
            "Failed to write capture result to standard output:\n\n{error}"
        ))
    })?;

    Ok(())
}

fn materialize(arguments: &MaterializeArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let state = validate_state(&arguments.state)?;
    let (candidate_bytes, candidate) =
        read_utf8_json_object_with_bytes(&arguments.candidate, "candidate settings")?;
    authorize_candidate(&candidate, &state, "Materialization")?;

    let selection_contents =
        read_utf8_file(&arguments.selection, "materialization selection JSON").map_err(invalid)?;
    let selection: MaterializationSelectionDocument = serde_json::from_str(&selection_contents)
        .map_err(|error| {
            invalid_json(
                "materialization selection JSON",
                &arguments.selection,
                error,
            )
        })?;
    let mut ids = HashSet::new();
    let mut selections = HashSet::new();
    let mut materialized = Vec::with_capacity(selection.patterns.len());
    for (offset, selected) in selection.patterns.iter().enumerate() {
        if selected.id.is_empty() {
            return Err(invalid(
                "Materialization selection pattern IDs must be nonempty",
            ));
        }
        if !ids.insert(selected.id.as_str()) {
            return Err(invalid(
                "Materialization selection pattern IDs must be unique",
            ));
        }
        if !selections.insert((selected.bucket, selected.index)) {
            return Err(invalid(
                "Materialization selection terminal bucket/index pairs must be unique",
            ));
        }

        let pointer = pattern_pointer(selected.bucket, selected.index);
        if !pattern_is_authorized(&pointer, &state.scopes) {
            return Err(invalid(
                "A materialization selection terminal pattern object lies outside every authorized scope",
            ));
        }
        let (pattern, case_sensitive) =
            terminal_pattern(&candidate, selected.bucket, selected.index).map_err(invalid)?;
        let bytes = pattern.as_bytes().to_vec();
        let pattern_file = generated_pattern_filename(offset + 1, &selected.id);
        validate_generated_filename(&pattern_file)?;
        materialized.push(MaterializedPattern {
            catalog: ArtifactCatalogPattern {
                id: selected.id.clone(),
                bucket: selected.bucket,
                source_index: selected.index,
                case_sensitive,
                sha256: sha256_hex(&bytes),
                pattern_file,
            },
            bytes,
        });
    }

    let catalog = ArtifactCatalog {
        candidate_sha256: sha256_hex(&candidate_bytes),
        state_sha256: sha256_hex(&state.bytes),
        patterns: materialized
            .iter()
            .map(|pattern| pattern.catalog.clone())
            .collect(),
    };
    validate_artifact_catalog(&catalog).map_err(invalid)?;
    let catalog_bytes = serialize_artifact_catalog(&catalog)?;
    let mut artifacts = Vec::with_capacity(materialized.len() + 1);
    artifacts.extend(materialized.iter().map(|pattern| PendingArtifact {
        filename: pattern.catalog.pattern_file.clone(),
        bytes: pattern.bytes.clone(),
    }));
    artifacts.push(PendingArtifact {
        filename: ARTIFACT_CATALOG_FILE.to_owned(),
        bytes: catalog_bytes,
    });
    commit_artifacts(
        &arguments.output,
        &artifacts,
        ArtifactOperation::Materialization,
    )?;

    writeln!(
        stdout,
        "Materialized {} {} in `{}`",
        materialized.len(),
        if materialized.len() == 1 {
            "pattern"
        } else {
            "patterns"
        },
        arguments.output.display()
    )
    .map_err(|error| {
        invalid(format!(
            "Failed to write materialization result to standard output:\n\n{error}"
        ))
    })?;
    for pattern in materialized.iter().take(MAX_REPORTED_MATERIALIZED_ITEMS) {
        writeln!(
            stdout,
            "  {} -> {}",
            display_id(&pattern.catalog.id),
            pattern.catalog.pattern_file
        )
        .map_err(|error| {
            invalid(format!(
                "Failed to write materialization result to standard output:\n\n{error}"
            ))
        })?;
    }
    let omitted = materialized
        .len()
        .saturating_sub(MAX_REPORTED_MATERIALIZED_ITEMS);
    if omitted > 0 {
        writeln!(stdout, "  … {omitted} additional pattern artifacts omitted").map_err(
            |error| {
                invalid(format!(
                    "Failed to write materialization result to standard output:\n\n{error}"
                ))
            },
        )?;
    }
    writeln!(stdout, "  catalog -> {ARTIFACT_CATALOG_FILE}").map_err(|error| {
        invalid(format!(
            "Failed to write materialization result to standard output:\n\n{error}"
        ))
    })?;

    Ok(())
}

fn validate_relative_artifact_path(path: &str) -> Result<PathBuf, AppError> {
    let path = PathBuf::from(path);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(invalid(
            "State artifact paths must be nonempty and relative",
        ));
    }
    if !path
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(invalid(
            "State artifact paths must not contain root, parent, or current-directory components",
        ));
    }

    Ok(path)
}

fn read_state_artifact(
    state_path: &Path,
    relative_path: &str,
    description: &str,
) -> Result<Vec<u8>, AppError> {
    let relative_path = validate_relative_artifact_path(relative_path)?;
    let base = state_path.parent().unwrap_or_else(|| Path::new("."));
    let mut current = base.to_owned();
    let component_count = relative_path.components().count();

    for (index, component) in relative_path.components().enumerate() {
        let Component::Normal(component) = component else {
            return Err(invalid("State artifact path is unsafe"));
        };
        current.push(component);
        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            invalid(format!(
                "Failed to inspect {description} artifact `{}`:\n\n{error}",
                current.display()
            ))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(invalid(format!(
                "The {description} artifact `{}` must not be a symbolic link",
                current.display()
            )));
        }
        if index + 1 < component_count && !metadata.is_dir() {
            return Err(invalid(format!(
                "A {description} artifact path component `{}` is not a directory",
                current.display()
            )));
        }
        if index + 1 == component_count && !metadata.is_file() {
            return Err(invalid(format!(
                "The {description} artifact `{}` must be a regular file",
                current.display()
            )));
        }
    }

    read_bytes(&current, &format!("{description} artifact"))
}

fn read_state_document(path: &Path) -> Result<(Vec<u8>, StateDocument), AppError> {
    ensure_no_symlink_components(path).map_err(|error| invalid(error.to_string()))?;
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        invalid(format!(
            "Failed to inspect state manifest `{}`:\n\n{error}",
            path.display()
        ))
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(invalid(format!(
            "State manifest `{}` must be a regular file and not a symbolic link",
            path.display()
        )));
    }
    let bytes = read_bytes(path, "state manifest")?;
    let (bytes, contents) = decode_utf8(bytes, path, "state manifest")?;
    let document: StateDocument = serde_json::from_str(&contents)
        .map_err(|error| invalid_json("state manifest JSON", path, error))?;

    Ok((bytes, document))
}

fn validate_state(path: &Path) -> Result<ValidatedState, AppError> {
    let (state_bytes, document) = read_state_document(path)?;
    if !is_valid_sha256(&document.baseline_sha256) {
        return Err(invalid(
            "State manifest baseline SHA-256 must be 64 lowercase hexadecimal characters",
        ));
    }

    let baseline_relative = validate_relative_artifact_path(&document.baseline_file)?;
    let baseline_bytes = read_state_artifact(path, &document.baseline_file, "baseline")?;
    if sha256_hex(&baseline_bytes) != document.baseline_sha256 {
        return Err(invalid(
            "Baseline artifact SHA-256 does not match the state manifest",
        ));
    }
    let baseline = parse_json_object(&baseline_bytes, "baseline artifact", &baseline_relative)?;
    let scopes = validate_scopes(&baseline, &document.scopes).map_err(invalid)?;

    let mut ids = HashSet::new();
    let mut selections = HashSet::new();
    let mut artifact_paths = HashSet::new();
    artifact_paths.insert(baseline_relative);
    let mut patterns = Vec::with_capacity(document.patterns.len());

    for state_pattern in &document.patterns {
        if state_pattern.id.is_empty() {
            return Err(invalid("State pattern IDs must be nonempty"));
        }
        if !ids.insert(state_pattern.id.as_str()) {
            return Err(invalid("State pattern IDs must be unique"));
        }
        if !selections.insert((state_pattern.bucket, state_pattern.source_index)) {
            return Err(invalid(
                "State terminal bucket/source-index pairs must be unique",
            ));
        }
        if !is_valid_sha256(&state_pattern.sha256) {
            return Err(invalid(
                "State pattern SHA-256 values must be 64 lowercase hexadecimal characters",
            ));
        }
        let relative = validate_relative_artifact_path(&state_pattern.pattern_file)?;
        if !artifact_paths.insert(relative) {
            return Err(invalid("State artifact paths must be unique"));
        }

        let pointer = pattern_pointer(state_pattern.bucket, state_pattern.source_index);
        if !pattern_is_authorized(&pointer, &scopes) {
            return Err(invalid(
                "A recorded baseline pattern object lies outside every authorized scope",
            ));
        }
        let bytes = read_state_artifact(path, &state_pattern.pattern_file, "pattern")?;
        if sha256_hex(&bytes) != state_pattern.sha256 {
            return Err(invalid(format!(
                "Pattern artifact SHA-256 does not match the state manifest for ID `{}`",
                display_id(&state_pattern.id)
            )));
        }
        std::str::from_utf8(&bytes).map_err(|_| {
            invalid(format!(
                "Pattern artifact for ID `{}` is not valid UTF-8",
                display_id(&state_pattern.id)
            ))
        })?;
        let (baseline_pattern, baseline_case_sensitive) =
            terminal_pattern(&baseline, state_pattern.bucket, state_pattern.source_index).map_err(
                |_| {
                    invalid(format!(
                        "Recorded baseline source is missing or invalid for ID `{}`",
                        display_id(&state_pattern.id)
                    ))
                },
            )?;
        if baseline_pattern.as_bytes() != bytes
            || baseline_case_sensitive != state_pattern.case_sensitive
        {
            return Err(invalid(format!(
                "Recorded baseline source identity does not match artifacts for ID `{}`",
                display_id(&state_pattern.id)
            )));
        }

        patterns.push(LoadedPattern {
            id: state_pattern.id.clone(),
            bucket: state_pattern.bucket,
            source_index: state_pattern.source_index,
            case_sensitive: state_pattern.case_sensitive,
            bytes,
        });
    }

    Ok(ValidatedState {
        baseline,
        bytes: state_bytes,
        document,
        patterns,
        scopes,
    })
}

fn current_bucket(settings: &Value, bucket: Bucket) -> Option<&Vec<Value>> {
    let tokens = vec![
        "agent".to_owned(),
        "tool_permissions".to_owned(),
        "tools".to_owned(),
        "terminal".to_owned(),
        bucket.label().to_owned(),
    ];
    pointer_value(settings, &tokens).ok()?.as_array()
}

fn required_terminal_pattern_array<'a>(
    settings: &'a Value,
    bucket: Bucket,
    settings_label: &str,
) -> Result<&'a Vec<Value>, AppError> {
    current_bucket(settings, bucket).ok_or_else(|| {
        invalid(format!(
            "The {settings_label} terminal permission bucket `{}` must be an array",
            bucket.label()
        ))
    })
}

type ExactPatternLookup<'settings> = HashMap<(Bucket, &'settings [u8], bool), Vec<usize>>;

pub(crate) struct CurrentPatternIndex<'settings> {
    indexes: ExactPatternLookup<'settings>,
}

impl<'settings> CurrentPatternIndex<'settings> {
    pub(crate) fn indexes(
        &self,
        bucket: Bucket,
        bytes: &'settings [u8],
        case_sensitive: bool,
    ) -> &[usize] {
        self.indexes
            .get(&(bucket, bytes, case_sensitive))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

pub(crate) fn index_current_terminal_patterns<'settings, I, F>(
    buckets: I,
    mut bucket_values: F,
) -> CurrentPatternIndex<'settings>
where
    I: IntoIterator<Item = Bucket>,
    F: FnMut(Bucket) -> Option<&'settings [Value]>,
{
    let mut indexes = ExactPatternLookup::new();
    let relevant_buckets: HashSet<Bucket> = buckets.into_iter().collect();

    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        if !relevant_buckets.contains(&bucket) {
            continue;
        }
        let Some(values) = bucket_values(bucket) else {
            continue;
        };

        for (index, value) in values.iter().enumerate() {
            let Some(object) = value.as_object() else {
                continue;
            };
            let Some(candidate) = object.get("pattern").and_then(Value::as_str) else {
                continue;
            };
            let Some(case_sensitive) = object.get("case_sensitive").and_then(Value::as_bool) else {
                continue;
            };
            indexes
                .entry((bucket, candidate.as_bytes(), case_sensitive))
                .or_default()
                .push(index);
        }
    }

    CurrentPatternIndex { indexes }
}

#[derive(Clone, Copy)]
enum ReindexFailureKind {
    Missing,
    Duplicate,
}

impl ReindexFailureKind {
    fn label(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Duplicate => "duplicate",
        }
    }
}

struct ReindexFailure<'a> {
    pattern: &'a LoadedPattern,
    kind: ReindexFailureKind,
}

struct MovedMapping<'a> {
    pattern: &'a LoadedPattern,
    index: usize,
}

fn verify(arguments: &VerifyArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let state = validate_state(&arguments.state)?;
    let settings = read_utf8_json_object(&arguments.settings, "current settings")?;
    if state.patterns.is_empty() {
        for (index, tokens) in state.scopes.iter().enumerate() {
            let baseline_value = pointer_value(&state.baseline, tokens)
                .map_err(|_| invalid("Validated baseline scope became unavailable"))?;
            let current_value = pointer_value(&settings, tokens).map_err(|_| {
                refused(format!(
                    "Verification refused because current settings no longer contain authorized scope {}",
                    index + 1
                ))
            })?;
            if !semantic_json_equal(current_value, baseline_value) {
                return Err(refused(format!(
                    "Verification refused because current authorized scope {} drifted from the captured baseline",
                    index + 1
                )));
            }
        }
        writeln!(
            stdout,
            "Verified {} authorized {} against the captured baseline",
            state.scopes.len(),
            if state.scopes.len() == 1 {
                "scope"
            } else {
                "scopes"
            }
        )
        .map_err(|error| {
            invalid(format!(
                "Failed to write verification result to standard output:\n\n{error}"
            ))
        })?;
        return Ok(());
    }

    let index = index_current_terminal_patterns(
        state.patterns.iter().map(|pattern| pattern.bucket),
        |bucket| current_bucket(&settings, bucket).map(Vec::as_slice),
    );
    let mut failures = BoundedIssues::new(MAX_REPORTED_VERIFY_ITEMS);
    let mut moved = BoundedIssues::new(MAX_REPORTED_VERIFY_ITEMS);
    let mut missing = 0;
    let mut duplicate = 0;
    let mut unchanged = 0;

    for pattern in &state.patterns {
        let indexes = index.indexes(pattern.bucket, &pattern.bytes, pattern.case_sensitive);
        match indexes {
            [] => {
                missing += 1;
                failures.push(ReindexFailure {
                    pattern,
                    kind: ReindexFailureKind::Missing,
                });
            }
            [current_index] if *current_index == pattern.source_index => unchanged += 1,
            [current_index] => moved.push(MovedMapping {
                pattern,
                index: *current_index,
            }),
            _ => {
                duplicate += 1;
                failures.push(ReindexFailure {
                    pattern,
                    kind: ReindexFailureKind::Duplicate,
                });
            }
        }
    }

    if failures.total_count() > 0 {
        let mut message = format!(
            "Failed to uniquely reindex {} {} in current settings. Missing: {missing}. Duplicate matches: {duplicate}",
            state.patterns.len(),
            if state.patterns.len() == 1 {
                "pattern"
            } else {
                "patterns"
            }
        );
        for failure in failures.issues() {
            message.push_str(&format!(
                "\n  {} -> {}[{}]",
                display_id(&failure.pattern.id),
                failure.pattern.bucket.label(),
                failure.kind.label()
            ));
        }
        let omitted = failures.omitted_count();
        if omitted > 0 {
            message.push_str(&format!(
                "\n  … {omitted} additional reindex failures omitted"
            ));
        }
        return Err(refused(message));
    }

    for mapping in moved.issues() {
        writeln!(
            stdout,
            "{} -> {}[{}]",
            display_id(&mapping.pattern.id),
            mapping.pattern.bucket.label(),
            mapping.index
        )
        .map_err(|error| {
            invalid(format!(
                "Failed to write verification result to standard output:\n\n{error}"
            ))
        })?;
    }
    let omitted = moved.omitted_count();
    if omitted > 0 {
        writeln!(stdout, "… {omitted} additional moved mappings omitted").map_err(|error| {
            invalid(format!(
                "Failed to write verification result to standard output:\n\n{error}"
            ))
        })?;
    }
    writeln!(
        stdout,
        "Verified {} {}: {unchanged} unchanged and {} moved",
        state.patterns.len(),
        if state.patterns.len() == 1 {
            "pattern"
        } else {
            "patterns"
        },
        moved.total_count()
    )
    .map_err(|error| {
        invalid(format!(
            "Failed to write verification result to standard output:\n\n{error}"
        ))
    })?;

    Ok(())
}

pub(crate) fn semantic_json_equal(first: &Value, second: &Value) -> bool {
    match (first, second) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(first), Value::Bool(second)) => first == second,
        (Value::Number(first), Value::Number(second)) => first == second,
        (Value::String(first), Value::String(second)) => first == second,
        (Value::Array(first), Value::Array(second)) => {
            first.len() == second.len()
                && first
                    .iter()
                    .zip(second)
                    .all(|(first, second)| semantic_json_equal(first, second))
        }
        (Value::Object(first), Value::Object(second)) => {
            first.len() == second.len()
                && first.iter().all(|(key, first_value)| {
                    second
                        .get(key)
                        .is_some_and(|second_value| semantic_json_equal(first_value, second_value))
                })
        }
        _ => false,
    }
}

fn validate_catalog_candidate_sources(
    catalog: &LoadedArtifactCatalog,
    candidate: &Value,
    scopes: &[Vec<String>],
) -> Result<(), AppError> {
    for (index, pattern) in catalog.patterns.iter().enumerate() {
        let definition = &pattern.definition;
        let pointer = pattern_pointer(definition.bucket, definition.source_index);
        if !pattern_is_authorized(&pointer, scopes) {
            return Err(invalid(format!(
                "Artifact catalog pattern {} lies outside every authorized scope",
                index + 1
            )));
        }
        let (candidate_pattern, candidate_case_sensitive) =
            terminal_pattern(candidate, definition.bucket, definition.source_index).map_err(
                |_| {
                    invalid(format!(
                        "Artifact catalog candidate source is missing or invalid for pattern {}",
                        index + 1
                    ))
                },
            )?;
        if candidate_pattern.as_bytes() != pattern.pattern.as_bytes()
            || candidate_case_sensitive != definition.case_sensitive
        {
            return Err(invalid(format!(
                "Artifact catalog candidate source identity does not match pattern {}",
                index + 1
            )));
        }
    }

    Ok(())
}

fn remove_terminal_sources(
    settings: &mut Value,
    sources: &HashMap<Bucket, HashSet<usize>>,
) -> Result<(), AppError> {
    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let Some(indexes) = sources.get(&bucket) else {
            continue;
        };
        if indexes.is_empty() {
            continue;
        }
        let tokens = vec![
            "agent".to_owned(),
            "tool_permissions".to_owned(),
            "tools".to_owned(),
            "terminal".to_owned(),
            bucket.label().to_owned(),
        ];
        let values = pointer_value_mut(settings, &tokens)
            .map_err(|_| invalid("A validated terminal permission bucket became unavailable"))?
            .as_array_mut()
            .ok_or_else(|| invalid("A validated terminal permission bucket is not an array"))?;
        let original = std::mem::take(values);
        *values = original
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| (!indexes.contains(&index)).then_some(value))
            .collect();
    }

    Ok(())
}

/// Resolve declared owner membership against the captured state and materialized catalog. Every
/// state pattern and every catalog entry must be claimed exactly once, so no member can be reused
/// and none can be silently omitted.
struct OwnerGraph<'a> {
    spec: &'a OwnerSpec,
    state_by_id: BTreeMap<&'a str, &'a LoadedPattern>,
    catalog_by_id: BTreeMap<&'a str, &'a ArtifactCatalogPattern>,
    candidate_member_ids: BTreeSet<&'a str>,
}

fn resolve_owner_graph<'a>(
    state: &'a ValidatedState,
    catalog: &'a LoadedArtifactCatalog,
    spec: &'a OwnerSpec,
) -> Result<OwnerGraph<'a>, AppError> {
    validate_owner_spec(spec).map_err(invalid)?;

    let state_by_id = state
        .patterns
        .iter()
        .map(|pattern| (pattern.id.as_str(), pattern))
        .collect::<BTreeMap<_, _>>();
    let catalog_by_id = catalog
        .document
        .patterns
        .iter()
        .map(|pattern| (pattern.id.as_str(), pattern))
        .collect::<BTreeMap<_, _>>();

    let mut claimed_baseline = BTreeSet::new();
    let mut candidate_member_ids = BTreeSet::new();
    for owner in &spec.owners {
        for member in &owner.baseline_members {
            if !state_by_id.contains_key(member.as_str()) {
                return Err(invalid(format!(
                    "Owner operation `{}` claims baseline member `{}`, which the captured state does not contain",
                    display_id(&owner.id),
                    display_id(member)
                )));
            }
            claimed_baseline.insert(member.as_str());
        }
        for member in &owner.candidate_members {
            if !catalog_by_id.contains_key(member.as_str()) {
                return Err(invalid(format!(
                    "Owner operation `{}` claims candidate member `{}`, which the catalog does not contain",
                    display_id(&owner.id),
                    display_id(member)
                )));
            }
            candidate_member_ids.insert(member.as_str());
        }
    }

    for pattern in &state.patterns {
        if !claimed_baseline.contains(pattern.id.as_str()) {
            return Err(invalid(format!(
                "Captured baseline member `{}` is not claimed by any owner operation",
                display_id(&pattern.id)
            )));
        }
    }

    let mut claimed_candidate = candidate_member_ids.clone();
    for overlap in &spec.overlaps {
        if !catalog_by_id.contains_key(overlap.as_str()) {
            return Err(invalid(format!(
                "Declared overlap `{}` is not present in the catalog",
                display_id(overlap)
            )));
        }
        claimed_candidate.insert(overlap.as_str());
    }
    for pattern in &catalog.document.patterns {
        if !claimed_candidate.contains(pattern.id.as_str()) {
            return Err(invalid(format!(
                "Catalog entry `{}` is neither an owner candidate member nor a declared overlap",
                display_id(&pattern.id)
            )));
        }
    }

    Ok(OwnerGraph {
        spec,
        state_by_id,
        catalog_by_id,
        candidate_member_ids,
    })
}

fn bucket_positions<'a>(
    ids: impl Iterator<Item = &'a str>,
    lookup: impl Fn(&str) -> Option<(Bucket, usize)>,
) -> HashMap<Bucket, HashSet<usize>> {
    let mut positions: HashMap<Bucket, HashSet<usize>> = HashMap::new();
    for id in ids {
        if let Some((bucket, index)) = lookup(id) {
            positions.entry(bucket).or_default().insert(index);
        }
    }

    positions
}

/// Authorize the complete owner transformation. Per-owner accounting is independent, and ordered
/// remainder equality plus per-bucket count reconciliation prevent one owner’s undeclared change
/// from cancelling another’s.
fn authorize_owner_operations(
    candidate: &Value,
    state: &ValidatedState,
    graph: &OwnerGraph<'_>,
) -> Result<(), AppError> {
    let baseline_positions = bucket_positions(graph.state_by_id.keys().copied(), |id| {
        graph
            .state_by_id
            .get(id)
            .map(|pattern| (pattern.bucket, pattern.source_index))
    });
    let candidate_positions = bucket_positions(graph.candidate_member_ids.iter().copied(), |id| {
        graph
            .catalog_by_id
            .get(id)
            .map(|pattern| (pattern.bucket, pattern.source_index))
    });

    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let baseline =
            required_terminal_pattern_array(&state.baseline, bucket, "captured baseline")?;
        let current = required_terminal_pattern_array(candidate, bucket, "candidate")?;
        let removed = baseline_positions.get(&bucket).map_or(0, HashSet::len);
        let added = candidate_positions.get(&bucket).map_or(0, HashSet::len);
        let expected = baseline
            .len()
            .checked_sub(removed)
            .and_then(|length| length.checked_add(added))
            .ok_or_else(|| invalid("Owner membership exceeds the captured bucket length"))?;
        if current.len() != expected {
            return Err(refused(format!(
                "Promotion refused because candidate bucket `{}` holds {} entries where declared owner membership reconciles to {expected}",
                bucket.label(),
                current.len()
            )));
        }
    }

    let mut baseline_remainder = state.baseline.clone();
    remove_terminal_sources(&mut baseline_remainder, &baseline_positions)?;
    let mut candidate_remainder = candidate.clone();
    remove_terminal_sources(&mut candidate_remainder, &candidate_positions)?;

    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let baseline =
            required_terminal_pattern_array(&baseline_remainder, bucket, "captured baseline")?;
        let current =
            required_terminal_pattern_array(&candidate_remainder, bucket, "candidate remainder")?;
        if baseline.len() != current.len()
            || !baseline
                .iter()
                .zip(current)
                .all(|(baseline, current)| semantic_json_equal(baseline, current))
        {
            return Err(refused(format!(
                "Promotion refused because the outside-owner remainder for `{}` differs from the captured baseline",
                bucket.label()
            )));
        }
    }

    // Values outside the terminal arrays are governed by authorized-scope equality, so comparing the
    // complete objects here would wrongly refuse a legitimate scope-only change.
    Ok(())
}

/// Verify every supplemental classification. Ordinary lexical inventory cannot supply this proof,
/// so the shared wrapper-aware inference runs directly on each declared witness.
fn verify_supplemental_ownership(
    candidate: &Value,
    state: &ValidatedState,
    graph: &OwnerGraph<'_>,
    validation_ids: &BTreeSet<String>,
) -> Result<(), AppError> {
    let owner_of_baseline = graph
        .spec
        .owners
        .iter()
        .flat_map(|owner| {
            owner
                .baseline_members
                .iter()
                .map(move |id| (id.as_str(), owner))
        })
        .collect::<BTreeMap<_, _>>();
    let owner_of_candidate = graph
        .spec
        .owners
        .iter()
        .flat_map(|owner| {
            owner
                .candidate_members
                .iter()
                .map(move |id| (id.as_str(), owner))
        })
        .collect::<BTreeMap<_, _>>();

    let mut supplemental_keys = BTreeSet::new();
    for record in &graph.spec.supplemental {
        supplemental_keys.insert((record.side, record.member_id.as_str()));
    }

    for record in &graph.spec.supplemental {
        let (owner, position, pattern_text, case_sensitive) = match record.side {
            SupplementalSide::Baseline => {
                let owner = owner_of_baseline.get(record.member_id.as_str()).ok_or_else(|| {
                    invalid(format!(
                        "Supplemental baseline member `{}` is not claimed by any owner operation",
                        display_id(&record.member_id)
                    ))
                })?;
                let pattern = graph.state_by_id[record.member_id.as_str()];
                let text = std::str::from_utf8(&pattern.bytes)
                    .map_err(|_| invalid("A captured pattern artifact is not valid UTF-8"))?;
                (
                    *owner,
                    TerminalPosition {
                        bucket: pattern.bucket,
                        index: pattern.source_index,
                    },
                    text.to_owned(),
                    pattern.case_sensitive,
                )
            }
            SupplementalSide::Candidate => {
                let owner = owner_of_candidate.get(record.member_id.as_str()).ok_or_else(|| {
                    invalid(format!(
                        "Supplemental candidate member `{}` is not claimed by any owner operation",
                        display_id(&record.member_id)
                    ))
                })?;
                let definition = graph.catalog_by_id[record.member_id.as_str()];
                let entry = snapshot_pattern_at(
                    candidate,
                    TerminalPosition {
                        bucket: definition.bucket,
                        index: definition.source_index,
                    },
                )
                .map_err(invalid)?;
                (
                    *owner,
                    TerminalPosition {
                        bucket: definition.bucket,
                        index: definition.source_index,
                    },
                    entry.pattern,
                    entry.case_sensitive,
                )
            }
        };

        let matcher = owner_source_matcher(&owner.inventory_owner).map_err(invalid)?;
        if matcher.captures(&pattern_text).is_some() {
            return Err(invalid(format!(
                "Supplemental member `{}` at `{}` is lexically visible for `{}` and must use ordinary inventory",
                display_id(&record.member_id),
                position.label(),
                owner.inventory_owner
            )));
        }

        let regex = compile_pattern(&pattern_text, case_sensitive).map_err(|error| {
            let detail = match error {
                PatternError::Empty => "the pattern is empty".to_owned(),
                PatternError::Invalid(error) => regex_error_summary(&error),
            };
            invalid(format!(
                "Failed to compile supplemental member `{}`. {detail}",
                display_id(&record.member_id)
            ))
        })?;

        let mut satisfied = false;
        for evidence in &record.classification_evidence {
            match evidence.kind {
                permission_patterns::EvidenceKind::ValidationEntry => {
                    if !validation_ids.contains(&evidence.value) {
                        return Err(invalid(format!(
                            "Supplemental member `{}` references validation entry `{}`, which the bundle does not bind",
                            display_id(&record.member_id),
                            display_id(&evidence.value)
                        )));
                    }
                }
                permission_patterns::EvidenceKind::NormalizedWitness => {
                    if !regex.is_match(&evidence.value) {
                        return Err(invalid(format!(
                            "Supplemental member `{}` declares a witness its own pattern does not match",
                            display_id(&record.member_id)
                        )));
                    }
                    let inferred = infer_witness_owner(&evidence.value).map_err(|error| {
                        invalid(format!(
                            "Supplemental member `{}` declares an unsupported or ambiguous witness. {error}",
                            display_id(&record.member_id)
                        ))
                    })?;
                    if inferred.owner == record.declared_owner
                        && inferred.inventory_owner == owner.inventory_owner
                        && inferred.repository_scope == record.repository_scope
                    {
                        satisfied = true;
                    }
                }
            }
        }
        if !satisfied {
            return Err(invalid(format!(
                "No supplemental witness for `{}` independently infers owner `{}`, inventory owner `{}`, and repository scope `{}`",
                display_id(&record.member_id),
                display_id(&record.declared_owner),
                owner.inventory_owner,
                record.repository_scope.label()
            )));
        }
    }

    // Every remaining owner member must be an ordinary lexical inventory hit on its own side.
    for owner in &graph.spec.owners {
        let matcher = owner_source_matcher(&owner.inventory_owner).map_err(invalid)?;
        for member in &owner.baseline_members {
            if supplemental_keys.contains(&(SupplementalSide::Baseline, member.as_str())) {
                continue;
            }
            let pattern = graph.state_by_id[member.as_str()];
            let text = std::str::from_utf8(&pattern.bytes)
                .map_err(|_| invalid("A captured pattern artifact is not valid UTF-8"))?;
            if matcher.captures(text).is_none() {
                return Err(invalid(format!(
                    "Baseline member `{}` is lexically invisible for `{}` and requires a supplemental declaration",
                    display_id(member),
                    owner.inventory_owner
                )));
            }
        }
        for member in &owner.candidate_members {
            if supplemental_keys.contains(&(SupplementalSide::Candidate, member.as_str())) {
                continue;
            }
            let definition = graph.catalog_by_id[member.as_str()];
            let entry = snapshot_pattern_at(
                candidate,
                TerminalPosition {
                    bucket: definition.bucket,
                    index: definition.source_index,
                },
            )
            .map_err(invalid)?;
            if matcher.captures(&entry.pattern).is_none() {
                return Err(invalid(format!(
                    "Candidate member `{}` is lexically invisible for `{}` and requires a supplemental declaration",
                    display_id(member),
                    owner.inventory_owner
                )));
            }
        }
    }

    let _ = state;
    Ok(())
}

/// Verify every optional candidate-only visibility rewrite. This proves only the supported
/// syntactic transformation invariant, never general regex-language equivalence.
fn verify_visibility_rewrites(candidate: &Value, graph: &OwnerGraph<'_>) -> Result<(), AppError> {
    for rewrite in &graph.spec.visibility_rewrites {
        let owner = graph
            .spec
            .owners
            .iter()
            .find(|owner| owner.baseline_members.contains(&rewrite.baseline_member_id))
            .ok_or_else(|| {
                invalid(format!(
                    "Visibility rewrite baseline member `{}` is not claimed by any owner operation",
                    display_id(&rewrite.baseline_member_id)
                ))
            })?;
        if !owner
            .candidate_members
            .contains(&rewrite.candidate_member_id)
        {
            return Err(invalid(format!(
                "Visibility rewrite members `{}` and `{}` belong to different owner operations",
                display_id(&rewrite.baseline_member_id),
                display_id(&rewrite.candidate_member_id)
            )));
        }
        if owner.operation != OwnerOperationKind::Replace {
            return Err(invalid(format!(
                "Owner operation `{}` uses a visibility rewrite, which requires a `replace` shape",
                display_id(&owner.id)
            )));
        }

        let supplemental = graph
            .spec
            .supplemental
            .iter()
            .find(|record| {
                record.side == SupplementalSide::Baseline
                    && record.member_id == rewrite.baseline_member_id
            })
            .ok_or_else(|| {
                invalid(format!(
                    "Visibility rewrite baseline member `{}` must also be declared supplemental",
                    display_id(&rewrite.baseline_member_id)
                ))
            })?;
        if supplemental.declared_owner != rewrite.recovered_owner {
            return Err(invalid(format!(
                "Visibility rewrite for `{}` recovers owner `{}` while its supplemental record declares `{}`",
                display_id(&rewrite.baseline_member_id),
                display_id(&rewrite.recovered_owner),
                display_id(&supplemental.declared_owner)
            )));
        }

        let baseline_pattern = graph.state_by_id[rewrite.baseline_member_id.as_str()];
        let baseline_text = std::str::from_utf8(&baseline_pattern.bytes)
            .map_err(|_| invalid("A captured pattern artifact is not valid UTF-8"))?;
        let definition = graph.catalog_by_id[rewrite.candidate_member_id.as_str()];
        let candidate_entry = snapshot_pattern_at(
            candidate,
            TerminalPosition {
                bucket: definition.bucket,
                index: definition.source_index,
            },
        )
        .map_err(invalid)?;
        if candidate_entry.case_sensitive != baseline_pattern.case_sensitive {
            return Err(invalid(format!(
                "Visibility rewrite for `{}` changes the case setting, so it is not behavior-preserving",
                display_id(&rewrite.baseline_member_id)
            )));
        }

        let matcher = owner_source_matcher(&rewrite.recovered_owner).map_err(invalid)?;
        if matcher.captures(baseline_text).is_some() {
            return Err(invalid(format!(
                "Visibility rewrite baseline member `{}` is already lexically visible",
                display_id(&rewrite.baseline_member_id)
            )));
        }
        if matcher.captures(&candidate_entry.pattern).is_none() {
            return Err(invalid(format!(
                "Visibility rewrite for `{}` does not make owner `{}` lexically visible",
                display_id(&rewrite.candidate_member_id),
                display_id(&rewrite.recovered_owner)
            )));
        }

        verify_visibility_transformation(
            &rewrite.transformation,
            baseline_text,
            &candidate_entry.pattern,
        )
        .map_err(|error| {
            invalid(format!(
                "Visibility rewrite for `{}` is refused. {error}",
                display_id(&rewrite.baseline_member_id)
            ))
        })?;
    }

    Ok(())
}

fn authorize_candidate(
    candidate: &Value,
    state: &ValidatedState,
    operation: &str,
) -> Result<(), AppError> {
    let mut normalized_candidate = candidate.clone();
    for (index, tokens) in state.scopes.iter().enumerate() {
        let baseline_value = pointer_value(&state.baseline, tokens)
            .map_err(|_| invalid("Validated baseline scope became unavailable"))?
            .clone();
        replace_pointer_value(&mut normalized_candidate, tokens, baseline_value).map_err(|_| {
            refused(format!(
                "{operation} refused because candidate settings do not contain authorized scope {}",
                index + 1
            ))
        })?;
    }
    if !semantic_json_equal(&normalized_candidate, &state.baseline) {
        return Err(refused(format!(
            "{operation} refused because candidate settings differ from the captured baseline outside authorized scopes"
        )));
    }

    Ok(())
}

fn unique_temporary_sibling(destination: &Path) -> Result<(File, PathBuf), String> {
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    destination
        .file_name()
        .ok_or_else(|| "Live settings destination has no filename".to_owned())?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("System clock is before the Unix epoch:\n\n{error}"))?
        .as_nanos();

    for attempt in 0..100_u64 {
        let sequence = NEXT_TEMPORARY_ID.fetch_add(1, Ordering::Relaxed);
        let sibling_name = OsString::from(format!(
            ".permission-candidate-{}-{timestamp}-{sequence}-{attempt}.tmp",
            process::id()
        ));
        let path = parent.join(sibling_name);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((file, path)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "Failed to create atomic sibling `{}`:\n\n{error}",
                    path.display()
                ));
            }
        }
    }

    Err("Failed to allocate a unique atomic sibling after 100 attempts".to_owned())
}

pub(crate) fn atomic_replace_with<F>(
    destination: &Path,
    bytes: &[u8],
    before_rename: F,
) -> Result<(), String>
where
    F: FnOnce(&Path) -> io::Result<()>,
{
    let permissions = fs::metadata(destination)
        .map_err(|error| format!("Failed to read live settings permissions:\n\n{error}"))?
        .permissions();
    let (mut file, temporary_path) = unique_temporary_sibling(destination)?;
    let temporary = TemporarySibling::new(temporary_path.clone());

    file.set_permissions(permissions)
        .map_err(|error| format!("Failed to copy live settings permissions:\n\n{error}"))?;
    file.write_all(bytes)
        .map_err(|error| format!("Failed to write atomic sibling:\n\n{error}"))?;
    file.sync_all()
        .map_err(|error| format!("Failed to sync atomic sibling:\n\n{error}"))?;
    drop(file);
    before_rename(&temporary_path)
        .map_err(|error| format!("Failed to complete atomic replacement:\n\n{error}"))?;
    fs::rename(&temporary_path, destination)
        .map_err(|error| format!("Failed to atomically replace live settings:\n\n{error}"))?;
    temporary.preserve();

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum BestEffortReplaceError {
    Changed,
    Invalid(String),
}

pub(crate) fn atomic_replace_with_best_effort_recheck<F>(
    destination: &Path,
    bytes: &[u8],
    expected_bytes: &[u8],
    before_recheck: F,
) -> Result<(), BestEffortReplaceError>
where
    F: FnOnce(&Path) -> io::Result<()>,
{
    atomic_replace_with_best_effort_recheck_and_hook(
        destination,
        bytes,
        expected_bytes,
        before_recheck,
        |_| Ok(()),
    )
}

pub(crate) fn atomic_replace_with_best_effort_recheck_and_hook<F, G>(
    destination: &Path,
    bytes: &[u8],
    expected_bytes: &[u8],
    before_recheck: F,
    after_recheck: G,
) -> Result<(), BestEffortReplaceError>
where
    F: FnOnce(&Path) -> io::Result<()>,
    G: FnOnce(&Path) -> io::Result<()>,
{
    let mut changed = false;
    let result = atomic_replace_with(destination, bytes, |temporary_path| {
        before_recheck(temporary_path)?;
        // Pathname replacement cannot conditionally compare destination contents, so a writer can still race after this read
        let current_bytes = fs::read(destination)?;
        if current_bytes != expected_bytes {
            changed = true;
            return Err(io::Error::other(
                "live settings changed while promotion output was prepared",
            ));
        }
        after_recheck(temporary_path)
    });

    match result {
        Ok(()) => Ok(()),
        Err(_) if changed => Err(BestEffortReplaceError::Changed),
        Err(error) => Err(BestEffortReplaceError::Invalid(error)),
    }
}

/// The complete sealed graph, loaded and hash-verified from one bundle.
struct LoadedBundle {
    root: PathBuf,
    bytes: Vec<u8>,
    document: Bundle,
    candidate: Value,
    state: ValidatedState,
    catalog: LoadedArtifactCatalog,
    spec: OwnerSpec,
}

fn bundle_artifact_path(root: &Path, artifact: &BoundArtifact) -> Result<PathBuf, AppError> {
    permission_patterns::resolve_within_root(root, &artifact.path).map_err(invalid)
}

fn read_bound(
    root: &Path,
    artifact: &BoundArtifact,
    description: &str,
) -> Result<Vec<u8>, AppError> {
    let bytes =
        permission_patterns::read_regular_file_within_root(root, &artifact.path, description)
            .map_err(invalid)?;
    if sha256_hex(&bytes) != artifact.sha256 {
        return Err(invalid(format!(
            "The bound {description} does not match its recorded SHA-256"
        )));
    }

    Ok(bytes)
}

/// Bundle schema, artifact integrity, state and catalog validation, compilation, and owner-spec
/// structure.
fn load_bundle(path: &Path) -> Result<LoadedBundle, AppError> {
    let bundle_bytes = read_bytes(path, "bundle")?;
    let document: Bundle = parse_strict_json(&bundle_bytes, "Bundle").map_err(invalid)?;
    let root = path
        .parent()
        .map(Path::to_owned)
        .unwrap_or_else(|| PathBuf::from("."));

    let baseline_bytes = read_bound(&root, &document.baseline, "baseline settings")?;
    let candidate_bytes = read_bound(&root, &document.candidate, "candidate settings")?;
    let state_bytes = read_bound(&root, &document.state, "state manifest")?;
    let spec_bytes = read_bound(&root, &document.owner_spec, "owner spec")?;
    let catalog_path = bundle_artifact_path(&root, &document.catalog)?;
    let catalog_bytes = read_bound(&root, &document.catalog, "artifact catalog")?;
    let _ = catalog_bytes;

    let state_path = bundle_artifact_path(&root, &document.state)?;
    let state = validate_state(&state_path)?;
    if state.bytes != state_bytes {
        return Err(invalid(
            "The bound state manifest changed while it was validated",
        ));
    }
    if state.document.baseline_sha256 != document.baseline.sha256 {
        return Err(invalid(
            "The bundle baseline artifact differs from the baseline recorded by the state manifest",
        ));
    }
    let _ = baseline_bytes;

    let candidate = parse_json_object(&candidate_bytes, "candidate settings", path)?;
    let catalog = load_bound_artifact_catalog(&catalog_path, &candidate_bytes, &state.bytes)
        .map_err(invalid)?;
    let spec: OwnerSpec = parse_strict_json(&spec_bytes, "Owner spec").map_err(invalid)?;

    // Every sealed candidate and captured regex must compile with the Zed-compatible engine.
    for pattern in &catalog.patterns {
        compile_pattern(&pattern.pattern, pattern.definition.case_sensitive).map_err(|error| {
            let detail = match error {
                PatternError::Empty => "the pattern is empty".to_owned(),
                PatternError::Invalid(error) => regex_error_summary(&error),
            };
            invalid(format!(
                "Catalog pattern `{}` does not compile. {detail}",
                display_id(&pattern.definition.id)
            ))
        })?;
    }
    for pattern in &state.patterns {
        let text = std::str::from_utf8(&pattern.bytes)
            .map_err(|_| invalid("A captured pattern artifact is not valid UTF-8"))?;
        compile_pattern(text, pattern.case_sensitive).map_err(|error| {
            let detail = match error {
                PatternError::Empty => "the pattern is empty".to_owned(),
                PatternError::Invalid(error) => regex_error_summary(&error),
            };
            invalid(format!(
                "Captured pattern `{}` does not compile. {detail}",
                display_id(&pattern.id)
            ))
        })?;
    }

    Ok(LoadedBundle {
        root,
        bytes: bundle_bytes,
        document,
        candidate,
        state,
        catalog,
        spec,
    })
}

/// Evidence integrity. Required kinds must be present, every binding must match the sealed
/// graph, and each recorded input closure is independently recomputed.
fn verify_evidence(bundle: &LoadedBundle) -> Result<BTreeSet<String>, AppError> {
    let mut entry_ids = BTreeSet::new();
    let mut kinds: BTreeMap<ResultKind, Vec<&ValidationEntry>> = BTreeMap::new();

    for entry in &bundle.document.validation {
        if entry.id.is_empty() || !entry_ids.insert(entry.id.clone()) {
            return Err(invalid(
                "Bundle validation entry IDs must be nonempty and unique",
            ));
        }
        let manifest_path = bundle_artifact_path(&bundle.root, &entry.manifest)?;
        read_bound(&bundle.root, &entry.manifest, "validation manifest")?;
        let result_bytes = read_bound(&bundle.root, &entry.result, "validation result")?;
        let result: ValidationResult =
            parse_strict_json(&result_bytes, "Validation result").map_err(invalid)?;
        if result.kind != entry.kind {
            return Err(invalid(format!(
                "Validation entry `{}` declares kind `{}` while its result records `{}`",
                display_id(&entry.id),
                entry.kind.label(),
                result.kind.label()
            )));
        }
        if result.outcome != OUTCOME_PASSED {
            return Err(invalid(format!(
                "Validation entry `{}` records outcome `{}`",
                display_id(&entry.id),
                display_id(&result.outcome)
            )));
        }

        // The auxiliary artifact must be declared exactly when the recorded result used one. Audit
        // kinds bind a manifest binding in this slot, while manifest-relative kinds bind an overlay.
        let binds_overlay = matches!(
            entry.kind,
            ResultKind::MatcherSuite | ResultKind::Comparison | ResultKind::LayerDecision
        );
        let auxiliary = match (&entry.overlay, &result.bound_inputs.overlay) {
            (None, None) => None,
            (Some(declared), Some(recorded)) => {
                if declared.path != recorded.path || declared.sha256 != recorded.sha256 {
                    return Err(invalid(format!(
                        "Validation entry `{}` declares an overlay that differs from the one its result recorded",
                        display_id(&entry.id)
                    )));
                }
                read_bound(
                    &bundle.root,
                    declared,
                    if binds_overlay {
                        "path overlay"
                    } else {
                        "manifest binding"
                    },
                )?;
                Some(declared)
            }
            _ => {
                return Err(invalid(format!(
                    "Validation entry `{}` and its result disagree about overlay use",
                    display_id(&entry.id)
                )));
            }
        };
        // Resolving an overlay loads `path-overlay.json` beside the declared artifact, so the bound
        // bytes and the loaded overlay must be the same file.
        let overlay = match (binds_overlay, auxiliary) {
            (true, Some(declared)) => {
                let path = bundle_artifact_path(&bundle.root, declared)?;
                if path.file_name() != Some(OsStr::new(PATH_OVERLAY_FILE)) {
                    return Err(invalid(format!(
                        "Validation entry `{}` binds an overlay that is not `{PATH_OVERLAY_FILE}`",
                        display_id(&entry.id)
                    )));
                }
                let directory = path
                    .parent()
                    .ok_or_else(|| invalid("A path overlay must live inside a directory"))?
                    .to_owned();
                Some(ResolvedOverlay::load(&directory).map_err(invalid)?)
            }
            _ => None,
        };

        if result.bound_inputs.manifest_sha256.as_deref() != Some(entry.manifest.sha256.as_str()) {
            return Err(invalid(format!(
                "Validation entry `{}` binds a manifest its result did not record",
                display_id(&entry.id)
            )));
        }
        if let Some(catalog) = result.bound_inputs.catalog_sha256.as_deref()
            && catalog != bundle.document.catalog.sha256
        {
            return Err(invalid(format!(
                "Validation entry `{}` recorded a different artifact catalog",
                display_id(&entry.id)
            )));
        }
        if let Some(settings) = result.bound_inputs.settings_sha256.as_deref()
            && settings != bundle.document.candidate.sha256
            && entry.kind != ResultKind::DeleteAllAudit
        {
            return Err(invalid(format!(
                "Validation entry `{}` recorded settings that are not the sealed candidate",
                display_id(&entry.id)
            )));
        }

        let mut builder = InputClosureBuilder::new(&bundle.root).map_err(invalid)?;
        let context = ClosureContext {
            overlay: overlay.as_ref(),
        };
        let recompute = match entry.kind {
            ResultKind::MatcherSuite => {
                resolve_suite_closure(&mut builder, &context, &manifest_path)
            }
            ResultKind::Comparison => {
                resolve_comparison_closure(&mut builder, &context, &manifest_path)
            }
            ResultKind::LayerDecision => {
                resolve_layer_closure(&mut builder, &context, &manifest_path)
            }
            ResultKind::OwnerAudit | ResultKind::CandidateInventory => {
                let candidate_path =
                    bundle_artifact_path(&bundle.root, &bundle.document.candidate)?;
                let binding = entry
                    .overlay
                    .as_ref()
                    .map(|artifact| bundle_artifact_path(&bundle.root, artifact))
                    .transpose()?;
                resolve_audit_closure(
                    &mut builder,
                    &manifest_path,
                    &candidate_path,
                    binding.as_deref(),
                )
            }
            ResultKind::InventoryQuery | ResultKind::DeleteAllAudit => {
                return Err(invalid(format!(
                    "Validation entry `{}` binds kind `{}`, which a bundle never accepts",
                    display_id(&entry.id),
                    entry.kind.label()
                )));
            }
        };
        recompute.map_err(invalid)?;
        let recomputed = builder.finish().map_err(invalid)?;
        verify_input_closure(&result.bound_inputs.input_closure, &recomputed, 10).map_err(
            |error| {
                invalid(format!(
                    "Validation entry `{}` is stale. {error}",
                    display_id(&entry.id)
                ))
            },
        )?;

        kinds.entry(entry.kind).or_default().push(entry);
    }

    let changes_patterns =
        !bundle.spec.owners.is_empty() || !bundle.catalog.document.patterns.is_empty();
    if changes_patterns {
        for kind in [
            ResultKind::Comparison,
            ResultKind::LayerDecision,
            ResultKind::MatcherSuite,
        ] {
            if !kinds.contains_key(&kind) {
                return Err(invalid(format!(
                    "A terminal-pattern change requires `{}` evidence",
                    kind.label()
                )));
            }
        }
    }

    Ok(entry_ids)
}

/// Resolve where each audited entry sits in the settings the audit actually ran against. A refreshed
/// graph keeps its reviewed manifest bytes, so its recorded positions are only meaningful once the
/// bound rebinding is applied.
fn effective_audit_positions(
    view: &permission_patterns::AuditManifestView,
    binding: Option<&ManifestBinding>,
) -> Result<Vec<TerminalPosition>, AppError> {
    let Some(binding) = binding else {
        return Ok(view.entries.iter().map(|entry| entry.position).collect());
    };

    view.entries
        .iter()
        .map(|entry| {
            binding.entry_position(&entry.id).ok_or_else(|| {
                invalid(format!(
                    "The bound manifest binding does not rebind audited entry `{}`",
                    display_id(&entry.id)
                ))
            })
        })
        .collect()
}

/// Load the manifest binding an audit entry declares, so its reviewed positions can be rebound.
fn entry_manifest_binding(
    root: &Path,
    entry: &ValidationEntry,
) -> Result<Option<ManifestBinding>, AppError> {
    if !matches!(
        entry.kind,
        ResultKind::OwnerAudit | ResultKind::CandidateInventory
    ) {
        return Ok(None);
    }
    let Some(declared) = entry.overlay.as_ref() else {
        return Ok(None);
    };
    let bytes = read_bound(root, declared, "manifest binding")?;
    let binding: ManifestBinding =
        parse_strict_json(&bytes, "Manifest binding").map_err(invalid)?;
    binding.validate().map_err(invalid)?;

    Ok(Some(binding))
}

/// Derive each audit entry’s covered owners from the reviewed manifest and require the sealed
/// record to equal that derivation. Authored labels are never trusted.
fn verify_owner_coverage(bundle: &LoadedBundle, graph: &OwnerGraph<'_>) -> Result<(), AppError> {
    let catalog_position = graph
        .catalog_by_id
        .iter()
        .map(|(id, pattern)| ((pattern.bucket, pattern.source_index), *id))
        .collect::<BTreeMap<_, _>>();
    let owner_of_candidate = graph
        .spec
        .owners
        .iter()
        .flat_map(|owner| {
            owner
                .candidate_members
                .iter()
                .map(move |id| (id.as_str(), owner))
        })
        .collect::<BTreeMap<_, _>>();

    let mut audited_owners = BTreeSet::new();
    let mut emptied_owners = BTreeSet::new();

    for entry in &bundle.document.validation {
        match entry.kind {
            ResultKind::OwnerAudit => {
                let manifest_path = bundle_artifact_path(&bundle.root, &entry.manifest)?;
                let manifest_bytes = read_bytes(&manifest_path, "owner audit manifest")?;
                let view = permission_patterns::parse_audit_manifest_view(&manifest_bytes)
                    .map_err(invalid)?;
                let binding = entry_manifest_binding(&bundle.root, entry)?;
                let audited_positions = effective_audit_positions(&view, binding.as_ref())?;
                let mut derived = BTreeSet::new();
                for position in &audited_positions {
                    let key = (position.bucket, position.index);
                    if let Some(member) = catalog_position.get(&key)
                        && let Some(owner) = owner_of_candidate.get(*member)
                    {
                        derived.insert(owner.id.clone());
                    }
                }
                let declared = entry.owner_ids.iter().cloned().collect::<BTreeSet<_>>();
                if derived != declared {
                    return Err(invalid(format!(
                        "Validation entry `{}` records owner coverage that differs from the coverage its manifest establishes",
                        display_id(&entry.id)
                    )));
                }
                for owner_id in &derived {
                    let owner = graph
                        .spec
                        .owners
                        .iter()
                        .find(|owner| owner.id == *owner_id)
                        .ok_or_else(|| invalid("A derived owner is absent from the owner spec"))?;
                    let supplemental = graph
                        .spec
                        .supplemental
                        .iter()
                        .filter(|record| record.side == SupplementalSide::Candidate)
                        .map(|record| record.member_id.as_str())
                        .collect::<BTreeSet<_>>();
                    for member in &owner.candidate_members {
                        if supplemental.contains(member.as_str()) {
                            continue;
                        }
                        let definition = graph.catalog_by_id[member.as_str()];
                        let present = audited_positions.iter().any(|position| {
                            position.bucket == definition.bucket
                                && position.index == definition.source_index
                        });
                        if !present {
                            return Err(invalid(format!(
                                "Owner `{}` has visible candidate member `{}` absent from its audit evidence",
                                display_id(owner_id),
                                display_id(member)
                            )));
                        }
                    }
                    audited_owners.insert(owner_id.clone());
                }
            }
            ResultKind::CandidateInventory => {
                let manifest_path = bundle_artifact_path(&bundle.root, &entry.manifest)?;
                let manifest_bytes = read_bytes(&manifest_path, "zero-owner manifest")?;
                let manifest: permission_patterns::ZeroOwnerManifest =
                    parse_strict_json(&manifest_bytes, "Zero-owner manifest").map_err(invalid)?;
                let derived = graph
                    .spec
                    .owners
                    .iter()
                    .filter(|owner| {
                        owner.inventory_owner == manifest.inventory_owner
                            && owner.candidate_members.is_empty()
                    })
                    .map(|owner| owner.id.clone())
                    .collect::<BTreeSet<_>>();
                let declared = entry.owner_ids.iter().cloned().collect::<BTreeSet<_>>();
                if derived != declared {
                    return Err(invalid(format!(
                        "Validation entry `{}` records deleted owners that differ from the operations its manifest proves empty",
                        display_id(&entry.id)
                    )));
                }
                emptied_owners.extend(derived);
            }
            _ => {}
        }
    }

    for owner in &graph.spec.owners {
        if owner.candidate_members.is_empty() {
            if !emptied_owners.contains(&owner.id) {
                return Err(invalid(format!(
                    "Owner operation `{}` removes every candidate member and requires zero-owner evidence",
                    display_id(&owner.id)
                )));
            }
        } else if !audited_owners.contains(&owner.id) {
            return Err(invalid(format!(
                "Owner operation `{}` retains candidate members and requires owner-audit evidence",
                display_id(&owner.id)
            )));
        }
    }

    Ok(())
}

/// Derive the owner coverage one evidence manifest establishes. `seal` records this derivation and
/// `preflight` recomputes it, so an authored label is never trusted.
fn derive_entry_owner_ids(
    kind: ResultKind,
    manifest_path: &Path,
    binding: Option<&ManifestBinding>,
    graph: &OwnerGraph<'_>,
) -> Result<Vec<String>, AppError> {
    let catalog_position = graph
        .catalog_by_id
        .iter()
        .map(|(id, pattern)| ((pattern.bucket, pattern.source_index), *id))
        .collect::<BTreeMap<_, _>>();
    let owner_of_candidate = graph
        .spec
        .owners
        .iter()
        .flat_map(|owner| {
            owner
                .candidate_members
                .iter()
                .map(move |id| (id.as_str(), owner))
        })
        .collect::<BTreeMap<_, _>>();

    let derived: BTreeSet<String> = match kind {
        ResultKind::OwnerAudit => {
            let bytes = read_bytes(manifest_path, "owner audit manifest")?;
            let view = permission_patterns::parse_audit_manifest_view(&bytes).map_err(invalid)?;
            let mut owners = BTreeSet::new();
            for position in effective_audit_positions(&view, binding)? {
                let key = (position.bucket, position.index);
                if let Some(member) = catalog_position.get(&key)
                    && let Some(owner) = owner_of_candidate.get(*member)
                {
                    owners.insert(owner.id.clone());
                }
            }
            owners
        }
        ResultKind::CandidateInventory => {
            let bytes = read_bytes(manifest_path, "zero-owner manifest")?;
            let manifest: permission_patterns::ZeroOwnerManifest =
                parse_strict_json(&bytes, "Zero-owner manifest").map_err(invalid)?;
            graph
                .spec
                .owners
                .iter()
                .filter(|owner| {
                    owner.inventory_owner == manifest.inventory_owner
                        && owner.candidate_members.is_empty()
                })
                .map(|owner| owner.id.clone())
                .collect()
        }
        _ => BTreeSet::new(),
    };

    Ok(derived.into_iter().collect())
}

fn bound_graph_artifact(
    root: &Path,
    path: &Path,
    description: &str,
) -> Result<(BoundArtifact, Vec<u8>), AppError> {
    let bytes = read_bytes(path, description)?;
    let relative = relative_within_root(root, path).map_err(invalid)?;

    Ok((
        BoundArtifact {
            path: relative,
            sha256: sha256_hex(&bytes),
        },
        bytes,
    ))
}

/// Bind the reviewed graph and its fresh evidence into one bundle. Sealing establishes integrity and
/// recorded workflow completion only, never user authorization.
fn seal(arguments: &SealArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let root = arguments
        .output
        .parent()
        .map(Path::to_owned)
        .unwrap_or_else(|| PathBuf::from("."));

    let state = validate_state(&arguments.state)?;
    let (candidate_artifact, candidate_bytes) =
        bound_graph_artifact(&root, &arguments.candidate, "candidate settings")?;
    let candidate =
        parse_json_object(&candidate_bytes, "candidate settings", &arguments.candidate)?;
    let catalog = load_bound_artifact_catalog(&arguments.catalog, &candidate_bytes, &state.bytes)
        .map_err(invalid)?;
    let (spec_artifact, spec_bytes) =
        bound_graph_artifact(&root, &arguments.owner_spec, "owner spec")?;
    let spec: OwnerSpec = parse_strict_json(&spec_bytes, "Owner spec").map_err(invalid)?;
    let graph = resolve_owner_graph(&state, &catalog, &spec)?;
    validate_catalog_candidate_sources(&catalog, &candidate, &state.scopes)?;

    let (state_artifact, _) = bound_graph_artifact(&root, &arguments.state, "state manifest")?;
    let (catalog_artifact, _) =
        bound_graph_artifact(&root, &arguments.catalog, "artifact catalog")?;
    let baseline_path = arguments
        .state
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(&state.document.baseline_file);
    let (baseline_artifact, _) = bound_graph_artifact(&root, &baseline_path, "baseline settings")?;

    let plan_bytes = read_bytes(&arguments.validation, "validation manifest")?;
    let plan: ValidationPlan =
        parse_strict_json(&plan_bytes, "Validation manifest").map_err(invalid)?;

    let mut validation = Vec::with_capacity(plan.results.len());
    for entry in &plan.results {
        let manifest_path = root.join(
            permission_patterns::validate_safe_relative_path(&entry.manifest).map_err(invalid)?,
        );
        let result_path = root.join(
            permission_patterns::validate_safe_relative_path(&entry.result).map_err(invalid)?,
        );
        let (manifest_artifact, _) =
            bound_graph_artifact(&root, &manifest_path, "validation manifest")?;
        let (result_artifact, _) = bound_graph_artifact(&root, &result_path, "validation result")?;
        // Manifest-relative kinds bind a path overlay here. Audit kinds bind the manifest binding
        // that rebinds their reviewed positions, so coverage is derived from the rebound positions.
        let binds_overlay = matches!(
            entry.kind,
            ResultKind::MatcherSuite | ResultKind::Comparison | ResultKind::LayerDecision
        );
        let (overlay, binding) = match entry.overlay.as_ref() {
            Some(declared) => {
                let declared_path = root.join(
                    permission_patterns::validate_safe_relative_path(declared).map_err(invalid)?,
                );
                let (artifact, bytes) = bound_graph_artifact(
                    &root,
                    &declared_path,
                    if binds_overlay {
                        "path overlay"
                    } else {
                        "manifest binding"
                    },
                )?;
                let binding = if binds_overlay {
                    None
                } else {
                    let binding: ManifestBinding =
                        parse_strict_json(&bytes, "Manifest binding").map_err(invalid)?;
                    binding.validate().map_err(invalid)?;
                    Some(binding)
                };
                (Some(artifact), binding)
            }
            None => (None, None),
        };

        validation.push(ValidationEntry {
            id: entry.id.clone(),
            kind: entry.kind,
            owner_ids: derive_entry_owner_ids(
                entry.kind,
                &manifest_path,
                binding.as_ref(),
                &graph,
            )?,
            manifest: manifest_artifact,
            result: result_artifact,
            overlay,
        });
    }

    let document = Bundle {
        baseline: baseline_artifact,
        candidate: candidate_artifact,
        state: state_artifact,
        catalog: catalog_artifact,
        owner_spec: spec_artifact,
        validation,
        lineage: None,
    };
    let bytes = serialize_pretty_json_bytes(&document, "bundle").map_err(invalid)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&arguments.output)
        .map_err(|error| {
            invalid(format!(
                "Failed to create bundle `{}`:\n\n{error}",
                arguments.output.display()
            ))
        })?;
    file.write_all(&bytes)
        .map_err(|error| invalid(format!("Failed to write bundle:\n\n{error}")))?;
    drop(file);

    // Sealing fails closed: the freshly written bundle is verified through the same route that
    // preflight and promotion use.
    let sealed = load_bundle(&arguments.output)?;
    let sealed_graph = resolve_owner_graph(&sealed.state, &sealed.catalog, &sealed.spec)?;
    verify_evidence(&sealed)?;
    verify_owner_coverage(&sealed, &sealed_graph)?;

    writeln!(
        stdout,
        "Sealed {} owner {} and {} validation {} into `{}`",
        spec.owners.len(),
        if spec.owners.len() == 1 {
            "operation"
        } else {
            "operations"
        },
        plan.results.len(),
        if plan.results.len() == 1 {
            "entry"
        } else {
            "entries"
        },
        arguments.output.display()
    )
    .map_err(|error| invalid(format!("Failed to write seal result:\n\n{error}")))?;
    writeln!(
        stdout,
        "  The bundle establishes integrity and recorded workflow completion. It is not user approval"
    )
    .map_err(|error| invalid(format!("Failed to write seal result:\n\n{error}")))?;

    Ok(())
}

struct PreflightOutcome {
    bundle: LoadedBundle,
    live_bytes: Vec<u8>,
    output: Vec<u8>,
    unchanged: bool,
}

/// Run every deterministic promotion check up to the mutation boundary. `preflight` and
/// `promote --write` share this sequence, so a passing rehearsal and the in-process run agree.
fn preflight_promotion(settings: &Path, bundle_path: &Path) -> Result<PreflightOutcome, AppError> {
    let bundle = load_bundle(bundle_path)?;
    let graph = resolve_owner_graph(&bundle.state, &bundle.catalog, &bundle.spec)?;

    validate_catalog_candidate_sources(&bundle.catalog, &bundle.candidate, &bundle.state.scopes)?;
    let validation_ids = verify_evidence(&bundle)?;
    verify_owner_coverage(&bundle, &graph)?;
    verify_supplemental_ownership(&bundle.candidate, &bundle.state, &graph, &validation_ids)?;
    verify_visibility_rewrites(&bundle.candidate, &graph)?;
    authorize_candidate(&bundle.candidate, &bundle.state, "Promotion")?;

    ensure_no_symlink_components(settings).map_err(|error| {
        let message = error.to_string();
        match error {
            PathInspectionError::Io(_) => invalid(message),
            PathInspectionError::Symlink(_) => refused(message),
        }
    })?;
    let destination_metadata = fs::symlink_metadata(settings).map_err(|error| {
        invalid(format!(
            "Failed to inspect live settings destination `{}`:\n\n{error}",
            settings.display()
        ))
    })?;
    if !destination_metadata.is_file() || destination_metadata.file_type().is_symlink() {
        return Err(refused(
            "Live settings destination must be a regular file and not a symbolic link",
        ));
    }

    let (live_bytes, live) = read_json_object(settings, "live settings")?;
    for (index, tokens) in bundle.state.scopes.iter().enumerate() {
        let baseline_value = pointer_value(&bundle.state.baseline, tokens)
            .map_err(|_| invalid("Validated baseline scope became unavailable"))?;
        let live_value = pointer_value(&live, tokens).map_err(|_| {
            refused(format!(
                "Promotion refused because live settings no longer contain authorized scope {}",
                index + 1
            ))
        })?;
        if !semantic_json_equal(live_value, baseline_value) {
            return Err(refused(format!(
                "Promotion refused because live authorized scope {} drifted from the captured baseline",
                index + 1
            )));
        }
    }

    authorize_owner_operations(&bundle.candidate, &bundle.state, &graph)?;

    let mut merged = live;
    for (index, tokens) in bundle.state.scopes.iter().enumerate() {
        let candidate_value = pointer_value(&bundle.candidate, tokens)
            .map_err(|_| {
                refused(format!(
                    "Promotion refused because candidate settings do not contain authorized scope {}",
                    index + 1
                ))
            })?
            .clone();
        replace_pointer_value(&mut merged, tokens, candidate_value).map_err(|_| {
            refused(format!(
                "Promotion refused because live settings do not contain authorized scope {}",
                index + 1
            ))
        })?;
    }
    let output = serialize_pretty_json(&merged).map_err(invalid)?;
    let unchanged = output == live_bytes;

    Ok(PreflightOutcome {
        bundle,
        live_bytes,
        output,
        unchanged,
    })
}

fn parent_relative(base: &str, file: &str) -> String {
    match base.rfind('/') {
        Some(offset) => format!("{}/{file}", &base[..offset]),
        None => file.to_owned(),
    }
}

fn manifest_position(value: &Value) -> Result<TerminalPosition, AppError> {
    let bucket = value
        .get("bucket")
        .and_then(Value::as_str)
        .and_then(Bucket::parse)
        .ok_or_else(|| invalid("A reviewed manifest position must declare a valid bucket"))?;
    let index = value
        .get("index")
        .and_then(Value::as_u64)
        .and_then(|index| usize::try_from(index).ok())
        .ok_or_else(|| invalid("A reviewed manifest position must declare an index"))?;

    Ok(TerminalPosition { bucket, index })
}

/// Build the transient rebinding one reviewed audit manifest needs against the refreshed candidate.
/// Every snapshot-dependent position the audit binary rebinds is covered, so reviewed manifest bytes
/// stay byte-identical and no hash, path, or index is edited by hand.
fn build_manifest_binding(
    manifest_bytes: &[u8],
    source_settings_sha256: &str,
    source_binding: Option<&ManifestBinding>,
    settings_sha256: &str,
    remap: &BTreeMap<TerminalPosition, TerminalPosition>,
) -> Result<ManifestBinding, AppError> {
    let manifest: Value =
        parse_strict_json(manifest_bytes, "Reviewed audit manifest").map_err(invalid)?;
    if let Some(binding) = source_binding
        && binding.settings_sha256 != source_settings_sha256
    {
        return Err(invalid(
            "The source manifest binding does not bind the sealed candidate settings",
        ));
    }
    let relocate = |source: TerminalPosition| {
        remap.get(&source).copied().ok_or_else(|| {
            refused(format!(
                "Refresh refused because reviewed manifest position `{}` does not relocate uniquely",
                source.label()
            ))
        })
    };

    let mut entries = Vec::new();
    if let Some(values) = manifest.get("entries").and_then(Value::as_array) {
        for value in values {
            let id = value
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid("A reviewed audit manifest entry must declare `id`"))?;
            let reviewed = manifest_position(value)?;
            let source = match source_binding {
                Some(binding) => binding.entry_position(id).ok_or_else(|| {
                    invalid(format!(
                        "The source manifest binding does not rebind audited entry `{}`",
                        display_id(id)
                    ))
                })?,
                None => reviewed,
            };
            let target = relocate(source)?;
            entries.push(BoundEntryPosition {
                id: id.to_owned(),
                bucket: target.bucket,
                index: target.index,
            });
        }
    }

    let mut sources = BTreeSet::new();
    let mut positions = Vec::new();
    for key in ["excluded_candidates", "retained_owner_entries"] {
        let Some(values) = manifest.get(key).and_then(Value::as_array) else {
            continue;
        };
        for value in values {
            let reviewed = manifest_position(value)?;
            let source = match source_binding {
                Some(binding) => binding.remapped(reviewed).ok_or_else(|| {
                    invalid(format!(
                        "The source manifest binding does not rebind position `{}`",
                        reviewed.label()
                    ))
                })?,
                None => reviewed,
            };
            let target = relocate(source)?;
            if sources.insert(reviewed) {
                positions.push(PositionRemap {
                    from: BoundPosition {
                        bucket: reviewed.bucket,
                        index: reviewed.index,
                    },
                    to: BoundPosition {
                        bucket: target.bucket,
                        index: target.index,
                    },
                });
            }
        }
    }

    let binding = ManifestBinding {
        settings_sha256: settings_sha256.to_owned(),
        entries,
        positions,
    };
    binding.validate().map_err(invalid)?;

    Ok(binding)
}

fn terminal_identity(value: &Value) -> Option<(String, bool)> {
    let object = value.as_object()?;
    Some((
        object.get("pattern")?.as_str()?.to_owned(),
        object.get("case_sensitive")?.as_bool()?,
    ))
}

fn unique_identity_mappings(reviewed: &[Value], current: &[Value]) -> Vec<(usize, usize)> {
    let reviewed_identities = reviewed.iter().map(terminal_identity).collect::<Vec<_>>();
    let current_identities = current.iter().map(terminal_identity).collect::<Vec<_>>();
    let mut mappings = Vec::new();

    for (reviewed_index, identity) in reviewed_identities.iter().enumerate() {
        let Some(identity) = identity else {
            continue;
        };
        if reviewed_identities
            .iter()
            .filter(|candidate| candidate.as_ref() == Some(identity))
            .count()
            != 1
        {
            continue;
        }
        let current_matches = current_identities
            .iter()
            .enumerate()
            .filter(|(_, candidate)| candidate.as_ref() == Some(identity))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if let [current_index] = current_matches.as_slice() {
            mappings.push((reviewed_index, *current_index));
        }
    }

    mappings
}

fn locate_unique(
    values: &[Value],
    identity: &(String, bool),
    description: &str,
) -> Result<usize, AppError> {
    let matches = values
        .iter()
        .enumerate()
        .filter(|(_, value)| terminal_identity(value).as_ref() == Some(identity))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [index] => Ok(*index),
        [] => Err(refused(format!(
            "Refresh refused because {description} is missing"
        ))),
        _ => Err(refused(format!(
            "Refresh refused because {description} no longer identifies one unique object"
        ))),
    }
}

/// Rebuild a reviewed candidate against current settings without a task-authored replay script.
/// Placement replays the reviewed candidate’s own ordering relative to its unchanged remainder.
fn refresh(arguments: &RefreshArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let bundle = load_bundle(&arguments.bundle)?;
    let graph = resolve_owner_graph(&bundle.state, &bundle.catalog, &bundle.spec)?;
    verify_evidence(&bundle)?;
    verify_owner_coverage(&bundle, &graph)?;

    let output_absolute = normalized_absolute_path(&arguments.output)?;
    let projected_output = inspect_refresh_output_path(&output_absolute)?;
    let bundle_root = fs::canonicalize(&bundle.root).map_err(|error| {
        invalid(format!(
            "Failed to resolve the reviewed graph root `{}`:\n\n{error}",
            bundle.root.display()
        ))
    })?;
    if projected_output.starts_with(&bundle_root) || bundle_root.starts_with(&projected_output) {
        return Err(invalid(
            "The refresh output directory must lie outside the reviewed graph root",
        ));
    }

    let (settings_bytes, settings) = read_json_object(&arguments.settings, "settings")?;

    // Relocate every baseline member uniquely by exact decoded bytes and case setting.
    let mut relocated: HashMap<Bucket, BTreeMap<usize, &LoadedPattern>> = HashMap::new();
    let mut relocation_report = Vec::new();
    for pattern in &bundle.state.patterns {
        let text = std::str::from_utf8(&pattern.bytes)
            .map_err(|_| invalid("A captured pattern artifact is not valid UTF-8"))?
            .to_owned();
        let values = required_terminal_pattern_array(&settings, pattern.bucket, "settings")?;
        let index = locate_unique(
            values,
            &(text, pattern.case_sensitive),
            &format!("baseline member `{}`", display_id(&pattern.id)),
        )?;
        relocated
            .entry(pattern.bucket)
            .or_default()
            .insert(index, pattern);
        relocation_report.push(format!(
            "{} {} -> {}",
            display_id(&pattern.id),
            TerminalPosition {
                bucket: pattern.bucket,
                index: pattern.source_index
            }
            .label(),
            TerminalPosition {
                bucket: pattern.bucket,
                index
            }
            .label()
        ));
    }

    let mut candidate = settings.clone();
    let mut drift = Vec::new();
    // Reviewed candidate position -> refreshed candidate position, for every element whose origin
    // the replay establishes. Reviewed audit manifests are rebound through this map.
    let mut position_remap: BTreeMap<TerminalPosition, TerminalPosition> = BTreeMap::new();

    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let reviewed =
            required_terminal_pattern_array(&bundle.candidate, bucket, "reviewed candidate")?;
        let member_indexes = graph
            .candidate_member_ids
            .iter()
            .filter_map(|id| graph.catalog_by_id.get(*id))
            .filter(|definition| definition.bucket == bucket)
            .map(|definition| definition.source_index)
            .collect::<BTreeSet<_>>();

        // Reviewed gaps count the retained remainder elements preceding each candidate member.
        let mut reviewed_remainder = Vec::new();
        let mut reviewed_remainder_source = Vec::new();
        let mut placements: Vec<(usize, usize, Value)> = Vec::new();
        for (index, value) in reviewed.iter().enumerate() {
            if member_indexes.contains(&index) {
                placements.push((reviewed_remainder.len(), index, value.clone()));
            } else {
                reviewed_remainder.push(value.clone());
                reviewed_remainder_source.push(index);
            }
        }

        let current = required_terminal_pattern_array(&settings, bucket, "settings")?;
        let removed = relocated.get(&bucket).cloned().unwrap_or_default();
        let current_remainder = current
            .iter()
            .enumerate()
            .filter(|(index, _)| !removed.contains_key(index))
            .map(|(_, value)| value.clone())
            .collect::<Vec<_>>();
        if current_remainder.len() != reviewed_remainder.len() {
            drift.push(format!(
                "{} remainder {} -> {}",
                bucket.label(),
                reviewed_remainder.len(),
                current_remainder.len()
            ));
        }

        let mut insertions: Vec<(usize, usize, Value)> = Vec::new();
        for (gap, source_index, value) in placements {
            let offset = if gap == 0 {
                0
            } else if gap == reviewed_remainder.len() {
                current_remainder.len()
            } else {
                let left = terminal_identity(&reviewed_remainder[gap - 1]).ok_or_else(|| {
                    invalid("A reviewed remainder entry is not a terminal pattern object")
                })?;
                let right = terminal_identity(&reviewed_remainder[gap]).ok_or_else(|| {
                    invalid("A reviewed remainder entry is not a terminal pattern object")
                })?;
                let left_index = locate_unique(
                    &current_remainder,
                    &left,
                    &format!("the left boundary of a reviewed `{}` gap", bucket.label()),
                )?;
                let right_index = locate_unique(
                    &current_remainder,
                    &right,
                    &format!("the right boundary of a reviewed `{}` gap", bucket.label()),
                )?;
                if left_index >= right_index {
                    return Err(refused(format!(
                        "Refresh refused because a reviewed `{}` gap reordered across its boundaries",
                        bucket.label()
                    )));
                }
                left_index + 1
            };
            insertions.push((offset, source_index, value));
        }

        // Map only identities that are unique on both sides. Unrelated count drift therefore keeps
        // stable exclusions rebindable without guessing among duplicate or changed remainder entries.
        let mut origins = vec![None; current_remainder.len()];
        for (reviewed_index, current_index) in
            unique_identity_mappings(&reviewed_remainder, &current_remainder)
        {
            origins[current_index] = Some(reviewed_remainder_source[reviewed_index]);
        }
        let mut rebuilt = current_remainder;
        for (offset, source_index, value) in insertions.into_iter().rev() {
            let at = offset.min(rebuilt.len());
            rebuilt.insert(at, value);
            origins.insert(at, Some(source_index));
        }
        for (index, origin) in origins.iter().enumerate() {
            let Some(source_index) = *origin else {
                continue;
            };
            position_remap.insert(
                TerminalPosition {
                    bucket,
                    index: source_index,
                },
                TerminalPosition { bucket, index },
            );
        }

        let tokens = vec![
            "agent".to_owned(),
            "tool_permissions".to_owned(),
            "tools".to_owned(),
            "terminal".to_owned(),
            bucket.label().to_owned(),
        ];
        let target = pointer_value_mut(&mut candidate, &tokens)
            .map_err(|_| invalid("A validated terminal permission bucket became unavailable"))?;
        *target = Value::Array(rebuilt);
    }

    let candidate_bytes = serialize_pretty_json(&candidate).map_err(invalid)?;

    // Reproduce the reviewed graph-relative layout so reviewed manifests run through an overlay.
    let mut artifacts = BTreeMap::new();
    let mut overlay_paths = BTreeSet::new();
    let mut replaced_paths = BTreeSet::new();
    let mut state_patterns = Vec::with_capacity(bundle.state.patterns.len());
    for pattern in &bundle.state.document.patterns {
        let relative = parent_relative(&bundle.document.state.path, &pattern.pattern_file);
        let loaded = bundle
            .state
            .patterns
            .iter()
            .find(|loaded| loaded.id == pattern.id)
            .ok_or_else(|| invalid("A validated state pattern disappeared"))?;
        let index = relocated
            .get(&loaded.bucket)
            .and_then(|entries| {
                entries
                    .iter()
                    .find(|(_, candidate)| candidate.id == loaded.id)
                    .map(|(index, _)| *index)
            })
            .ok_or_else(|| invalid("A relocated baseline member disappeared"))?;
        add_refresh_artifact(&mut artifacts, relative.clone(), loaded.bytes.clone())?;
        overlay_paths.insert(relative);
        state_patterns.push(StatePattern {
            id: pattern.id.clone(),
            bucket: pattern.bucket,
            source_index: index,
            case_sensitive: pattern.case_sensitive,
            sha256: pattern.sha256.clone(),
            pattern_file: pattern.pattern_file.clone(),
        });
    }

    let state_document = StateDocument {
        baseline_file: bundle.state.document.baseline_file.clone(),
        baseline_sha256: sha256_hex(&settings_bytes),
        scopes: bundle.state.document.scopes.clone(),
        patterns: state_patterns,
    };
    let state_bytes =
        serialize_pretty_json_bytes(&state_document, "state manifest").map_err(invalid)?;

    let mut catalog_patterns = Vec::with_capacity(bundle.catalog.document.patterns.len());
    for pattern in &bundle.catalog.patterns {
        let relative = parent_relative(
            &bundle.document.catalog.path,
            &pattern.definition.pattern_file,
        );
        let values = required_terminal_pattern_array(
            &candidate,
            pattern.definition.bucket,
            "refreshed candidate",
        )?;
        let index = locate_unique(
            values,
            &(pattern.pattern.clone(), pattern.definition.case_sensitive),
            &format!("catalog member `{}`", display_id(&pattern.definition.id)),
        )?;
        add_refresh_artifact(
            &mut artifacts,
            relative.clone(),
            pattern.pattern.clone().into_bytes(),
        )?;
        overlay_paths.insert(relative);
        catalog_patterns.push(ArtifactCatalogPattern {
            id: pattern.definition.id.clone(),
            bucket: pattern.definition.bucket,
            source_index: index,
            case_sensitive: pattern.definition.case_sensitive,
            sha256: pattern.definition.sha256.clone(),
            pattern_file: pattern.definition.pattern_file.clone(),
        });
    }

    let baseline_relative = parent_relative(
        &bundle.document.state.path,
        &bundle.state.document.baseline_file,
    );
    add_refresh_artifact(
        &mut artifacts,
        baseline_relative.clone(),
        settings_bytes.clone(),
    )?;
    replaced_paths.insert(baseline_relative.clone());
    overlay_paths.insert(baseline_relative);
    add_refresh_artifact(
        &mut artifacts,
        bundle.document.candidate.path.clone(),
        candidate_bytes.clone(),
    )?;
    replaced_paths.insert(bundle.document.candidate.path.clone());
    overlay_paths.insert(bundle.document.candidate.path.clone());
    add_refresh_artifact(
        &mut artifacts,
        bundle.document.state.path.clone(),
        state_bytes.clone(),
    )?;
    replaced_paths.insert(bundle.document.state.path.clone());
    overlay_paths.insert(bundle.document.state.path.clone());

    let catalog_document = ArtifactCatalog {
        candidate_sha256: sha256_hex(&candidate_bytes),
        state_sha256: sha256_hex(&state_bytes),
        patterns: catalog_patterns,
    };
    validate_artifact_catalog(&catalog_document).map_err(invalid)?;
    let catalog_bytes =
        serialize_pretty_json_bytes(&catalog_document, "artifact catalog").map_err(invalid)?;
    add_refresh_artifact(
        &mut artifacts,
        bundle.document.catalog.path.clone(),
        catalog_bytes,
    )?;
    replaced_paths.insert(bundle.document.catalog.path.clone());
    overlay_paths.insert(bundle.document.catalog.path.clone());

    // The owner spec carries stable semantics only, so it is copied byte-for-byte.
    let spec_bytes = read_bound(&bundle.root, &bundle.document.owner_spec, "owner spec")?;
    add_refresh_artifact(
        &mut artifacts,
        bundle.document.owner_spec.path.clone(),
        spec_bytes,
    )?;
    overlay_paths.insert(bundle.document.owner_spec.path.clone());

    // Every reviewed manifest is reproduced byte-for-byte at its graph-relative path, and every
    // audit manifest gains the binding that rebinds its snapshot-dependent positions. Sealing the
    // refreshed graph therefore needs no manual hash, path, or index editing.
    let candidate_sha256 = sha256_hex(&candidate_bytes);
    let mut plan_entries = Vec::with_capacity(bundle.document.validation.len());
    let mut source_results = Vec::with_capacity(bundle.document.validation.len());
    for (index, entry) in bundle.document.validation.iter().enumerate() {
        let manifest_bytes = read_bound(&bundle.root, &entry.manifest, "validation manifest")?;
        let source_binding = entry_manifest_binding(&bundle.root, entry)?;
        let auxiliary = match entry.kind {
            ResultKind::OwnerAudit | ResultKind::CandidateInventory => {
                let binding = build_manifest_binding(
                    &manifest_bytes,
                    &bundle.document.candidate.sha256,
                    source_binding.as_ref(),
                    &candidate_sha256,
                    &position_remap,
                )?;
                let binding_bytes =
                    serialize_pretty_json_bytes(&binding, "manifest binding").map_err(invalid)?;
                let relative = format!("{}.binding.json", entry.manifest.path);
                add_refresh_artifact(&mut artifacts, relative.clone(), binding_bytes)?;
                Some(relative)
            }
            _ => Some(PATH_OVERLAY_FILE.to_owned()),
        };
        add_refresh_artifact(&mut artifacts, entry.manifest.path.clone(), manifest_bytes)?;
        overlay_paths.insert(entry.manifest.path.clone());

        let result_bytes = read_bound(&bundle.root, &entry.result, "validation result")?;
        let result: ValidationResult =
            parse_strict_json(&result_bytes, "Validation result").map_err(invalid)?;
        source_results.push(result);
        plan_entries.push(ValidationPlanEntry {
            id: entry.id.clone(),
            kind: entry.kind,
            manifest: entry.manifest.path.clone(),
            result: format!("{}.refreshed-{}.json", entry.result.path, index + 1),
            overlay: auxiliary,
        });
    }

    // Preserve every exact file-backed input from the sealed closures unless refresh intentionally
    // regenerated that graph path. Repeated declarations collapse only when their bytes agree.
    for result in &source_results {
        for record in &result.bound_inputs.input_closure.records {
            if record.role == permission_patterns::ROLE_OVERLAY
                || record.role == permission_patterns::ROLE_BINDING
                || replaced_paths.contains(&record.path)
            {
                continue;
            }
            let bytes = permission_patterns::read_regular_file_within_root(
                &bundle.root,
                &record.path,
                "sealed validation input",
            )
            .map_err(invalid)?;
            if sha256_hex(&bytes) != record.sha256 {
                return Err(invalid(format!(
                    "Sealed validation input `{}` changed while the refresh graph was prepared",
                    record.path
                )));
            }
            add_refresh_artifact(&mut artifacts, record.path.clone(), bytes)?;
            overlay_paths.insert(record.path.clone());
        }
    }

    let overlay = PathOverlay {
        paths: overlay_paths.into_iter().collect(),
    };
    let overlay_bytes = serialize_pretty_json_bytes(&overlay, "path overlay").map_err(invalid)?;
    add_refresh_artifact(&mut artifacts, PATH_OVERLAY_FILE.to_owned(), overlay_bytes)?;

    let plan = ValidationPlan {
        results: plan_entries,
    };
    let plan_bytes = serialize_pretty_json_bytes(&plan, "validation plan").map_err(invalid)?;
    add_refresh_artifact(
        &mut artifacts,
        "validation-plan.json".to_owned(),
        plan_bytes,
    )?;

    let report = serde_json::json!({
        "source_bundle_sha256": sha256_hex(&bundle.bytes),
        "refreshed_settings_sha256": sha256_hex(&settings_bytes),
        "relocated_members": relocation_report,
        "outside_owner_drift": drift,
    });
    let report_bytes = serialize_pretty_json(&report).map_err(invalid)?;
    add_refresh_artifact(
        &mut artifacts,
        "refresh-report.json".to_owned(),
        report_bytes,
    )?;

    commit_refresh_artifacts(&output_absolute, &artifacts)?;

    writeln!(
        stdout,
        "Refreshed {} owner {} into `{}`",
        bundle.spec.owners.len(),
        if bundle.spec.owners.len() == 1 {
            "operation"
        } else {
            "operations"
        },
        arguments.output.display()
    )
    .map_err(|error| invalid(format!("Failed to write refresh result:\n\n{error}")))?;
    writeln!(
        stdout,
        "  The refreshed graph is unsealed. Run each validator in `validation-plan.json`, then `seal`"
    )
    .map_err(|error| invalid(format!("Failed to write refresh result:\n\n{error}")))?;

    Ok(())
}

fn preflight(arguments: &PreflightArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let outcome = preflight_promotion(&arguments.settings, &arguments.bundle)?;
    writeln!(
        stdout,
        "Preflight passed for {} authorized {} bound by `{}`",
        outcome.bundle.state.document.scopes.len(),
        if outcome.bundle.state.document.scopes.len() == 1 {
            "scope"
        } else {
            "scopes"
        },
        arguments.bundle.display()
    )
    .map_err(|error| invalid(format!("Failed to write preflight result:\n\n{error}")))?;
    writeln!(
        stdout,
        "  {} owner {}, {} validation {}, live settings would be {}",
        outcome.bundle.spec.owners.len(),
        if outcome.bundle.spec.owners.len() == 1 {
            "operation"
        } else {
            "operations"
        },
        outcome.bundle.document.validation.len(),
        if outcome.bundle.document.validation.len() == 1 {
            "entry"
        } else {
            "entries"
        },
        if outcome.unchanged {
            "unchanged"
        } else {
            "replaced"
        }
    )
    .map_err(|error| invalid(format!("Failed to write preflight result:\n\n{error}")))?;
    writeln!(
        stdout,
        "  This rehearsal expires as soon as live settings can change and never authorizes promotion"
    )
    .map_err(|error| invalid(format!("Failed to write preflight result:\n\n{error}")))?;

    Ok(())
}

fn promote(arguments: &PromoteArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    // The authoritative run happens here, immediately before the mutation boundary.
    let outcome = preflight_promotion(&arguments.settings, &arguments.bundle)?;

    if outcome.unchanged {
        writeln!(
            stdout,
            "Live settings unchanged at `{}`",
            arguments.settings.display()
        )
        .map_err(|error| {
            invalid(format!(
                "Failed to write promotion result to standard output:\n\n{error}"
            ))
        })?;
        return Ok(());
    }

    match atomic_replace_with_best_effort_recheck(
        &arguments.settings,
        &outcome.output,
        &outcome.live_bytes,
        |_| Ok(()),
    ) {
        Ok(()) => {}
        Err(BestEffortReplaceError::Changed) => {
            return Err(refused(
                "Promotion refused because live settings changed while promotion output was prepared",
            ));
        }
        Err(BestEffortReplaceError::Invalid(error)) => return Err(invalid(error)),
    }

    writeln!(
        stdout,
        "Promoted {} authorized {} into `{}`",
        outcome.bundle.state.document.scopes.len(),
        if outcome.bundle.state.document.scopes.len() == 1 {
            "scope"
        } else {
            "scopes"
        },
        arguments.settings.display()
    )
    .map_err(|error| {
        invalid(format!(
            "Failed to write promotion result to standard output:\n\n{error}"
        ))
    })?;
    writeln!(
        stdout,
        "  Wrote {} bytes with SHA-256 {}. A later writer can still replace the file after the final recheck",
        outcome.output.len(),
        sha256_hex(&outcome.output)
    )
    .map_err(|error| {
        invalid(format!(
            "Failed to write promotion result to standard output:\n\n{error}"
        ))
    })?;

    Ok(())
}

fn report_error(stderr: &mut dyn Write, error: &AppError) {
    let _ = writeln!(stderr, "permission-candidate: {}", error.message());
}

pub(crate) fn run<I>(arguments: I, stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8
where
    I: IntoIterator<Item = OsString>,
{
    let parsed = match parse_arguments(arguments) {
        Ok(parsed) => parsed,
        Err(error) => {
            report_error(stderr, &error);
            return error.status();
        }
    };

    let result = match parsed {
        ParsedArguments::Help => {
            if let Err(error) = stdout.write_all(HELP.as_bytes()) {
                Err(invalid(format!(
                    "Failed to write help to standard output:\n\n{error}"
                )))
            } else {
                Ok(())
            }
        }
        ParsedArguments::Run(Operation::Capture(arguments)) => capture(&arguments, stdout),
        ParsedArguments::Run(Operation::Materialize(arguments)) => materialize(&arguments, stdout),
        ParsedArguments::Run(Operation::Verify(arguments)) => verify(&arguments, stdout),
        ParsedArguments::Run(Operation::Preflight(arguments)) => preflight(&arguments, stdout),
        ParsedArguments::Run(Operation::Promote(arguments)) => promote(&arguments, stdout),
        ParsedArguments::Run(Operation::Refresh(arguments)) => refresh(&arguments, stdout),
        ParsedArguments::Run(Operation::Seal(arguments)) => seal(&arguments, stdout),
    };

    match result {
        Ok(()) => STATUS_SUCCESS,
        Err(error) => {
            report_error(stderr, &error);
            error.status()
        }
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
