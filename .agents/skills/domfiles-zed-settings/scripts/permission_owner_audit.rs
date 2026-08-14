#[allow(dead_code)]
#[path = "helpers/permission_patterns.rs"]
mod permission_patterns;

use permission_patterns::{
    Bucket, PatternError, compile_pattern, is_valid_sha256, read_utf8_file, regex_error_summary,
    sha256_hex,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    io::Write,
    path::{Path, PathBuf},
};

const HELP: &str = concat!(
    "Usage:\n",
    "  permission-owner-audit --settings <settings-path> --manifest <manifest-path>\n",
    "  permission-owner-audit --settings <settings-path> --owner <top-level-executable> ",
    "[--after <inventory-cursor>]\n",
    "  permission-owner-audit --help\n",
    "\n",
    "Audit declared terminal permission owners, roles, ordering, and finite discovery coverage,\n",
    "or list bounded terminal pattern inventory candidates by top-level executable token.\n",
    "\n",
    "Options:\n",
    "  --after <cursor>          Resume inventory strictly after a candidate in the same exact\n",
    "                           settings snapshot and owner inventory. Valid only with `--owner`\n",
    "                           and accepted at most once\n",
    "  --help                   Print help. Must be used alone\n",
    "  --manifest <path>        Audit entries declared by the canonical manifest schema\n",
    "  --owner <token>          List inventory candidates for one top-level executable token.\n",
    "                           Mutually exclusive with `--manifest`. The token must match\n",
    "                           `[A-Za-z0-9_.+-]+`\n",
    "  --settings <path>        Read Zed settings containing terminal permission buckets\n",
    "\n",
    "Canonical manifest schema (unknown fields are rejected):\n",
    "  {\n",
    "    \"settings_sha256\": \"<exact-settings-sha256>\",\n",
    "    \"inventory_owner\": \"<top-level-executable>\",\n",
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
    "    ],\n",
    "    \"excluded_candidates\": [\n",
    "      {\n",
    "        \"bucket\": \"always_allow|always_confirm|always_deny\",\n",
    "        \"index\": 0,\n",
    "        \"owner\": \"<semantic owner outside inventory_owner>\",\n",
    "        \"witness\": \"<one normalized permission input>\",\n",
    "        \"reason\": \"<nonempty semantic exclusion reason>\"\n",
    "      }\n",
    "    ]\n",
    "  }\n",
    "\n",
    "Manifest contract:\n",
    "  Omit `discovery_coverage` and `discovery_inputs` for direct and wrapped entries. Discovery\n",
    "  entries require a coverage kind and nonempty inputs containing `witness`. `complete_finite`\n",
    "  claims the complete finite grammar and enables duplicate-coverage findings. `representative`\n",
    "  records bounded cases for variable grammar and leaves complete coverage to matcher suites.\n",
    "  `settings_sha256` must bind the exact settings bytes. `inventory_owner` must be a nonempty\n",
    "  top-level executable token, and `entries` must be nonempty. Entry and exclusion positions\n",
    "  must be unique and disjoint. Their union must exactly classify the independently recomputed\n",
    "  lexical inventory for `inventory_owner`. Every entry must infer to that owner group. Every\n",
    "  exclusion requires a matching normalized witness that infers to its declared outside owner.\n",
    "  Exclusion reasons must be nonempty. Sort tuples must be unique within a bucket.\n",
    "  Omit `case_insensitive_reason` for case-sensitive patterns. A selected case-insensitive\n",
    "  pattern requires a nonempty reason recording its verified command-specific exception.\n",
    "  Each selected decoded pattern must contain at most 999 Unicode scalar values.\n",
    "  Completeness spans use the independently inferred bucket, semantic owner, and Git repository\n",
    "  scope: general, exact top-level agent worktree, or traversal-free descendant fixture.\n",
    "  Declared roles and sort keys participate in ordering but never partition completeness.\n",
    "  A Git discovery-to-direct or discovery-to-wrapped gap is accepted only for the same\n",
    "  inferred Git owner and repository scope when every intervening index is a manifest entry\n",
    "  that independently infers to a Git owner.\n",
    "  Semantic owners name the independently inferred executable or manager, for example\n",
    "  `npm`, `git:hash-object`, or `git:root`.\n",
    "  Actual bucket order is checked by (`owner_sort_key`, `section_sort_key`, role order\n",
    "  `discovery`/`direct`/`wrapped`, `owner`, `pattern_sort_key`).\n",
    "\n",
    "Inventory contract:\n",
    "  Inventory inspects regex source text without compiling permission patterns. Owner\n",
    "  occurrences must have source-text token boundaries. Matches are inventory candidates,\n",
    "  not semantic ownership proof. At most 100 candidates are printed per page. Each hit\n",
    "  contains only its bucket/index, decoded character count, required boolean `case_sensitive`\n",
    "  setting, and a preview of at most 160 Unicode scalar values beginning at the owner token.\n",
    "  Every page reports the exact settings SHA-256. `--after` accepts only the opaque cursor\n",
    "  reported by the previous page for the same owner and exact settings bytes. Any settings\n",
    "  change invalidates the cursor and requires restarting from the first page.\n",
    "\n",
    "Output:\n",
    "  Help, successful manifest-audit results, and inventory are written to standard output.\n",
    "  Audit findings and errors are written to standard error. A successful manifest audit\n",
    "  reports entry, owner-group, and bucket counts only. Findings report at most 10 bounded\n",
    "  entry ID previews with concise reasons. Inventory reports the exact settings SHA-256,\n",
    "  bounded source-text candidates, exact total and remaining counts, and an\n",
    "  opaque continuation cursor only when another page exists. Regex bodies and witness inputs\n",
    "  are never printed as complete fields.\n",
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
const MAX_DISPLAY_ID_CHARACTERS: usize = 80;
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
    settings_sha256: String,
    inventory_owner: String,
    entries: Vec<ManifestEntry>,
    excluded_candidates: Vec<ExcludedCandidate>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExcludedCandidate {
    bucket: Bucket,
    index: usize,
    owner: String,
    witness: String,
    reason: String,
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
    Inventory {
        after: Option<InventoryCursor>,
        owner: String,
    },
    ManifestAudit {
        manifest: PathBuf,
    },
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RepositoryScope {
    AgentWorktree,
    FixtureRepository,
    General,
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidatePosition {
    bucket: Bucket,
    index: usize,
}

impl CandidatePosition {
    fn parse(value: &str) -> Result<Self, String> {
        let Some((bucket, index)) = value.split_once('[') else {
            return Err(Self::invalid_cursor_message());
        };
        let Some(index) = index.strip_suffix(']') else {
            return Err(Self::invalid_cursor_message());
        };
        if index.is_empty()
            || (index.len() > 1 && index.starts_with('0'))
            || !index.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(Self::invalid_cursor_message());
        }
        let bucket = Bucket::parse(bucket).ok_or_else(Self::invalid_cursor_message)?;
        let index = index
            .parse::<usize>()
            .map_err(|_| Self::invalid_cursor_message())?;

        Ok(Self { bucket, index })
    }

    fn id(self) -> String {
        format!("{}[{}]", self.bucket.label(), self.index)
    }

    fn invalid_cursor_message() -> String {
        "Inventory cursor position must use canonical `always_allow[<index>]`, `always_confirm[<index>]`, or `always_deny[<index>]` syntax".to_owned()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryCursor {
    settings_sha256: String,
    owner: String,
    position: CandidatePosition,
}

impl InventoryCursor {
    fn parse(value: &str) -> Result<Self, String> {
        let mut fields = value.splitn(3, ':');
        let settings_sha256 = fields.next().unwrap_or_default();
        let owner = fields.next().unwrap_or_default();
        let position = fields.next().unwrap_or_default();
        if !is_valid_sha256(settings_sha256) || validate_owner(owner).is_err() {
            return Err(Self::invalid_message());
        }
        let position = CandidatePosition::parse(position).map_err(|_| Self::invalid_message())?;

        Ok(Self {
            settings_sha256: settings_sha256.to_owned(),
            owner: owner.to_owned(),
            position,
        })
    }

    fn encoded(&self) -> String {
        format!(
            "{}:{}:{}",
            self.settings_sha256,
            self.owner,
            self.position.id()
        )
    }

    fn invalid_message() -> String {
        "Option `--after` requires an exact inventory cursor reported by the previous page"
            .to_owned()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryHit {
    case_sensitive: bool,
    character_count: usize,
    position: CandidatePosition,
    preview: String,
}

struct InventoryPage {
    hits: Vec<InventoryHit>,
    next_cursor: Option<InventoryCursor>,
    remaining_count: usize,
    settings_sha256: String,
    total_count: usize,
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

fn display_id(id: &str) -> String {
    let mut output = String::new();
    let mut characters = id.chars();

    for _ in 0..MAX_DISPLAY_ID_CHARACTERS {
        let Some(character) = characters.next() else {
            return output;
        };
        output.push(if character.is_control() {
            '?'
        } else {
            character
        });
    }
    if characters.next().is_some() {
        output.push('…');
    }

    output
}

pub(crate) fn settings_sha256(settings_json: &str) -> String {
    sha256_hex(settings_json.as_bytes())
}

fn json_error_summary(error: &serde_json::Error, description: &str) -> String {
    let summary = match error.classify() {
        serde_json::error::Category::Data => {
            format!("{description} JSON data does not match the required schema")
        }
        serde_json::error::Category::Eof => {
            format!("{description} JSON ends before a complete value")
        }
        serde_json::error::Category::Io => format!("Failed to read {description} JSON"),
        serde_json::error::Category::Syntax => format!("{description} JSON syntax is invalid"),
    };

    format!(
        "{summary} at line {}, column {}",
        error.line(),
        error.column()
    )
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
    let mut after = None;
    let mut manifest = None;
    let mut owner = None;
    let mut settings = None;

    while let Some(argument) = arguments.next() {
        let Some(option) = argument.to_str() else {
            return Err("Option names must be valid UTF-8".to_owned());
        };

        match option {
            "--after" => {
                if after.is_some() {
                    return Err("Option `--after` may be specified only once".to_owned());
                }
                let Some(value) = arguments.next() else {
                    return Err("Option `--after` requires a cursor".to_owned());
                };
                let value = value
                    .into_string()
                    .map_err(|_| "Option `--after` must be valid UTF-8".to_owned())?;
                after = Some(InventoryCursor::parse(&value)?);
            }
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
                    "Unknown option `{}`. Run `permission-owner-audit --help` for usage",
                    display_id(option)
                ));
            }
        }
    }

    let settings = settings
        .ok_or_else(|| "Missing required option `--settings <settings-path>`".to_owned())?;
    let operation = match (manifest, owner, after) {
        (Some(_), Some(_), _) => {
            return Err("Options `--manifest` and `--owner` are mutually exclusive".to_owned());
        }
        (Some(_), None, Some(_)) | (None, None, Some(_)) => {
            return Err("Option `--after` is valid only with `--owner`".to_owned());
        }
        (Some(manifest), None, None) => Operation::ManifestAudit { manifest },
        (None, Some(owner), after) => Operation::Inventory { after, owner },
        (None, None, None) => {
            return Err(
                "Missing required operation `--manifest <manifest-path>` or `--owner <top-level-executable>`"
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
    if !is_valid_sha256(&manifest.settings_sha256) {
        return Err(
            "Manifest `settings_sha256` must be 64 lowercase hexadecimal characters".to_owned(),
        );
    }
    if validate_owner(&manifest.inventory_owner).is_err() {
        return Err(
            "Manifest `inventory_owner` requires a nonempty token matching `[A-Za-z0-9_.+-]+`"
                .to_owned(),
        );
    }
    if manifest.entries.is_empty() {
        return Err("Manifest `entries` must be nonempty".to_owned());
    }

    let mut ids = BTreeSet::new();
    let mut positions = BTreeMap::new();
    let mut sort_tuples = BTreeMap::new();

    for entry in &manifest.entries {
        if entry.id.is_empty() {
            return Err("Manifest entry IDs must be nonempty".to_owned());
        }
        if !ids.insert(entry.id.clone()) {
            return Err(format!(
                "Duplicate manifest entry ID `{}`",
                display_id(&entry.id)
            ));
        }
        if entry.owner.is_empty() {
            return Err(format!(
                "Manifest entry `{}` must declare a nonempty `owner`",
                display_id(&entry.id)
            ));
        }
        if entry
            .case_insensitive_reason
            .as_ref()
            .is_some_and(|reason| reason.trim().is_empty())
        {
            return Err(format!(
                "Manifest entry `{}` must declare a nonempty `case_insensitive_reason`",
                display_id(&entry.id)
            ));
        }

        let position = CandidatePosition {
            bucket: entry.bucket,
            index: entry.index,
        };
        if let Some(existing_id) = positions.insert(position, entry.id.clone()) {
            return Err(format!(
                "Manifest entries `{}` and `{}` select the same `{}` index {}",
                display_id(&existing_id),
                display_id(&entry.id),
                entry.bucket.label(),
                entry.index
            ));
        }

        let sort_key = (entry.bucket, SortTuple::from_entry(entry));
        if let Some(existing_id) = sort_tuples.insert(sort_key, entry.id.clone()) {
            return Err(format!(
                "Manifest entries `{}` and `{}` have the same sort tuple in `{}`",
                display_id(&existing_id),
                display_id(&entry.id),
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
                    "Discovery entry `{}` must declare `discovery_coverage`",
                    display_id(&entry.id)
                ));
            }
            (Role::Discovery, Some(_), None) => {
                return Err(format!(
                    "Discovery entry `{}` must declare `discovery_inputs`",
                    display_id(&entry.id)
                ));
            }
            (Role::Discovery, Some(_), Some(inputs)) if inputs.is_empty() => {
                return Err(format!(
                    "Discovery entry `{}` must declare at least one `discovery_inputs` value",
                    display_id(&entry.id)
                ));
            }
            (Role::Discovery, Some(_), Some(inputs)) if !inputs.contains(&entry.witness) => {
                return Err(format!(
                    "Discovery entry `{}` must include its `witness` in `discovery_inputs`",
                    display_id(&entry.id)
                ));
            }
            (Role::Discovery, Some(_), Some(_)) => {}
            (_, Some(_), _) | (_, _, Some(_)) => {
                return Err(format!(
                    "Non-discovery entry `{}` must omit `discovery_coverage` and `discovery_inputs`",
                    display_id(&entry.id)
                ));
            }
            (_, None, None) => {}
        }
    }

    let mut excluded_positions = BTreeSet::new();
    for excluded in &manifest.excluded_candidates {
        let position = CandidatePosition {
            bucket: excluded.bucket,
            index: excluded.index,
        };
        if excluded.owner.is_empty() {
            return Err(format!(
                "Excluded candidate `{}` must declare a nonempty `owner`",
                position.id()
            ));
        }
        if excluded.witness.is_empty() {
            return Err(format!(
                "Excluded candidate `{}` must declare a nonempty `witness`",
                position.id()
            ));
        }
        if excluded.reason.trim().is_empty() {
            return Err(format!(
                "Excluded candidate `{}` must declare a nonempty `reason`",
                position.id()
            ));
        }
        if !excluded_positions.insert(position) {
            return Err(format!(
                "Duplicate excluded candidate position `{}`",
                position.id()
            ));
        }
        if let Some(entry_id) = positions.get(&position) {
            return Err(format!(
                "Manifest entry `{}` and an excluded candidate both classify `{}`",
                display_id(entry_id),
                position.id()
            ));
        }
    }

    Ok(())
}

fn parse_manifest_json(manifest_json: &str) -> Result<Manifest, String> {
    serde_json::from_str(manifest_json).map_err(|error| json_error_summary(&error, "Manifest"))
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
        r"(?:^|[^A-Za-z0-9_.+-])({escaped_owner})(?:$|[^A-Za-z0-9_.+-])"
    ))
    .map_err(|error| {
        format!(
            "Failed to build the owner source-text matcher:\n\n{}",
            regex_error_summary(&error)
        )
    })
}

fn for_each_inventory_candidate(
    settings: &Value,
    owner: &str,
    mut visit: impl FnMut(InventoryHit),
) -> Result<(), String> {
    let terminal = terminal_buckets(settings)?;
    let matcher = owner_source_matcher(owner)?;

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
            let position = CandidatePosition {
                bucket: bucket_kind,
                index,
            };
            let id = position.id();
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

            visit(InventoryHit {
                case_sensitive,
                character_count: pattern.chars().count(),
                position,
                preview,
            });
        }
    }

    Ok(())
}

fn inventory_settings(
    settings: &Value,
    settings_sha256: &str,
    owner: &str,
    after: Option<CandidatePosition>,
) -> Result<InventoryPage, String> {
    let mut cursor_found = after.is_none();
    let mut hits = Vec::with_capacity(MAX_INVENTORY_HITS);
    let mut remaining_count = 0;
    let mut total_count = 0;

    for_each_inventory_candidate(settings, owner, |hit| {
        total_count += 1;
        if !cursor_found {
            if Some(hit.position) == after {
                cursor_found = true;
            }
            return;
        }
        if Some(hit.position) == after {
            return;
        }
        if hits.len() < MAX_INVENTORY_HITS {
            hits.push(hit);
        } else {
            remaining_count += 1;
        }
    })?;

    if !cursor_found {
        return Err(
            "Inventory cursor is missing or no longer identifies a lexical candidate. Restart from the first page"
                .to_owned(),
        );
    }
    let next_cursor = (remaining_count > 0).then(|| InventoryCursor {
        settings_sha256: settings_sha256.to_owned(),
        owner: owner.to_owned(),
        position: hits
            .last()
            .expect("A continuing page must contain a hit")
            .position,
    });

    Ok(InventoryPage {
        hits,
        next_cursor,
        remaining_count,
        settings_sha256: settings_sha256.to_owned(),
        total_count,
    })
}

fn inventory_json(
    settings_json: &str,
    owner: &str,
    after: Option<InventoryCursor>,
) -> Result<InventoryPage, String> {
    validate_owner(owner)?;
    let settings_sha256 = settings_sha256(settings_json);
    let after_position = if let Some(cursor) = after {
        if cursor.settings_sha256 != settings_sha256 || cursor.owner != owner {
            return Err(
                "Inventory cursor does not match the current settings snapshot and owner. Restart from the first page"
                    .to_owned(),
            );
        }
        Some(cursor.position)
    } else {
        None
    };
    let settings: Value = serde_json::from_str(settings_json)
        .map_err(|error| json_error_summary(&error, "Settings"))?;
    inventory_settings(&settings, &settings_sha256, owner, after_position)
}

fn validate_manifest_coverage(settings: &Value, manifest: &Manifest) -> Result<(), String> {
    let entry_positions = manifest
        .entries
        .iter()
        .map(|entry| CandidatePosition {
            bucket: entry.bucket,
            index: entry.index,
        })
        .collect::<BTreeSet<_>>();
    let excluded_positions = manifest
        .excluded_candidates
        .iter()
        .map(|excluded| CandidatePosition {
            bucket: excluded.bucket,
            index: excluded.index,
        })
        .collect::<BTreeSet<_>>();
    let classified_positions = entry_positions
        .union(&excluded_positions)
        .copied()
        .collect::<BTreeSet<_>>();
    let mut unexpected_positions = classified_positions.clone();
    let mut missing_count = 0;
    let mut missing_positions = Vec::with_capacity(MAX_REPORTED_FINDINGS);

    for_each_inventory_candidate(settings, &manifest.inventory_owner, |hit| {
        if !classified_positions.contains(&hit.position) {
            missing_count += 1;
            if missing_positions.len() < MAX_REPORTED_FINDINGS {
                missing_positions.push(hit.position);
            }
        }
        unexpected_positions.remove(&hit.position);
    })?;

    let unexpected_count = unexpected_positions.len();
    if missing_count == 0 && unexpected_count == 0 {
        return Ok(());
    }

    let mut details = missing_positions
        .into_iter()
        .map(|position| format!("missing `{}`", position.id()))
        .collect::<Vec<_>>();
    details.extend(
        unexpected_positions
            .iter()
            .take(MAX_REPORTED_FINDINGS - details.len())
            .map(|position| format!("unexpected `{}`", position.id())),
    );
    let detail_suffix = if details.is_empty() {
        String::new()
    } else {
        format!(". Positions: {}", details.join(", "))
    };

    Err(format!(
        "Manifest lexical-candidate coverage mismatch: {missing_count} missing candidate positions and {unexpected_count} unexpected classified positions{detail_suffix}"
    ))
}

fn terminal_pattern_at(
    settings: &Value,
    position: CandidatePosition,
) -> Result<(&str, bool), String> {
    let terminal = terminal_buckets(settings)?;
    let value = terminal
        .get(position.bucket.label())
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "Settings terminal bucket `{}` must be an array",
                position.bucket.label()
            )
        })?
        .get(position.index)
        .ok_or_else(|| format!("Classified candidate `{}` is missing", position.id()))?;
    let object = value
        .as_object()
        .ok_or_else(|| format!("Classified candidate `{}` must be an object", position.id()))?;
    let pattern = object
        .get("pattern")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "Classified candidate `{}` must contain string `pattern`",
                position.id()
            )
        })?;
    let case_sensitive = object
        .get("case_sensitive")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            format!(
                "Classified candidate `{}` must contain boolean `case_sensitive`",
                position.id()
            )
        })?;

    Ok((pattern, case_sensitive))
}

