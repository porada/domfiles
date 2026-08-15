#[allow(dead_code)]
#[path = "helpers/permission_patterns.rs"]
pub(crate) mod permission_patterns;

pub(crate) use permission_patterns::{
    BoundArtifact, Bucket, Bundle, DeleteAllManifest, InferredOwner, ManifestBinding, OwnerSpec,
    PatternError, RepositoryScope, Role, StateDocument, SupplementalSide, TerminalPosition,
    ZeroOwnerManifest, compile_pattern, infer_git_ordering_role, infer_owner_role,
    infer_repository_scope, infer_witness_owner, is_valid_sha256, lexical_inventory_positions,
    manager_group, parse_strict_json, read_regular_file_within_root, read_utf8_file,
    regex_error_summary, sha256_hex, terminal_bucket_array,
    terminal_pattern_at as snapshot_pattern_at, validate_owner_spec,
};
use permission_patterns::{
    BoundInputs, InputClosureBuilder, OUTCOME_PASSED, ROLE_MANIFEST, ROLE_SETTINGS, ResultKind,
    ValidationResult, parse_audit_manifest_view, relative_within_root, resolve_audit_closure,
    resolve_inventory_closure, write_validation_result,
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
    "  permission-owner-audit --settings <settings-path> --manifest <manifest-path> [--binding <path>] [--graph-root <dir>] [--result-out <path>]\n",
    "  permission-owner-audit --settings <settings-path> --zero-owner-manifest <path> [--binding <path>] [--graph-root <dir>] [--result-out <path>]\n",
    "  permission-owner-audit --settings <promoted-settings-path> --delete-all-manifest <path> [--graph-root <dir>] [--result-out <path>]\n",
    "  permission-owner-audit --settings <settings-path> --owner <top-level-executable> [--after <inventory-cursor>] [--graph-root <dir>] [--result-out <path>]\n",
    "  permission-owner-audit --help\n",
    "\n",
    "Audit declared terminal permission owners, roles, ordering, and finite discovery coverage,\n",
    "verify that deleted owners retain no owned entry before sealing or after promotion,\n",
    "or list bounded terminal pattern inventory candidates by top-level executable token.\n",
    "\n",
    "Options:\n",
    "  --after <cursor>         Resume inventory strictly after a candidate in the same exact\n",
    "                           settings snapshot and owner inventory. Valid only with `--owner`\n",
    "                           and accepted at most once\n",
    "  --binding <path>         Apply a transient position rebinding in memory. Valid with `--manifest`\n",
    "                           or `--zero-owner-manifest`. The reviewed manifest file is never rewritten\n",
    "  --delete-all-manifest <path>\n",
    "                           Verify after promotion that a delete-all owner retains nothing\n",
    "  --graph-root <dir>       Anchor input-closure resolution and containment to one bundle graph root\n",
    "  --help                   Print help. Must be used alone\n",
    "  --manifest <path>        Audit entries declared by the canonical manifest schema\n",
    "  --owner <token>          List inventory candidates for one top-level executable token.\n",
    "                           Mutually exclusive with `--manifest`. The token must match\n",
    "                           `[A-Za-z0-9_.+-]+`\n",
    "  --result-out <path>      Write hash-bound reviewed workflow evidence to a new file. Requires\n",
    "                           `--graph-root`, and with `--owner` a complete inventory without `--after`\n",
    "  --settings <path>        Read Zed settings containing terminal permission buckets\n",
    "  --zero-owner-manifest <path>\n",
    "                           Verify before sealing that the deleted owners retain no owned entry\n",
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
    "Zero-owner manifest schema (unknown fields are rejected):\n",
    "  {\n",
    "    \"settings_sha256\": \"<exact-candidate-settings-sha256>\",\n",
    "    \"inventory_owner\": \"<top-level-executable>\",\n",
    "    \"excluded_candidates\": [ { \"bucket\": \"...\", \"index\": 0, \"sha256\": \"...\", \"owner\": \"...\", \"witness\": \"...\", \"reason\": \"...\" } ],\n",
    "    \"retained_owner_entries\": [ { \"bucket\": \"...\", \"index\": 0, \"sha256\": \"...\", \"owner_operation_id\": \"...\", \"witness\": \"...\" } ]\n",
    "  }\n",
    "  It declares no entries, so every recomputed lexical hit must be classified exactly once.\n",
    "  Each exclusion witness must infer outside `inventory_owner`, and each retained witness inside it.\n",
    "  The recomputed hit set and the declared classification set must be equal in both directions.\n",
    "  A raw `--owner` inventory records `inventory_query` evidence and never satisfies this requirement.\n",
    "\n",
    "Delete-all manifest schema (unknown fields are rejected):\n",
    "  It binds the sealed bundle by path and SHA-256, restates the promoted authorized scopes,\n",
    "  names the deleted owner IDs, and declares every absent baseline and lexically hidden member.\n",
    "  Verification requires exact promoted-scope identity, byte-exact absence of every declared and\n",
    "  hidden member across the complete arrays, zero owner-owned entries in a fresh inventory,\n",
    "  byte-identical retained exclusions, and both-direction classification equality.\n",
    "  Absence scans run independently of any remainder comparison.\n",
    "\n",
    "Evidence:\n",
    "  `--result-out` records the evaluator, outcome, bound manifest, settings, binding, inventory owner,\n",
    "  and a complete input closure over every file read, hashed with a length-prefixed encoding.\n",
    "  Evidence is hash-bound reviewed workflow evidence. It does not prove that this tool ran and is not authenticated.\n",
    "  Existing result paths are never overwritten.\n",
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

const MAX_DISPLAY_ID_CHARACTERS: usize = 80;
const MAX_INVENTORY_HITS: usize = 100;
const MAX_PATTERN_CHARACTERS: usize = 999;
const MAX_PREVIEW_CHARACTERS: usize = 160;
const MAX_REPORTED_FINDINGS: usize = 10;
const STATUS_ERROR: u8 = 2;
const STATUS_FINDINGS: u8 = 1;
const STATUS_SUCCESS: u8 = 0;

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
    DeleteAll {
        manifest: PathBuf,
    },
    Inventory {
        after: Option<InventoryCursor>,
        owner: String,
    },
    ManifestAudit {
        binding: Option<PathBuf>,
        manifest: PathBuf,
    },
    ZeroOwner {
        binding: Option<PathBuf>,
        manifest: PathBuf,
    },
}

