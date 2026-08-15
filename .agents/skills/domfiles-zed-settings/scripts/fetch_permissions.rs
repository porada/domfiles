use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, HashSet},
    env,
    ffi::{OsStr, OsString},
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    net::IpAddr,
    path::{Component, Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

const BASELINE_DESCRIPTION: &str = "baseline settings";
const BUNDLE_FILE: &str = "fetch-validation.json";
const CANDIDATE_DESCRIPTION: &str = "candidate settings";
const EXACT_PREFIX: &str = "^(?i:https://";
const HOST_AND_SUBDOMAINS_PREFIX: &str = "^(?i:https://(?:[^./?#:@]+\\.)*";
const HOST_BOUNDARY_SUFFIX: &str = ")(?:[/?#]|$)";
const MAX_REPRESENTED_HOSTNAMES: usize = 256;

const PATTERN_FILE_PREFIX: &str = "fetch-pattern";
const STATE_DESCRIPTION: &str = "candidate state";
const STATUS_ERROR: u8 = 2;
const STATUS_REFUSED: u8 = 1;
const STATUS_SUCCESS: u8 = 0;
const SUBDOMAINS_PREFIX: &str = "^(?i:https://(?:[^./?#:@]+\\.)+";

static NEXT_TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

const HELP: &str = concat!(
    "Usage:\n",
    "  fetch-permissions apply --baseline <path> --candidate <path> --state <path> --output <directory> --coverage exact-hostname --hostname <hostname> --write\n",
    "  fetch-permissions apply --baseline <path> --candidate <path> --state <path> --output <directory> --coverage subdomains-only --hostname <hostname> --write\n",
    "  fetch-permissions apply --baseline <path> --candidate <path> --state <path> --output <directory> --coverage exact-hostname-plus-subdomains --hostname <hostname> --write\n",
    "  fetch-permissions apply --baseline <path> --candidate <path> --state <path> --output <directory> --coverage path-qualified-url --url-prefix <https-url-prefix> --write\n",
    "  fetch-permissions validate --baseline <path> --candidate <path> --state <path> --bundle <path>\n",
    "  fetch-permissions --help\n",
    "\n",
    "Prepare and validate one bounded Zed fetch-permission candidate without making a network request\n",
    "\n",
    "Hostname coverage adds persistent `network_hosts` grants. Each grant covers every port and becomes part of the sandbox network floor for later sandboxed terminal processes. Terminal commands still require their independent terminal permission\n",
    "\n",
    "Modes:\n",
    "  apply     Require a byte-identical captured candidate, add one canonical fetch allowance and its authorized sandbox hosts, validate the standard corpus, write bound artifacts, and atomically replace the candidate\n",
    "  validate  Rebuild the expected candidate from the captured baseline and verify the candidate, state binding, exact artifacts, ordering, alignment, and standard corpus\n",
    "\n",
    "Coverage inputs:\n",
    "  --coverage <coverage>               One of `exact-hostname`, `subdomains-only`, `exact-hostname-plus-subdomains`, or `path-qualified-url`\n",
    "  --hostname <hostname>               Lowercase ASCII is canonical. Case variants are normalized. IP literals, ports, userinfo, wildcards, and URL syntax are rejected\n",
    "  --url-prefix <https-url-prefix>     Credential-free canonical ASCII HTTPS path prefix ending in `/`. Use uppercase `%HH` escapes. Ports, queries, fragments, userinfo, encoded slashes, and dot segments use the generic fallback\n",
    "\n",
    "Artifact and mutation options:\n",
    "  --baseline <path>    Immutable `baseline-settings.json` from permission-candidate capture\n",
    "  --bundle <path>      Existing `fetch-validation.json` used by `validate`\n",
    "  --candidate <path>   Captured `candidate-settings.json`. `apply` requires it to equal the baseline before mutation\n",
    "  --output <directory> New explicit artifact directory used by `apply`. Existing paths are refused\n",
    "  --state <path>       Opaque `state.json` from the same capture. Its exact bytes are bound without interpreting its schema\n",
    "  --write              Required exact mutation guard for `apply`\n",
    "\n",
    "Output contract:\n",
    "  Added regex artifacts contain the exact decoded candidate pattern bytes with no newline, normalization, quoting, or reserialization\n",
    "  `fetch-validation.json` binds the exact baseline, candidate, and state bytes plus the bounded standard-corpus results\n",
    "  The output directory is created only when the complete plan is valid. A failed candidate replacement removes it\n",
    "  The candidate is replaced through a create-new same-directory sibling after a best-effort concurrent-write recheck\n",
    "  No mode reads live settings, changes `.config/zed/settings.json`, promotes a candidate, or contacts a destination\n",
    "\n",
    "Exit statuses:\n",
    "  0  Apply, validation, or help succeeded\n",
    "  1  The request was already covered, the candidate was not fresh, or guarded mutation was refused\n",
    "  2  Arguments, configuration, artifacts, or I/O were invalid\n",
);

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Bucket {
    #[serde(rename = "always_allow")]
    Allow,
    #[serde(rename = "always_confirm")]
    Confirm,
    #[serde(rename = "always_deny")]
    Deny,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Decision {
    Allow,
    Confirm,
    Deny,
}

impl Decision {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "allow" => Some(Self::Allow),
            "confirm" => Some(Self::Confirm),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HostScopeGroup {
    ExactOrPath,
    HostAndSubdomains,
    SubdomainsOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case", tag = "coverage")]
enum Request {
    ExactHostname { hostname: String },
    ExactHostnamePlusSubdomains { hostname: String },
    PathQualifiedUrl { url_prefix: String },
    SubdomainsOnly { hostname: String },
}

impl Request {
    fn validate_canonical(&self) -> Result<(), AppError> {
        let canonical = match self {
            Self::ExactHostname { hostname }
            | Self::ExactHostnamePlusSubdomains { hostname }
            | Self::SubdomainsOnly { hostname } => canonical_hostname(hostname)?,
            Self::PathQualifiedUrl { url_prefix } => canonical_url_prefix(url_prefix)?,
        };
        let input = match self {
            Self::ExactHostname { hostname }
            | Self::ExactHostnamePlusSubdomains { hostname }
            | Self::SubdomainsOnly { hostname } => hostname,
            Self::PathQualifiedUrl { url_prefix } => url_prefix,
        };
        if canonical != *input {
            return Err(invalid(
                "Fetch request inputs must use canonical fast-path syntax",
            ));
        }
        Ok(())
    }

    fn hostname(&self) -> &str {
        match self {
            Self::ExactHostname { hostname }
            | Self::ExactHostnamePlusSubdomains { hostname }
            | Self::SubdomainsOnly { hostname } => hostname,
            Self::PathQualifiedUrl { url_prefix } => {
                let rest = url_prefix
                    .strip_prefix("https://")
                    .expect("validated URL prefixes are canonical");
                rest.split_once('/')
                    .map(|(hostname, _)| hostname)
                    .expect("validated URL prefixes contain a path")
            }
        }
    }

    fn generated_pattern(&self) -> String {
        let hostname = escape_hostname(self.hostname());
        match self {
            Self::ExactHostname { .. } => {
                format!("^(?i:https://{hostname})(?:[/?#]|$)")
            }
            Self::ExactHostnamePlusSubdomains { .. } => {
                format!("^(?i:https://(?:[^./?#:@]+\\.)*{hostname})(?:[/?#]|$)")
            }
            Self::SubdomainsOnly { .. } => {
                format!("^(?i:https://(?:[^./?#:@]+\\.)+{hostname})(?:[/?#]|$)")
            }
            Self::PathQualifiedUrl { url_prefix } => {
                let rest = url_prefix
                    .strip_prefix("https://")
                    .expect("validated URL prefixes are canonical");
                let (_, path) = rest
                    .split_once('/')
                    .expect("validated URL prefixes contain a path");
                format!("^(?i:https://{hostname})/{}", escape_path(path))
            }
        }
    }

    fn required_network_hosts(&self) -> Vec<String> {
        match self {
            Self::ExactHostname { hostname } => vec![hostname.clone()],
            Self::ExactHostnamePlusSubdomains { hostname } => {
                vec![format!("*.{hostname}"), hostname.clone()]
            }
            Self::SubdomainsOnly { hostname } => vec![format!("*.{hostname}")],
            Self::PathQualifiedUrl { .. } => Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct RuleState {
    always_allow: bool,
    always_confirm: bool,
    always_deny: bool,
    final_decision: Decision,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct CaseRecord {
    baseline: RuleState,
    candidate: RuleState,
    expected_generated_match: bool,
    input: String,
    intended: bool,
    name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct PatternArtifact {
    pattern_file: String,
    sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ValidationBundle {
    added_network_hosts: Vec<String>,
    baseline_sha256: String,
    candidate_sha256: String,
    cases: Vec<CaseRecord>,
    pattern_artifacts: Vec<PatternArtifact>,
    request: Request,
    state_sha256: String,
}

#[derive(Clone)]
struct Rule {
    bucket: Bucket,
    pattern: String,
    regex: Regex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NetworkGrant {
    base: String,
    wildcard: bool,
}

#[derive(Clone)]
struct SettingsSnapshot {
    allow_patterns: Vec<Rule>,
    all_patterns: Vec<Rule>,
    default: Decision,
    network_hosts: Vec<NetworkGrant>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PatternSortKey {
    hostname: String,
}

struct PatternClassification {
    group: HostScopeGroup,
    host_boundary: bool,
    represented_hostnames: Vec<String>,
    sort_key: PatternSortKey,
}

struct StandardCase {
    expected_generated_match: bool,
    input: String,
    intended: bool,
    name: &'static str,
}

struct Plan {
    added_network_hosts: Vec<String>,
    added_patterns: Vec<String>,
    candidate_bytes: Vec<u8>,
    cases: Vec<CaseRecord>,
}

struct ApplyArguments {
    baseline: PathBuf,
    candidate: PathBuf,
    output: PathBuf,
    request: Request,
    state: PathBuf,
}

struct ValidateArguments {
    baseline: PathBuf,
    bundle: PathBuf,
    candidate: PathBuf,
    state: PathBuf,
}

enum Operation {
    Apply(ApplyArguments),
    Validate(ValidateArguments),
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

impl AppError {
    fn status(&self) -> u8 {
        match self {
            Self::Invalid(_) => STATUS_ERROR,
            Self::Refused(_) => STATUS_REFUSED,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) | Self::Refused(message) => formatter.write_str(message),
        }
    }
}

fn invalid(message: impl Into<String>) -> AppError {
    AppError::Invalid(message.into())
}

fn refused(message: impl Into<String>) -> AppError {
    AppError::Refused(message.into())
}

fn set_once<T>(slot: &mut Option<T>, value: T, option: &str) -> Result<(), AppError> {
    if slot.replace(value).is_some() {
        return Err(invalid(format!(
            "Option `{option}` may be specified only once"
        )));
    }
    Ok(())
}

fn next_value(
    arguments: &mut impl Iterator<Item = OsString>,
    option: &str,
) -> Result<OsString, AppError> {
    arguments
        .next()
        .ok_or_else(|| invalid(format!("Option `{option}` requires a value")))
}

fn utf8_option(value: OsString, option: &str) -> Result<String, AppError> {
    value
        .into_string()
        .map_err(|_| invalid(format!("Option `{option}` requires valid UTF-8")))
}

fn required_path(value: Option<PathBuf>, option: &str) -> Result<PathBuf, AppError> {
    value.ok_or_else(|| invalid(format!("Option `{option}` is required")))
}

fn parse_apply_arguments(arguments: Vec<OsString>) -> Result<ApplyArguments, AppError> {
    let mut baseline = None;
    let mut candidate = None;
    let mut coverage = None;
    let mut hostname = None;
    let mut output = None;
    let mut state = None;
    let mut url_prefix = None;
    let mut write = false;
    let mut arguments = arguments.into_iter();

    while let Some(argument) = arguments.next() {
        match argument.to_str() {
            Some("--baseline") => set_once(
                &mut baseline,
                PathBuf::from(next_value(&mut arguments, "--baseline")?),
                "--baseline",
            )?,
            Some("--candidate") => set_once(
                &mut candidate,
                PathBuf::from(next_value(&mut arguments, "--candidate")?),
                "--candidate",
            )?,
            Some("--coverage") => set_once(
                &mut coverage,
                utf8_option(next_value(&mut arguments, "--coverage")?, "--coverage")?,
                "--coverage",
            )?,
            Some("--hostname") => set_once(
                &mut hostname,
                utf8_option(next_value(&mut arguments, "--hostname")?, "--hostname")?,
                "--hostname",
            )?,
            Some("--output") => set_once(
                &mut output,
                PathBuf::from(next_value(&mut arguments, "--output")?),
                "--output",
            )?,
            Some("--state") => set_once(
                &mut state,
                PathBuf::from(next_value(&mut arguments, "--state")?),
                "--state",
            )?,
            Some("--url-prefix") => set_once(
                &mut url_prefix,
                utf8_option(next_value(&mut arguments, "--url-prefix")?, "--url-prefix")?,
                "--url-prefix",
            )?,
            Some("--write") if !write => write = true,
            Some("--write") => return Err(invalid("Option `--write` may be specified only once")),
            Some(_) => return Err(invalid("Apply received an unknown option")),
            None => return Err(invalid("Apply options must be valid UTF-8")),
        }
    }

    if !write {
        return Err(invalid("Apply requires the exact `--write` guard"));
    }
    let coverage = coverage.ok_or_else(|| invalid("Option `--coverage` is required"))?;
    let request = match coverage.as_str() {
        "exact-hostname" => Request::ExactHostname {
            hostname: canonical_hostname(
                hostname
                    .as_deref()
                    .ok_or_else(|| invalid("Coverage `exact-hostname` requires `--hostname`"))?,
            )?,
        },
        "exact-hostname-plus-subdomains" => Request::ExactHostnamePlusSubdomains {
            hostname: canonical_hostname(hostname.as_deref().ok_or_else(|| {
                invalid("Coverage `exact-hostname-plus-subdomains` requires `--hostname`")
            })?)?,
        },
        "subdomains-only" => Request::SubdomainsOnly {
            hostname: canonical_hostname(
                hostname
                    .as_deref()
                    .ok_or_else(|| invalid("Coverage `subdomains-only` requires `--hostname`"))?,
            )?,
        },
        "path-qualified-url" => Request::PathQualifiedUrl {
            url_prefix: canonical_url_prefix(url_prefix.as_deref().ok_or_else(|| {
                invalid("Coverage `path-qualified-url` requires `--url-prefix`")
            })?)?,
        },
        _ => {
            return Err(invalid(
                "Option `--coverage` must be `exact-hostname`, `subdomains-only`, `exact-hostname-plus-subdomains`, or `path-qualified-url`",
            ));
        }
    };

    match request {
        Request::PathQualifiedUrl { .. } if hostname.is_some() => {
            return Err(invalid(
                "Coverage `path-qualified-url` does not accept `--hostname`",
            ));
        }
        Request::PathQualifiedUrl { .. } => {}
        _ if url_prefix.is_some() => {
            return Err(invalid("Hostname coverage does not accept `--url-prefix`"));
        }
        _ => {}
    }

    Ok(ApplyArguments {
        baseline: required_path(baseline, "--baseline")?,
        candidate: required_path(candidate, "--candidate")?,
        output: required_path(output, "--output")?,
        request,
        state: required_path(state, "--state")?,
    })
}

fn parse_validate_arguments(arguments: Vec<OsString>) -> Result<ValidateArguments, AppError> {
    let mut baseline = None;
    let mut bundle = None;
    let mut candidate = None;
    let mut state = None;
    let mut arguments = arguments.into_iter();

    while let Some(argument) = arguments.next() {
        match argument.to_str() {
            Some("--baseline") => set_once(
                &mut baseline,
                PathBuf::from(next_value(&mut arguments, "--baseline")?),
                "--baseline",
            )?,
            Some("--bundle") => set_once(
                &mut bundle,
                PathBuf::from(next_value(&mut arguments, "--bundle")?),
                "--bundle",
            )?,
            Some("--candidate") => set_once(
                &mut candidate,
                PathBuf::from(next_value(&mut arguments, "--candidate")?),
                "--candidate",
            )?,
            Some("--state") => set_once(
                &mut state,
                PathBuf::from(next_value(&mut arguments, "--state")?),
                "--state",
            )?,
            Some(_) => return Err(invalid("Validate received an unknown option")),
            None => return Err(invalid("Validate options must be valid UTF-8")),
        }
    }

    Ok(ValidateArguments {
        baseline: required_path(baseline, "--baseline")?,
        bundle: required_path(bundle, "--bundle")?,
        candidate: required_path(candidate, "--candidate")?,
        state: required_path(state, "--state")?,
    })
}

fn parse_arguments<I>(arguments: I) -> Result<ParsedArguments, AppError>
where
    I: IntoIterator<Item = OsString>,
{
    let mut arguments = arguments.into_iter();
    let Some(operation) = arguments.next() else {
        return Err(invalid("A mode or `--help` is required"));
    };
    if operation == OsStr::new("--help") {
        if arguments.next().is_some() {
            return Err(invalid("Option `--help` must be used alone"));
        }
        return Ok(ParsedArguments::Help);
    }
    let remaining: Vec<OsString> = arguments.collect();
    match operation.to_str() {
        Some("apply") => Ok(ParsedArguments::Run(Operation::Apply(
            parse_apply_arguments(remaining)?,
        ))),
        Some("validate") => Ok(ParsedArguments::Run(Operation::Validate(
            parse_validate_arguments(remaining)?,
        ))),
        Some(_) => Err(invalid("An unknown mode was requested")),
        None => Err(invalid("The mode must be valid UTF-8")),
    }
}

fn canonical_hostname(input: &str) -> Result<String, AppError> {
    if input.is_empty() || input.trim() != input {
        return Err(invalid(
            "Hostname values must be nonempty and contain no surrounding whitespace",
        ));
    }
    if !input.is_ascii() {
        return Err(invalid(
            "Non-ASCII hostnames require the generic permission workflow",
        ));
    }
    if input.contains(['/', '\\', ':', '@', '*', '?', '#']) || input.ends_with('.') {
        return Err(invalid(
            "Hostname values must not contain URL syntax, ports, userinfo, wildcards, or a trailing dot",
        ));
    }
    let hostname = input.to_ascii_lowercase();
    if hostname.parse::<IpAddr>().is_ok()
        || hostname == "localhost"
        || hostname.ends_with(".localhost")
    {
        return Err(invalid(
            "IP literals and localhost names are outside hostname allowance scope",
        ));
    }
    if hostname.len() > 253 || !hostname.contains('.') {
        return Err(invalid(
            "Hostname values must be bounded, dot-qualified DNS names",
        ));
    }
    for label in hostname.split('.') {
        if label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
            || !label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            return Err(invalid(
                "Hostname values must use canonical ASCII DNS labels",
            ));
        }
    }
    Ok(hostname)
}

fn is_upper_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'A'..=b'F').contains(&byte)
}

fn decoded_dot_segment(segment: &[u8]) -> bool {
    let mut decoded = Vec::with_capacity(segment.len());
    let mut index = 0;
    while index < segment.len() {
        if segment[index] == b'%' {
            let high = segment[index + 1];
            let low = segment[index + 2];
            let value = |byte: u8| {
                if byte.is_ascii_digit() {
                    byte - b'0'
                } else {
                    byte - b'A' + 10
                }
            };
            decoded.push((value(high) << 4) | value(low));
            index += 3;
        } else {
            decoded.push(segment[index]);
            index += 1;
        }
    }
    decoded == b"." || decoded == b".."
}

fn canonical_url_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%' {
            if index + 2 >= bytes.len()
                || !is_upper_hex(bytes[index + 1])
                || !is_upper_hex(bytes[index + 2])
                || matches!(&bytes[index + 1..=index + 2], b"2F" | b"5C")
            {
                return false;
            }
            index += 3;
            continue;
        }
        if !(byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
                    | b':'
                    | b'@'
                    | b'/'
            ))
        {
            return false;
        }
        index += 1;
    }
    !bytes.split(|byte| *byte == b'/').any(decoded_dot_segment)
}

fn canonical_url_prefix(input: &str) -> Result<String, AppError> {
    if input.is_empty()
        || input.trim() != input
        || !input.is_ascii()
        || input.bytes().any(|byte| byte.is_ascii_whitespace())
    {
        return Err(invalid(
            "Path-qualified URL prefixes must be nonempty canonical ASCII",
        ));
    }
    let Some((scheme, rest)) = input.split_once("://") else {
        return Err(invalid("Path-qualified URL prefixes must use HTTPS"));
    };
    if !scheme.eq_ignore_ascii_case("https") {
        return Err(invalid("Path-qualified URL prefixes must use HTTPS"));
    }
    let Some((hostname, path)) = rest.split_once('/') else {
        return Err(invalid(
            "Path-qualified URL prefixes must include a non-root path ending in `/`",
        ));
    };
    if hostname.contains(['@', ':', '[', ']']) {
        return Err(invalid(
            "Ports, userinfo, and IP-literal URL authorities require the generic permission workflow",
        ));
    }
    let hostname = canonical_hostname(hostname)?;
    if path.is_empty()
        || !path.ends_with('/')
        || path.contains(['?', '#', '\\'])
        || !canonical_url_path(path)
    {
        return Err(invalid(
            "Path-qualified fast-path URLs must use canonical URI path bytes and uppercase percent escapes, end in `/`, and contain no query, fragment, encoded slash, backslash, or dot segment",
        ));
    }
    Ok(format!("https://{hostname}/{path}"))
}

fn escape_hostname(hostname: &str) -> String {
    hostname.replace('.', "\\.")
}

fn path_regex_metacharacter(character: char) -> bool {
    matches!(
        character,
        '\\' | '.' | '^' | '$' | '|' | '?' | '*' | '+' | '(' | ')' | '[' | ']' | '{' | '}'
    )
}

fn escape_path(path: &str) -> String {
    let mut escaped = String::with_capacity(path.len());
    for character in path.chars() {
        if path_regex_metacharacter(character) {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push(HEX[usize::from(byte >> 4)] as char);
        output.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    output
}

fn parse_json_object(bytes: &[u8], description: &str) -> Result<Value, AppError> {
    let value: Value = serde_json::from_slice(bytes).map_err(|error| {
        invalid(format!(
            "Failed to parse {description} JSON at line {}, column {}",
            error.line(),
            error.column()
        ))
    })?;
    if !value.is_object() {
        return Err(invalid(format!(
            "The {description} must contain a JSON object"
        )));
    }
    Ok(value)
}

fn object_member<'a>(
    value: &'a Value,
    keys: &[&str],
    description: &str,
) -> Result<&'a Value, AppError> {
    let mut current = value;
    for key in keys {
        current = current
            .as_object()
            .and_then(|object| object.get(*key))
            .ok_or_else(|| invalid(format!("Settings do not contain {description}")))?;
    }
    Ok(current)
}

fn object_member_mut<'a>(
    value: &'a mut Value,
    keys: &[&str],
    description: &str,
) -> Result<&'a mut Value, AppError> {
    let mut current = value;
    for key in keys {
        current = current
            .as_object_mut()
            .and_then(|object| object.get_mut(*key))
            .ok_or_else(|| invalid(format!("Settings do not contain {description}")))?;
    }
    Ok(current)
}

fn compile_rule(pattern: &str, case_sensitive: bool) -> Result<Regex, AppError> {
    if pattern.is_empty() {
        return Err(invalid("Fetch permission patterns must be nonempty"));
    }
    RegexBuilder::new(pattern)
        .case_insensitive(!case_sensitive)
        .build()
        .map_err(|_| invalid("A fetch permission pattern failed to compile"))
}

fn parse_rule_array(
    fetch: &Map<String, Value>,
    field: &str,
    bucket: Bucket,
) -> Result<Vec<Rule>, AppError> {
    let Some(value) = fetch.get(field) else {
        return Ok(Vec::new());
    };
    let array = value
        .as_array()
        .ok_or_else(|| invalid(format!("Fetch `{field}` must be an array")))?;
    let mut rules = Vec::with_capacity(array.len());
    let mut identities = HashSet::new();
    for entry in array {
        let object = entry
            .as_object()
            .ok_or_else(|| invalid(format!("Every fetch `{field}` entry must be an object")))?;
        if object.len() != 2
            || !object.contains_key("pattern")
            || !object.contains_key("case_sensitive")
        {
            return Err(invalid(format!(
                "Every fetch `{field}` entry must contain only `pattern` and `case_sensitive`"
            )));
        }
        let pattern = object
            .get("pattern")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid(format!("Every fetch `{field}` pattern must be a string")))?
            .to_owned();
        let case_sensitive = object
            .get("case_sensitive")
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                invalid(format!(
                    "Every fetch `{field}` `case_sensitive` value must be boolean"
                ))
            })?;
        if !identities.insert((pattern.clone(), case_sensitive)) {
            return Err(invalid(format!(
                "Fetch `{field}` contains a duplicate pattern object"
            )));
        }
        if bucket == Bucket::Allow && !case_sensitive {
            return Err(invalid(
                "Every automatically allowed fetch pattern must set `case_sensitive` to `true`",
            ));
        }
        rules.push(Rule {
            bucket,
            regex: compile_rule(&pattern, case_sensitive)?,
            pattern,
        });
    }
    Ok(rules)
}