fn validate_excluded_candidates(settings: &Value, manifest: &Manifest) -> Result<(), String> {
    for excluded in &manifest.excluded_candidates {
        let position = CandidatePosition {
            bucket: excluded.bucket,
            index: excluded.index,
        };
        let (pattern, case_sensitive) = terminal_pattern_at(settings, position)?;
        let regex = match compile_pattern(pattern, case_sensitive) {
            Ok(regex) => regex,
            Err(PatternError::Empty) => {
                return Err(format!(
                    "Excluded candidate `{}` contains an empty regex",
                    position.id()
                ));
            }
            Err(PatternError::Invalid(error)) => {
                return Err(format!(
                    "Excluded candidate `{}` contains an invalid regex: {}",
                    position.id(),
                    regex_error_summary(&error)
                ));
            }
        };
        if !regex.is_match(&excluded.witness) {
            return Err(format!(
                "Excluded candidate `{}` does not match its witness",
                position.id()
            ));
        }
        let inferred = infer_owner_role(&excluded.witness, &[]).map_err(|_| {
            format!(
                "Excluded candidate `{}` has an unsupported or ambiguous witness owner",
                position.id()
            )
        })?;
        if inferred.owner != excluded.owner {
            return Err(format!(
                "Excluded candidate `{}` declares an owner that differs from its inferred owner",
                position.id()
            ));
        }
        if manager_group(&inferred.owner) == manifest.inventory_owner {
            return Err(format!(
                "Excluded candidate `{}` infers to the manifest inventory owner",
                position.id()
            ));
        }
    }

    Ok(())
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
                "Manifest entry `{}` selects missing `{}` index {}",
                display_id(&declaration.id),
                declaration.bucket.label(),
                declaration.index
            )
        })?;
        let object = value.as_object().ok_or_else(|| {
            format!(
                "Selected `{}` index {} for `{}` must be an object",
                declaration.bucket.label(),
                declaration.index,
                display_id(&declaration.id)
            )
        })?;
        let pattern = object
            .get("pattern")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!(
                    "Selected `{}` index {} for `{}` must contain string `pattern`",
                    declaration.bucket.label(),
                    declaration.index,
                    display_id(&declaration.id)
                )
            })?;
        let case_sensitive = object
            .get("case_sensitive")
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                format!(
                    "Selected `{}` index {} for `{}` must contain boolean `case_sensitive`",
                    declaration.bucket.label(),
                    declaration.index,
                    display_id(&declaration.id)
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
                return Err("`xargs` has an invalid `--max-args` value".to_owned());
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
                return Err("`xargs` has a missing numeric option value".to_owned());
            };
            if !is_positive_integer(value) {
                return Err("`xargs` has an invalid numeric option value".to_owned());
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
            return Err("`xargs` uses an unsupported or ambiguous option".to_owned());
        }

        return Ok(index);
    }

    Err("`xargs` witness does not identify a child executable".to_owned())
}

