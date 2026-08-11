#[allow(dead_code)]
#[path = "helpers/permission-patterns.rs"]
mod permission_patterns;

use permission_patterns::{
    BoundedIssues, Bucket, PatternError, compile_pattern, read_utf8_file, regex_error_summary,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    io::Write,
    path::PathBuf,
};

const HELP: &str = concat!(
    "Usage: zed-permission-owner-audit --settings <settings.json> --manifest <manifest.json>\n",
    "       zed-permission-owner-audit --settings <settings.json> --owner <top-level-executable>\n",
    "\n",
    "Audit declared terminal permission owners, roles, ordering, and finite discovery coverage,\n",
    "or list bounded terminal pattern inventory candidates by top-level executable token.\n",
    "\n",
    "Options:\n",
    "  --help                 Print help. This option must be used alone\n",
    "  --manifest <path>      Read the version-1 audit manifest\n",
    "  --owner <token>        Inventory one top-level executable token. Mutually exclusive\n",
    "                         with --manifest. The token must match [A-Za-z0-9_.+-]+\n",
    "  --settings <path>      Read Zed settings containing terminal permission buckets\n",
    "\n",
    "Version-1 manifest schema (unknown fields are rejected):\n",
    "  {\n",
    "    \"version\": 1,\n",
    "    \"entries\": [\n",
    "      {\n",
    "        \"id\": \"<unique nonempty ID>\",\n",
    "        \"bucket\": \"always_allow|always_confirm|always_deny\",\n",
    "        \"index\": 0,\n",
    "        \"owner\": \"<semantic owner>\",\n",
    "        \"owner_sort_key\": \"<top-level owner key>\",\n",
    "        \"section_sort_key\": \"<domain section key>\",\n",
    "        \"role\": \"discovery|direct|wrapped\",\n",
    "        \"pattern_sort_key\": \"<stable role-local key>\",\n",
    "        \"witness\": \"<one normalized permission input>\",\n",
    "        \"case_insensitive_reason\": \"<verified command-specific requirement>\",\n",
    "        \"discovery_coverage\": \"complete_finite|representative\",\n",
    "        \"discovery_inputs\": [\"<normalized discovery input>\"]\n",
    "      }\n",
    "    ]\n",
    "  }\n",
    "\n",
    "Manifest contract:\n",
    "  Omit discovery_coverage and discovery_inputs for direct and wrapped entries. Discovery\n",
    "  entries require a coverage kind and nonempty inputs containing witness. complete_finite\n",
    "  claims the complete finite grammar and enables duplicate-coverage findings. representative\n",
    "  records bounded cases for variable grammar and leaves complete coverage to matcher suites.\n",
    "  IDs and bucket/index pairs must be unique. Sort tuples must be unique within a bucket.\n",
    "  Omit case_insensitive_reason for case-sensitive patterns. A selected case-insensitive\n",
    "  pattern requires a nonempty reason recording its verified command-specific exception.\n",
    "  The manifest must declare complete owner groups for every group it audits.\n",
    "  Semantic owners name the independently inferred executable or manager, for example\n",
    "  npm, git:hash-object, or git:root. Section keys place domain sections such as Git\n",
    "  root/discovery, direct-subcommand, and compound-workflow groups.\n",
    "  Actual bucket order is checked by (owner_sort_key, section_sort_key, role order\n",
    "  discovery/direct/wrapped, owner, pattern_sort_key).\n",
    "\n",
    "Inventory contract:\n",
    "  Inventory inspects regex source text without compiling permission patterns. Owner\n",
    "  occurrences must have source-text token boundaries. Matches are inventory candidates,\n",
    "  not semantic owner proof. At most 100 hits are printed. Each hit contains only its\n",
    "  bucket/index, decoded character count, required boolean case_sensitive setting, and\n",
    "  a preview of at most 160 Unicode scalar values beginning at the owner token.\n",
    "\n",
    "Output:\n",
    "  A successful manifest audit prints entry, owner-group, and bucket counts only. Audit\n",
    "  findings report at most 10 entry IDs with concise reasons. Inventory prints bounded\n",
    "  source-text candidates, an omitted count when needed, and an exact total. Regex bodies\n",
    "  and witness inputs are never printed as complete fields.\n",
    "\n",
    "Exit statuses:\n",
    "  0  Audit passed, inventory completed including zero hits, or help displayed\n",
    "  1  Manifest audit findings were reported\n",
    "  2  Invalid arguments or data, or an I/O failure\n",
);

