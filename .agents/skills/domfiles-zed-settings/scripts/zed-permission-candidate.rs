#[allow(dead_code)]
#[path = "helpers/permission-patterns.rs"]
mod permission_patterns;

pub(crate) use permission_patterns::Bucket;
use permission_patterns::{BoundedIssues, read_utf8_file};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
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

const BASELINE_FILE: &str = "baseline-settings.json";
const CANDIDATE_FILE: &str = "candidate-settings.json";
const MAX_REPORTED_ITEMS: usize = 100;
const MAX_REPORTED_VERIFY_ITEMS: usize = 10;
const STATE_FILE: &str = "state.json";
const STATUS_ERROR: u8 = 2;
const STATUS_REFUSED: u8 = 1;
const STATUS_SUCCESS: u8 = 0;
const VERSION: u64 = 1;

static NEXT_TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

const HELP: &str = concat!(
    "Usage:\n",
    "  zed-permission-candidate capture --settings <path> --selection <selection.json> --output <directory>\n",
    "  zed-permission-candidate verify --settings <path> --state <state.json>\n",
    "  zed-permission-candidate promote --settings <live> --candidate <candidate> --state <state.json> --write\n",
    "\n",
    "Capture exact Zed terminal-permission candidates, verify current indexes, and promote authorized scopes\n",
    "\n",
    "Modes:\n",
    "  capture  Read exact settings bytes, validate selected terminal pattern objects, and create artifacts\n",
    "  verify   Validate every state artifact, then locate each pattern by bucket, exact UTF-8 bytes, and case setting\n",
    "  promote  Validate every state artifact and guard, merge authorized candidate scopes into live settings, and atomically replace live settings\n",
    "\n",
    "Options:\n",
    "  --candidate <path>          Candidate JSON object used only by promote\n",
    "  --help                      Print this complete help when used alone\n",
    "  --output <directory>        Explicit capture artifact directory\n",
    "  --selection <path>          Version-1 capture selection JSON\n",
    "  --settings <path>           Baseline or current settings for capture and verify, or the live destination for promote\n",
    "  --state <path>              Version-1 state manifest used by verify and promote\n",
    "  --write                     Required exact mutation guard for promote\n",
    "\n",
    "Selection JSON schema (unknown fields are rejected):\n",
    "  {\"version\":1,\"scopes\":[\"/json/pointer\"],\"patterns\":[{\"id\":\"nonempty\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"index\":0}]}\n",
    "  `scopes` and `patterns` must be nonempty. Pattern IDs and bucket/index selections must be unique\n",
    "  Scopes must be existing, non-root RFC 6901 pointers with no duplicates or parent/child overlap\n",
    "  Every selected pattern object must lie within an authorized scope and contain string `pattern` and boolean `case_sensitive` fields\n",
    "\n",
    "State JSON schema (unknown fields are rejected):\n",
    "  {\"version\":1,\"baseline_file\":\"relative path\",\"baseline_sha256\":\"64 lowercase hex characters\",\"scopes\":[\"/json/pointer\"],\"patterns\":[{\"id\":\"nonempty\",\"bucket\":\"always_allow|always_confirm|always_deny\",\"source_index\":0,\"case_sensitive\":true,\"sha256\":\"64 lowercase hex characters\",\"pattern_file\":\"relative path\"}]}\n",
    "  Relative baseline and pattern paths resolve from the state manifest’s parent\n",
    "  The manifest records hashes but does not authenticate itself\n",
    "\n",
    "Capture contract:\n",
    "  Settings must parse as a JSON object and are retained byte-for-byte as immutable `baseline-settings.json`\n",
    "  An editable byte-identical `candidate-settings.json` is created beside the baseline\n",
    "  Pattern files contain exact decoded UTF-8 pattern bytes with no added newline\n",
    "  Generated pattern names use a sequence and sanitized ID. Raw IDs never become paths\n",
    "  The output directory may already exist, but symlink traversal, non-directories, existing artifact paths, and overwrite are refused\n",
    "\n",
    "Verify contract:\n",
    "  Baseline and pattern hashes, JSON structure, scopes, UTF-8, and recorded baseline source identities are validated before reindexing\n",
    "  Missing or duplicate exact current matches are refused\n",
    "  Successful output contains at most 10 moved `id -> bucket[index]` metadata lines, an omission summary when needed, and aggregate counts\n",
    "  Missing or duplicate refusal reports at most 10 exceptional IDs and counts every failure\n",
    "\n",
    "Promote contract:\n",
    "  `--write` is mandatory. There is no force option\n",
    "  Live values at every authorized scope must equal the captured baseline values\n",
    "  Candidate changes outside authorized scopes are refused, and absent parents are never created\n",
    "  Candidate scope values are merged into the live object read for promotion, preserving its out-of-scope values\n",
    "  Changed output uses tabs, preserves object insertion order, and ends with exactly one newline\n",
    "  Byte-identical output leaves live settings untouched\n",
    "  Changed output is written to a create-new same-directory sibling, assigned live permissions, synced, and atomically renamed\n",
    "  Promotion rechecks live bytes immediately before rename on a best-effort basis. A writer can still race after that check\n",
    "  The live destination and every traversed component must not be a symlink\n",
    "\n",
    "Output never includes pattern bodies or candidate/settings contents\n",
    "\n",
    "Exit statuses:\n",
    "  0  Capture, verification, promotion, unchanged promotion, or help succeeded\n",
    "  1  Current state could not be uniquely reindexed or a guarded promotion was refused\n",
    "  2  Arguments or data were invalid, or an I/O operation failed\n",
);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SelectionDocument {
    version: u64,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StateDocument {
    version: u64,
    baseline_file: String,
    baseline_sha256: String,
    scopes: Vec<String>,
    patterns: Vec<StatePattern>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StatePattern {
    id: String,
    bucket: Bucket,
    source_index: usize,
    case_sensitive: bool,
    sha256: String,
    pattern_file: String,
}

struct CaptureArguments {
    settings: PathBuf,
    selection: PathBuf,
    output: PathBuf,
}

struct VerifyArguments {
    settings: PathBuf,
    state: PathBuf,
}

struct PromoteArguments {
    settings: PathBuf,
    candidate: PathBuf,
    state: PathBuf,
}

enum Operation {
    Capture(CaptureArguments),
    Verify(VerifyArguments),
    Promote(PromoteArguments),
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

struct LoadedPattern {
    id: String,
    bucket: Bucket,
    source_index: usize,
    case_sensitive: bool,
    bytes: Vec<u8>,
}

struct ValidatedState {
    baseline: Value,
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
            "Missing mode `capture`, `verify`, or `promote`. Run `zed-permission-candidate --help` for usage",
        ));
    };
    let Some(mode) = mode.to_str() else {
        return Err(invalid("The mode must be valid UTF-8"));
    };
    let options = &arguments[1..];

    match mode {
        "capture" => parse_capture_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Capture(arguments))),
        "verify" => parse_verify_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Verify(arguments))),
        "promote" => parse_promote_arguments(options)
            .map(|arguments| ParsedArguments::Run(Operation::Promote(arguments))),
        _ => Err(invalid(format!(
            "Unknown mode `{mode}`. Run `zed-permission-candidate --help` for usage"
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
                    "Unknown capture option `{option}`. Run `zed-permission-candidate --help` for usage"
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
                    "Unknown verify option `{option}`. Run `zed-permission-candidate --help` for usage"
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
    let mut candidate = None;
    let mut settings = None;
    let mut state = None;
    let mut write = false;
    let mut index = 0;

    while index < options.len() {
        let option = option_name(&options[index])?;
        match option {
            "--candidate" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut candidate, path, option)?;
            }
            "--settings" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut settings, path, option)?;
            }
            "--state" => {
                let path = take_path(options, &mut index, option)?;
                set_once(&mut state, path, option)?;
            }
            "--write" => {
                if write {
                    return Err(invalid("Option `--write` may be specified only once"));
                }
                write = true;
            }
            _ => {
                return Err(invalid(format!(
                    "Unknown promote option `{option}`. Run `zed-permission-candidate --help` for usage"
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
        settings: settings.ok_or_else(|| invalid("Missing required option `--settings <live>`"))?,
        candidate: candidate
            .ok_or_else(|| invalid("Missing required option `--candidate <candidate>`"))?,
        state: state.ok_or_else(|| invalid("Missing required option `--state <state.json>`"))?,
    })
}

fn read_bytes(path: &Path, description: &str) -> Result<Vec<u8>, AppError> {
    fs::read(path).map_err(|error| {
        invalid(format!(
            "Failed to read {description} `{}`: {error}",
            path.display()
        ))
    })
}

fn invalid_json(description: &str, path: &Path, error: serde_json::Error) -> AppError {
    let category = match error.classify() {
        serde_json::error::Category::Data => "does not match the required schema",
        serde_json::error::Category::Eof => "ends before a complete JSON value",
        serde_json::error::Category::Io => "could not be read as JSON",
        serde_json::error::Category::Syntax => "contains invalid JSON syntax",
    };

    invalid(format!(
        "Invalid {description} `{}` at line {}, column {}: {category}",
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

fn read_utf8_json_object(path: &Path, description: &str) -> Result<Value, AppError> {
    let contents = read_utf8_file(path, description).map_err(invalid)?;

    parse_json_object(contents.as_bytes(), description, path)
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

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push(HEX[usize::from(byte >> 4)] as char);
        output.push(HEX[usize::from(byte & 0x0f)] as char);
    }

    output
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
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
                PathInspectionError::Io(format!("Failed to resolve the current directory: {error}"))
            })
    }
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
                "Failed to inspect path component `{}`: {error}",
                current.display()
            ))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(PathInspectionError::Symlink(current));
        }
    }

    Ok(())
}