fn parse_network_grant(value: &str) -> Result<NetworkGrant, AppError> {
    let (wildcard, base) = value
        .strip_prefix("*.")
        .map_or((false, value), |base| (true, base));
    let canonical = canonical_hostname(base)?;
    if canonical != base {
        return Err(invalid(
            "Sandbox network host entries must use lowercase canonical hostnames",
        ));
    }
    Ok(NetworkGrant {
        base: canonical,
        wildcard,
    })
}

fn parse_snapshot(settings: &Value) -> Result<SettingsSnapshot, AppError> {
    let fetch = object_member(
        settings,
        &["agent", "tool_permissions", "tools", "fetch"],
        "`agent.tool_permissions.tools.fetch`",
    )?
    .as_object()
    .ok_or_else(|| invalid("`agent.tool_permissions.tools.fetch` must be an object"))?;
    let default = fetch
        .get("default")
        .and_then(Value::as_str)
        .and_then(Decision::parse)
        .ok_or_else(|| invalid("Fetch `default` must be `allow`, `confirm`, or `deny`"))?;
    if default != Decision::Confirm {
        return Err(invalid("Fetch `default` must remain `confirm`"));
    }

    let allow_patterns = parse_rule_array(fetch, "always_allow", Bucket::Allow)?;
    let confirm_patterns = parse_rule_array(fetch, "always_confirm", Bucket::Confirm)?;
    let deny_patterns = parse_rule_array(fetch, "always_deny", Bucket::Deny)?;
    let mut all_patterns =
        Vec::with_capacity(allow_patterns.len() + confirm_patterns.len() + deny_patterns.len());
    all_patterns.extend(allow_patterns.iter().cloned());
    all_patterns.extend(confirm_patterns);
    all_patterns.extend(deny_patterns);

    let network_values = object_member(
        settings,
        &["agent", "sandbox_permissions", "network_hosts"],
        "`agent.sandbox_permissions.network_hosts`",
    )?
    .as_array()
    .ok_or_else(|| invalid("`agent.sandbox_permissions.network_hosts` must be an array"))?;
    let mut network_hosts = Vec::with_capacity(network_values.len());
    let mut identities = HashSet::new();
    for value in network_values {
        let value = value
            .as_str()
            .ok_or_else(|| invalid("Every sandbox network host must be a string"))?;
        let grant = parse_network_grant(value)?;
        if !identities.insert((grant.wildcard, grant.base.clone())) {
            return Err(invalid("Sandbox network hosts contain a duplicate entry"));
        }
        network_hosts.push(grant);
    }

    Ok(SettingsSnapshot {
        allow_patterns,
        all_patterns,
        default,
        network_hosts,
    })
}