const MAX_INVENTORY_HITS: usize = 100;
const MAX_PATTERN_CHARACTERS: usize = 999;
const MAX_PREVIEW_CHARACTERS: usize = 160;
const MAX_REPORTED_FINDINGS: usize = 10;
const STATUS_ERROR: u8 = 2;
const STATUS_FINDINGS: u8 = 1;
const STATUS_SUCCESS: u8 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Role {
    Discovery,
    Direct,
    Wrapped,
}

impl Role {
    fn label(self) -> &'static str {
        match self {
            Self::Discovery => "discovery",
            Self::Direct => "direct",
            Self::Wrapped => "wrapped",
        }
    }

    fn order(self) -> u8 {
        match self {
            Self::Discovery => 0,
            Self::Direct => 1,
            Self::Wrapped => 2,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u64,
    entries: Vec<ManifestEntry>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestEntry {
    id: String,
    bucket: Bucket,
    index: usize,
    owner: String,
    owner_sort_key: String,
    section_sort_key: String,
    role: Role,
    pattern_sort_key: String,
    witness: String,
    case_insensitive_reason: Option<String>,
    discovery_coverage: Option<DiscoveryCoverage>,
    discovery_inputs: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum DiscoveryCoverage {
    CompleteFinite,
    Representative,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SortTuple {
    owner_sort_key: String,
    section_sort_key: String,
    role_order: u8,
    owner: String,
    pattern_sort_key: String,
}

impl SortTuple {
    fn from_entry(entry: &ManifestEntry) -> Self {
        Self {
            owner_sort_key: entry.owner_sort_key.clone(),
            section_sort_key: entry.section_sort_key.clone(),
            role_order: entry.role.order(),
            owner: entry.owner.clone(),
            pattern_sort_key: entry.pattern_sort_key.clone(),
        }
    }
}

enum Operation {
    Inventory { owner: String },
    ManifestAudit { manifest: PathBuf },
}

struct Arguments {
    operation: Operation,
    settings: PathBuf,
}

enum ParsedArguments {
    Help,
    Run(Arguments),
}

struct SelectedEntry {
    case_sensitive: bool,
    declaration: ManifestEntry,
    finding_key: usize,
    pattern: String,
    regex: Option<Regex>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InferredOwner {
    pub(crate) owner: String,
    pub(crate) role: Role,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Finding {
    pub(crate) id: String,
    pub(crate) reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuditReport {
    pub(crate) bucket_count: usize,
    pub(crate) entry_count: usize,
    pub(crate) finding_count: usize,
    pub(crate) findings: Vec<Finding>,
    pub(crate) owner_group_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryHit {
    case_sensitive: bool,
    character_count: usize,
    id: String,
    preview: String,
}

fn validate_owner(owner: &str) -> Result<(), String> {
    if !owner.is_empty()
        && owner.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.' | '_')
        })
    {
        return Ok(());
    }

    Err("Option `--owner` requires a nonempty token matching `[A-Za-z0-9_.+-]+`".to_owned())
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
    let mut manifest = None;
    let mut owner = None;
    let mut settings = None;

    while let Some(argument) = arguments.next() {
        let Some(option) = argument.to_str() else {
            return Err("Option names must be valid UTF-8".to_owned());
        };

        match option {
            "--help" => return Err("Option `--help` must be used alone".to_owned()),
            "--manifest" => {
                if manifest.is_some() {
                    return Err("Option `--manifest` may be specified only once".to_owned());
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--manifest` requires a path".to_owned());
                };
                manifest = Some(PathBuf::from(path));
            }
            "--owner" => {
                if owner.is_some() {
                    return Err("Option `--owner` may be specified only once".to_owned());
                }
                let Some(value) = arguments.next() else {
                    return Err("Option `--owner` requires a token".to_owned());
                };
                let value = value
                    .into_string()
                    .map_err(|_| "Option `--owner` must be valid UTF-8".to_owned())?;
                validate_owner(&value)?;
                owner = Some(value);
            }
            "--settings" => {
                if settings.is_some() {
                    return Err("Option `--settings` may be specified only once".to_owned());
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--settings` requires a path".to_owned());
                };
                settings = Some(PathBuf::from(path));
            }
            _ => {
                return Err(format!(
                    "Unknown option `{option}`. Run `zed-permission-owner-audit --help` for usage"
                ));
            }
        }
    }

    let settings = settings
        .ok_or_else(|| "Missing required option `--settings <settings.json>`".to_owned())?;
    let operation = match (manifest, owner) {
        (Some(_), Some(_)) => {
            return Err("Options `--manifest` and `--owner` are mutually exclusive".to_owned());
        }
        (Some(manifest), None) => Operation::ManifestAudit { manifest },
        (None, Some(owner)) => Operation::Inventory { owner },
        (None, None) => {
            return Err(
                "Missing required operation `--manifest <manifest.json>` or `--owner <top-level-executable>`"
                    .to_owned(),
            );
        }
    };

    Ok(ParsedArguments::Run(Arguments {
        operation,
        settings,
    }))
}

fn validate_manifest(manifest: &Manifest) -> Result<(), String> {
    if manifest.version != 1 {
        return Err(format!(
            "Unsupported manifest version `{}`. Expected version `1`",
            manifest.version
        ));
    }

    let mut ids = BTreeSet::new();
    let mut positions = BTreeMap::new();
    let mut sort_tuples = BTreeMap::new();

    for entry in &manifest.entries {
        if entry.id.is_empty() {
            return Err("Manifest entry IDs must be nonempty".to_owned());
        }
        if !ids.insert(entry.id.clone()) {
            return Err(format!("Duplicate manifest entry ID `{}`", entry.id));
        }
        if entry.owner.is_empty() {
            return Err(format!(
                "Manifest entry `{}` must declare a nonempty owner",
                entry.id
            ));
        }
        if entry
            .case_insensitive_reason
            .as_ref()
            .is_some_and(|reason| reason.trim().is_empty())
        {
            return Err(format!(
                "Manifest entry `{}` must declare a nonempty case_insensitive_reason",
                entry.id
            ));
        }

        if let Some(existing_id) = positions.insert((entry.bucket, entry.index), entry.id.clone()) {
            return Err(format!(
                "Manifest entries `{existing_id}` and `{}` select the same {} index {}",
                entry.id,
                entry.bucket.label(),
                entry.index
            ));
        }

        let sort_key = (entry.bucket, SortTuple::from_entry(entry));
        if let Some(existing_id) = sort_tuples.insert(sort_key, entry.id.clone()) {
            return Err(format!(
                "Manifest entries `{existing_id}` and `{}` have the same sort tuple in {}",
                entry.id,
                entry.bucket.label()
            ));
        }

        match (
            entry.role,
            entry.discovery_coverage,
            entry.discovery_inputs.as_ref(),
        ) {
            (Role::Discovery, None, _) => {
                return Err(format!(
                    "Discovery entry `{}` must declare discovery_coverage",
                    entry.id
                ));
            }
            (Role::Discovery, Some(_), None) => {
                return Err(format!(
                    "Discovery entry `{}` must declare discovery_inputs",
                    entry.id
                ));
            }
            (Role::Discovery, Some(_), Some(inputs)) if inputs.is_empty() => {
                return Err(format!(
                    "Discovery entry `{}` must declare at least one discovery input",
                    entry.id
                ));
            }
            (Role::Discovery, Some(_), Some(inputs)) if !inputs.contains(&entry.witness) => {
                return Err(format!(
                    "Discovery entry `{}` must include its witness in discovery_inputs",
                    entry.id
                ));
            }
            (Role::Discovery, Some(_), Some(_)) => {}
            (_, Some(_), _) | (_, _, Some(_)) => {
                return Err(format!(
                    "Non-discovery entry `{}` must omit discovery_coverage and discovery_inputs",
                    entry.id
                ));
            }
            (_, None, None) => {}
        }
    }

    Ok(())
}

fn terminal_buckets(settings: &Value) -> Result<&serde_json::Map<String, Value>, String> {
    settings
        .get("agent")
        .and_then(Value::as_object)
        .and_then(|agent| agent.get("tool_permissions"))
        .and_then(Value::as_object)
        .and_then(|permissions| permissions.get("tools"))
        .and_then(Value::as_object)
        .and_then(|tools| tools.get("terminal"))
        .and_then(Value::as_object)
        .ok_or_else(|| {
            "Settings must contain object `.agent.tool_permissions.tools.terminal`".to_owned()
        })
}

fn owner_source_matcher(owner: &str) -> Result<Regex, String> {
    let escaped_owner = regex::escape(owner);
    Regex::new(&format!(
        r"(?:^|[^A-Za-z0-9_-])({escaped_owner})(?:$|[^A-Za-z0-9_-])"
    ))
    .map_err(|error| {
        format!(
            "Failed to build the owner source-text matcher: {}",
            regex_error_summary(&error)
        )
    })
}

fn inventory_settings(
    settings: &Value,
    owner: &str,
) -> Result<BoundedIssues<InventoryHit>, String> {
    let terminal = terminal_buckets(settings)?;
    let matcher = owner_source_matcher(owner)?;
    let mut hits = BoundedIssues::new(MAX_INVENTORY_HITS);

    for bucket_kind in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let bucket = terminal
            .get(bucket_kind.label())
            .and_then(Value::as_array)
            .ok_or_else(|| {
                format!(
                    "Settings terminal bucket `{}` must be an array",
                    bucket_kind.label()
                )
            })?;

        for (index, value) in bucket.iter().enumerate() {
            let id = format!("{}[{index}]", bucket_kind.label());
            let object = value
                .as_object()
                .ok_or_else(|| format!("Settings terminal entry `{id}` must be an object"))?;
            let pattern = object
                .get("pattern")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    format!("Settings terminal entry `{id}` must contain string `pattern`")
                })?;
            let case_sensitive = object
                .get("case_sensitive")
                .and_then(Value::as_bool)
                .ok_or_else(|| {
                    format!("Settings terminal entry `{id}` must contain boolean `case_sensitive`")
                })?;
            let Some(owner_match) = matcher
                .captures(pattern)
                .and_then(|captures| captures.get(1))
            else {
                continue;
            };
            let preview = pattern[owner_match.start()..]
                .chars()
                .take(MAX_PREVIEW_CHARACTERS)
                .collect();

            hits.push(InventoryHit {
                case_sensitive,
                character_count: pattern.chars().count(),
                id,
                preview,
            });
        }
    }

    Ok(hits)
}

fn inventory_json(settings_json: &str, owner: &str) -> Result<BoundedIssues<InventoryHit>, String> {
    validate_owner(owner)?;
    let settings: Value = serde_json::from_str(settings_json)
        .map_err(|error| format!("Invalid settings JSON: {error}"))?;
    inventory_settings(&settings, owner)
}

fn selected_entries(
    settings: &Value,
    manifest: Manifest,
) -> Result<(Vec<SelectedEntry>, usize), String> {
    let terminal = terminal_buckets(settings)?;
    let mut selected = Vec::with_capacity(manifest.entries.len());
    for (finding_key, declaration) in manifest.entries.into_iter().enumerate() {
        let bucket = terminal
            .get(declaration.bucket.label())
            .and_then(Value::as_array)
            .ok_or_else(|| {
                format!(
                    "Settings terminal bucket `{}` must be an array",
                    declaration.bucket.label()
                )
            })?;
        let value = bucket.get(declaration.index).ok_or_else(|| {
            format!(
                "Manifest entry `{}` selects missing {} index {}",
                declaration.id,
                declaration.bucket.label(),
                declaration.index
            )
        })?;
        let object = value.as_object().ok_or_else(|| {
            format!(
                "Selected {} index {} for `{}` must be an object",
                declaration.bucket.label(),
                declaration.index,
                declaration.id
            )
        })?;
        let pattern = object
            .get("pattern")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!(
                    "Selected {} index {} for `{}` must contain string `pattern`",
                    declaration.bucket.label(),
                    declaration.index,
                    declaration.id
                )
            })?;
        let case_sensitive = object
            .get("case_sensitive")
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                format!(
                    "Selected {} index {} for `{}` must contain boolean `case_sensitive`",
                    declaration.bucket.label(),
                    declaration.index,
                    declaration.id
                )
            })?;

        selected.push(SelectedEntry {
            case_sensitive,
            declaration,
            finding_key,
            pattern: pattern.to_owned(),
            regex: None,
        });
    }

    let bucket_count = manifest_bucket_count(&selected);
    Ok((selected, bucket_count))
}