struct Arguments {
    graph_root: Option<PathBuf>,
    operation: Operation,
    result_out: Option<PathBuf>,
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
    let mut binding = None;
    let mut delete_all_manifest = None;
    let mut graph_root = None;
    let mut manifest = None;
    let mut owner = None;
    let mut result_out = None;
    let mut settings = None;
    let mut zero_owner_manifest = None;

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
            "--binding" => {
                if binding.is_some() {
                    return Err("Option `--binding` may be specified only once".to_owned());
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--binding` requires a path".to_owned());
                };
                binding = Some(PathBuf::from(path));
            }
            "--delete-all-manifest" => {
                if delete_all_manifest.is_some() {
                    return Err(
                        "Option `--delete-all-manifest` may be specified only once".to_owned()
                    );
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--delete-all-manifest` requires a path".to_owned());
                };
                delete_all_manifest = Some(PathBuf::from(path));
            }
            "--graph-root" => {
                if graph_root.is_some() {
                    return Err("Option `--graph-root` may be specified only once".to_owned());
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--graph-root` requires a directory".to_owned());
                };
                graph_root = Some(PathBuf::from(path));
            }
            "--result-out" => {
                if result_out.is_some() {
                    return Err("Option `--result-out` may be specified only once".to_owned());
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--result-out` requires a path".to_owned());
                };
                result_out = Some(PathBuf::from(path));
            }
            "--zero-owner-manifest" => {
                if zero_owner_manifest.is_some() {
                    return Err(
                        "Option `--zero-owner-manifest` may be specified only once".to_owned()
                    );
                }
                let Some(path) = arguments.next() else {
                    return Err("Option `--zero-owner-manifest` requires a path".to_owned());
                };
                zero_owner_manifest = Some(PathBuf::from(path));
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
    let selected = [
        delete_all_manifest.is_some(),
        manifest.is_some(),
        owner.is_some(),
        zero_owner_manifest.is_some(),
    ]
    .into_iter()
    .filter(|value| *value)
    .count();
    if selected > 1 {
        return Err(
            "Options `--delete-all-manifest`, `--manifest`, `--owner`, and `--zero-owner-manifest` are mutually exclusive"
                .to_owned(),
        );
    }
    if after.is_some() && owner.is_none() {
        return Err("Option `--after` is valid only with `--owner`".to_owned());
    }
    if binding.is_some() && manifest.is_none() && zero_owner_manifest.is_none() {
        return Err(
            "Option `--binding` is valid only with `--manifest` or `--zero-owner-manifest`"
                .to_owned(),
        );
    }
    if result_out.is_some() && graph_root.is_none() {
        return Err("Option `--result-out` requires `--graph-root`".to_owned());
    }
    if after.is_some() && result_out.is_some() {
        return Err("Option `--result-out` is valid only for a complete inventory".to_owned());
    }

    let operation = if let Some(manifest) = delete_all_manifest {
        Operation::DeleteAll { manifest }
    } else if let Some(manifest) = zero_owner_manifest {
        Operation::ZeroOwner { binding, manifest }
    } else if let Some(manifest) = manifest {
        Operation::ManifestAudit { binding, manifest }
    } else if let Some(owner) = owner {
        Operation::Inventory { after, owner }
    } else {
        return Err(
            "Missing required operation `--delete-all-manifest <path>`, `--manifest <manifest-path>`, `--owner <top-level-executable>`, or `--zero-owner-manifest <path>`"
                .to_owned(),
        );
    };

    Ok(ParsedArguments::Run(Arguments {
        graph_root,
        operation,
        result_out,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ZeroOwnerReport {
    pub(crate) exclusion_count: usize,
    pub(crate) inventory_owner: String,
    pub(crate) retained_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DeleteAllReport {
    pub(crate) absent_count: usize,
    pub(crate) exclusion_count: usize,
    pub(crate) inventory_owner: String,
    pub(crate) owner_count: usize,
}

fn parse_settings_object(settings_json: &str) -> Result<Value, String> {
    let settings: Value = serde_json::from_str(settings_json)
        .map_err(|error| json_error_summary(&error, "settings"))?;
    if !settings.is_object() {
        return Err("Settings must parse as a JSON object".to_owned());
    }

    Ok(settings)
}

fn bind_position(
    binding: Option<&ManifestBinding>,
    position: TerminalPosition,
) -> TerminalPosition {
    binding
        .and_then(|binding| binding.remapped(position))
        .unwrap_or(position)
}

/// Confirm one declared classification against the exact pattern at its position, its normalized
/// witness, and the independently inferred owner of that witness.
fn verify_classification(
    settings: &Value,
    position: TerminalPosition,
    sha256: &str,
    witness: &str,
    inventory_owner: &str,
    expect_inside: bool,
    label: &str,
) -> Result<(), String> {
    let entry = snapshot_pattern_at(settings, position)?;
    if sha256_hex(entry.pattern.as_bytes()) != sha256 {
        return Err(format!(
            "The {label} at `{}` is not byte-identical to its declared identity",
            position.label()
        ));
    }
    let regex = compile_pattern(&entry.pattern, entry.case_sensitive).map_err(|error| {
        let detail = match error {
            PatternError::Empty => "the pattern is empty".to_owned(),
            PatternError::Invalid(error) => regex_error_summary(&error),
        };
        format!(
            "Failed to compile the {label} at `{}`. {detail}",
            position.label()
        )
    })?;
    if !regex.is_match(witness) {
        return Err(format!(
            "The {label} witness at `{}` does not match its pattern",
            position.label()
        ));
    }
    let inferred = infer_witness_owner(witness).map_err(|error| {
        format!(
            "The {label} witness at `{}` is unsupported or ambiguous. {error}",
            position.label()
        )
    })?;
    let inside = inferred.inventory_owner == inventory_owner;
    if inside != expect_inside {
        return Err(format!(
            "The {label} witness at `{}` infers owner `{}`, which is {} the inventory owner `{inventory_owner}`",
            position.label(),
            inferred.owner,
            if inside { "inside" } else { "outside" }
        ));
    }

    Ok(())
}

/// Prove that every deleted owner retains no owned entry in the candidate. Each recomputed lexical
/// hit must be classified exactly once, so a nonzero hit count never stands in for the claim.
pub(crate) fn verify_zero_owner(
    settings_json: &str,
    manifest_bytes: &[u8],
    binding: Option<&ManifestBinding>,
) -> Result<ZeroOwnerReport, String> {
    let manifest: ZeroOwnerManifest = parse_strict_json(manifest_bytes, "Zero-owner manifest")?;
    validate_owner(&manifest.inventory_owner)?;

    let expected_sha256 = binding
        .map(|binding| binding.settings_sha256.clone())
        .unwrap_or_else(|| manifest.settings_sha256.clone());
    if !is_valid_sha256(&expected_sha256) {
        return Err(
            "The zero-owner manifest settings SHA-256 must be 64 lowercase hexadecimal characters"
                .to_owned(),
        );
    }
    if settings_sha256(settings_json) != expected_sha256 {
        return Err(
            "The zero-owner manifest does not bind the exact candidate settings bytes".to_owned(),
        );
    }

    let settings = parse_settings_object(settings_json)?;
    let recomputed = lexical_inventory_positions(&settings, &manifest.inventory_owner)?;

    let mut declared = BTreeSet::new();
    for exclusion in &manifest.excluded_candidates {
        if exclusion.reason.trim().is_empty() {
            return Err("Every zero-owner exclusion requires a nonempty reason".to_owned());
        }
        let position = bind_position(
            binding,
            TerminalPosition {
                bucket: exclusion.bucket,
                index: exclusion.index,
            },
        );
        if !declared.insert(position) {
            return Err(format!(
                "Position `{}` is classified more than once",
                position.label()
            ));
        }
        verify_classification(
            &settings,
            position,
            &exclusion.sha256,
            &exclusion.witness,
            &manifest.inventory_owner,
            false,
            "outside-owner exclusion",
        )?;
    }
    for retained in &manifest.retained_owner_entries {
        if retained.owner_operation_id.is_empty() {
            return Err("Every retained owner entry must name its owner-spec operation".to_owned());
        }
        let position = bind_position(
            binding,
            TerminalPosition {
                bucket: retained.bucket,
                index: retained.index,
            },
        );
        if !declared.insert(position) {
            return Err(format!(
                "Position `{}` is classified more than once",
                position.label()
            ));
        }
        verify_classification(
            &settings,
            position,
            &retained.sha256,
            &retained.witness,
            &manifest.inventory_owner,
            true,
            "retained owner entry",
        )?;
    }

    let undeclared = recomputed.difference(&declared).collect::<Vec<_>>();
    if let Some(position) = undeclared.first() {
        return Err(format!(
            "Lexical candidate `{}` is not classified by the zero-owner manifest ({} unclassified)",
            position.label(),
            undeclared.len()
        ));
    }
    let missing = declared.difference(&recomputed).collect::<Vec<_>>();
    if let Some(position) = missing.first() {
        return Err(format!(
            "Declared classification `{}` is not a current lexical candidate ({} stale)",
            position.label(),
            missing.len()
        ));
    }

    Ok(ZeroOwnerReport {
        exclusion_count: manifest.excluded_candidates.len(),
        inventory_owner: manifest.inventory_owner,
        retained_count: manifest.retained_owner_entries.len(),
    })
}

fn read_bound_graph_artifact(
    base: &Path,
    artifact: &BoundArtifact,
    description: &str,
) -> Result<Vec<u8>, String> {
    let bytes = read_regular_file_within_root(base, &artifact.path, description)?;
    if sha256_hex(&bytes) != artifact.sha256 {
        return Err(format!(
            "The bound {description} does not match its recorded SHA-256"
        ));
    }

    Ok(bytes)
}

/// Prove after promotion that a delete-all owner retains nothing, including every lexically hidden
/// member, using byte-exact absence scans over the complete arrays.
pub(crate) fn verify_delete_all(
    settings_json: &str,
    manifest_bytes: &[u8],
    manifest_parent: &Path,
) -> Result<DeleteAllReport, String> {
    let manifest: DeleteAllManifest = parse_strict_json(manifest_bytes, "Delete-all manifest")?;
    validate_owner(&manifest.inventory_owner)?;
    if !is_valid_sha256(&manifest.settings_sha256) || !is_valid_sha256(&manifest.bundle_sha256) {
        return Err(
            "Delete-all manifest SHA-256 values must be 64 lowercase hexadecimal characters"
                .to_owned(),
        );
    }
    if settings_sha256(settings_json) != manifest.settings_sha256 {
        return Err("The delete-all manifest does not bind the exact promoted settings".to_owned());
    }

    let bundle_bytes =
        read_regular_file_within_root(manifest_parent, &manifest.bundle_file, "sealed bundle")?;
    if sha256_hex(&bundle_bytes) != manifest.bundle_sha256 {
        return Err("The sealed bundle does not match its recorded SHA-256".to_owned());
    }
    let bundle: Bundle = parse_strict_json(&bundle_bytes, "Bundle")?;
    let bundle_parent = manifest_parent
        .join(&manifest.bundle_file)
        .parent()
        .unwrap_or(manifest_parent)
        .to_owned();

    let state_bytes = read_bound_graph_artifact(&bundle_parent, &bundle.state, "state manifest")?;
    let state: StateDocument = parse_strict_json(&state_bytes, "State manifest")?;
    let spec_bytes = read_bound_graph_artifact(&bundle_parent, &bundle.owner_spec, "owner spec")?;
    let spec: OwnerSpec = parse_strict_json(&spec_bytes, "Owner spec")?;
    validate_owner_spec(&spec)?;

    if manifest.promoted_scopes != state.scopes {
        return Err(
            "The delete-all manifest promoted scopes differ from the captured authorized scopes"
                .to_owned(),
        );
    }

    let derived_owners = spec
        .owners
        .iter()
        .filter(|owner| {
            owner.inventory_owner == manifest.inventory_owner && owner.candidate_members.is_empty()
        })
        .collect::<Vec<_>>();
    let derived_ids = derived_owners
        .iter()
        .map(|owner| owner.id.clone())
        .collect::<BTreeSet<_>>();
    let declared_ids = manifest
        .deleted_owner_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if derived_ids != declared_ids {
        return Err(
            "The declared deleted owner IDs differ from the owner-spec operations this manifest proves empty"
                .to_owned(),
        );
    }
    if derived_ids.is_empty() {
        return Err(
            "A delete-all verification requires at least one owner operation with no candidate member"
                .to_owned(),
        );
    }

    let supplemental_ids = spec
        .supplemental
        .iter()
        .filter(|record| record.side == SupplementalSide::Baseline)
        .map(|record| record.member_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut required_plain = BTreeMap::new();
    let mut required_hidden = BTreeMap::new();
    for owner in &derived_owners {
        for member in &owner.baseline_members {
            let pattern = state
                .patterns
                .iter()
                .find(|pattern| pattern.id == *member)
                .ok_or_else(|| {
                    format!(
                        "Deleted owner member `{}` is absent from the captured state",
                        display_id(member)
                    )
                })?;
            if supplemental_ids.contains(member.as_str()) {
                required_hidden.insert(member.clone(), pattern.sha256.clone());
            } else {
                required_plain.insert(member.clone(), pattern.sha256.clone());
            }
        }
    }

    let settings = parse_settings_object(settings_json)?;
    let mut present = BTreeSet::new();
    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let values = terminal_bucket_array(&settings, bucket)?;
        for index in 0..values.len() {
            let entry = snapshot_pattern_at(&settings, TerminalPosition { bucket, index })?;
            present.insert(sha256_hex(entry.pattern.as_bytes()));
        }
    }

    for (declared, required, label) in [
        (
            &manifest.absent_baseline_members,
            &required_plain,
            "baseline member",
        ),
        (
            &manifest.absent_supplemental_members,
            &required_hidden,
            "lexically hidden baseline member",
        ),
    ] {
        let declared_map = declared
            .iter()
            .map(|member| (member.id.clone(), member.sha256.clone()))
            .collect::<BTreeMap<_, _>>();
        if declared_map != *required {
            return Err(format!(
                "The declared absent {label} set differs from the owner-spec and state membership"
            ));
        }
        for (id, sha256) in required {
            if present.contains(sha256) {
                return Err(format!(
                    "The {label} `{}` is still present in the promoted settings",
                    display_id(id)
                ));
            }
        }
    }

    let recomputed = lexical_inventory_positions(&settings, &manifest.inventory_owner)?;
    let mut declared_positions = BTreeSet::new();
    for exclusion in &manifest.retained_exclusions {
        if exclusion.reason.trim().is_empty() {
            return Err("Every retained exclusion requires a nonempty reason".to_owned());
        }
        let position = TerminalPosition {
            bucket: exclusion.bucket,
            index: exclusion.index,
        };
        if !declared_positions.insert(position) {
            return Err(format!(
                "Retained exclusion `{}` is declared more than once",
                position.label()
            ));
        }
        verify_classification(
            &settings,
            position,
            &exclusion.sha256,
            &exclusion.witness,
            &manifest.inventory_owner,
            false,
            "retained exclusion",
        )?;
    }
    let undeclared = recomputed
        .difference(&declared_positions)
        .collect::<Vec<_>>();
    if let Some(position) = undeclared.first() {
        return Err(format!(
            "Lexical candidate `{}` remains after promotion and is not a classified exclusion ({} unclassified)",
            position.label(),
            undeclared.len()
        ));
    }
    let missing = declared_positions
        .difference(&recomputed)
        .collect::<Vec<_>>();
    if let Some(position) = missing.first() {
        return Err(format!(
            "Retained exclusion `{}` is not a current lexical candidate ({} stale)",
            position.label(),
            missing.len()
        ));
    }

    Ok(DeleteAllReport {
        absent_count: required_plain.len() + required_hidden.len(),
        exclusion_count: manifest.retained_exclusions.len(),
        inventory_owner: manifest.inventory_owner,
        owner_count: derived_ids.len(),
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

fn report_zero_owner(stdout: &mut dyn Write, report: &ZeroOwnerReport) -> Result<(), String> {
    writeln!(
        stdout,
        "Verified zero `{}` owner entries with {} outside-owner {} and {} retained owner {}",
        report.inventory_owner,
        report.exclusion_count,
        count_label(report.exclusion_count, "exclusion", "exclusions"),
        report.retained_count,
        count_label(report.retained_count, "entry", "entries")
    )
    .map_err(|error| format!("Failed to write zero-owner result to standard output:\n\n{error}"))
}

fn report_delete_all(stdout: &mut dyn Write, report: &DeleteAllReport) -> Result<(), String> {
    writeln!(
        stdout,
        "Verified {} deleted `{}` {} with {} absent {} and {} retained {}",
        report.owner_count,
        report.inventory_owner,
        count_label(report.owner_count, "owner", "owners"),
        report.absent_count,
        count_label(report.absent_count, "member", "members"),
        report.exclusion_count,
        count_label(report.exclusion_count, "exclusion", "exclusions")
    )
    .map_err(|error| format!("Failed to write delete-all result to standard output:\n\n{error}"))
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

/// Apply a transient rebinding to a reviewed manifest in memory. Only snapshot-dependent positions
/// and the settings hash change, so every semantic field keeps its reviewed value.
fn apply_manifest_binding(
    manifest_json: &str,
    binding: &ManifestBinding,
) -> Result<String, String> {
    binding.validate()?;
    let mut manifest: Value = serde_json::from_str(manifest_json)
        .map_err(|error| json_error_summary(&error, "manifest"))?;
    let object = manifest
        .as_object_mut()
        .ok_or_else(|| "The manifest must be a JSON object".to_owned())?;
    object.insert(
        "settings_sha256".to_owned(),
        Value::String(binding.settings_sha256.clone()),
    );

    if let Some(entries) = object.get_mut("entries").and_then(Value::as_array_mut) {
        for entry in entries {
            let id = entry
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| "A manifest entry must declare `id`".to_owned())?
                .to_owned();
            let Some(position) = binding.entry_position(&id) else {
                return Err(format!(
                    "The manifest binding does not rebind entry `{}`",
                    display_id(&id)
                ));
            };
            let entry = entry
                .as_object_mut()
                .ok_or_else(|| "A manifest entry must be an object".to_owned())?;
            entry.insert(
                "bucket".to_owned(),
                Value::String(position.bucket.label().to_owned()),
            );
            entry.insert("index".to_owned(), Value::from(position.index));
        }
    }

    for key in ["excluded_candidates", "retained_owner_entries"] {
        let Some(values) = object.get_mut(key).and_then(Value::as_array_mut) else {
            continue;
        };
        for value in values {
            let position = view_bound_position(value)?;
            let Some(target) = binding.remapped(position) else {
                return Err(format!(
                    "The manifest binding does not rebind position `{}`",
                    position.label()
                ));
            };
            let value = value
                .as_object_mut()
                .ok_or_else(|| "A manifest position must be an object".to_owned())?;
            value.insert(
                "bucket".to_owned(),
                Value::String(target.bucket.label().to_owned()),
            );
            value.insert("index".to_owned(), Value::from(target.index));
        }
    }

    serde_json::to_string(&manifest)
        .map_err(|error| format!("Failed to apply the manifest binding:\n\n{error}"))
}

fn view_bound_position(value: &Value) -> Result<TerminalPosition, String> {
    let bucket = value
        .get("bucket")
        .and_then(Value::as_str)
        .and_then(Bucket::parse)
        .ok_or_else(|| "A manifest position must declare a valid bucket".to_owned())?;
    let index = value
        .get("index")
        .and_then(Value::as_u64)
        .and_then(|index| usize::try_from(index).ok())
        .ok_or_else(|| "A manifest position must declare an index".to_owned())?;

    Ok(TerminalPosition { bucket, index })
}

fn load_manifest_binding(path: Option<&PathBuf>) -> Result<Option<ManifestBinding>, String> {
    let Some(path) = path else {
        return Ok(None);
    };
    let bytes = std::fs::read(path).map_err(|error| {
        format!(
            "Failed to read manifest binding `{}`:\n\n{error}",
            path.display()
        )
    })?;
    let binding: ManifestBinding = parse_strict_json(&bytes, "Manifest binding")?;
    binding.validate()?;

    Ok(Some(binding))
}

/// Record hash-bound reviewed workflow evidence for one audit route.
fn record_audit_evidence(
    arguments: &Arguments,
    kind: ResultKind,
    evaluator: &str,
    manifest: Option<&Path>,
    binding: Option<&Path>,
    inventory_owner: Option<&str>,
    counts: BTreeMap<String, u64>,
) -> Result<(), String> {
    let Some(result_out) = arguments.result_out.as_deref() else {
        return Ok(());
    };
    let Some(graph_root) = arguments.graph_root.as_deref() else {
        return Err("Option `--result-out` requires `--graph-root`".to_owned());
    };

    let mut builder = InputClosureBuilder::new(graph_root)?;
    match manifest {
        Some(manifest) => {
            resolve_audit_closure(&mut builder, manifest, &arguments.settings, binding)?
        }
        None => resolve_inventory_closure(&mut builder, &arguments.settings)?,
    }
    let closure = builder.finish()?;
    let single = |role: &str| {
        closure
            .records
            .iter()
            .find(|record| record.role == role)
            .map(|record| record.sha256.clone())
    };
    let overlay = match binding {
        Some(binding) => {
            let bytes = std::fs::read(binding)
                .map_err(|error| format!("Failed to read the manifest binding:\n\n{error}"))?;
            Some(BoundArtifact {
                path: relative_within_root(
                    graph_root,
                    &std::fs::canonicalize(binding).unwrap_or_else(|_| binding.to_owned()),
                )?,
                sha256: sha256_hex(&bytes),
            })
        }
        None => None,
    };

    write_validation_result(
        result_out,
        &ValidationResult {
            kind,
            evaluator: evaluator.to_owned(),
            outcome: OUTCOME_PASSED.to_owned(),
            bound_inputs: BoundInputs {
                manifest_sha256: single(ROLE_MANIFEST),
                catalog_sha256: None,
                settings_sha256: single(ROLE_SETTINGS),
                inventory_owner: inventory_owner.map(str::to_owned),
                overlay,
                input_closure: closure,
            },
            counts,
        },
    )
}

fn audit_counts<const N: usize>(entries: [(&str, usize); N]) -> BTreeMap<String, u64> {
    entries
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value as u64))
        .collect()
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
                Operation::Inventory {
                    ref after,
                    ref owner,
                } => {
                    let page = match inventory_json(&settings, owner, after.clone()) {
                        Ok(page) => page,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    if let Err(error) = record_audit_evidence(
                        &arguments,
                        ResultKind::InventoryQuery,
                        "domfiles-zed-settings-permission-owner-audit --owner",
                        None,
                        None,
                        Some(owner),
                        audit_counts([("lexical_candidates", page.total_count)]),
                    ) {
                        report_error(stderr, &error);
                        return STATUS_ERROR;
                    }
                    match report_inventory(stdout, &page) {
                        Ok(()) => STATUS_SUCCESS,
                        Err(error) => {
                            report_error(stderr, &error);
                            STATUS_ERROR
                        }
                    }
                }
                Operation::ManifestAudit {
                    ref binding,
                    ref manifest,
                } => {
                    let manifest_path = manifest.clone();
                    let manifest_json = match read_utf8_file(&manifest_path, "manifest") {
                        Ok(manifest) => manifest,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    let loaded_binding = match load_manifest_binding(binding.as_ref()) {
                        Ok(binding) => binding,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    let manifest_json = match loaded_binding
                        .as_ref()
                        .map(|binding| apply_manifest_binding(&manifest_json, binding))
                        .transpose()
                    {
                        Ok(rebound) => rebound.unwrap_or(manifest_json),
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    let inventory_owner = parse_audit_manifest_view(manifest_json.as_bytes())
                        .map(|view| view.inventory_owner)
                        .ok();
                    let report = match audit_json(&settings, &manifest_json) {
                        Ok(report) => report,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };

                    if report.finding_count == 0 {
                        if let Err(error) = record_audit_evidence(
                            &arguments,
                            ResultKind::OwnerAudit,
                            "domfiles-zed-settings-permission-owner-audit --manifest",
                            Some(&manifest_path),
                            binding.as_deref(),
                            inventory_owner.as_deref(),
                            audit_counts([
                                ("entries", report.entry_count),
                                ("owner_groups", report.owner_group_count),
                                ("buckets", report.bucket_count),
                            ]),
                        ) {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
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
                Operation::ZeroOwner {
                    ref binding,
                    ref manifest,
                } => {
                    let manifest_path = manifest.clone();
                    let manifest_bytes = match std::fs::read(&manifest_path) {
                        Ok(bytes) => bytes,
                        Err(error) => {
                            report_error(
                                stderr,
                                &format!(
                                    "Failed to read zero-owner manifest `{}`:\n\n{error}",
                                    manifest_path.display()
                                ),
                            );
                            return STATUS_ERROR;
                        }
                    };
                    let loaded_binding = match load_manifest_binding(binding.as_ref()) {
                        Ok(binding) => binding,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_ERROR;
                        }
                    };
                    let report = match verify_zero_owner(
                        &settings,
                        &manifest_bytes,
                        loaded_binding.as_ref(),
                    ) {
                        Ok(report) => report,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_FINDINGS;
                        }
                    };
                    if let Err(error) = record_audit_evidence(
                        &arguments,
                        ResultKind::CandidateInventory,
                        "domfiles-zed-settings-permission-owner-audit --zero-owner-manifest",
                        Some(&manifest_path),
                        binding.as_deref(),
                        Some(&report.inventory_owner),
                        audit_counts([
                            ("outside_owner_exclusions", report.exclusion_count),
                            ("retained_owner_entries", report.retained_count),
                        ]),
                    ) {
                        report_error(stderr, &error);
                        return STATUS_ERROR;
                    }
                    match report_zero_owner(stdout, &report) {
                        Ok(()) => STATUS_SUCCESS,
                        Err(error) => {
                            report_error(stderr, &error);
                            STATUS_ERROR
                        }
                    }
                }
                Operation::DeleteAll { ref manifest } => {
                    let manifest_path = manifest.clone();
                    let manifest_bytes = match std::fs::read(&manifest_path) {
                        Ok(bytes) => bytes,
                        Err(error) => {
                            report_error(
                                stderr,
                                &format!(
                                    "Failed to read delete-all manifest `{}`:\n\n{error}",
                                    manifest_path.display()
                                ),
                            );
                            return STATUS_ERROR;
                        }
                    };
                    let parent = manifest_path
                        .parent()
                        .unwrap_or_else(|| Path::new("."))
                        .to_owned();
                    let report = match verify_delete_all(&settings, &manifest_bytes, &parent) {
                        Ok(report) => report,
                        Err(error) => {
                            report_error(stderr, &error);
                            return STATUS_FINDINGS;
                        }
                    };
                    if let Err(error) = record_audit_evidence(
                        &arguments,
                        ResultKind::DeleteAllAudit,
                        "domfiles-zed-settings-permission-owner-audit --delete-all-manifest",
                        Some(&manifest_path),
                        None,
                        Some(&report.inventory_owner),
                        audit_counts([
                            ("absent_members", report.absent_count),
                            ("retained_exclusions", report.exclusion_count),
                            ("deleted_owners", report.owner_count),
                        ]),
                    ) {
                        report_error(stderr, &error);
                        return STATUS_ERROR;
                    }
                    match report_delete_all(stdout, &report) {
                        Ok(()) => STATUS_SUCCESS,
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

#[cfg(not(test))]
fn main() -> std::process::ExitCode {
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    std::process::ExitCode::from(run(std::env::args_os().skip(1), &mut stdout, &mut stderr))
}