fn prepare_output_directory(output: &Path) -> Result<bool, AppError> {
    match fs::symlink_metadata(output) {
        Ok(metadata) => {
            ensure_no_symlink_components(output).map_err(|error| invalid(error.to_string()))?;
            if !metadata.is_dir() {
                return Err(invalid(format!(
                    "Capture output `{}` must be a real directory",
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
                    "Failed to create capture output directory `{}`: {error}",
                    output.display()
                ))
            })?;
            ensure_no_symlink_components(output).map_err(|error| invalid(error.to_string()))?;
            Ok(true)
        }
        Err(error) => Err(invalid(format!(
            "Failed to inspect capture output `{}`: {error}",
            output.display()
        ))),
    }
}

fn preflight_artifacts(output: &Path, filenames: &[String]) -> Result<(), AppError> {
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
                    "Refusing to overwrite existing capture artifact `{}`",
                    path.display()
                )));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to inspect capture artifact `{}`: {error}",
                    path.display()
                )));
            }
        }
    }

    Ok(())
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), AppError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            invalid(format!(
                "Failed to create artifact `{}` without overwrite: {error}",
                path.display()
            ))
        })?;

    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(invalid(format!(
            "Failed to write artifact `{}`: {error}",
            path.display()
        )));
    }

    Ok(())
}