fn manifest_bucket_count(entries: &[SelectedEntry]) -> usize {
    entries
        .iter()
        .map(|entry| entry.declaration.bucket)
        .collect::<BTreeSet<_>>()
        .len()
}

fn is_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return false;
    };

    (first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn is_positive_integer(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('0')
        && value.chars().all(|character| character.is_ascii_digit())
}

fn parse_xargs_child(tokens: &[&str]) -> Result<usize, String> {
    let mut index = 1;

    while index < tokens.len() {
        let token = tokens[index];
        if matches!(
            token,
            "--exit" | "--no-run-if-empty" | "--null" | "--verbose"
        ) {
            index += 1;
            continue;
        }
        if let Some(value) = token.strip_prefix("--max-args=") {
            if !is_positive_integer(value) {
                return Err("xargs has an invalid `--max-args` value".to_owned());
            }
            index += 1;
            continue;
        }
        if token.len() > 1
            && token.starts_with('-')
            && token[1..]
                .chars()
                .all(|character| matches!(character, '0' | 'r' | 't' | 'x'))
        {
            index += 1;
            continue;
        }
        if token == "-L" || token == "-n" {
            let Some(value) = tokens.get(index + 1) else {
                return Err("xargs has a missing numeric option value".to_owned());
            };
            if !is_positive_integer(value) {
                return Err("xargs has an invalid numeric option value".to_owned());
            }
            index += 2;
            continue;
        }
        if let Some(value) = token
            .strip_prefix("-L")
            .or_else(|| token.strip_prefix("-n"))
            && is_positive_integer(value)
        {
            index += 1;
            continue;
        }
        if token.starts_with('-') {
            return Err("xargs uses an unsupported or ambiguous option".to_owned());
        }

        return Ok(index);
    }

    Err("xargs witness does not identify a child executable".to_owned())
}

fn git_owner(tokens: &[&str]) -> Result<String, String> {
    let mut index = 1;

    while index < tokens.len() {
        match tokens[index] {
            "--no-optional-locks" | "--no-pager" => index += 1,
            "-C" => {
                if tokens.get(index + 1).is_none() {
                    return Err("Git `-C` is missing its path".to_owned());
                }
                index += 2;
            }
            _ => break,
        }
    }

    let Some(token) = tokens.get(index) else {
        return Ok("git:root".to_owned());
    };
    if token.starts_with('-') {
        Ok("git:root".to_owned())
    } else {
        Ok(format!("git:{token}"))
    }
}

fn manager_after_corepack<'a>(tokens: &'a [&'a str]) -> Option<&'a str> {
    match tokens {
        ["corepack", manager, ..] if matches!(*manager, "npm" | "pnpm" | "yarn") => Some(*manager),
        _ => None,
    }
}

fn owner_for_executable(tokens: &[&str]) -> Result<String, String> {
    let Some(executable) = tokens.first().copied() else {
        return Err("Witness does not identify an executable".to_owned());
    };

    if executable == "git" {
        git_owner(tokens)
    } else if let Some(manager) = manager_after_corepack(tokens) {
        Ok(manager.to_owned())
    } else {
        Ok(executable.to_owned())
    }
}

pub(crate) fn infer_owner_role(
    witness: &str,
    discovery_inputs: &[String],
) -> Result<InferredOwner, String> {
    if witness.is_empty()
        || witness.starts_with(' ')
        || witness.ends_with(' ')
        || witness
            .chars()
            .any(|character| character.is_whitespace() && character != ' ')
    {
        return Err("Witness is empty or not normalized".to_owned());
    }
    let tokens = witness.split(' ').collect::<Vec<_>>();
    if tokens.iter().any(|token| token.is_empty()) {
        return Err("Witness contains repeated spaces".to_owned());
    }

    let mut index = 0;
    while tokens.get(index).is_some_and(|token| is_assignment(token)) {
        index += 1;
    }
    let has_nohup_wrapper = tokens.get(index) == Some(&"nohup");
    if has_nohup_wrapper {
        index += 1;
    }
    let tokens = &tokens[index..];
    let Some(executable) = tokens.first() else {
        return Err("Witness does not identify an executable".to_owned());
    };

    let is_discovery = discovery_inputs.iter().any(|input| input == witness);
    if *executable == "xargs" {
        if tokens.len() == 1 {
            return Ok(InferredOwner {
                owner: "xargs".to_owned(),
                role: if has_nohup_wrapper {
                    Role::Wrapped
                } else if is_discovery {
                    Role::Discovery
                } else {
                    Role::Direct
                },
            });
        }

        return match parse_xargs_child(tokens) {
            Ok(child_index) => Ok(InferredOwner {
                owner: owner_for_executable(&tokens[child_index..])?,
                role: Role::Wrapped,
            }),
            Err(_) if is_discovery && tokens.len() == 2 => Ok(InferredOwner {
                owner: "xargs".to_owned(),
                role: if has_nohup_wrapper {
                    Role::Wrapped
                } else {
                    Role::Discovery
                },
            }),
            Err(error) => Err(error),
        };
    }

    let role = if has_nohup_wrapper {
        Role::Wrapped
    } else if is_discovery {
        Role::Discovery
    } else {
        Role::Direct
    };

    Ok(InferredOwner {
        owner: owner_for_executable(tokens)?,
        role,
    })
}

struct FindingAccumulator {
    finding_count: usize,
    findings: Vec<Finding>,
    stored_indexes: Vec<Option<usize>>,
    unique_ids: Vec<bool>,
}

impl FindingAccumulator {
    fn new(entry_count: usize) -> Self {
        Self {
            finding_count: 0,
            findings: Vec::with_capacity(MAX_REPORTED_FINDINGS),
            stored_indexes: vec![None; entry_count],
            unique_ids: vec![false; entry_count],
        }
    }

    fn add(&mut self, key: usize, id: &str, reason: impl Into<String>) {
        let reason = reason.into();
        if let Some(index) = self.stored_indexes[key] {
            let finding = &mut self.findings[index];
            if !finding
                .reason
                .split("; ")
                .any(|existing| existing == reason)
            {
                finding.reason.push_str("; ");
                finding.reason.push_str(&reason);
            }
            return;
        }
        if self.unique_ids[key] {
            return;
        }

        self.unique_ids[key] = true;
        self.finding_count += 1;
        if self.findings.len() < MAX_REPORTED_FINDINGS {
            self.stored_indexes[key] = Some(self.findings.len());
            self.findings.push(Finding {
                id: id.to_owned(),
                reason,
            });
        }
    }

    fn into_parts(self) -> (usize, Vec<Finding>) {
        (self.finding_count, self.findings)
    }
}

fn validate_selected_patterns(entries: &mut [SelectedEntry], findings: &mut FindingAccumulator) {
    for selected in entries {
        let id = selected.declaration.id.as_str();
        if !selected.case_sensitive && selected.declaration.case_insensitive_reason.is_none() {
            findings.add(selected.finding_key, id, "case_sensitive is not true");
        }
        if selected.case_sensitive && selected.declaration.case_insensitive_reason.is_some() {
            findings.add(
                selected.finding_key,
                id,
                "case_insensitive_reason is declared for a case-sensitive pattern",
            );
        }
        let character_count = selected.pattern.chars().count();
        if character_count > MAX_PATTERN_CHARACTERS {
            findings.add(
                selected.finding_key,
                id,
                format!("decoded pattern length is {character_count}, not less than 1000"),
            );
        }

        let regex = match compile_pattern(&selected.pattern, selected.case_sensitive) {
            Ok(regex) => regex,
            Err(PatternError::Empty) => {
                findings.add(selected.finding_key, id, "regex is empty");
                continue;
            }
            Err(PatternError::Invalid(error)) => {
                findings.add(
                    selected.finding_key,
                    id,
                    format!("regex is invalid: {}", regex_error_summary(&error)),
                );
                continue;
            }
        };

        if !regex.is_match(&selected.declaration.witness) {
            findings.add(
                selected.finding_key,
                id,
                "pattern does not match its witness",
            );
        }
        if let Some(discovery_inputs) = &selected.declaration.discovery_inputs
            && discovery_inputs.iter().any(|input| !regex.is_match(input))
        {
            findings.add(
                selected.finding_key,
                id,
                "pattern does not match every declared discovery input",
            );
        }
        selected.regex = Some(regex);
    }
}

fn validate_inferred_owners(entries: &[SelectedEntry], findings: &mut FindingAccumulator) {
    for selected in entries {
        let declaration = &selected.declaration;
        let discovery_inputs = declaration.discovery_inputs.as_deref().unwrap_or(&[]);
        match infer_owner_role(&declaration.witness, discovery_inputs) {
            Ok(inferred) => {
                if inferred.owner != declaration.owner {
                    findings.add(
                        selected.finding_key,
                        &declaration.id,
                        "declared owner differs from inferred owner",
                    );
                }
                if inferred.role != declaration.role {
                    findings.add(
                        selected.finding_key,
                        &declaration.id,
                        format!(
                            "declared role `{}` differs from inferred role `{}`",
                            declaration.role.label(),
                            inferred.role.label()
                        ),
                    );
                }
            }
            Err(_) => findings.add(
                selected.finding_key,
                &declaration.id,
                "witness owner or role is unsupported or ambiguous",
            ),
        }
    }
}

fn validate_bucket_order(entries: &[SelectedEntry], findings: &mut FindingAccumulator) {
    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let mut actual = entries
            .iter()
            .filter(|entry| entry.declaration.bucket == bucket)
            .collect::<Vec<_>>();
        actual.sort_by_key(|entry| entry.declaration.index);

        let mut expected = actual.clone();
        expected.sort_by_key(|entry| SortTuple::from_entry(&entry.declaration));

        for (actual_entry, expected_entry) in actual.iter().zip(&expected) {
            if actual_entry.declaration.id != expected_entry.declaration.id {
                findings.add(
                    actual_entry.finding_key,
                    &actual_entry.declaration.id,
                    format!(
                        "{} index order differs from declared sort order",
                        bucket.label()
                    ),
                );
            }
        }
    }
}

fn validate_owner_spans(entries: &[SelectedEntry], findings: &mut FindingAccumulator) {
    let mut groups: BTreeMap<(Bucket, String), Vec<&SelectedEntry>> = BTreeMap::new();
    for entry in entries {
        groups
            .entry((entry.declaration.bucket, entry.declaration.owner.clone()))
            .or_default()
            .push(entry);
    }

    for ((bucket, _), group) in groups {
        let minimum = group
            .iter()
            .map(|entry| entry.declaration.index)
            .min()
            .expect("Owner group must contain an entry");
        let maximum = group
            .iter()
            .map(|entry| entry.declaration.index)
            .max()
            .expect("Owner group must contain an entry");
        let declared_indexes = group
            .iter()
            .map(|entry| entry.declaration.index)
            .collect::<BTreeSet<_>>();

        for index in minimum..=maximum {
            if !declared_indexes.contains(&index) {
                for entry in &group {
                    findings.add(
                        entry.finding_key,
                        &entry.declaration.id,
                        format!(
                            "owner group does not completely occupy {} index {index} inside its span",
                            bucket.label()
                        ),
                    );
                }
                break;
            }
        }
    }
}

fn manager_group(owner: &str) -> &str {
    owner.split_once(':').map_or(owner, |(manager, _)| manager)
}

fn validate_discovery_redundancy(entries: &[SelectedEntry], findings: &mut FindingAccumulator) {
    let mut retained_by_manager: BTreeMap<&str, Vec<&SelectedEntry>> = BTreeMap::new();
    for entry in entries
        .iter()
        .filter(|entry| entry.declaration.bucket == Bucket::Allow)
    {
        retained_by_manager
            .entry(manager_group(&entry.declaration.owner))
            .or_default()
            .push(entry);
    }

    for selected in entries.iter().filter(|entry| {
        entry.declaration.bucket == Bucket::Allow && entry.declaration.role == Role::Discovery
    }) {
        let declaration = &selected.declaration;
        if declaration.discovery_coverage != Some(DiscoveryCoverage::CompleteFinite) {
            continue;
        }
        let Some(inputs) = declaration.discovery_inputs.as_ref() else {
            continue;
        };
        let retained = retained_by_manager
            .get(manager_group(&declaration.owner))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let has_other_entry = retained
            .iter()
            .any(|other| other.declaration.id != declaration.id);

        if has_other_entry
            && inputs.iter().all(|input| {
                retained.iter().any(|other| {
                    other.declaration.id != declaration.id
                        && other
                            .regex
                            .as_ref()
                            .is_some_and(|regex| regex.is_match(input))
                })
            })
        {
            findings.add(
                selected.finding_key,
                &declaration.id,
                "discovery entry is redundant within its always_allow manager group",
            );
        }
    }
}

pub(crate) fn audit_json(settings_json: &str, manifest_json: &str) -> Result<AuditReport, String> {
    let settings: Value = serde_json::from_str(settings_json)
        .map_err(|error| format!("Invalid settings JSON: {error}"))?;
    let manifest: Manifest = serde_json::from_str(manifest_json)
        .map_err(|error| format!("Invalid manifest JSON: {error}"))?;
    validate_manifest(&manifest)?;

    let (mut entries, bucket_count) = selected_entries(&settings, manifest)?;
    let entry_count = entries.len();
    let owner_group_count = entries
        .iter()
        .map(|entry| (entry.declaration.bucket, entry.declaration.owner.clone()))
        .collect::<BTreeSet<_>>()
        .len();
    let mut findings = FindingAccumulator::new(entry_count);

    validate_selected_patterns(&mut entries, &mut findings);
    validate_inferred_owners(&entries, &mut findings);
    validate_bucket_order(&entries, &mut findings);
    validate_owner_spans(&entries, &mut findings);
    validate_discovery_redundancy(&entries, &mut findings);
    let (finding_count, findings) = findings.into_parts();

    Ok(AuditReport {
        bucket_count,
        entry_count,
        finding_count,
        findings,
        owner_group_count,
    })
}

fn count_label(count: usize, singular: &'static str, plural: &'static str) -> &'static str {
    if count == 1 { singular } else { plural }
}