fn concatenate_hostname_expansions(
    prefixes: &BTreeSet<String>,
    suffixes: &BTreeSet<String>,
) -> Option<BTreeSet<String>> {
    if prefixes.len().checked_mul(suffixes.len())? > MAX_REPRESENTED_HOSTNAMES {
        return None;
    }
    let mut combined = BTreeSet::new();
    for prefix in prefixes {
        for suffix in suffixes {
            let mut hostname = String::with_capacity(prefix.len() + suffix.len());
            hostname.push_str(prefix);
            hostname.push_str(suffix);
            if hostname.len() > 253 {
                return None;
            }
            combined.insert(hostname);
        }
    }
    Some(combined)
}

struct HostnameExpressionParser<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> HostnameExpressionParser<'a> {
    fn new(expression: &'a str) -> Self {
        Self {
            bytes: expression.as_bytes(),
            index: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    fn parse_alternatives(&mut self) -> Option<BTreeSet<String>> {
        let mut alternatives = BTreeSet::new();
        loop {
            alternatives.extend(self.parse_sequence()?);
            if alternatives.len() > MAX_REPRESENTED_HOSTNAMES {
                return None;
            }
            match self.peek() {
                Some(b'|') => self.index += 1,
                Some(b')') => return Some(alternatives),
                _ => return None,
            }
        }
    }

    fn parse_piece(&mut self) -> Option<BTreeSet<String>> {
        if self
            .bytes
            .get(self.index..self.index.saturating_add(3))
            .is_some_and(|prefix| prefix == b"(?:")
        {
            self.index += 3;
            let mut expansions = self.parse_alternatives()?;
            if self.peek() != Some(b')') {
                return None;
            }
            self.index += 1;
            if self.peek() == Some(b'?') {
                self.index += 1;
                expansions.insert(String::new());
            }
            return Some(expansions);
        }

        if self.peek() == Some(b'\\') {
            if self.bytes.get(self.index + 1) != Some(&b'.') {
                return None;
            }
            self.index += 2;
            return Some(BTreeSet::from([".".to_owned()]));
        }

        let start = self.index;
        while self
            .peek()
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            self.index += 1;
        }
        if self.index == start {
            return None;
        }
        let literal = std::str::from_utf8(&self.bytes[start..self.index])
            .ok()?
            .to_owned();
        Some(BTreeSet::from([literal]))
    }

    fn parse_sequence(&mut self) -> Option<BTreeSet<String>> {
        let mut expansions = BTreeSet::from([String::new()]);
        let mut found_piece = false;
        while !matches!(self.peek(), None | Some(b'|' | b')')) {
            expansions = concatenate_hostname_expansions(&expansions, &self.parse_piece()?)?;
            found_piece = true;
        }
        found_piece.then_some(expansions)
    }
}

