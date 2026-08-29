use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Deserializer, de};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const BUCKET_SHAPE: Shape = Shape::Array(&Shape::Object(&[
    ("case_sensitive", &LEAF_SHAPE),
    ("pattern", &LEAF_SHAPE),
]));
const COMPARISON_CASE_FIELDS: [&str; 3] = ["baseline", "candidate", "input"];
const COMPARISON_FIELDS: [&str; 1] = ["cases"];
const COMPARISON_SHAPE: Shape = Shape::Object(&[(
    "cases",
    &Shape::Array(&Shape::Object(&[
        ("baseline", &STATE_SHAPE),
        ("candidate", &STATE_SHAPE),
        ("input", &LEAF_SHAPE),
    ])),
)]);
const DECISION_CASE_FIELDS: [&str; 2] = ["expected", "input"];
const FETCH_FIELDS: [&str; 4] = ["always_allow", "always_confirm", "always_deny", "default"];
/// Ordered segments from a settings root to the fetch permission object
const FETCH_PATH: [&str; 4] = ["agent", "tool_permissions", "tools", "fetch"];
const FETCH_SHAPE: Shape = Shape::Object(&[
    ("always_allow", &BUCKET_SHAPE),
    ("always_confirm", &BUCKET_SHAPE),
    ("always_deny", &BUCKET_SHAPE),
    ("default", &LEAF_SHAPE),
]);
pub(crate) const HELP: &str = concat!(
    "Usage:\n",
    "  pattern-match --baseline-settings <path> --candidate-settings <path> --comparison-file <path>\n",
    "  pattern-match --help\n",
    "  pattern-match --layer-file <path> --settings <path>\n",
    "\n",
    "Verify the configured Zed fetch permission layer of one settings file against a declared case manifest, or compare a baseline and candidate settings file across one declared corpus\n",
    "\n",
    "Options:\n",
    "  --baseline-settings <path>   Read the baseline settings file of a comparison\n",
    "  --candidate-settings <path>  Read the candidate settings file of a comparison\n",
    "  --comparison-file <path>     Read the strict JSON comparison manifest\n",
    "  --help                       Print help. Must be used alone\n",
    "  --layer-file <path>          Read the strict JSON layer manifest\n",
    "  --settings <path>            Read the settings file of a layer evaluation\n",
    "\n",
    "Settings input:\n",
    "  Each settings file is strict UTF-8 JSON with an object root and no duplicate object keys\n",
    "  Every object along the `agent.tool_permissions.tools.fetch` path must exist. Fields outside the selected fetch object are ignored\n",
    "  Fetch object: {\"always_allow\":[<pattern>,…],\"always_confirm\":[<pattern>,…],\"always_deny\":[<pattern>,…],\"default\":\"allow|confirm|deny\"}\n",
    "  Pattern: {\"case_sensitive\":true|false,\"pattern\":\"<regex>\"}\n",
    "  `default` is required. Each bucket array is optional, and an absent array is empty\n",
    "  An empty pattern or a pattern over 1,000 Unicode scalars is a finding and is never compiled. Every remaining pattern compiles once with its configured case setting\n",
    "\n",
    "Strict UTF-8 JSON layer manifest:\n",
    "  Root: {\"decision_cases\":[<decision-case>,…],\"pattern_cases\":[<pattern-case>,…]}\n",
    "  Decision case: {\"expected\":<state>,\"input\":\"<url>\"}\n",
    "  Pattern case: {\"bucket\":\"always_allow|always_confirm|always_deny\",\"expected_match\":true|false,\"index\":0,\"input\":\"<url>\"}\n",
    "  State: {\"always_allow\":true|false,\"always_confirm\":true|false,\"always_deny\":true|false,\"decision\":\"allow|confirm|deny\"}\n",
    "  Both arrays are required. `decision_cases` is nonempty, and `pattern_cases` is empty only when the settings configure no pattern\n",
    "  Unknown fields are rejected, and every input is single-line inert text\n",
    "  Every configured pattern requires one matching and one nonmatching pattern case\n",
    "  Every nonempty bucket and the configured default require one decision case that identifies them as the deciding source\n",
    "  Each declared state must follow `deny`, `confirm`, `allow`, then `default` precedence\n",
    "\n",
    "Strict UTF-8 JSON comparison manifest:\n",
    "  Root: {\"cases\":[<comparison-case>,…]}\n",
    "  Comparison case: {\"baseline\":<state>,\"candidate\":<state>,\"input\":\"<url>\"}\n",
    "  The `cases` array is required and nonempty. Each state is complete and resolves against its own settings file\n",
    "  This route does not validate a repair from an invalid baseline\n",
    "\n",
    "Output:\n",
    "  A verified run writes one summary of the evaluated counts to standard output\n",
    "  Findings write the exact total, at most the first 100 finding details of the complete invocation, and the omitted count to standard error\n",
    "  Contract-invalid input writes one diagnostic to standard error\n",
    "  A required write that fails exits with status 2, and its diagnostic appears only while standard error still accepts output\n",
    "  Findings and diagnostics identify files by role, manifest cases by array and zero-based index, and settings patterns by bucket and zero-based index\n",
    "  They never echo a path, case input, manifest value, pattern, or settings value\n",
    "  A duplicate key names its containing object through declared field, array, and index segments alone\n",
    "  Each finding or diagnostic is limited to 512 UTF-8 bytes and complete standard error to 64 KiB\n",
    "\n",
    "Limitations:\n",
    "  Every selected file is read once. Nothing is written, no configuration is read from the environment, no request is made, no settings are discovered, and no case input is executed\n",
    "  Results establish the configured fetch layer of the selected files only. They do not establish Zed settings discovery or layering, redirect handling, sandbox host authorization, prompt display, or runtime network access\n",
    "  A verified comparison establishes the declared corpus rather than formal regex-language equivalence\n",
    "\n",
    "Exit statuses:\n",
    "  0  Every configured pattern and declared expectation passed, or help displayed\n",
    "  1  A configured pattern or a declared expectation failed\n",
    "  2  Contract-invalid input, invalid arguments, malformed input, a failed required write, or an unreadable file\n",
);
const LAYER_FIELDS: [&str; 2] = ["decision_cases", "pattern_cases"];
const LAYER_SHAPE: Shape = Shape::Object(&[
    (
        "decision_cases",
        &Shape::Array(&Shape::Object(&[
            ("expected", &STATE_SHAPE),
            ("input", &LEAF_SHAPE),
        ])),
    ),
    (
        "pattern_cases",
        &Shape::Array(&Shape::Object(&[
            ("bucket", &LEAF_SHAPE),
            ("expected_match", &LEAF_SHAPE),
            ("index", &LEAF_SHAPE),
            ("input", &LEAF_SHAPE),
        ])),
    ),
]);
const LEAF_SHAPE: Shape = Shape::Leaf;
/// Every Unicode scalar that ends a line, so a manifest input stays one reviewable line under any
/// reader that treats more than carriage return and line feed as a break
pub(crate) const LINE_BREAKS: [char; 10] = [
    '\u{a}', '\u{b}', '\u{c}', '\u{d}', '\u{1c}', '\u{1d}', '\u{1e}', '\u{85}', '\u{2028}',
    '\u{2029}',
];
const MAX_DETAIL_BYTES: usize = 512;
const MAX_PATTERN_SCALARS: usize = 1_000;
const MAX_REPORTED_FINDINGS: usize = 100;
const MAX_STANDARD_ERROR_BYTES: usize = 64 * 1024;
const NAME: &str = "pattern-match";
const PATTERN_CASE_FIELDS: [&str; 4] = ["bucket", "expected_match", "index", "input"];
const PATTERN_FIELDS: [&str; 2] = ["case_sensitive", "pattern"];
const SETTINGS_SHAPE: Shape = Shape::Path(&FETCH_PATH, &FETCH_SHAPE);
const STATE_FIELDS: [&str; 4] = ["always_allow", "always_confirm", "always_deny", "decision"];
const STATE_SHAPE: Shape = Shape::Object(&[
    ("always_allow", &LEAF_SHAPE),
    ("always_confirm", &LEAF_SHAPE),
    ("always_deny", &LEAF_SHAPE),
    ("decision", &LEAF_SHAPE),
]);
const STATUS_ERROR: u8 = 2;
const STATUS_FINDINGS: u8 = 1;
const STATUS_VERIFIED: u8 = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Bucket {
    Allow,
    Confirm,
    Deny,
}