struct GitPrefix {
    repository_scope: RepositoryScope,
    subcommand_index: usize,
}

fn parse_git_prefix(tokens: &[&str]) -> Result<GitPrefix, String> {
    let mut index = 1;
    let mut repository_scope = None;

    while index < tokens.len() {
        match tokens[index] {
            "--no-optional-locks" | "--no-pager" => index += 1,
            "-C" => {
                let Some(path) = tokens.get(index + 1) else {
                    return Err("Git `-C` is missing its path".to_owned());
                };
                repository_scope = Some(if repository_scope.is_some() {
                    RepositoryScope::General
                } else {
                    repository_scope_from_path(path)
                });
                index += 2;
            }
            "-c" if tokens.get(index + 1).copied() == Some("commit.gpgsign=false") => {
                index += 2;
            }
            _ => break,
        }
    }

    Ok(GitPrefix {
        repository_scope: repository_scope.unwrap_or(RepositoryScope::General),
        subcommand_index: index,
    })
}

fn git_owner(tokens: &[&str]) -> Result<String, String> {
    let prefix = parse_git_prefix(tokens)?;
    let Some(token) = tokens.get(prefix.subcommand_index) else {
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

fn is_agent_name(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };

    (first.is_ascii_alphanumeric() || matches!(first, '-' | '_'))
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '.' | '_')
        })
}