fn represented_hostname_expression(expression: &str) -> Option<(Vec<String>, usize)> {
    let mut parser = HostnameExpressionParser::new(expression);
    let expansions = parser.parse_sequence()?;
    if parser.peek() != Some(b')') {
        return None;
    }
    let mut represented = BTreeSet::new();
    for hostname in expansions {
        represented.insert(canonical_hostname(&hostname).ok()?);
    }
    if represented.is_empty() || represented.len() > MAX_REPRESENTED_HOSTNAMES {
        return None;
    }
    Some((represented.into_iter().collect(), parser.index))
}

fn canonical_path_pattern_tail(tail: &str) -> bool {
    let Some(pattern_path) = tail.strip_prefix(")/") else {
        return false;
    };
    let mut path = String::with_capacity(pattern_path.len());
    let mut characters = pattern_path.chars();
    while let Some(character) = characters.next() {
        if character == '\\' {
            let Some(escaped) = characters.next() else {
                return false;
            };
            if !path_regex_metacharacter(escaped) {
                return false;
            }
            path.push(escaped);
        } else {
            if path_regex_metacharacter(character) {
                return false;
            }
            path.push(character);
        }
    }
    !path.is_empty()
        && path.ends_with('/')
        && !path.contains(['?', '#', '\\'])
        && canonical_url_path(&path)
        && escape_path(&path) == pattern_path
}