fn report_error(stderr: &mut dyn Write, message: &str) {
    let _ = writeln!(stderr, "zed-permission-owner-audit: {message}");
}

fn report_findings(stderr: &mut dyn Write, report: &AuditReport) -> Result<(), String> {
    writeln!(
        stderr,
        "zed-permission-owner-audit: {} {} across {} {}",
        report.finding_count,
        count_label(report.finding_count, "finding", "findings"),
        report.entry_count,
        count_label(report.entry_count, "entry", "entries")
    )
    .map_err(|error| format!("Failed to write audit findings: {error}"))?;

    for finding in &report.findings {
        writeln!(stderr, "  `{}`: {}", finding.id, finding.reason)
            .map_err(|error| format!("Failed to write audit findings: {error}"))?;
    }

    let omitted = report.finding_count - report.findings.len();
    if omitted > 0 {
        writeln!(stderr, "  … {omitted} additional findings omitted")
            .map_err(|error| format!("Failed to write audit findings: {error}"))?;
    }

    Ok(())
}

fn report_inventory(
    stdout: &mut dyn Write,
    hits: &BoundedIssues<InventoryHit>,
) -> Result<(), String> {
    writeln!(stdout, "Inventory candidates only—not semantic owner proof")
        .map_err(|error| format!("Failed to write inventory result: {error}"))?;

    for hit in hits.issues() {
        let preview = serde_json::to_string(&hit.preview)
            .map_err(|error| format!("Failed to encode inventory preview: {error}"))?;
        writeln!(
            stdout,
            "{} characters={} case_sensitive={} preview={preview}",
            hit.id, hit.character_count, hit.case_sensitive
        )
        .map_err(|error| format!("Failed to write inventory result: {error}"))?;
    }

    if hits.omitted_count() > 0 {
        writeln!(
            stdout,
            "… {} additional inventory candidates omitted",
            hits.omitted_count()
        )
        .map_err(|error| format!("Failed to write inventory result: {error}"))?;
    }

    writeln!(stdout, "Total inventory candidates: {}", hits.total_count())
        .map_err(|error| format!("Failed to write inventory result: {error}"))
}