fn repository_scope_from_path(path: &str) -> RepositoryScope {
    if Path::new(path).is_absolute()
        || path
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return RepositoryScope::General;
    }
    let mut components = path.split('/');
    let Some(root) = components.next() else {
        return RepositoryScope::General;
    };
    let Some(name) = root.strip_prefix(".agent-") else {
        return RepositoryScope::General;
    };
    if !is_agent_name(name) {
        return RepositoryScope::General;
    }

    if components.next().is_some() {
        RepositoryScope::FixtureRepository
    } else {
        RepositoryScope::AgentWorktree
    }
}

fn git_tokens_after_wrappers<'a>(tokens: &'a [&'a str]) -> Option<&'a [&'a str]> {
    let mut index = 0;
    while tokens.get(index).is_some_and(|token| is_assignment(token)) {
        index += 1;
    }
    if tokens.get(index) == Some(&"nohup") {
        index += 1;
    }
    let command = &tokens[index..];
    if command.first() == Some(&"xargs") {
        let child_index = parse_xargs_child(command).ok()?;
        return Some(&command[child_index..]).filter(|child| child.first() == Some(&"git"));
    }

    Some(command).filter(|command| command.first() == Some(&"git"))
}

fn infer_repository_scope(witness: &str) -> RepositoryScope {
    let tokens = witness.split(' ').collect::<Vec<_>>();
    let Some(git_tokens) = git_tokens_after_wrappers(&tokens) else {
        return RepositoryScope::General;
    };

    parse_git_prefix(git_tokens)
        .map(|prefix| prefix.repository_scope)
        .unwrap_or(RepositoryScope::General)
}

fn infer_git_ordering_role(witness: &str) -> Option<Role> {
    let tokens = witness.split(' ').collect::<Vec<_>>();
    let mut index = 0;
    while tokens.get(index).is_some_and(|token| is_assignment(token)) {
        index += 1;
    }

    let mut wrapped = false;
    if tokens.get(index) == Some(&"nohup") {
        wrapped = true;
        index += 1;
    }
    let mut command = &tokens[index..];
    if command.first() == Some(&"xargs") {
        let child_index = parse_xargs_child(command).ok()?;
        command = &command[child_index..];
        wrapped = true;
    }
    if command.first() != Some(&"git") {
        return None;
    }
    if wrapped {
        return Some(Role::Wrapped);
    }

    let prefix = parse_git_prefix(command).ok()?;
    let command = &command[prefix.subcommand_index..];
    let is_discovery = matches!(
        command,
        ["-h"
            | "-help"
            | "-v"
            | "-version"
            | "--help"
            | "--list-cmds=builtins"
            | "--list-cmds=parseopt"
            | "--man-path"
            | "--version"]
    ) || matches!(command, [subcommand, "-h" | "--help"] if !subcommand.starts_with('-'));

    Some(if is_discovery {
        Role::Discovery
    } else {
        Role::Direct
    })
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
    } else if *executable == "git" {
        infer_git_ordering_role(witness).unwrap_or(Role::Direct)
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
                id: display_id(id),
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
            findings.add(
                selected.finding_key,
                id,
                "`case_sensitive` is `false` without `case_insensitive_reason`",
            );
        }
        if selected.case_sensitive && selected.declaration.case_insensitive_reason.is_some() {
            findings.add(
                selected.finding_key,
                id,
                "case-sensitive pattern must omit `case_insensitive_reason`",
            );
        }
        let character_count = selected.pattern.chars().count();
        if character_count > MAX_PATTERN_CHARACTERS {
            findings.add(
                selected.finding_key,
                id,
                format!(
                    "decoded pattern contains {character_count} Unicode scalar values. Maximum: {MAX_PATTERN_CHARACTERS}"
                ),
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

fn validate_inferred_owners(
    entries: &[SelectedEntry],
    inventory_owner: &str,
    findings: &mut FindingAccumulator,
) {
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
                if manager_group(&inferred.owner) != inventory_owner {
                    findings.add(
                        selected.finding_key,
                        &declaration.id,
                        "inferred owner lies outside the manifest inventory owner",
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
                        "`{}` index order differs from declared sort order",
                        bucket.label()
                    ),
                );
            }
        }
    }
}