impl Bucket {
    /// Ordered to match the documented settings-input processing order and the compiled bucket
    /// indexes, so both stay in step with the reported finding order
    pub(crate) const ALL: [Self; 3] = [Self::Allow, Self::Confirm, Self::Deny];

    fn index(self) -> usize {
        match self {
            Self::Allow => 0,
            Self::Confirm => 1,
            Self::Deny => 2,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Allow => "always_allow",
            Self::Confirm => "always_confirm",
            Self::Deny => "always_deny",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|bucket| bucket.label() == value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Decision {
    Allow,
    Confirm,
    Deny,
}

impl Decision {
    const ALL: [Self; 3] = [Self::Allow, Self::Confirm, Self::Deny];

    fn label(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Confirm => "confirm",
            Self::Deny => "deny",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|decision| decision.label() == value)
    }
}

/// One complete matched-bucket state and the final configured decision it produces
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    allow: bool,
    confirm: bool,
    decision: Decision,
    deny: bool,
}

impl State {
    fn follows_precedence(self, default: Decision) -> bool {
        self.decision == resolve_decision(self.allow, self.confirm, self.deny, default)
    }

    /// Whether this state identifies `source` as the single deciding source. A more restrictive
    /// bucket shadows a less restrictive one, so only the unshadowed flags are constrained
    fn identifies(self, source: Source, default: Decision) -> bool {
        match source {
            Source::Bucket(Bucket::Allow) => {
                self.allow && !self.confirm && !self.deny && self.decision == Decision::Allow
            }
            Source::Bucket(Bucket::Confirm) => {
                self.confirm && !self.deny && self.decision == Decision::Confirm
            }
            Source::Bucket(Bucket::Deny) => self.deny && self.decision == Decision::Deny,
            Source::Default => {
                !self.allow && !self.confirm && !self.deny && self.decision == default
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Source {
    Bucket(Bucket),
    Default,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Role {
    BaselineSettings,
    CandidateSettings,
    ComparisonManifest,
    LayerManifest,
    Settings,
}

impl Role {
    fn label(self) -> &'static str {
        match self {
            Self::BaselineSettings => "baseline settings",
            Self::CandidateSettings => "candidate settings",
            Self::ComparisonManifest => "comparison manifest",
            Self::LayerManifest => "layer manifest",
            Self::Settings => "settings",
        }
    }

    /// The schema this role’s document declares, which bounds every duplicate-key location to
    /// declared names
    fn shape(self) -> Shape {
        match self {
            Self::BaselineSettings | Self::CandidateSettings | Self::Settings => SETTINGS_SHAPE,
            Self::ComparisonManifest => COMPARISON_SHAPE,
            Self::LayerManifest => LAYER_SHAPE,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum RouteKind {
    Comparison,
    Layer,
}

impl RouteKind {
    fn label(self) -> &'static str {
        match self {
            Self::Comparison => "comparison",
            Self::Layer => "layer",
        }
    }
}

/// Alphabetized so the derived order matches the help option list a contract test compares
/// against this set
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Parameter {
    BaselineSettings,
    CandidateSettings,
    ComparisonFile,
    Help,
    LayerFile,
    Settings,
}

impl Parameter {
    pub(crate) const ALL: [Self; 6] = [
        Self::BaselineSettings,
        Self::CandidateSettings,
        Self::ComparisonFile,
        Self::Help,
        Self::LayerFile,
        Self::Settings,
    ];

    pub(crate) fn option(self) -> &'static str {
        match self {
            Self::BaselineSettings => "--baseline-settings",
            Self::CandidateSettings => "--candidate-settings",
            Self::ComparisonFile => "--comparison-file",
            Self::Help => "--help",
            Self::LayerFile => "--layer-file",
            Self::Settings => "--settings",
        }
    }

    pub(crate) fn route(self) -> Option<RouteKind> {
        match self {
            Self::BaselineSettings | Self::CandidateSettings | Self::ComparisonFile => {
                Some(RouteKind::Comparison)
            }
            Self::Help => None,
            Self::LayerFile | Self::Settings => Some(RouteKind::Layer),
        }
    }

    fn parse(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|parameter| parameter.option() == token)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Route {
    Comparison {
        baseline_settings: PathBuf,
        candidate_settings: PathBuf,
        comparison_file: PathBuf,
    },
    Help,
    Layer {
        layer_file: PathBuf,
        settings: PathBuf,
    },
}

/// Counts every finding while retaining only the details that can be reported, so an input with
/// many failures grows the exact total instead of the retained set
#[derive(Debug, Default)]
pub(crate) struct Findings {
    details: Vec<String>,
    total: usize,
}

impl Findings {
    pub(crate) fn details(&self) -> &[String] {
        &self.details
    }

    fn is_empty(&self) -> bool {
        self.total == 0
    }

    /// Builds a detail only while the retained set has room, so a finding past the bound costs one
    /// count rather than a materialized body
    pub(crate) fn push(&mut self, detail: impl FnOnce() -> String) {
        self.total += 1;
        if self.details.len() < MAX_REPORTED_FINDINGS {
            self.details.push(detail());
        }
    }

    pub(crate) fn total(&self) -> usize {
        self.total
    }
}

enum Report {
    Findings(Findings),
    Help,
    Verified(String),
}

/// Retains every object entry, including repeats, so a later scan can reject duplicate keys that
/// serde_json’s map types would otherwise collapse
#[derive(Debug)]
pub(crate) enum Json {
    Array(Vec<Json>),
    Bool(bool),
    Null,
    /// `Some` holds a nonnegative integer that fits in `u64`. Every other number is `None`, which
    /// is all the schema needs to reject a noninteger array index
    Number(Option<u64>),
    Object(Vec<(String, Json)>),
    String(String),
}

impl Json {
    fn entry(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(entries) => field(entries, key),
            _ => None,
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            Self::Array(_) => "an array",
            Self::Bool(_) => "a Boolean",
            Self::Null => "null",
            Self::Number(_) => "a number",
            Self::Object(_) => "an object",
            Self::String(_) => "a string",
        }
    }
}

impl<'de> Deserialize<'de> for Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonVisitor)
    }
}

struct JsonVisitor;

impl<'de> de::Visitor<'de> for JsonVisitor {
    type Value = Json;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any JSON value")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Json, E> {
        Ok(Json::Bool(value))
    }

    fn visit_f64<E: de::Error>(self, _value: f64) -> Result<Json, E> {
        Ok(Json::Number(None))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Json, E> {
        Ok(Json::Number(u64::try_from(value).ok()))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Json, E> {
        Ok(Json::Number(Some(value)))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Json, E> {
        Ok(Json::String(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Json, E> {
        Ok(Json::String(value))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Json, E> {
        Ok(Json::Null)
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Json, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some(entry) = access.next_element()? {
            entries.push(entry);
        }

        Ok(Json::Array(entries))
    }

    fn visit_map<A>(self, mut access: A) -> Result<Json, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some(entry) = access.next_entry()? {
            entries.push(entry);
        }

        Ok(Json::Object(entries))
    }
}

/// The structure one document role declares. A duplicate-key location descends this shape so every
/// rendered segment is a declared name rather than a key the inspected document supplies
#[derive(Clone, Copy, Debug)]
enum Shape {
    Array(&'static Shape),
    /// A declared value the schema does not descend into
    Leaf,
    Object(&'static [(&'static str, &'static Shape)]),
    /// A chain of single-field objects ending at the target shape, so one ordered path constant
    /// serves both projection and location
    Path(&'static [&'static str], &'static Shape),
}

#[derive(Clone, Copy, Debug)]
enum Segment {
    Field(&'static str),
    Index(usize),
}

/// Locates the object containing a duplicate key through declared field names, array names, and
/// zero-based indexes alone
#[derive(Debug)]
struct Location {
    /// Whether the schema declares every step from the document root, so the segments reach the
    /// containing object itself rather than its nearest declared ancestor
    declared: bool,
    segments: Vec<Segment>,
}

impl Location {
    /// The object being visited, before its ancestors prepend the segments that reach it
    fn here() -> Self {
        Self {
            declared: true,
            segments: Vec::new(),
        }
    }

    /// Prepends the segment reaching this location from its parent. An undeclared step drops every
    /// segment below it, so no location names a key the schema does not declare
    fn below(mut self, segment: Option<Segment>) -> Self {
        let Some(segment) = segment else {
            return Self {
                declared: false,
                segments: Vec::new(),
            };
        };
        self.segments.insert(0, segment);

        self
    }

    fn render(&self) -> String {
        let mut rendered = String::new();

        for segment in &self.segments {
            match segment {
                Segment::Field(name) => {
                    if !rendered.is_empty() {
                        rendered.push('.');
                    }
                    rendered.push_str(name);
                }
                Segment::Index(index) => rendered.push_str(&format!("[{index}]")),
            }
        }

        rendered
    }
}

pub(crate) struct FetchPattern {
    case_sensitive: bool,
    pattern: String,
}

pub(crate) struct FetchLayer {
    buckets: [Vec<FetchPattern>; 3],
    default: Decision,
}

impl FetchLayer {
    pub(crate) fn patterns(&self, bucket: Bucket) -> &[FetchPattern] {
        &self.buckets[bucket.index()]
    }

    pub(crate) fn total(&self) -> usize {
        self.buckets.iter().map(Vec::len).sum()
    }
}

pub(crate) struct CompiledLayer {
    buckets: [Vec<Regex>; 3],
    default: Decision,
}

impl CompiledLayer {
    pub(crate) fn patterns(&self, bucket: Bucket) -> &[Regex] {
        &self.buckets[bucket.index()]
    }

    fn observe(&self, input: &str) -> State {
        let matched = |bucket: Bucket| {
            self.patterns(bucket)
                .iter()
                .any(|regex| regex.is_match(input))
        };
        let allow = matched(Bucket::Allow);
        let confirm = matched(Bucket::Confirm);
        let deny = matched(Bucket::Deny);

        State {
            allow,
            confirm,
            decision: resolve_decision(allow, confirm, deny, self.default),
            deny,
        }
    }
}

struct DecisionCase {
    expected: State,
    input: String,
}

struct PatternCase {
    bucket: Bucket,
    expected_match: bool,
    index: usize,
    input: String,
}

struct LayerManifest {
    decision_cases: Vec<DecisionCase>,
    pattern_cases: Vec<PatternCase>,
}

struct ComparisonCase {
    baseline: State,
    candidate: State,
    input: String,
}

struct ComparisonManifest {
    cases: Vec<ComparisonCase>,
}

fn resolve_decision(allow: bool, confirm: bool, deny: bool, default: Decision) -> Decision {
    if deny {
        Decision::Deny
    } else if confirm {
        Decision::Confirm
    } else if allow {
        Decision::Allow
    } else {
        default
    }
}

fn field<'a>(entries: &'a [(String, Json)], key: &str) -> Option<&'a Json> {
    entries
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value)
}

/// Resolves the declared name and child shape of one object key, so a location segment can never
/// carry a name the schema does not declare at that position
fn declared_field(shape: Shape, key: &str) -> Option<(&'static str, Shape)> {
    match shape {
        Shape::Object(fields) => fields
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(name, child)| (*name, **child)),
        Shape::Path(segments, target) => match segments {
            [name, rest @ ..] if *name == key => Some((
                *name,
                if rest.is_empty() {
                    *target
                } else {
                    Shape::Path(rest, target)
                },
            )),
            _ => None,
        },
        Shape::Array(_) | Shape::Leaf => None,
    }
}

fn declared_entry(shape: Shape) -> Option<Shape> {
    match shape {
        Shape::Array(entry) => Some(*entry),
        Shape::Leaf | Shape::Object(_) | Shape::Path(..) => None,
    }
}

/// Finds the first duplicate object key in document order, checking each object’s own keys before
/// descending into its entries, so the reported location is deterministic
fn duplicate_key_location(value: &Json, shape: Option<Shape>) -> Option<Location> {
    match value {
        Json::Array(entries) => {
            let entry_shape = shape.and_then(declared_entry);

            entries.iter().enumerate().find_map(|(index, entry)| {
                let found = duplicate_key_location(entry, entry_shape)?;

                Some(found.below(entry_shape.map(|_| Segment::Index(index))))
            })
        }
        Json::Object(entries) => {
            let mut seen = BTreeSet::new();
            if entries.iter().any(|(key, _)| !seen.insert(key.as_str())) {
                return Some(Location::here());
            }

            entries.iter().find_map(|(key, entry)| {
                let declared = shape.and_then(|shape| declared_field(shape, key));
                let found = duplicate_key_location(entry, declared.map(|(_, child)| child))?;

                Some(found.below(declared.map(|(name, _)| Segment::Field(name))))
            })
        }
        _ => None,
    }
}

/// Reports where a duplicate key sits without naming it, because the key itself, its object’s
/// values, and any undeclared ancestor name could reproduce caller-selected content
fn duplicate_key_failure(role: Role, location: &Location) -> String {
    let label = role.label();
    let rendered = location.render();

    match (location.declared, rendered.is_empty()) {
        (true, true) => format!("The {label} JSON root contains a duplicate object key"),
        (true, false) => {
            format!("The {label} JSON contains a duplicate object key in `{rendered}`")
        }
        (false, true) => format!(
            "The {label} JSON contains a duplicate object key in an undeclared object below its root"
        ),
        (false, false) => format!(
            "The {label} JSON contains a duplicate object key in an undeclared object below `{rendered}`"
        ),
    }
}

fn join_terms<T: AsRef<str>>(terms: &[T], conjunction: &str) -> String {
    match terms {
        [] => String::new(),
        [only] => only.as_ref().to_owned(),
        [first, second] => format!("{} {conjunction} {}", first.as_ref(), second.as_ref()),
        [leading @ .., last] => {
            let head = leading
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>()
                .join(", ");

            format!("{head}, {conjunction} {}", last.as_ref())
        }
    }
}

fn count_of(count: usize, singular: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {singular}s")
    }
}

fn quoted(terms: &[&str], conjunction: &str) -> String {
    let quoted = terms
        .iter()
        .map(|term| format!("`{term}`"))
        .collect::<Vec<_>>();

    join_terms(&quoted, conjunction)
}

pub(crate) fn truncate_bytes(text: &str, limit: usize) -> String {
    if text.len() <= limit {
        return text.to_owned();
    }

    let mut end = limit;
    while !text.is_char_boundary(end) {
        end -= 1;
    }

    text[..end].to_owned()
}

fn is_option_name(token: &str) -> bool {
    token.strip_prefix("--").is_some_and(|name| {
        name.starts_with(|character: char| character.is_ascii_lowercase())
            && name
                .chars()
                .all(|character| character.is_ascii_lowercase() || character == '-')
    })
}

fn unsupported_argument(argument: &OsStr) -> String {
    // Classify the `--` prefix before requiring UTF-8, so a malformed option token is not reported
    // as a positional argument
    if !argument.to_string_lossy().starts_with("--") {
        return "Positional arguments are not supported".to_owned();
    }
    let Some(token) = argument.to_str() else {
        return format!("Unknown option. Run `{NAME} --help` for the supported options");
    };
    if token.contains('=') {
        return "An option and its value must be separated by a space".to_owned();
    }
    if is_option_name(token) {
        return format!("Unknown option `{token}`");
    }

    format!("Unknown option. Run `{NAME} --help` for the supported options")
}

fn route_options(route: RouteKind) -> Vec<&'static str> {
    Parameter::ALL
        .into_iter()
        .filter(|parameter| parameter.route() == Some(route))
        .map(Parameter::option)
        .collect()
}

fn route_requirement(route: RouteKind) -> String {
    format!(
        "The {} route requires {}",
        route.label(),
        quoted(&route_options(route), "and")
    )
}

pub(crate) fn parse_arguments<I>(arguments: I) -> Result<Route, String>
where
    I: IntoIterator<Item = OsString>,
{
    let mut arguments = arguments.into_iter();
    let mut help = false;
    let mut values: BTreeMap<Parameter, PathBuf> = BTreeMap::new();

    while let Some(argument) = arguments.next() {
        let Some(parameter) = argument.to_str().and_then(Parameter::parse) else {
            return Err(unsupported_argument(&argument));
        };
        if parameter == Parameter::Help {
            if help {
                return Err(repeated_option(parameter));
            }
            help = true;
            continue;
        }
        let Some(value) = arguments.next() else {
            return Err(format!(
                "The `{}` option requires a value",
                parameter.option()
            ));
        };
        if values.insert(parameter, PathBuf::from(value)).is_some() {
            return Err(repeated_option(parameter));
        }
    }

    if help {
        if values.is_empty() {
            return Ok(Route::Help);
        }

        return Err(format!(
            "The `{}` option must be used alone",
            Parameter::Help.option()
        ));
    }

    let selected = |route: RouteKind| {
        values
            .keys()
            .filter(|parameter| parameter.route() == Some(route))
            .count()
    };
    let comparison = selected(RouteKind::Comparison);
    let layer = selected(RouteKind::Layer);

    if comparison > 0 && layer > 0 {
        return Err("Options from different routes must not be combined".to_owned());
    }
    if comparison > 0 {
        if comparison < route_options(RouteKind::Comparison).len() {
            return Err(route_requirement(RouteKind::Comparison));
        }

        return Ok(Route::Comparison {
            baseline_settings: take(&mut values, Parameter::BaselineSettings)?,
            candidate_settings: take(&mut values, Parameter::CandidateSettings)?,
            comparison_file: take(&mut values, Parameter::ComparisonFile)?,
        });
    }
    if layer > 0 {
        if layer < route_options(RouteKind::Layer).len() {
            return Err(route_requirement(RouteKind::Layer));
        }

        return Ok(Route::Layer {
            layer_file: take(&mut values, Parameter::LayerFile)?,
            settings: take(&mut values, Parameter::Settings)?,
        });
    }

    Err(format!(
        "Select one route. Run `{NAME} --help` for the supported routes"
    ))
}

fn repeated_option(parameter: Parameter) -> String {
    format!("The `{}` option must not be repeated", parameter.option())
}

fn take(
    values: &mut BTreeMap<Parameter, PathBuf>,
    parameter: Parameter,
) -> Result<PathBuf, String> {
    values.remove(&parameter).ok_or_else(|| {
        parameter
            .route()
            .map_or_else(String::new, route_requirement)
    })
}

fn access_failure(kind: io::ErrorKind) -> &'static str {
    match kind {
        io::ErrorKind::NotFound => "does not exist",
        io::ErrorKind::PermissionDenied => "cannot be read with the current permissions",
        _ => "could not be read",
    }
}

fn read_file(path: &Path, role: Role) -> Result<Vec<u8>, String> {
    let label = role.label();
    let metadata = fs::metadata(path)
        .map_err(|error| format!("The {label} file {}", access_failure(error.kind())))?;
    if !metadata.is_file() {
        return Err(format!("The {label} file must be a regular file"));
    }

    fs::read(path).map_err(|error| format!("The {label} file {}", access_failure(error.kind())))
}

fn decode_utf8(bytes: Vec<u8>, role: Role) -> Result<String, String> {
    String::from_utf8(bytes).map_err(|_| format!("The {} file is not valid UTF-8", role.label()))
}

/// Reports only the serde_json error category and position, because an upstream message can quote
/// the offending field name or value
fn json_failure(error: &serde_json::Error, role: Role) -> String {
    let label = role.label();
    let summary = match error.classify() {
        serde_json::error::Category::Data => {
            format!("The {label} JSON does not match the required schema")
        }
        serde_json::error::Category::Eof => {
            format!("The {label} JSON ends before a complete value")
        }
        serde_json::error::Category::Io => format!("The {label} JSON could not be read"),
        serde_json::error::Category::Syntax => format!("The {label} JSON syntax is invalid"),
    };

    format!(
        "{summary} at line {}, column {}",
        error.line(),
        error.column()
    )
}

pub(crate) fn parse_json(text: &str, role: Role) -> Result<Json, String> {
    let document: Json = serde_json::from_str(text).map_err(|error| json_failure(&error, role))?;
    if let Some(location) = duplicate_key_location(&document, Some(role.shape())) {
        return Err(duplicate_key_failure(role, &location));
    }

    Ok(document)
}

fn require_object<'a>(value: &'a Json, subject: &str) -> Result<&'a [(String, Json)], String> {
    match value {
        Json::Object(entries) => Ok(entries),
        _ => Err(format!(
            "The {subject} must be an object rather than {}",
            value.kind()
        )),
    }
}

fn reject_unknown_fields(
    entries: &[(String, Json)],
    allowed: &[&str],
    subject: &str,
) -> Result<(), String> {
    if entries
        .iter()
        .any(|(key, _)| !allowed.contains(&key.as_str()))
    {
        return Err(format!(
            "The {subject} permits only {}",
            quoted(allowed, "and")
        ));
    }

    Ok(())
}

fn require_array<'a>(
    entries: &'a [(String, Json)],
    key: &str,
    subject: &str,
) -> Result<&'a [Json], String> {
    match field(entries, key) {
        Some(Json::Array(values)) => Ok(values),
        _ => Err(format!("The {subject} requires a `{key}` array")),
    }
}