fn classify_pattern(rule: &Rule) -> Result<PatternClassification, AppError> {
    let (group, expression) = if let Some(expression) =
        rule.pattern.strip_prefix(HOST_AND_SUBDOMAINS_PREFIX)
    {
        (HostScopeGroup::HostAndSubdomains, expression)
    } else if let Some(expression) = rule.pattern.strip_prefix(SUBDOMAINS_PREFIX) {
        (HostScopeGroup::SubdomainsOnly, expression)
    } else if let Some(expression) = rule.pattern.strip_prefix(EXACT_PREFIX) {
        (HostScopeGroup::ExactOrPath, expression)
    } else {
        return Err(invalid(
            "A fetch allow pattern uses a grammar outside the fast path. Use the generic permission workflow",
        ));
    };
    let Some((represented_hostnames, tail_start)) = represented_hostname_expression(expression)
    else {
        return Err(invalid(
            "A fetch allow pattern uses a grammar outside the fast path. Use the generic permission workflow",
        ));
    };
    let tail = &expression[tail_start..];
    let host_boundary = tail == HOST_BOUNDARY_SUFFIX;
    if group != HostScopeGroup::ExactOrPath && !host_boundary {
        return Err(invalid(
            "A wildcard fetch pattern is not anchored at the hostname boundary",
        ));
    }
    if group == HostScopeGroup::ExactOrPath
        && !host_boundary
        && (represented_hostnames.len() != 1 || !canonical_path_pattern_tail(tail))
    {
        return Err(invalid(
            "A fetch allow pattern uses a grammar outside the fast path. Use the generic permission workflow",
        ));
    }

    let hostname = represented_hostnames
        .first()
        .expect("represented hostnames are nonempty")
        .clone();
    Ok(PatternClassification {
        group,
        host_boundary,
        represented_hostnames,
        sort_key: PatternSortKey { hostname },
    })
}

fn is_strict_subdomain(hostname: &str, parent: &str) -> bool {
    hostname
        .strip_suffix(parent)
        .and_then(|prefix| prefix.strip_suffix('.'))
        .is_some_and(|prefix| !prefix.is_empty())
}

fn network_covers_exact(grants: &[NetworkGrant], hostname: &str) -> bool {
    grants.iter().any(|grant| {
        if grant.wildcard {
            is_strict_subdomain(hostname, &grant.base)
        } else {
            grant.base == hostname
        }
    })
}

fn network_covers_wildcard(grants: &[NetworkGrant], hostname: &str) -> bool {
    grants.iter().any(|grant| {
        grant.wildcard && (grant.base == hostname || is_strict_subdomain(hostname, &grant.base))
    })
}

fn canonical_pattern_for_scope(group: HostScopeGroup, hostname: &str) -> String {
    let hostname = escape_hostname(hostname);
    match group {
        HostScopeGroup::ExactOrPath => {
            format!("^(?i:https://{hostname})(?:[/?#]|$)")
        }
        HostScopeGroup::HostAndSubdomains => {
            format!("^(?i:https://(?:[^./?#:@]+\\.)*{hostname})(?:[/?#]|$)")
        }
        HostScopeGroup::SubdomainsOnly => {
            format!("^(?i:https://(?:[^./?#:@]+\\.)+{hostname})(?:[/?#]|$)")
        }
    }
}

fn canonical_rule_scope(rule: &Rule) -> Option<(HostScopeGroup, String)> {
    let classification = classify_pattern(rule).ok()?;
    if !classification.host_boundary || classification.represented_hostnames.len() != 1 {
        return None;
    }
    let hostname = classification
        .represented_hostnames
        .into_iter()
        .next()
        .expect("one represented hostname is present");
    (rule.pattern == canonical_pattern_for_scope(classification.group, &hostname))
        .then_some((classification.group, hostname))
}

fn scope_covers_exact(group: HostScopeGroup, base: &str, hostname: &str) -> bool {
    match group {
        HostScopeGroup::ExactOrPath => base == hostname,
        HostScopeGroup::HostAndSubdomains => {
            base == hostname || is_strict_subdomain(hostname, base)
        }
        HostScopeGroup::SubdomainsOnly => is_strict_subdomain(hostname, base),
    }
}

fn scope_covers_subdomains(group: HostScopeGroup, base: &str, hostname: &str) -> bool {
    match group {
        HostScopeGroup::ExactOrPath => false,
        HostScopeGroup::HostAndSubdomains | HostScopeGroup::SubdomainsOnly => {
            base == hostname || is_strict_subdomain(hostname, base)
        }
    }
}

fn canonical_pattern_coverage(
    snapshot: &SettingsSnapshot,
    request: &Request,
    generated_pattern: &str,
) -> bool {
    if matches!(request, Request::PathQualifiedUrl { .. }) {
        return snapshot
            .allow_patterns
            .iter()
            .any(|rule| rule.pattern == generated_pattern);
    }
    let hostname = request.hostname();
    let scopes: Vec<(HostScopeGroup, String)> = snapshot
        .allow_patterns
        .iter()
        .filter_map(canonical_rule_scope)
        .collect();
    let exact_covered = scopes
        .iter()
        .any(|(group, base)| scope_covers_exact(*group, base, hostname));
    let subdomains_covered = scopes
        .iter()
        .any(|(group, base)| scope_covers_subdomains(*group, base, hostname));
    match request {
        Request::ExactHostname { .. } => exact_covered,
        Request::ExactHostnamePlusSubdomains { .. } => exact_covered && subdomains_covered,
        Request::SubdomainsOnly { .. } => subdomains_covered,
        Request::PathQualifiedUrl { .. } => unreachable!(),
    }
}

fn network_sort_key(grant: &NetworkGrant) -> (u8, &str) {
    (u8::from(!grant.wildcard), grant.base.as_str())
}

fn audit_snapshot(snapshot: &SettingsSnapshot) -> Result<(), AppError> {
    for pair in snapshot.network_hosts.windows(2) {
        if network_sort_key(&pair[0]) > network_sort_key(&pair[1]) {
            return Err(invalid(
                "Sandbox network hosts are not ordered by wildcard group and represented hostname",
            ));
        }
    }

    let mut previous = None;
    for rule in &snapshot.allow_patterns {
        let classification = classify_pattern(rule)?;
        if previous
            .as_ref()
            .is_some_and(|previous: &PatternSortKey| previous > &classification.sort_key)
        {
            return Err(invalid(
                "Fetch allow patterns are not ordered by represented hostname",
            ));
        }
        previous = Some(classification.sort_key.clone());

        if !classification.host_boundary {
            continue;
        }
        for hostname in &classification.represented_hostnames {
            let aligned = match classification.group {
                HostScopeGroup::HostAndSubdomains => {
                    network_covers_exact(&snapshot.network_hosts, hostname)
                        && network_covers_wildcard(&snapshot.network_hosts, hostname)
                }
                HostScopeGroup::SubdomainsOnly => {
                    network_covers_wildcard(&snapshot.network_hosts, hostname)
                }
                HostScopeGroup::ExactOrPath => {
                    network_covers_exact(&snapshot.network_hosts, hostname)
                }
            };
            if !aligned {
                return Err(invalid(
                    "A hostname-wide fetch allowance and sandbox network scope are misaligned",
                ));
            }
        }
    }

    for grant in &snapshot.network_hosts {
        let covered = if grant.wildcard {
            snapshot.allow_patterns.iter().any(|rule| {
                rule.regex
                    .is_match(&format!("https://probe.{}", grant.base))
                    && rule
                        .regex
                        .is_match(&format!("https://deep.probe.{}", grant.base))
            })
        } else {
            snapshot
                .allow_patterns
                .iter()
                .any(|rule| rule.regex.is_match(&format!("https://{}", grant.base)))
        };
        if !covered {
            return Err(invalid(
                "A sandbox network host has no corresponding hostname-wide fetch allowance",
            ));
        }
    }
    Ok(())
}

fn evaluate(input: &str, snapshot: &SettingsSnapshot) -> RuleState {
    let mut state = RuleState {
        always_allow: false,
        always_confirm: false,
        always_deny: false,
        final_decision: snapshot.default,
    };
    for rule in &snapshot.all_patterns {
        if !rule.regex.is_match(input) {
            continue;
        }
        match rule.bucket {
            Bucket::Allow => state.always_allow = true,
            Bucket::Confirm => state.always_confirm = true,
            Bucket::Deny => state.always_deny = true,
        }
    }
    state.final_decision = if state.always_deny {
        Decision::Deny
    } else if state.always_confirm {
        Decision::Confirm
    } else if state.always_allow {
        Decision::Allow
    } else {
        snapshot.default
    };
    state
}

fn uppercase_ascii(value: &str) -> String {
    value.to_ascii_uppercase()
}

fn path_case_variant(url_prefix: &str) -> Option<String> {
    let mut bytes = url_prefix.as_bytes().to_vec();
    let path_start = url_prefix
        .strip_prefix("https://")
        .and_then(|rest| rest.find('/').map(|index| index + "https://".len()))
        .expect("validated URL prefix contains a path");
    for byte in &mut bytes[path_start..] {
        if byte.is_ascii_lowercase() {
            *byte = byte.to_ascii_uppercase();
            break;
        }
        if byte.is_ascii_uppercase() {
            *byte = byte.to_ascii_lowercase();
            break;
        }
    }
    let variant = String::from_utf8(bytes).expect("validated URL prefix is ASCII");
    (variant != url_prefix).then_some(variant)
}