struct InferredEntry<'a> {
    entry: &'a SelectedEntry,
    ordering_role: Option<Role>,
    owner: InferredOwner,
    repository_scope: RepositoryScope,
}

fn permits_git_ordering_separation(
    earlier: &InferredEntry<'_>,
    later: &InferredEntry<'_>,
    inferred_by_position: &BTreeMap<CandidatePosition, InferredEntry<'_>>,
) -> bool {
    if !earlier.owner.owner.starts_with("git:")
        || earlier.ordering_role != Some(Role::Discovery)
        || !matches!(later.ordering_role, Some(Role::Direct | Role::Wrapped))
    {
        return false;
    }

    let bucket = earlier.entry.declaration.bucket;
    ((earlier.entry.declaration.index + 1)..later.entry.declaration.index).all(|index| {
        inferred_by_position
            .get(&CandidatePosition { bucket, index })
            .is_some_and(|entry| entry.owner.owner.starts_with("git:"))
    })
}

fn validate_owner_spans(entries: &[SelectedEntry], findings: &mut FindingAccumulator) {
    let mut inferred_by_position = BTreeMap::new();
    for entry in entries {
        let discovery_inputs = entry.declaration.discovery_inputs.as_deref().unwrap_or(&[]);
        let Ok(owner) = infer_owner_role(&entry.declaration.witness, discovery_inputs) else {
            continue;
        };
        inferred_by_position.insert(
            CandidatePosition {
                bucket: entry.declaration.bucket,
                index: entry.declaration.index,
            },
            InferredEntry {
                entry,
                ordering_role: infer_git_ordering_role(&entry.declaration.witness),
                owner,
                repository_scope: infer_repository_scope(&entry.declaration.witness),
            },
        );
    }

    let mut groups: BTreeMap<(Bucket, String, RepositoryScope), Vec<&InferredEntry<'_>>> =
        BTreeMap::new();
    for inferred in inferred_by_position.values() {
        groups
            .entry((
                inferred.entry.declaration.bucket,
                inferred.owner.owner.clone(),
                inferred.repository_scope,
            ))
            .or_default()
            .push(inferred);
    }

    for ((bucket, _, _), mut group) in groups {
        group.sort_by_key(|entry| entry.entry.declaration.index);
        for pair in group.windows(2) {
            let earlier = pair[0];
            let later = pair[1];
            if later.entry.declaration.index == earlier.entry.declaration.index + 1
                || permits_git_ordering_separation(earlier, later, &inferred_by_position)
            {
                continue;
            }

            let missing_index = earlier.entry.declaration.index + 1;
            for entry in &group {
                findings.add(
                    entry.entry.finding_key,
                    &entry.entry.declaration.id,
                    format!(
                        "owner-scope group does not completely occupy `{}` index {missing_index} inside its span",
                        bucket.label()
                    ),
                );
            }
            break;
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
                "discovery entry is redundant within its `always_allow` manager group",
            );
        }
    }
}