fn report_success(stdout: &mut dyn Write, report: &AuditReport) -> Result<(), String> {
    writeln!(
        stdout,
        "Audited {} {} across {} {} and {} {}",
        report.entry_count,
        count_label(report.entry_count, "entry", "entries"),
        report.owner_group_count,
        count_label(report.owner_group_count, "owner group", "owner groups"),
        report.bucket_count,
        count_label(report.bucket_count, "bucket", "buckets")
    )
    .map_err(|error| format!("Failed to write audit result: {error}"))
}

pub(crate) fn run<I>(arguments: I, stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8
where
    I: IntoIterator<Item = OsString>,
{
    let parsed = match parse_arguments(arguments) {
        Ok(parsed) => parsed,
        Err(error) => {
            report_error(stderr, &error);
            return STATUS_ERROR;
        }
    };

    match parsed {
        ParsedArguments::Help => match stdout.write_all(HELP.as_bytes()) {
            Ok(()) => STATUS_SUCCESS,
            Err(error) => {
                report_error(stderr, &format!("Failed to write help: {error}"));
                STATUS_ERROR
            }
        },
        ParsedArguments::Run(arguments) => {
            let settings = match read_utf8_file(&arguments.settings, "settings") {
                Ok(settings) => settings,
                Err(error) => {
                    report_error(stderr, &error);
                    return STATUS_ERROR;
                }
            };

            match arguments.operation {
                Operation::Inventory { owner } => {
                    let hits = match inventory_json(&settings, &owner) {
                        Ok(hits) => hits,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    match report_inventory(stdout, &hits) {
                        Ok(()) => STATUS_SUCCESS,
                        Err(error) => {
                            report_error(stderr, &error);
                            STATUS_ERROR
                        }
                    }
                }
                Operation::ManifestAudit { manifest } => {
                    let manifest = match read_utf8_file(&manifest, "manifest") {
                        Ok(manifest) => manifest,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    let report = match audit_json(&settings, &manifest) {
                        Ok(report) => report,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };

                    if report.finding_count == 0 {
                        match report_success(stdout, &report) {
                            Ok(()) => STATUS_SUCCESS,
                            Err(error) => {
                                report_error(stderr, &error);
                                STATUS_ERROR
                            }
                        }
                    } else {
                        match report_findings(stderr, &report) {
                            Ok(()) => STATUS_FINDINGS,
                            Err(error) => {
                                report_error(stderr, &error);
                                STATUS_ERROR
                            }
                        }
                    }
                }
            }
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