fn standard_cases(request: &Request) -> Vec<StandardCase> {
    let hostname = request.hostname();
    let upper_hostname = uppercase_ascii(hostname);
    match request {
        Request::ExactHostname { .. } => vec![
            StandardCase {
                name: "exact HTTPS apex",
                input: format!("https://{hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "scheme and hostname case variant",
                input: format!("HTTPS://{upper_hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "path at hostname boundary",
                input: format!("https://{hostname}/path"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "query at hostname boundary",
                input: format!("https://{hostname}?query=value"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "fragment at hostname boundary",
                input: format!("https://{hostname}#fragment"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "HTTP scheme",
                input: format!("http://{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "subdomain",
                input: format!("https://sub.{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "explicit port",
                input: format!("https://{hostname}:443"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "userinfo",
                input: format!("https://user@{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "lookalike broader hostname",
                input: format!("https://{hostname}.invalid"),
                expected_generated_match: false,
                intended: false,
            },
        ],
        Request::ExactHostnamePlusSubdomains { .. } => vec![
            StandardCase {
                name: "exact HTTPS apex",
                input: format!("https://{hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "one subdomain",
                input: format!("https://sub.{hostname}/path"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "multiple subdomain levels",
                input: format!("https://deep.sub.{hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "scheme and hostname case variant",
                input: format!("HTTPS://SUB.{upper_hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "HTTP scheme",
                input: format!("http://{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "explicit port",
                input: format!("https://{hostname}:443"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "userinfo",
                input: format!("https://user@{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "lookalike broader hostname",
                input: format!("https://{hostname}.invalid"),
                expected_generated_match: false,
                intended: false,
            },
        ],
        Request::SubdomainsOnly { .. } => vec![
            StandardCase {
                name: "one subdomain",
                input: format!("https://sub.{hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "multiple subdomain levels",
                input: format!("https://deep.sub.{hostname}/path"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "scheme and hostname case variant",
                input: format!("HTTPS://SUB.{upper_hostname}"),
                expected_generated_match: true,
                intended: true,
            },
            StandardCase {
                name: "apex",
                input: format!("https://{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "HTTP scheme",
                input: format!("http://sub.{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "explicit port",
                input: format!("https://sub.{hostname}:443"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "userinfo",
                input: format!("https://user@sub.{hostname}"),
                expected_generated_match: false,
                intended: false,
            },
            StandardCase {
                name: "lookalike broader hostname",
                input: format!("https://sub.{hostname}.invalid"),
                expected_generated_match: false,
                intended: false,
            },
        ],
        Request::PathQualifiedUrl { url_prefix } => {
            let rest = url_prefix.strip_prefix("https://").expect("canonical URL");
            let (_, path) = rest.split_once('/').expect("canonical URL has a path");
            let mut cases = vec![
                StandardCase {
                    name: "exact HTTPS path prefix",
                    input: url_prefix.clone(),
                    expected_generated_match: true,
                    intended: true,
                },
                StandardCase {
                    name: "path descendant",
                    input: format!("{url_prefix}child"),
                    expected_generated_match: true,
                    intended: true,
                },
                StandardCase {
                    name: "scheme and hostname case variant",
                    input: format!("HTTPS://{upper_hostname}/{path}"),
                    expected_generated_match: true,
                    intended: true,
                },
                StandardCase {
                    name: "HTTP scheme",
                    input: url_prefix.replacen("https://", "http://", 1),
                    expected_generated_match: false,
                    intended: false,
                },
                StandardCase {
                    name: "subdomain",
                    input: format!("https://sub.{hostname}/{path}"),
                    expected_generated_match: false,
                    intended: false,
                },
                StandardCase {
                    name: "explicit port",
                    input: format!("https://{hostname}:443/{path}"),
                    expected_generated_match: false,
                    intended: false,
                },
                StandardCase {
                    name: "userinfo",
                    input: format!("https://user@{hostname}/{path}"),
                    expected_generated_match: false,
                    intended: false,
                },
                StandardCase {
                    name: "sibling path",
                    input: format!("https://{hostname}/different-path/"),
                    expected_generated_match: false,
                    intended: false,
                },
                StandardCase {
                    name: "lookalike broader hostname",
                    input: format!("https://{hostname}.invalid/{path}"),
                    expected_generated_match: false,
                    intended: false,
                },
            ];
            if let Some(input) = path_case_variant(url_prefix) {
                cases.push(StandardCase {
                    name: "path case variant",
                    input,
                    expected_generated_match: false,
                    intended: false,
                });
            }
            cases
        }
    }
}

fn fetch_allow_array_mut(settings: &mut Value) -> Result<&mut Vec<Value>, AppError> {
    object_member_mut(
        settings,
        &[
            "agent",
            "tool_permissions",
            "tools",
            "fetch",
            "always_allow",
        ],
        "`agent.tool_permissions.tools.fetch.always_allow`",
    )?
    .as_array_mut()
    .ok_or_else(|| invalid("Fetch `always_allow` must be an array"))
}

fn network_hosts_array_mut(settings: &mut Value) -> Result<&mut Vec<Value>, AppError> {
    object_member_mut(
        settings,
        &["agent", "sandbox_permissions", "network_hosts"],
        "`agent.sandbox_permissions.network_hosts`",
    )?
    .as_array_mut()
    .ok_or_else(|| invalid("Sandbox `network_hosts` must be an array"))
}

fn pattern_object(pattern: String) -> Value {
    let mut object = Map::new();
    object.insert("pattern".to_owned(), Value::String(pattern));
    object.insert("case_sensitive".to_owned(), Value::Bool(true));
    Value::Object(object)
}

fn serialize_pretty_json<T: Serialize>(value: &T) -> Result<Vec<u8>, AppError> {
    let mut bytes = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    value
        .serialize(&mut serializer)
        .map_err(|error| invalid(format!("Failed to serialize JSON: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn build_plan(baseline: &Value, request: &Request) -> Result<Plan, AppError> {
    request.validate_canonical()?;
    let baseline_snapshot = parse_snapshot(baseline)?;
    audit_snapshot(&baseline_snapshot)?;
    let generated_pattern = request.generated_pattern();
    let generated_regex = compile_rule(&generated_pattern, true)?;
    let corpus = standard_cases(request);

    let corpus_covered = corpus.iter().filter(|case| case.intended).all(|case| {
        baseline_snapshot
            .allow_patterns
            .iter()
            .any(|rule| rule.regex.is_match(&case.input))
    });
    let pattern_covered =
        canonical_pattern_coverage(&baseline_snapshot, request, &generated_pattern);
    if corpus_covered && !pattern_covered {
        return Err(invalid(
            "Existing fetch patterns appear to cover the bounded corpus without a canonical structural proof. Use the generic permission workflow",
        ));
    }
    let mut added_network_hosts = Vec::new();
    for required in request.required_network_hosts() {
        let grant = parse_network_grant(&required)?;
        let covered = if grant.wildcard {
            network_covers_wildcard(&baseline_snapshot.network_hosts, &grant.base)
        } else {
            network_covers_exact(&baseline_snapshot.network_hosts, &grant.base)
        };
        if !covered {
            added_network_hosts.push(required);
        }
    }
    let mut added_patterns = Vec::new();
    if !pattern_covered {
        added_patterns.push(generated_pattern.clone());
    }
    if added_patterns.is_empty() && added_network_hosts.is_empty() {
        return Err(refused(
            "Existing allowances already provide the selected fetch and sandbox coverage",
        ));
    }

    let mut candidate = baseline.clone();
    if !added_patterns.is_empty() {
        let new_rule = Rule {
            bucket: Bucket::Allow,
            regex: generated_regex.clone(),
            pattern: generated_pattern.clone(),
        };
        let new_key = classify_pattern(&new_rule)?.sort_key;
        let existing_keys: Vec<PatternSortKey> = baseline_snapshot
            .allow_patterns
            .iter()
            .map(|rule| classify_pattern(rule).map(|classification| classification.sort_key))
            .collect::<Result<_, _>>()?;
        let position = existing_keys
            .iter()
            .position(|key| key > &new_key)
            .unwrap_or(existing_keys.len());
        fetch_allow_array_mut(&mut candidate)?.insert(position, pattern_object(generated_pattern));
    }
    if !added_network_hosts.is_empty() {
        let array = network_hosts_array_mut(&mut candidate)?;
        for host in &added_network_hosts {
            array.push(Value::String(host.clone()));
        }
        array.sort_by(|left, right| {
            let left = parse_network_grant(left.as_str().expect("validated network host"))
                .expect("validated network host");
            let right = parse_network_grant(right.as_str().expect("validated network host"))
                .expect("validated network host");
            network_sort_key(&left).cmp(&network_sort_key(&right))
        });
    }

    let candidate_snapshot = parse_snapshot(&candidate)?;
    audit_snapshot(&candidate_snapshot)?;
    let mut cases = Vec::with_capacity(corpus.len());
    for case in corpus {
        let generated_match = generated_regex.is_match(&case.input);
        if generated_match != case.expected_generated_match {
            return Err(invalid(format!(
                "Canonical pattern validation failed for standard case `{}`",
                case.name
            )));
        }
        let baseline_state = evaluate(&case.input, &baseline_snapshot);
        let candidate_state = evaluate(&case.input, &candidate_snapshot);
        if case.intended && candidate_state.final_decision != Decision::Allow {
            return Err(refused(format!(
                "Configured deny or confirm precedence prevents automatic allowance for standard case `{}`",
                case.name
            )));
        }
        if !case.intended && baseline_state != candidate_state {
            return Err(invalid(format!(
                "The candidate changes a boundary case outside the selected coverage: `{}`",
                case.name
            )));
        }
        cases.push(CaseRecord {
            baseline: baseline_state,
            candidate: candidate_state,
            expected_generated_match: case.expected_generated_match,
            input: case.input,
            intended: case.intended,
            name: case.name.to_owned(),
        });
    }

    Ok(Plan {
        added_network_hosts,
        added_patterns,
        candidate_bytes: serialize_pretty_json(&candidate)?,
        cases,
    })
}

fn path_as_absolute(path: &Path) -> Result<PathBuf, AppError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        env::current_dir()
            .map(|current| current.join(path))
            .map_err(|error| invalid(format!("Failed to resolve a path: {error}")))
    }
}

fn ensure_no_symlink_components(path: &Path) -> Result<(), AppError> {
    let absolute = path_as_absolute(path)?;
    let mut current = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
            Component::RootDir | Component::Normal(_) => current.push(component.as_os_str()),
            Component::CurDir => continue,
            Component::ParentDir => {
                return Err(invalid(
                    "Paths must not contain parent-directory components",
                ));
            }
        }
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(invalid("A supplied path traverses a symbolic link"));
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to inspect a supplied path: {error}"
                )));
            }
        }
    }
    Ok(())
}

fn read_regular_file(path: &Path, description: &str) -> Result<Vec<u8>, AppError> {
    ensure_no_symlink_components(path)?;
    let metadata = fs::metadata(path)
        .map_err(|error| invalid(format!("Failed to inspect {description}: {error}")))?;
    if !metadata.is_file() {
        return Err(invalid(format!("The {description} must be a regular file")));
    }
    fs::read(path).map_err(|error| invalid(format!("Failed to read {description}: {error}")))
}

fn files_alias(left: &Path, right: &Path) -> Result<bool, AppError> {
    ensure_no_symlink_components(left)?;
    ensure_no_symlink_components(right)?;
    let left_canonical = fs::canonicalize(left)
        .map_err(|error| invalid(format!("Failed to resolve a capture file: {error}")))?;
    let right_canonical = fs::canonicalize(right)
        .map_err(|error| invalid(format!("Failed to resolve a capture file: {error}")))?;
    if left_canonical == right_canonical {
        return Ok(true);
    }
    #[cfg(unix)]
    {
        let left_metadata = fs::metadata(&left_canonical)
            .map_err(|error| invalid(format!("Failed to inspect a capture file: {error}")))?;
        let right_metadata = fs::metadata(&right_canonical)
            .map_err(|error| invalid(format!("Failed to inspect a capture file: {error}")))?;
        Ok(left_metadata.dev() == right_metadata.dev()
            && left_metadata.ino() == right_metadata.ino())
    }
    #[cfg(not(unix))]
    Ok(false)
}

fn ensure_distinct_capture_files(
    baseline: &Path,
    candidate: &Path,
    state: &Path,
) -> Result<(), AppError> {
    if files_alias(baseline, candidate)?
        || files_alias(baseline, state)?
        || files_alias(candidate, state)?
    {
        return Err(refused(
            "Baseline, candidate, and state must be three distinct regular files",
        ));
    }
    Ok(())
}

fn validate_artifact_path(path: &str) -> Result<PathBuf, AppError> {
    let path = PathBuf::from(path);
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(invalid(
            "Pattern artifact paths must be nonempty safe relative paths",
        ));
    }
    Ok(path)
}

fn prepare_output(
    output: &Path,
    baseline_bytes: &[u8],
    candidate_bytes: &[u8],
    state_bytes: &[u8],
    request: &Request,
    plan: &Plan,
) -> Result<PathBuf, AppError> {
    if output.exists() {
        return Err(refused("The output path already exists"));
    }
    let parent = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| invalid("The output directory must have an existing parent"))?;
    ensure_no_symlink_components(parent)?;
    if !fs::metadata(parent).is_ok_and(|metadata| metadata.is_dir()) {
        return Err(invalid(
            "The output directory parent must exist and be a directory",
        ));
    }
    fs::create_dir(output)
        .map_err(|error| invalid(format!("Failed to create the output directory: {error}")))?;

    let result = (|| {
        let mut pattern_artifacts = Vec::with_capacity(plan.added_patterns.len());
        for (index, pattern) in plan.added_patterns.iter().enumerate() {
            let filename = format!("{PATTERN_FILE_PREFIX}-{:02}.regex", index + 1);
            let path = output.join(&filename);
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .map_err(|error| {
                    invalid(format!("Failed to create a pattern artifact: {error}"))
                })?;
            file.write_all(pattern.as_bytes())
                .map_err(|error| invalid(format!("Failed to write a pattern artifact: {error}")))?;
            file.sync_all()
                .map_err(|error| invalid(format!("Failed to sync a pattern artifact: {error}")))?;
            pattern_artifacts.push(PatternArtifact {
                pattern_file: filename,
                sha256: sha256_hex(pattern.as_bytes()),
            });
        }
        let bundle = ValidationBundle {
            added_network_hosts: plan.added_network_hosts.clone(),
            baseline_sha256: sha256_hex(baseline_bytes),
            candidate_sha256: sha256_hex(candidate_bytes),
            cases: plan.cases.clone(),
            pattern_artifacts,
            request: request.clone(),
            state_sha256: sha256_hex(state_bytes),
        };
        let bundle_bytes = serialize_pretty_json(&bundle)?;
        let bundle_path = output.join(BUNDLE_FILE);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&bundle_path)
            .map_err(|error| invalid(format!("Failed to create the validation bundle: {error}")))?;
        file.write_all(&bundle_bytes)
            .map_err(|error| invalid(format!("Failed to write the validation bundle: {error}")))?;
        file.sync_all()
            .map_err(|error| invalid(format!("Failed to sync the validation bundle: {error}")))?;
        Ok(bundle_path)
    })();

    if result.is_err() {
        let _ = fs::remove_dir_all(output);
    }
    result
}

fn unique_temporary_sibling(destination: &Path) -> Result<(File, PathBuf), AppError> {
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| invalid("The candidate path must have a parent directory"))?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| invalid("The system clock is before the Unix epoch"))?
        .as_nanos();
    for attempt in 0..100_u64 {
        let sequence = NEXT_TEMPORARY_ID.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(
            ".fetch-permissions-{}-{timestamp}-{sequence}-{attempt}.tmp",
            process::id()
        ));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((file, path)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(invalid(format!(
                    "Failed to create an atomic candidate sibling: {error}"
                )));
            }
        }
    }
    Err(invalid(
        "Failed to allocate an atomic candidate sibling after 100 attempts",
    ))
}

fn atomic_replace_candidate<F>(
    destination: &Path,
    bytes: &[u8],
    expected_bytes: &[u8],
    before_recheck: F,
) -> Result<(), AppError>
where
    F: FnOnce(&Path) -> io::Result<()>,
{
    ensure_no_symlink_components(destination)?;
    let permissions = fs::metadata(destination)
        .map_err(|error| invalid(format!("Failed to inspect the candidate: {error}")))?
        .permissions();
    let (mut file, temporary_path) = unique_temporary_sibling(destination)?;
    let result = (|| {
        file.set_permissions(permissions)
            .map_err(|error| invalid(format!("Failed to set candidate permissions: {error}")))?;
        file.write_all(bytes)
            .map_err(|error| invalid(format!("Failed to write the candidate sibling: {error}")))?;
        file.sync_all()
            .map_err(|error| invalid(format!("Failed to sync the candidate sibling: {error}")))?;
        before_recheck(&temporary_path)
            .map_err(|error| invalid(format!("Failed before the candidate recheck: {error}")))?;
        let current = fs::read(destination)
            .map_err(|error| invalid(format!("Failed to recheck the candidate: {error}")))?;
        if current != expected_bytes {
            return Err(refused(
                "Candidate replacement refused because the candidate changed concurrently",
            ));
        }
        fs::rename(&temporary_path, destination).map_err(|error| {
            invalid(format!(
                "Failed to atomically replace the candidate: {error}"
            ))
        })?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    result
}

fn apply_with_hook<F>(
    arguments: &ApplyArguments,
    stdout: &mut dyn Write,
    before_recheck: F,
) -> Result<(), AppError>
where
    F: FnOnce(&Path) -> io::Result<()>,
{
    ensure_distinct_capture_files(&arguments.baseline, &arguments.candidate, &arguments.state)?;
    let baseline_bytes = read_regular_file(&arguments.baseline, BASELINE_DESCRIPTION)?;
    let candidate_bytes = read_regular_file(&arguments.candidate, CANDIDATE_DESCRIPTION)?;
    let state_bytes = read_regular_file(&arguments.state, STATE_DESCRIPTION)?;
    if baseline_bytes != candidate_bytes {
        return Err(refused(
            "Apply requires a candidate that is byte-identical to its captured baseline",
        ));
    }
    let baseline = parse_json_object(&baseline_bytes, BASELINE_DESCRIPTION)?;
    let plan = build_plan(&baseline, &arguments.request)?;
    let bundle_path = prepare_output(
        &arguments.output,
        &baseline_bytes,
        &plan.candidate_bytes,
        &state_bytes,
        &arguments.request,
        &plan,
    )?;
    if let Err(error) = atomic_replace_candidate(
        &arguments.candidate,
        &plan.candidate_bytes,
        &candidate_bytes,
        before_recheck,
    ) {
        let _ = fs::remove_dir_all(&arguments.output);
        return Err(error);
    }
    writeln!(
        stdout,
        "Prepared fetch candidate with {} added pattern(s), {} added sandbox host(s), and {} verified standard case(s). Validation bundle: {}",
        plan.added_patterns.len(),
        plan.added_network_hosts.len(),
        plan.cases.len(),
        bundle_path.display()
    )
    .map_err(|error| invalid(format!("Failed to write apply output: {error}")))?;
    Ok(())
}

fn apply(arguments: &ApplyArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    apply_with_hook(arguments, stdout, |_| Ok(()))
}

#[cfg(test)]
pub(crate) fn run_apply_with_hook<I, F>(
    arguments: I,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    before_recheck: F,
) -> u8
where
    I: IntoIterator<Item = OsString>,
    F: FnOnce(&Path) -> io::Result<()>,
{
    let parsed = match parse_arguments(arguments) {
        Ok(ParsedArguments::Run(Operation::Apply(arguments))) => arguments,
        Ok(_) => {
            report_error(stderr, &invalid("The test hook requires apply mode"));
            return STATUS_ERROR;
        }
        Err(error) => {
            report_error(stderr, &error);
            return error.status();
        }
    };
    match apply_with_hook(&parsed, stdout, before_recheck) {
        Ok(()) => STATUS_SUCCESS,
        Err(error) => {
            report_error(stderr, &error);
            error.status()
        }
    }
}

fn parse_bundle(bytes: &[u8]) -> Result<ValidationBundle, AppError> {
    serde_json::from_slice(bytes).map_err(|error| {
        invalid(format!(
            "Failed to parse the validation bundle at line {}, column {}",
            error.line(),
            error.column()
        ))
    })
}

fn validate(arguments: &ValidateArguments, stdout: &mut dyn Write) -> Result<(), AppError> {
    ensure_distinct_capture_files(&arguments.baseline, &arguments.candidate, &arguments.state)?;
    let baseline_bytes = read_regular_file(&arguments.baseline, BASELINE_DESCRIPTION)?;
    let candidate_bytes = read_regular_file(&arguments.candidate, CANDIDATE_DESCRIPTION)?;
    let state_bytes = read_regular_file(&arguments.state, STATE_DESCRIPTION)?;
    let bundle_bytes = read_regular_file(&arguments.bundle, "validation bundle")?;
    let bundle = parse_bundle(&bundle_bytes)?;

    if bundle.baseline_sha256 != sha256_hex(&baseline_bytes)
        || bundle.candidate_sha256 != sha256_hex(&candidate_bytes)
        || bundle.state_sha256 != sha256_hex(&state_bytes)
    {
        return Err(refused(
            "The validation bundle does not bind the supplied baseline, candidate, and state bytes",
        ));
    }
    let baseline = parse_json_object(&baseline_bytes, BASELINE_DESCRIPTION)?;
    let plan = build_plan(&baseline, &bundle.request)?;
    if plan.candidate_bytes != candidate_bytes {
        return Err(refused(
            "The candidate bytes do not equal the deterministic fetch fast-path result",
        ));
    }
    if bundle.added_network_hosts != plan.added_network_hosts || bundle.cases != plan.cases {
        return Err(invalid(
            "The validation bundle does not match the reconstructed fetch plan",
        ));
    }
    if bundle.pattern_artifacts.len() != plan.added_patterns.len() {
        return Err(invalid(
            "The validation bundle pattern artifact count is incorrect",
        ));
    }
    let bundle_parent = arguments
        .bundle
        .parent()
        .ok_or_else(|| invalid("The validation bundle must have a parent directory"))?;
    let mut seen_paths = HashSet::new();
    for (artifact, pattern) in bundle
        .pattern_artifacts
        .iter()
        .zip(plan.added_patterns.iter())
    {
        let relative = validate_artifact_path(&artifact.pattern_file)?;
        if !seen_paths.insert(relative.clone()) {
            return Err(invalid("Pattern artifact paths must be unique"));
        }
        let bytes = read_regular_file(&bundle_parent.join(relative), "pattern artifact")?;
        if bytes != pattern.as_bytes() || artifact.sha256 != sha256_hex(&bytes) {
            return Err(invalid(
                "A pattern artifact contains bytes outside its exact candidate pattern binding",
            ));
        }
    }

    writeln!(
        stdout,
        "Validated fetch candidate with {} pattern artifact(s), {} sandbox host addition(s), and {} standard case(s)",
        bundle.pattern_artifacts.len(),
        bundle.added_network_hosts.len(),
        bundle.cases.len()
    )
    .map_err(|error| invalid(format!("Failed to write validation output: {error}")))?;
    Ok(())
}

fn report_error(stderr: &mut dyn Write, error: &AppError) {
    let _ = writeln!(stderr, "{error}");
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
    match parsed {
        ParsedArguments::Help => {
            if let Err(error) = stdout.write_all(HELP.as_bytes()) {
                report_error(
                    stderr,
                    &invalid(format!("Failed to write help output: {error}")),
                );
                STATUS_ERROR
            } else {
                STATUS_SUCCESS
            }
        }
        ParsedArguments::Run(operation) => {
            let result = match operation {
                Operation::Apply(arguments) => apply(&arguments, stdout),
                Operation::Validate(arguments) => validate(&arguments, stdout),
            };
            match result {
                Ok(()) => STATUS_SUCCESS,
                Err(error) => {
                    report_error(stderr, &error);
                    error.status()
                }
            }
        }
    }
}

fn main() {
    let status = run(env::args_os().skip(1), &mut io::stdout(), &mut io::stderr());
    process::exit(i32::from(status));
}