fn require_nonempty_array<'a>(
    entries: &'a [(String, Json)],
    key: &str,
    subject: &str,
) -> Result<&'a [Json], String> {
    match require_array(entries, key, subject)? {
        [] => Err(format!("The {subject} requires a nonempty `{key}` array")),
        values => Ok(values),
    }
}

fn require_bool(entries: &[(String, Json)], key: &str, subject: &str) -> Result<bool, String> {
    match field(entries, key) {
        Some(Json::Bool(value)) => Ok(*value),
        _ => Err(format!("The {subject} requires a Boolean `{key}` value")),
    }
}

fn require_bucket(entries: &[(String, Json)], subject: &str) -> Result<Bucket, String> {
    match field(entries, "bucket").and_then(|value| match value {
        Json::String(text) => Bucket::parse(text),
        _ => None,
    }) {
        Some(bucket) => Ok(bucket),
        None => Err(format!(
            "The {subject} requires a `bucket` value of {}",
            quoted(&Bucket::ALL.map(Bucket::label), "or")
        )),
    }
}

fn require_decision(entries: &[(String, Json)], subject: &str) -> Result<Decision, String> {
    match field(entries, "decision").and_then(|value| match value {
        Json::String(text) => Decision::parse(text),
        _ => None,
    }) {
        Some(decision) => Ok(decision),
        None => Err(format!(
            "The {subject} requires a `decision` value of {}",
            quoted(&Decision::ALL.map(Decision::label), "or")
        )),
    }
}