pub(crate) fn serialize_pretty_json(value: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    value
        .serialize(&mut serializer)
        .map_err(|error| format!("Failed to serialize JSON: {error}"))?;
    bytes.push(b'\n');

    Ok(bytes)
}

fn serialize_state(state: &StateDocument) -> Result<Vec<u8>, AppError> {
    let value = serde_json::to_value(state)
        .map_err(|error| invalid(format!("Failed to serialize state manifest: {error}")))?;
    serialize_pretty_json(&value).map_err(invalid)
}

fn capture(arguments: &CaptureArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let (settings_bytes, settings) = read_json_object(&arguments.settings, "settings")?;
    let selection_contents =
        read_utf8_file(&arguments.selection, "selection JSON").map_err(invalid)?;
    let selection: SelectionDocument = serde_json::from_str(&selection_contents)
        .map_err(|error| invalid_json("selection JSON", &arguments.selection, error))?;
    if selection.version != VERSION {
        return Err(invalid("Selection JSON must use schema version 1"));
    }
    if selection.patterns.is_empty() {
        return Err(invalid("Selection JSON must contain at least one pattern"));
    }

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
        version: VERSION,
        baseline_file: BASELINE_FILE.to_owned(),
        baseline_sha256: sha256_hex(&settings_bytes),
        scopes: selection.scopes,
        patterns: captured
            .iter()
            .map(|pattern| pattern.state.clone())
            .collect(),
    };
    let state_bytes = serialize_state(&state)?;
    let output_created = prepare_output_directory(&arguments.output)?;
    let mut filenames = Vec::with_capacity(captured.len() + 3);
    filenames.push(BASELINE_FILE.to_owned());
    filenames.push(CANDIDATE_FILE.to_owned());
    filenames.extend(
        captured
            .iter()
            .map(|pattern| pattern.state.pattern_file.clone()),
    );
    filenames.push(STATE_FILE.to_owned());

    if let Err(error) = preflight_artifacts(&arguments.output, &filenames) {
        if output_created {
            let _ = fs::remove_dir(&arguments.output);
        }
        return Err(error);
    }

    let mut created = Vec::with_capacity(filenames.len());
    let write_result = (|| {
        let baseline_path = arguments.output.join(BASELINE_FILE);
        write_new_file(&baseline_path, &settings_bytes)?;
        created.push(baseline_path);

        let candidate_path = arguments.output.join(CANDIDATE_FILE);
        write_new_file(&candidate_path, &settings_bytes)?;
        created.push(candidate_path);

        for pattern in &captured {
            let path = arguments.output.join(&pattern.state.pattern_file);
            write_new_file(&path, &pattern.bytes)?;
            created.push(path);
        }

        let state_path = arguments.output.join(STATE_FILE);
        write_new_file(&state_path, &state_bytes)?;
        created.push(state_path);
        Ok(())
    })();

    if let Err(error) = write_result {
        for path in created.iter().rev() {
            let _ = fs::remove_file(path);
        }
        if output_created {
            let _ = fs::remove_dir(&arguments.output);
        }
        return Err(error);
    }

    writeln!(
        stdout,
        "Captured {} patterns in `{}`",
        captured.len(),
        arguments.output.display()
    )
    .map_err(|error| invalid(format!("Failed to write capture result: {error}")))?;
    writeln!(stdout, "  baseline -> {BASELINE_FILE}")
        .map_err(|error| invalid(format!("Failed to write capture result: {error}")))?;
    writeln!(stdout, "  candidate -> {CANDIDATE_FILE}")
        .map_err(|error| invalid(format!("Failed to write capture result: {error}")))?;
    for pattern in captured.iter().take(MAX_REPORTED_ITEMS) {
        writeln!(
            stdout,
            "  {} -> {}",
            display_id(&pattern.state.id),
            pattern.state.pattern_file
        )
        .map_err(|error| invalid(format!("Failed to write capture result: {error}")))?;
    }
    let omitted = captured.len().saturating_sub(MAX_REPORTED_ITEMS);
    if omitted > 0 {
        writeln!(stdout, "  … {omitted} additional pattern artifacts omitted")
            .map_err(|error| invalid(format!("Failed to write capture result: {error}")))?;
    }
    writeln!(stdout, "  state -> {STATE_FILE}")
        .map_err(|error| invalid(format!("Failed to write capture result: {error}")))?;

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
                "Failed to inspect {description} artifact `{}`: {error}",
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