pub(crate) fn audit_json(settings_json: &str, manifest_json: &str) -> Result<AuditReport, String> {
    let settings_sha256 = settings_sha256(settings_json);
    let settings: Value = serde_json::from_str(settings_json)
        .map_err(|error| json_error_summary(&error, "Settings"))?;
    let manifest = parse_manifest_json(manifest_json)?;
    validate_manifest(&manifest)?;
    if manifest.settings_sha256 != settings_sha256 {
        return Err(
            "Manifest `settings_sha256` does not match the exact settings input. Rebuild the inventory and manifest"
                .to_owned(),
        );
    }
    validate_manifest_coverage(&settings, &manifest)?;
    validate_excluded_candidates(&settings, &manifest)?;

    let inventory_owner = manifest.inventory_owner.clone();
    let (mut entries, bucket_count) = selected_entries(&settings, manifest)?;
    let entry_count = entries.len();
    let owner_group_count = entries
        .iter()
        .map(|entry| (entry.declaration.bucket, entry.declaration.owner.clone()))
        .collect::<BTreeSet<_>>()
        .len();
    let mut findings = FindingAccumulator::new(entry_count);

    validate_selected_patterns(&mut entries, &mut findings);
    validate_inferred_owners(&entries, &inventory_owner, &mut findings);
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
    let _ = writeln!(stderr, "permission-owner-audit: {message}");
}