fn require_index(entries: &[(String, Json)], subject: &str) -> Result<usize, String> {
    match field(entries, "index").and_then(|value| match value {
        Json::Number(Some(number)) => usize::try_from(*number).ok(),
        _ => None,
    }) {
        Some(index) => Ok(index),
        None => Err(format!(
            "The {subject} requires a nonnegative integer `index` value"
        )),
    }
}

fn require_input(entries: &[(String, Json)], subject: &str) -> Result<String, String> {
    let input = match field(entries, "input") {
        Some(Json::String(value)) => value,
        _ => return Err(format!("The {subject} requires a string `input` value")),
    };
    if input.contains(LINE_BREAKS) {
        return Err(format!(
            "The {subject} `input` value must not contain a line break"
        ));
    }

    Ok(input.clone())
}

fn require_state(value: &Json, subject: &str) -> Result<State, String> {
    let entries = require_object(value, subject)?;
    reject_unknown_fields(entries, &STATE_FIELDS, subject)?;

    Ok(State {
        allow: require_bool(entries, "always_allow", subject)?,
        confirm: require_bool(entries, "always_confirm", subject)?,
        decision: require_decision(entries, subject)?,
        deny: require_bool(entries, "always_deny", subject)?,
    })
}

/// Validates the fetch object in unknown-field, `default`, then bucket order, and each bucket by
/// ascending index, so the first reported projection error is deterministic
pub(crate) fn project_fetch_layer(document: &Json, role: Role) -> Result<FetchLayer, String> {
    let label = role.label();
    let mut fetch = require_object(document, &format!("{label} JSON root"))?;
    let mut current = document;

    for key in FETCH_PATH {
        let next = current.entry(key).ok_or_else(|| {
            format!("The {label} `agent.tool_permissions.tools.fetch` path is missing `{key}`")
        })?;
        fetch = require_object(next, &format!("{label} `{key}` value"))?;
        current = next;
    }

    reject_unknown_fields(fetch, &FETCH_FIELDS, &format!("{label} fetch object"))?;

    let default = match field(fetch, "default").and_then(|value| match value {
        Json::String(text) => Decision::parse(text),
        _ => None,
    }) {
        Some(decision) => decision,
        None => {
            return Err(format!(
                "The {label} fetch object requires a `default` value of {}",
                quoted(&Decision::ALL.map(Decision::label), "or")
            ));
        }
    };
    let mut buckets: [Vec<FetchPattern>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    for bucket in Bucket::ALL {
        let Some(value) = field(fetch, bucket.label()) else {
            continue;
        };
        let Json::Array(entries) = value else {
            return Err(format!(
                "The {label} `{}` value must be an array rather than {}",
                bucket.label(),
                value.kind()
            ));
        };

        for (index, entry) in entries.iter().enumerate() {
            let subject = format!("{label} `{}[{index}]` entry", bucket.label());
            let fields = require_object(entry, &subject)?;
            reject_unknown_fields(fields, &PATTERN_FIELDS, &subject)?;
            let case_sensitive = require_bool(fields, "case_sensitive", &subject)?;
            let pattern = match field(fields, "pattern") {
                Some(Json::String(value)) => value.clone(),
                _ => return Err(format!("The {subject} requires a string `pattern` value")),
            };
            buckets[bucket.index()].push(FetchPattern {
                case_sensitive,
                pattern,
            });
        }
    }

    Ok(FetchLayer { buckets, default })
}

fn parse_layer_manifest(document: &Json) -> Result<LayerManifest, String> {
    let label = Role::LayerManifest.label();
    let root = require_object(document, &format!("{label} JSON root"))?;
    reject_unknown_fields(root, &LAYER_FIELDS, &format!("{label} root"))?;

    let mut decision_cases = Vec::new();
    for (index, value) in require_nonempty_array(root, "decision_cases", label)?
        .iter()
        .enumerate()
    {
        let subject = format!("{label} `decision_cases[{index}]`");
        let entries = require_object(value, &subject)?;
        reject_unknown_fields(entries, &DECISION_CASE_FIELDS, &subject)?;
        let expected = field(entries, "expected")
            .ok_or_else(|| format!("The {subject} requires an `expected` state"))?;
        decision_cases.push(DecisionCase {
            expected: require_state(expected, &format!("{subject} `expected` state"))?,
            input: require_input(entries, &subject)?,
        });
    }

    // An empty `pattern_cases` array is structurally valid because its coverage depends on the
    // configured patterns, which the documented phase order resolves after settings projection
    let mut pattern_cases = Vec::new();
    for (index, value) in require_array(root, "pattern_cases", label)?
        .iter()
        .enumerate()
    {
        let subject = format!("{label} `pattern_cases[{index}]`");
        let entries = require_object(value, &subject)?;
        reject_unknown_fields(entries, &PATTERN_CASE_FIELDS, &subject)?;
        pattern_cases.push(PatternCase {
            bucket: require_bucket(entries, &subject)?,
            expected_match: require_bool(entries, "expected_match", &subject)?,
            index: require_index(entries, &subject)?,
            input: require_input(entries, &subject)?,
        });
    }

    Ok(LayerManifest {
        decision_cases,
        pattern_cases,
    })
}

fn parse_comparison_manifest(document: &Json) -> Result<ComparisonManifest, String> {
    let label = Role::ComparisonManifest.label();
    let root = require_object(document, &format!("{label} JSON root"))?;
    reject_unknown_fields(root, &COMPARISON_FIELDS, &format!("{label} root"))?;

    let mut cases = Vec::new();
    for (index, value) in require_nonempty_array(root, "cases", label)?
        .iter()
        .enumerate()
    {
        let subject = format!("{label} `cases[{index}]`");
        let entries = require_object(value, &subject)?;
        reject_unknown_fields(entries, &COMPARISON_CASE_FIELDS, &subject)?;
        let baseline = field(entries, "baseline")
            .ok_or_else(|| format!("The {subject} requires a `baseline` state"))?;
        let candidate = field(entries, "candidate")
            .ok_or_else(|| format!("The {subject} requires a `candidate` state"))?;
        cases.push(ComparisonCase {
            baseline: require_state(baseline, &format!("{subject} `baseline` state"))?,
            candidate: require_state(candidate, &format!("{subject} `candidate` state"))?,
            input: require_input(entries, &subject)?,
        });
    }

    Ok(ComparisonManifest { cases })
}

/// Validates manifest references, then declared-state precedence, then witness coverage, so the
/// first reported cross-file error is deterministic
fn validate_layer_references(
    manifest: &LayerManifest,
    settings: &FetchLayer,
) -> Result<(), String> {
    let label = Role::LayerManifest.label();

    for (index, case) in manifest.pattern_cases.iter().enumerate() {
        if case.index >= settings.patterns(case.bucket).len() {
            return Err(format!(
                "The {label} `pattern_cases[{index}]` index is outside the configured `{}` array",
                case.bucket.label()
            ));
        }
    }

    for (index, case) in manifest.decision_cases.iter().enumerate() {
        if !case.expected.follows_precedence(settings.default) {
            return Err(format!(
                "The {label} `decision_cases[{index}]` expected state declares a decision that does not follow `deny`, `confirm`, `allow`, then `default` precedence"
            ));
        }
    }

    for bucket in Bucket::ALL {
        for index in 0..settings.patterns(bucket).len() {
            for expected_match in [true, false] {
                let declared = manifest.pattern_cases.iter().any(|case| {
                    case.bucket == bucket
                        && case.index == index
                        && case.expected_match == expected_match
                });
                if !declared {
                    let polarity = if expected_match {
                        "matching"
                    } else {
                        "nonmatching"
                    };

                    return Err(format!(
                        "The {label} declares no {polarity} case for the configured `{}[{index}]` pattern. A pattern that cannot supply both polarities is unsupported by this workflow",
                        bucket.label()
                    ));
                }
            }
        }
    }

    for bucket in Bucket::ALL {
        if settings.patterns(bucket).is_empty() {
            continue;
        }
        let declared = manifest.decision_cases.iter().any(|case| {
            case.expected
                .identifies(Source::Bucket(bucket), settings.default)
        });
        if !declared {
            return Err(format!(
                "The {label} declares no decision case with `{}` as the deciding source. A fully shadowed bucket is unsupported by the ordinary change workflow",
                bucket.label()
            ));
        }
    }

    let declared = manifest
        .decision_cases
        .iter()
        .any(|case| case.expected.identifies(Source::Default, settings.default));
    if !declared {
        return Err(format!(
            "The {label} declares no decision case with the configured default as the deciding source. An unreachable default is unsupported by the ordinary change workflow"
        ));
    }

    Ok(())
}

fn validate_comparison_states(
    manifest: &ComparisonManifest,
    baseline: &FetchLayer,
    candidate: &FetchLayer,
) -> Result<(), String> {
    let label = Role::ComparisonManifest.label();

    for (index, case) in manifest.cases.iter().enumerate() {
        for (side, state, default) in [
            ("baseline", case.baseline, baseline.default),
            ("candidate", case.candidate, candidate.default),
        ] {
            if !state.follows_precedence(default) {
                return Err(format!(
                    "The {label} `cases[{index}]` `{side}` state declares a decision that does not follow `deny`, `confirm`, `allow`, then `default` precedence"
                ));
            }
        }
    }

    Ok(())
}

fn compilation_failure(error: &regex::Error) -> String {
    match error {
        regex::Error::CompiledTooBig(limit) => {
            format!("exceeds the {limit}-byte compiled regex size limit")
        }
        _ => "is not valid regex syntax".to_owned(),
    }
}

/// Compiles each configured pattern exactly once, in settings-input order, recording every
/// configuration finding in the caller’s budget. An empty pattern and a pattern over the
/// reviewability bound never reach the regex builder, so a rejected layer reports every
/// configuration finding instead of a compiled set whose indexes no longer align with the settings
/// arrays. A layer that contributes a finding compiles to `None`, because its remaining patterns
/// cannot answer an expectation
pub(crate) fn compile_fetch_layer(
    layer: &FetchLayer,
    role: Role,
    findings: &mut Findings,
) -> Option<CompiledLayer> {
    let label = role.label();
    let mut buckets: [Vec<Regex>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let counted = findings.total();

    for bucket in Bucket::ALL {
        for (index, pattern) in layer.patterns(bucket).iter().enumerate() {
            let subject = || format!("{label} `{}[{index}]` pattern", bucket.label());
            if pattern.pattern.is_empty() {
                findings.push(|| format!("The {} is empty", subject()));
                continue;
            }
            if pattern.pattern.chars().count() > MAX_PATTERN_SCALARS {
                findings.push(|| {
                    format!(
                        "The {} exceeds the 1,000-scalar reviewability bound",
                        subject()
                    )
                });
                continue;
            }
            match RegexBuilder::new(&pattern.pattern)
                .case_insensitive(!pattern.case_sensitive)
                .build()
            {
                Ok(regex) => buckets[bucket.index()].push(regex),
                Err(error) => {
                    findings.push(|| format!("The {} {}", subject(), compilation_failure(&error)));
                }
            }
        }
    }

    (findings.total() == counted).then_some(CompiledLayer {
        buckets,
        default: layer.default,
    })
}

/// Compiles both comparison layers against one retained-detail budget, so baseline findings consume
/// it before candidate findings and no later finding body is built
pub(crate) fn compile_comparison_layers(
    baseline: &FetchLayer,
    candidate: &FetchLayer,
    findings: &mut Findings,
) -> Option<(CompiledLayer, CompiledLayer)> {
    let compiled_baseline = compile_fetch_layer(baseline, Role::BaselineSettings, findings);
    let compiled_candidate = compile_fetch_layer(candidate, Role::CandidateSettings, findings);

    compiled_baseline.zip(compiled_candidate)
}

fn evaluate_layer(manifest: &LayerManifest, settings: &FetchLayer) -> Report {
    let label = Role::LayerManifest.label();
    let mut findings = Findings::default();
    let Some(compiled) = compile_fetch_layer(settings, Role::Settings, &mut findings) else {
        return Report::Findings(findings);
    };

    for (index, case) in manifest.pattern_cases.iter().enumerate() {
        let matched = compiled.patterns(case.bucket)[case.index].is_match(&case.input);
        if matched != case.expected_match {
            findings.push(|| {
                format!(
                    "The {label} `pattern_cases[{index}]` declared expectation disagrees with the configured `{}[{}]` pattern result",
                    case.bucket.label(),
                    case.index
                )
            });
        }
    }

    for (index, case) in manifest.decision_cases.iter().enumerate() {
        if compiled.observe(&case.input) != case.expected {
            findings.push(|| {
                format!(
                    "The {label} `decision_cases[{index}]` declared state disagrees with the configured result"
                )
            });
        }
    }

    if findings.is_empty() {
        return Report::Verified(format!(
            "Verified {}, {}, and {}",
            count_of(settings.total(), "configured pattern"),
            count_of(manifest.decision_cases.len(), "decision case"),
            count_of(manifest.pattern_cases.len(), "pattern case")
        ));
    }

    Report::Findings(findings)
}

fn evaluate_comparison(
    manifest: &ComparisonManifest,
    baseline: &FetchLayer,
    candidate: &FetchLayer,
) -> Report {
    let label = Role::ComparisonManifest.label();
    let mut findings = Findings::default();
    let Some((compiled_baseline, compiled_candidate)) =
        compile_comparison_layers(baseline, candidate, &mut findings)
    else {
        return Report::Findings(findings);
    };

    for (index, case) in manifest.cases.iter().enumerate() {
        for (side, declared, observed) in [
            (
                "baseline",
                case.baseline,
                compiled_baseline.observe(&case.input),
            ),
            (
                "candidate",
                case.candidate,
                compiled_candidate.observe(&case.input),
            ),
        ] {
            if declared != observed {
                findings.push(|| {
                    format!(
                        "The {label} `cases[{index}]` declared `{side}` state disagrees with the configured result"
                    )
                });
            }
        }
    }

    if findings.is_empty() {
        return Report::Verified(format!(
            "Verified {}, {}, and {}",
            count_of(baseline.total(), "baseline pattern"),
            count_of(candidate.total(), "candidate pattern"),
            count_of(manifest.cases.len(), "comparison case")
        ));
    }

    Report::Findings(findings)
}

/// Runs the validation phases in their documented order: arguments, file type and readability,
/// UTF-8 decoding, JSON parsing with duplicate-key detection, manifest structure, settings
/// projection, then cross-file references and coverage
fn execute<I>(arguments: I) -> Result<Report, String>
where
    I: IntoIterator<Item = OsString>,
{
    match parse_arguments(arguments)? {
        Route::Comparison {
            baseline_settings,
            candidate_settings,
            comparison_file,
        } => {
            let baseline_bytes = read_file(&baseline_settings, Role::BaselineSettings)?;
            let candidate_bytes = read_file(&candidate_settings, Role::CandidateSettings)?;
            let manifest_bytes = read_file(&comparison_file, Role::ComparisonManifest)?;

            let baseline_text = decode_utf8(baseline_bytes, Role::BaselineSettings)?;
            let candidate_text = decode_utf8(candidate_bytes, Role::CandidateSettings)?;
            let manifest_text = decode_utf8(manifest_bytes, Role::ComparisonManifest)?;

            let baseline_json = parse_json(&baseline_text, Role::BaselineSettings)?;
            let candidate_json = parse_json(&candidate_text, Role::CandidateSettings)?;
            let manifest_json = parse_json(&manifest_text, Role::ComparisonManifest)?;

            let manifest = parse_comparison_manifest(&manifest_json)?;

            let baseline = project_fetch_layer(&baseline_json, Role::BaselineSettings)?;
            let candidate = project_fetch_layer(&candidate_json, Role::CandidateSettings)?;

            validate_comparison_states(&manifest, &baseline, &candidate)?;

            Ok(evaluate_comparison(&manifest, &baseline, &candidate))
        }
        Route::Help => Ok(Report::Help),
        Route::Layer {
            layer_file,
            settings,
        } => {
            let manifest_bytes = read_file(&layer_file, Role::LayerManifest)?;
            let settings_bytes = read_file(&settings, Role::Settings)?;

            let manifest_text = decode_utf8(manifest_bytes, Role::LayerManifest)?;
            let settings_text = decode_utf8(settings_bytes, Role::Settings)?;

            let manifest_json = parse_json(&manifest_text, Role::LayerManifest)?;
            let settings_json = parse_json(&settings_text, Role::Settings)?;

            let manifest = parse_layer_manifest(&manifest_json)?;

            let layer = project_fetch_layer(&settings_json, Role::Settings)?;

            validate_layer_references(&manifest, &layer)?;

            Ok(evaluate_layer(&manifest, &layer))
        }
    }
}

fn diagnostic_line(text: &str) -> String {
    format!("{}\n", truncate_bytes(text, MAX_DETAIL_BYTES))
}

/// Drops trailing details until the total count, rendered details, and omitted count fit the
/// standard-error bound, so both counts survive truncation
pub(crate) fn render_findings(findings: &Findings) -> String {
    let total = findings.total();
    let mut shown = findings.details().len();

    loop {
        let mut rendered = diagnostic_line(&format!("{NAME}: {}", count_of(total, "finding")));
        for detail in &findings.details()[..shown] {
            rendered.push_str(&diagnostic_line(&format!("  {detail}")));
        }
        rendered.push_str(&diagnostic_line(&format!(
            "{NAME}: {} omitted",
            count_of(total - shown, "finding")
        )));

        if rendered.len() <= MAX_STANDARD_ERROR_BYTES || shown == 0 {
            return rendered;
        }
        shown -= 1;
    }
}

fn write_all(writer: &mut dyn Write, text: &str) -> bool {
    writer.write_all(text.as_bytes()).is_ok()
}

pub(crate) fn run<I>(arguments: I, stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8
where
    I: IntoIterator<Item = OsString>,
{
    match execute(arguments) {
        Ok(Report::Findings(findings)) => {
            if !write_all(stderr, &render_findings(&findings)) {
                return report_write_failure(stderr, "standard error");
            }

            STATUS_FINDINGS
        }
        Ok(Report::Help) => write_or_fail(stdout, stderr, HELP),
        Ok(Report::Verified(summary)) => write_or_fail(stdout, stderr, &diagnostic_line(&summary)),
        Err(diagnostic) => {
            write_all(stderr, &diagnostic_line(&format!("{NAME}: {diagnostic}")));

            STATUS_ERROR
        }
    }
}

fn write_or_fail(stdout: &mut dyn Write, stderr: &mut dyn Write, text: &str) -> u8 {
    if write_all(stdout, text) {
        return STATUS_VERIFIED;
    }

    report_write_failure(stderr, "standard output")
}

/// Reports a failed required write as an operational failure, leaving the diagnostic unwritten when
/// standard error is the stream that stopped accepting output
fn report_write_failure(stderr: &mut dyn Write, stream: &str) -> u8 {
    write_all(
        stderr,
        &diagnostic_line(&format!("{NAME}: Failed to write to {stream}")),
    );

    STATUS_ERROR
}

#[cfg(not(test))]
fn main() -> std::process::ExitCode {
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    std::process::ExitCode::from(run(std::env::args_os().skip(1), &mut stdout, &mut stderr))
}