fn read_state_document(path: &Path) -> Result<StateDocument, AppError> {
    ensure_no_symlink_components(path).map_err(|error| invalid(error.to_string()))?;
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        invalid(format!(
            "Failed to inspect state manifest `{}`: {error}",
            path.display()
        ))
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(invalid(format!(
            "State manifest `{}` must be a real regular file",
            path.display()
        )));
    }
    let contents = read_utf8_file(path, "state manifest").map_err(invalid)?;
    let document: StateDocument = serde_json::from_str(&contents)
        .map_err(|error| invalid_json("state manifest JSON", path, error))?;
    if document.version != VERSION {
        return Err(invalid("State manifest must use schema version 1"));
    }

    Ok(document)
}

fn validate_state(path: &Path) -> Result<ValidatedState, AppError> {
    let document = read_state_document(path)?;
    if document.patterns.is_empty() {
        return Err(invalid(
            "State manifest must contain at least one captured pattern",
        ));
    }
    if !valid_sha256(&document.baseline_sha256) {
        return Err(invalid(
            "State manifest baseline SHA-256 must be 64 lowercase hexadecimal characters",
        ));
    }

    let baseline_relative = validate_relative_artifact_path(&document.baseline_file)?;
    let baseline_bytes = read_state_artifact(path, &document.baseline_file, "baseline")?;
    if sha256_hex(&baseline_bytes) != document.baseline_sha256 {
        return Err(invalid("Baseline artifact SHA-256 does not match state"));
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
        if !valid_sha256(&state_pattern.sha256) {
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
                "Pattern artifact SHA-256 does not match state for ID `{}`",
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
            "Current settings could not uniquely reindex {} patterns: {missing} missing and {duplicate} duplicate",
            state.patterns.len()
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
        .map_err(|error| invalid(format!("Failed to write verification result: {error}")))?;
    }
    let omitted = moved.omitted_count();
    if omitted > 0 {
        writeln!(stdout, "… {omitted} additional moved mappings omitted")
            .map_err(|error| invalid(format!("Failed to write verification result: {error}")))?;
    }
    writeln!(
        stdout,
        "Verified {} patterns: {unchanged} unchanged and {} moved",
        state.patterns.len(),
        moved.total_count()
    )
    .map_err(|error| invalid(format!("Failed to write verification result: {error}")))?;

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

fn unique_temporary_sibling(destination: &Path) -> Result<(File, PathBuf), String> {
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    destination
        .file_name()
        .ok_or_else(|| "Live settings destination has no filename".to_owned())?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("System clock is before the Unix epoch: {error}"))?
        .as_nanos();

    for attempt in 0..100_u64 {
        let sequence = NEXT_TEMPORARY_ID.fetch_add(1, Ordering::Relaxed);
        let sibling_name = OsString::from(format!(
            ".zed-permission-candidate-{}-{timestamp}-{sequence}-{attempt}.tmp",
            process::id()
        ));
        let path = parent.join(sibling_name);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((file, path)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "Failed to create atomic sibling `{}`: {error}",
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
        .map_err(|error| format!("Failed to read live settings permissions: {error}"))?
        .permissions();
    let (mut file, temporary_path) = unique_temporary_sibling(destination)?;
    let temporary = TemporarySibling::new(temporary_path.clone());

    file.set_permissions(permissions)
        .map_err(|error| format!("Failed to copy live settings permissions: {error}"))?;
    file.write_all(bytes)
        .map_err(|error| format!("Failed to write atomic sibling: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("Failed to sync atomic sibling: {error}"))?;
    drop(file);
    before_rename(&temporary_path)
        .map_err(|error| format!("Atomic replacement was interrupted: {error}"))?;
    fs::rename(&temporary_path, destination)
        .map_err(|error| format!("Failed to atomically replace live settings: {error}"))?;
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
        Ok(())
    });

    match result {
        Ok(()) => Ok(()),
        Err(_) if changed => Err(BestEffortReplaceError::Changed),
        Err(error) => Err(BestEffortReplaceError::Invalid(error)),
    }
}

fn promote(arguments: &PromoteArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    let state = validate_state(&arguments.state)?;
    ensure_no_symlink_components(&arguments.settings).map_err(|error| {
        let message = error.to_string();
        match error {
            PathInspectionError::Io(_) => invalid(message),
            PathInspectionError::Symlink(_) => refused(message),
        }
    })?;
    let destination_metadata = fs::symlink_metadata(&arguments.settings).map_err(|error| {
        invalid(format!(
            "Failed to inspect live settings destination `{}`: {error}",
            arguments.settings.display()
        ))
    })?;
    if !destination_metadata.is_file() || destination_metadata.file_type().is_symlink() {
        return Err(refused(
            "Live settings destination must be a real regular file",
        ));
    }

    let (live_bytes, live) = read_json_object(&arguments.settings, "live settings")?;
    let candidate = read_utf8_json_object(&arguments.candidate, "candidate settings")?;

    for (index, tokens) in state.scopes.iter().enumerate() {
        let baseline_value = pointer_value(&state.baseline, tokens)
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

    let mut normalized_candidate = candidate.clone();
    for (index, tokens) in state.scopes.iter().enumerate() {
        let baseline_value = pointer_value(&state.baseline, tokens)
            .map_err(|_| invalid("Validated baseline scope became unavailable"))?
            .clone();
        replace_pointer_value(&mut normalized_candidate, tokens, baseline_value).map_err(|_| {
            refused(format!(
                "Promotion refused because candidate settings do not contain authorized scope {}",
                index + 1
            ))
        })?;
    }
    if !semantic_json_equal(&normalized_candidate, &state.baseline) {
        return Err(refused(
            "Promotion refused because candidate settings differ from the captured baseline outside authorized scopes",
        ));
    }

    let mut merged = live;
    for (index, tokens) in state.scopes.iter().enumerate() {
        let candidate_value = pointer_value(&candidate, tokens)
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
    if output == live_bytes {
        writeln!(
            stdout,
            "Live settings unchanged at `{}`",
            arguments.settings.display()
        )
        .map_err(|error| invalid(format!("Failed to write promotion result: {error}")))?;
        return Ok(());
    }

    match atomic_replace_with_best_effort_recheck(&arguments.settings, &output, &live_bytes, |_| {
        Ok(())
    }) {
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
        "Promoted {} authorized scopes into `{}`",
        state.document.scopes.len(),
        arguments.settings.display()
    )
    .map_err(|error| invalid(format!("Failed to write promotion result: {error}")))?;

    Ok(())
}

fn report_error(stderr: &mut dyn Write, error: &AppError) {
    let _ = writeln!(stderr, "zed-permission-candidate: {}", error.message());
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
                Err(invalid(format!("Failed to write help: {error}")))
            } else {
                Ok(())
            }
        }
        ParsedArguments::Run(Operation::Capture(arguments)) => capture(&arguments, stdout),
        ParsedArguments::Run(Operation::Verify(arguments)) => verify(&arguments, stdout),
        ParsedArguments::Run(Operation::Promote(arguments)) => promote(&arguments, stdout),
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