fn report_findings(stderr: &mut dyn Write, report: &AuditReport) -> Result<(), String> {
    writeln!(
        stderr,
        "permission-owner-audit: {} {} across {} {}",
        report.finding_count,
        count_label(report.finding_count, "finding", "findings"),
        report.entry_count,
        count_label(report.entry_count, "entry", "entries")
    )
    .map_err(|error| format!("Failed to write audit findings to standard error:\n\n{error}"))?;

    for finding in &report.findings {
        writeln!(stderr, "  `{}`: {}", finding.id, finding.reason).map_err(|error| {
            format!("Failed to write audit findings to standard error:\n\n{error}")
        })?;
    }

    let omitted = report.finding_count - report.findings.len();
    if omitted > 0 {
        writeln!(stderr, "  … {omitted} additional findings omitted").map_err(|error| {
            format!("Failed to write audit findings to standard error:\n\n{error}")
        })?;
    }

    Ok(())
}

fn report_inventory(stdout: &mut dyn Write, page: &InventoryPage) -> Result<(), String> {
    writeln!(
        stdout,
        "Inventory results are candidates, not semantic ownership proof"
    )
    .map_err(|error| {
        format!(
            "Failed to write inventory result to standard output:

{error}"
        )
    })?;

    writeln!(
        stdout,
        "Inventory settings SHA-256: {}",
        page.settings_sha256
    )
    .map_err(|error| format!("Failed to write inventory result to standard output:\n\n{error}"))?;

    for hit in &page.hits {
        let preview = serde_json::to_string(&hit.preview).map_err(|error| {
            format!(
                "Failed to encode inventory preview as JSON:

{error}"
            )
        })?;
        writeln!(
            stdout,
            "{} characters={} case_sensitive={} preview={preview}",
            hit.position.id(),
            hit.character_count,
            hit.case_sensitive
        )
        .map_err(|error| {
            format!(
                "Failed to write inventory result to standard output:

{error}"
            )
        })?;
    }

    writeln!(stdout, "Total inventory candidates: {}", page.total_count).map_err(|error| {
        format!(
            "Failed to write inventory result to standard output:

{error}"
        )
    })?;
    writeln!(
        stdout,
        "Inventory candidates remaining after this page: {}",
        page.remaining_count
    )
    .map_err(|error| {
        format!(
            "Failed to write inventory result to standard output:

{error}"
        )
    })?;
    if let Some(cursor) = &page.next_cursor {
        writeln!(stdout, "Next inventory cursor: {}", cursor.encoded()).map_err(|error| {
            format!(
                "Failed to write inventory result to standard output:

{error}"
            )
        })?;
    }

    Ok(())
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
    .map_err(|error| format!("Failed to write audit result to standard output:\n\n{error}"))
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
                report_error(
                    stderr,
                    &format!("Failed to write help to standard output:\n\n{error}"),
                );
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
                Operation::Inventory { after, owner } => {
                    let page = match inventory_json(&settings, &owner, after) {
                        Ok(page) => page,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    match report_inventory(stdout, &page) {
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
