use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::{self, File},
    io::Read,
    path::{Component, Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Bucket {
    #[serde(rename = "always_allow")]
    Allow,
    #[serde(rename = "always_confirm")]
    Confirm,
    #[serde(rename = "always_deny")]
    Deny,
}

impl Bucket {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "always_allow" => Some(Self::Allow),
            "always_confirm" => Some(Self::Confirm),
            "always_deny" => Some(Self::Deny),
            _ => None,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Allow => "always_allow",
            Self::Confirm => "always_confirm",
            Self::Deny => "always_deny",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FetchPermissionField {
    AlwaysAllow,
    AlwaysConfirm,
    AlwaysDeny,
    Default,
}

impl FetchPermissionField {
    pub(crate) const ALL: [Self; 4] = [
        Self::AlwaysAllow,
        Self::AlwaysConfirm,
        Self::AlwaysDeny,
        Self::Default,
    ];

    pub(crate) fn is_pattern_bucket(self) -> bool {
        self != Self::Default
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::AlwaysAllow => "always_allow",
            Self::AlwaysConfirm => "always_confirm",
            Self::AlwaysDeny => "always_deny",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LayerTool {
    Fetch,
    Terminal,
}

impl LayerTool {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "fetch" => Some(Self::Fetch),
            "terminal" => Some(Self::Terminal),
            _ => None,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Fetch => "fetch",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Decision {
    Allow,
    Confirm,
    Deny,
}

impl Decision {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "allow" => Some(Self::Allow),
            "confirm" => Some(Self::Confirm),
            "deny" => Some(Self::Deny),
            _ => None,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Confirm => "confirm",
            Self::Deny => "deny",
        }
    }

    pub(crate) fn most_restrictive(self, other: Self) -> Self {
        self.max(other)
    }
}

#[derive(Debug)]
pub(crate) enum PatternError {
    Empty,
    Invalid(regex::Error),
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

pub(crate) fn regex_error_summary(error: &regex::Error) -> String {
    let message = error.to_string();

    message
        .lines()
        .rev()
        .find_map(|line| line.trim().strip_prefix("error: "))
        .unwrap_or("Failed to compile regex")
        .to_owned()
}

pub(crate) fn read_utf8_file(path: &Path, description: &str) -> Result<String, String> {
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactCatalog {
    pub(crate) candidate_sha256: String,
    pub(crate) state_sha256: String,
    pub(crate) patterns: Vec<ArtifactCatalogPattern>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactCatalogPattern {
    pub(crate) id: String,
    pub(crate) bucket: Bucket,
    pub(crate) source_index: usize,
    pub(crate) case_sensitive: bool,
    pub(crate) sha256: String,
    pub(crate) pattern_file: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LoadedArtifactCatalog {
    pub(crate) document: ArtifactCatalog,
    pub(crate) patterns: Vec<LoadedArtifactPattern>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LoadedArtifactPattern {
    pub(crate) definition: ArtifactCatalogPattern,
    pub(crate) pattern: String,
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

pub(crate) fn is_valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_catalog_artifact_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err("Catalog artifact paths must be nonempty and relative".to_owned());
    }
    if !path
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(
            "Catalog artifact paths must not contain root, parent, or current-directory components"
                .to_owned(),
        );
    }

    Ok(path)
}

pub(crate) fn validate_artifact_catalog(catalog: &ArtifactCatalog) -> Result<(), String> {
    if !is_valid_sha256(&catalog.candidate_sha256) {
        return Err(
            "Artifact catalog candidate SHA-256 must be 64 lowercase hexadecimal characters"
                .to_owned(),
        );
    }
    if !is_valid_sha256(&catalog.state_sha256) {
        return Err(
            "Artifact catalog state SHA-256 must be 64 lowercase hexadecimal characters".to_owned(),
        );
    }

    let mut ids = HashSet::new();
    let mut sources = HashSet::new();
    let mut artifact_paths = HashSet::new();
    for pattern in &catalog.patterns {
        if pattern.id.is_empty() {
            return Err("Artifact catalog pattern IDs must be nonempty".to_owned());
        }
        if !ids.insert(pattern.id.as_str()) {
            return Err("Artifact catalog pattern IDs must be unique".to_owned());
        }
        if !sources.insert((pattern.bucket, pattern.source_index)) {
            return Err(
                "Artifact catalog terminal bucket/source-index pairs must be unique".to_owned(),
            );
        }
        if !is_valid_sha256(&pattern.sha256) {
            return Err(
                "Artifact catalog pattern SHA-256 values must be 64 lowercase hexadecimal characters"
                    .to_owned(),
            );
        }
        let artifact_path = validate_catalog_artifact_path(&pattern.pattern_file)?;
        if !artifact_paths.insert(artifact_path) {
            return Err("Artifact catalog pattern paths must be unique".to_owned());
        }
    }

    Ok(())
}

fn artifact_catalog_json_error(error: &serde_json::Error) -> String {
    let summary = match error.classify() {
        serde_json::error::Category::Data => {
            "Artifact catalog JSON data does not match the required schema"
        }
        serde_json::error::Category::Eof => "Artifact catalog JSON ends before a complete value",
        serde_json::error::Category::Io => "Failed to read artifact catalog JSON",
        serde_json::error::Category::Syntax => "Artifact catalog JSON syntax is invalid",
    };
    format!(
        "{summary} at line {}, column {}",
        error.line(),
        error.column()
    )
}

pub(crate) fn parse_artifact_catalog(bytes: &[u8]) -> Result<ArtifactCatalog, String> {
    let catalog: ArtifactCatalog =
        serde_json::from_slice(bytes).map_err(|error| artifact_catalog_json_error(&error))?;
    validate_artifact_catalog(&catalog)?;

    Ok(catalog)
}

pub(crate) fn verify_artifact_catalog_binding(
    catalog: &ArtifactCatalog,
    candidate_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<(), String> {
    validate_artifact_catalog(catalog)?;
    if sha256_hex(candidate_bytes) != catalog.candidate_sha256 {
        return Err("Candidate SHA-256 does not match the artifact catalog".to_owned());
    }
    if sha256_hex(state_bytes) != catalog.state_sha256 {
        return Err("State SHA-256 does not match the artifact catalog".to_owned());
    }

    Ok(())
}

fn path_as_absolute(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        Ok(path.to_owned())
    } else {
        std::env::current_dir()
            .map(|directory| directory.join(path))
            .map_err(|error| format!("Failed to resolve the current directory:\n\n{error}"))
    }
}

fn read_regular_file_without_symlinks(path: &Path, description: &str) -> Result<Vec<u8>, String> {
    let absolute = path_as_absolute(path)?;
    let mut current = PathBuf::new();
    let mut path_metadata = None;

    for component in absolute.components() {
        current.push(component.as_os_str());
        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound && current == absolute {
                format!(
                    "Failed to open {description} file `{}`:\n\n{error}",
                    absolute.display()
                )
            } else {
                format!(
                    "Failed to inspect {description} path `{}`:\n\n{error}",
                    current.display()
                )
            }
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "The {description} path `{}` must not traverse a symbolic link",
                current.display()
            ));
        }
        path_metadata = Some(metadata);
    }

    let path_metadata = path_metadata.ok_or_else(|| {
        format!(
            "The {description} path `{}` does not identify a file",
            absolute.display()
        )
    })?;
    if !path_metadata.is_file() {
        return Err(format!(
            "The {description} file `{}` must be a regular file",
            absolute.display()
        ));
    }
    let mut file = File::open(&absolute).map_err(|error| {
        format!(
            "Failed to open {description} file `{}`:\n\n{error}",
            absolute.display()
        )
    })?;
    let file_metadata = file.metadata().map_err(|error| {
        format!(
            "Failed to inspect opened {description} file `{}`:\n\n{error}",
            absolute.display()
        )
    })?;
    if !file_metadata.is_file() {
        return Err(format!(
            "The {description} file `{}` must be a regular file",
            absolute.display()
        ));
    }
    #[cfg(unix)]
    if path_metadata.dev() != file_metadata.dev() || path_metadata.ino() != file_metadata.ino() {
        return Err(format!(
            "The {description} file `{}` changed while it was inspected",
            absolute.display()
        ));
    }

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|error| {
        format!(
            "Failed to read {description} file `{}`:\n\n{error}",
            absolute.display()
        )
    })?;
    Ok(bytes)
}

pub(crate) fn read_hashed_utf8_file(
    path: &Path,
    expected_sha256: &str,
    description: &str,
) -> Result<String, String> {
    if !is_valid_sha256(expected_sha256) {
        return Err(format!(
            "Expected {description} SHA-256 must be 64 lowercase hexadecimal characters"
        ));
    }
    let bytes = read_regular_file_without_symlinks(path, description)?;
    if sha256_hex(&bytes) != expected_sha256 {
        return Err(format!(
            "The {description} file SHA-256 does not match the artifact catalog"
        ));
    }
    String::from_utf8(bytes).map_err(|_| format!("The {description} file is not valid UTF-8"))
}

fn read_artifact_catalog_document(path: &Path) -> Result<(PathBuf, ArtifactCatalog), String> {
    let catalog_path = path_as_absolute(path)?;
    let bytes = read_regular_file_without_symlinks(&catalog_path, "artifact catalog")?;
    let document = parse_artifact_catalog(&bytes)?;

    Ok((catalog_path, document))
}

fn load_artifact_catalog_patterns(
    catalog_path: &Path,
    document: &ArtifactCatalog,
) -> Result<Vec<LoadedArtifactPattern>, String> {
    let base = catalog_path.parent().unwrap_or_else(|| Path::new("."));
    let mut patterns = Vec::with_capacity(document.patterns.len());

    for (index, definition) in document.patterns.iter().enumerate() {
        let relative = validate_catalog_artifact_path(&definition.pattern_file)?;
        let pattern = read_hashed_utf8_file(
            &base.join(relative),
            &definition.sha256,
            &format!("catalog pattern {}", index + 1),
        )?;
        patterns.push(LoadedArtifactPattern {
            definition: definition.clone(),
            pattern,
        });
    }

    Ok(patterns)
}

pub(crate) fn load_artifact_catalog(path: &Path) -> Result<LoadedArtifactCatalog, String> {
    let (catalog_path, document) = read_artifact_catalog_document(path)?;
    let patterns = load_artifact_catalog_patterns(&catalog_path, &document)?;

    Ok(LoadedArtifactCatalog { document, patterns })
}

pub(crate) fn load_bound_artifact_catalog(
    path: &Path,
    candidate_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<LoadedArtifactCatalog, String> {
    let (catalog_path, document) = read_artifact_catalog_document(path)?;
    verify_artifact_catalog_binding(&document, candidate_bytes, state_bytes)?;
    let patterns = load_artifact_catalog_patterns(&catalog_path, &document)?;

    Ok(LoadedArtifactCatalog { document, patterns })
}

pub(crate) struct CompiledPattern {
    pub(crate) id: String,
    pub(crate) bucket: Bucket,
    pub(crate) regex: Regex,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MatchState {
    pub(crate) allow: bool,
    pub(crate) confirm: bool,
    pub(crate) deny: bool,
}

impl MatchState {
    pub(crate) fn evaluate(input: &str, patterns: &[CompiledPattern]) -> Self {
        let mut state = Self::default();

        for pattern in patterns {
            if !pattern.regex.is_match(input) {
                continue;
            }

            match pattern.bucket {
                Bucket::Allow => state.allow = true,
                Bucket::Confirm => state.confirm = true,
                Bucket::Deny => state.deny = true,
            }
        }

        state
    }

    pub(crate) fn matched(self, bucket: Bucket) -> bool {
        match bucket {
            Bucket::Allow => self.allow,
            Bucket::Confirm => self.confirm,
            Bucket::Deny => self.deny,
        }
    }

    pub(crate) fn decision(self, default: Decision) -> Decision {
        if self.deny {
            Decision::Deny
        } else if self.confirm {
            Decision::Confirm
        } else if self.allow {
            Decision::Allow
        } else {
            default
        }
    }
}

pub(crate) struct BoundedIssues<T> {
    issues: Vec<T>,
    limit: usize,
    total_count: usize,
}

impl<T> BoundedIssues<T> {
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            issues: Vec::with_capacity(limit),
            limit,
            total_count: 0,
        }
    }

    pub(crate) fn push(&mut self, issue: T) {
        self.total_count += 1;
        if self.issues.len() < self.limit {
            self.issues.push(issue);
        }
    }

    pub(crate) fn issues(&self) -> &[T] {
        &self.issues
    }

    pub(crate) fn total_count(&self) -> usize {
        self.total_count
    }

    pub(crate) fn omitted_count(&self) -> usize {
        self.total_count - self.issues.len()
    }
}

// One shared wrapper-aware owner model serves every binary. Supplemental ownership cannot reuse the
// lexical audit, because a supplemental member is intentionally absent from its recomputed hit set

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Role {
    Discovery,
    Direct,
    Wrapped,
}

impl Role {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Discovery => "discovery",
            Self::Direct => "direct",
            Self::Wrapped => "wrapped",
        }
    }

    pub(crate) fn order(self) -> u8 {
        match self {
            Self::Discovery => 0,
            Self::Direct => 1,
            Self::Wrapped => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RepositoryScope {
    AgentWorktree,
    FixtureRepository,
    General,
}

impl RepositoryScope {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::AgentWorktree => "agent worktree",
            Self::FixtureRepository => "fixture repository",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InferredOwner {
    pub(crate) owner: String,
    pub(crate) role: Role,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InferredWitnessOwner {
    pub(crate) owner: String,
    pub(crate) inventory_owner: String,
    pub(crate) repository_scope: RepositoryScope,
    pub(crate) role: Role,
}

pub(crate) fn manager_group(owner: &str) -> &str {
    owner.split_once(':').map_or(owner, |(manager, _)| manager)
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

pub(crate) fn infer_repository_scope(witness: &str) -> RepositoryScope {
    let tokens = witness.split(' ').collect::<Vec<_>>();
    let Some(git_tokens) = git_tokens_after_wrappers(&tokens) else {
        return RepositoryScope::General;
    };

    parse_git_prefix(git_tokens)
        .map(|prefix| prefix.repository_scope)
        .unwrap_or(RepositoryScope::General)
}

pub(crate) fn infer_git_ordering_role(witness: &str) -> Option<Role> {
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

/// Resolve one normalized witness to its semantic owner, top-level inventory token, repository
/// scope, and role through the shared inference model. A witness carrying no recognized wrapper
/// resolves to `direct`, because leading assignments never change the role
pub(crate) fn infer_witness_owner(witness: &str) -> Result<InferredWitnessOwner, String> {
    let inferred = infer_owner_role(witness, &[])?;
    let inventory_owner = manager_group(&inferred.owner).to_owned();

    Ok(InferredWitnessOwner {
        owner: inferred.owner,
        inventory_owner,
        repository_scope: infer_repository_scope(witness),
        role: inferred.role,
    })
}

pub(crate) const TERMINAL_POINTER: [&str; 4] = ["agent", "tool_permissions", "tools", "terminal"];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct TerminalPosition {
    pub(crate) bucket: Bucket,
    pub(crate) index: usize,
}

impl TerminalPosition {
    pub(crate) fn label(self) -> String {
        format!("{}[{}]", self.bucket.label(), self.index)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalPattern {
    pub(crate) pattern: String,
    pub(crate) case_sensitive: bool,
}

pub(crate) fn terminal_buckets(
    settings: &Value,
) -> Result<&serde_json::Map<String, Value>, String> {
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

pub(crate) fn terminal_bucket_array(
    settings: &Value,
    bucket: Bucket,
) -> Result<&Vec<Value>, String> {
    terminal_buckets(settings)?
        .get(bucket.label())
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "Settings terminal bucket `{}` must be an array",
                bucket.label()
            )
        })
}

pub(crate) fn terminal_pattern_at(
    settings: &Value,
    position: TerminalPosition,
) -> Result<TerminalPattern, String> {
    let label = position.label();
    let object = terminal_bucket_array(settings, position.bucket)?
        .get(position.index)
        .and_then(Value::as_object)
        .ok_or_else(|| format!("Settings terminal entry `{label}` must be an object"))?;
    let pattern = object
        .get("pattern")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!("Settings terminal entry `{label}` must contain string `pattern`")
        })?;
    let case_sensitive = object
        .get("case_sensitive")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            format!("Settings terminal entry `{label}` must contain boolean `case_sensitive`")
        })?;

    Ok(TerminalPattern {
        pattern: pattern.to_owned(),
        case_sensitive,
    })
}

pub(crate) fn validate_owner_token(owner: &str) -> Result<(), String> {
    if owner.is_empty() {
        return Err("The inventory owner token must be nonempty".to_owned());
    }
    if !owner
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'+' | b'-'))
    {
        return Err("The inventory owner token must match `[A-Za-z0-9_.+-]+`".to_owned());
    }

    Ok(())
}

pub(crate) fn owner_source_matcher(owner: &str) -> Result<Regex, String> {
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

/// Report whether one regex source text exposes `owner` under the inventory’s exact token-boundary
/// rule. Source-text adjacency, not compiled matching, decides lexical visibility
pub(crate) fn is_lexically_visible(pattern: &str, matcher: &Regex) -> bool {
    matcher.captures(pattern).is_some()
}

/// Recompute the complete lexical inventory position set for one top-level executable token
pub(crate) fn lexical_inventory_positions(
    settings: &Value,
    owner: &str,
) -> Result<BTreeSet<TerminalPosition>, String> {
    validate_owner_token(owner)?;
    let matcher = owner_source_matcher(owner)?;
    let mut positions = BTreeSet::new();

    for bucket in [Bucket::Allow, Bucket::Confirm, Bucket::Deny] {
        let values = terminal_bucket_array(settings, bucket)?;
        for index in 0..values.len() {
            let position = TerminalPosition { bucket, index };
            let entry = terminal_pattern_at(settings, position)?;
            if is_lexically_visible(&entry.pattern, &matcher) {
                positions.insert(position);
            }
        }
    }

    Ok(positions)
}

pub(crate) fn validate_safe_relative_path(path: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(path);
    if candidate.as_os_str().is_empty() || candidate.is_absolute() {
        return Err(format!("Path `{path}` must be nonempty and relative"));
    }
    if !candidate
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(format!(
            "Path `{path}` must not contain root, parent, or current-directory components"
        ));
    }

    Ok(candidate)
}

/// Resolve one safe-relative path beneath `root`, refusing traversal and symlinked components
pub(crate) fn resolve_within_root(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let safe = validate_safe_relative_path(relative)?;
    let resolved = root.join(safe);
    let absolute = path_as_absolute(&resolved)?;
    let root_absolute = path_as_absolute(root)?;
    if !absolute.starts_with(&root_absolute) {
        return Err(format!("Path `{relative}` resolves outside the graph root"));
    }

    Ok(absolute)
}

pub(crate) fn read_regular_file_within_root(
    root: &Path,
    relative: &str,
    description: &str,
) -> Result<Vec<u8>, String> {
    let absolute = resolve_within_root(root, relative)?;
    read_regular_file_without_symlinks(&absolute, description)
}

/// Express one absolute path as a safe-relative path beneath `root`
pub(crate) fn relative_within_root(root: &Path, absolute: &Path) -> Result<String, String> {
    let root_absolute = path_as_absolute(root)?;
    let target = path_as_absolute(absolute)?;
    let relative = target
        .strip_prefix(&root_absolute)
        .map_err(|_| format!("Path `{}` lies outside the graph root", target.display()))?;
    let mut rendered = String::new();
    for component in relative.components() {
        let Component::Normal(part) = component else {
            return Err(format!(
                "Path `{}` contains an unsupported component",
                target.display()
            ));
        };
        let part = part
            .to_str()
            .ok_or_else(|| "Graph paths must be valid UTF-8".to_owned())?;
        if !rendered.is_empty() {
            rendered.push('/');
        }
        rendered.push_str(part);
    }
    if rendered.is_empty() {
        return Err("A graph path must not be the root itself".to_owned());
    }

    Ok(rendered)
}

pub(crate) const MAX_CLOSURE_RECORDS: usize = 4096;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InputClosureRecord {
    pub(crate) role: String,
    pub(crate) path: String,
    pub(crate) sha256: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InputClosure {
    pub(crate) digest: String,
    pub(crate) records: Vec<InputClosureRecord>,
}

/// Hash the sorted closure with an explicit length prefix on every field. Delimiter-only framing
/// would let a crafted path forge a different record set that hashes identically
pub(crate) fn input_closure_digest(records: &[InputClosureRecord]) -> String {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&(records.len() as u64).to_le_bytes());
    for record in records {
        for field in [&record.role, &record.path, &record.sha256] {
            encoded.extend_from_slice(&(field.len() as u64).to_le_bytes());
            encoded.extend_from_slice(field.as_bytes());
        }
    }

    sha256_hex(&encoded)
}

/// Accumulate every file an evaluator actually reads, anchored to the bundle graph root
pub(crate) struct InputClosureBuilder {
    root: PathBuf,
    records: BTreeMap<(String, String), String>,
    overflowed: bool,
}

impl InputClosureBuilder {
    pub(crate) fn new(root: &Path) -> Result<Self, String> {
        let root = path_as_absolute(root)?;
        let metadata = fs::symlink_metadata(&root)
            .map_err(|error| format!("Failed to inspect the graph root:\n\n{error}"))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(
                "The graph root must be an existing directory and not a symbolic link".to_owned(),
            );
        }

        Ok(Self {
            root,
            records: BTreeMap::new(),
            overflowed: false,
        })
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    /// Record one already-read file. Repeated reads of the same role and path collapse, and a
    /// conflicting hash for the same key means the file changed while it was being evaluated
    pub(crate) fn record(
        &mut self,
        role: &str,
        absolute: &Path,
        bytes: &[u8],
    ) -> Result<(), String> {
        let path = relative_within_root(&self.root, absolute)?;
        let sha256 = sha256_hex(bytes);
        let key = (role.to_owned(), path);
        if let Some(existing) = self.records.get(&key)
            && *existing != sha256
        {
            return Err(format!(
                "The {role} input `{}` changed while it was evaluated",
                key.1
            ));
        }
        if self.records.len() >= MAX_CLOSURE_RECORDS && !self.records.contains_key(&key) {
            self.overflowed = true;
            return Err(format!(
                "The validator input closure exceeds {MAX_CLOSURE_RECORDS} records"
            ));
        }
        self.records.insert(key, sha256);

        Ok(())
    }

    /// Read one file beneath the graph root and record it in the closure
    pub(crate) fn read_recorded(
        &mut self,
        role: &str,
        absolute: &Path,
        description: &str,
    ) -> Result<Vec<u8>, String> {
        let bytes = read_regular_file_without_symlinks(absolute, description)?;
        self.record(role, absolute, &bytes)?;

        Ok(bytes)
    }

    pub(crate) fn finish(self) -> Result<InputClosure, String> {
        if self.overflowed {
            return Err(format!(
                "The validator input closure exceeds {MAX_CLOSURE_RECORDS} records"
            ));
        }
        let mut records = self
            .records
            .into_iter()
            .map(|((role, path), sha256)| InputClosureRecord { role, path, sha256 })
            .collect::<Vec<_>>();
        records.sort();

        Ok(InputClosure {
            digest: input_closure_digest(&records),
            records,
        })
    }
}

/// Summarize the difference between a recorded and a recomputed closure without emitting contents
pub(crate) fn describe_closure_difference(
    recorded: &InputClosure,
    recomputed: &InputClosure,
    limit: usize,
) -> Vec<String> {
    let recorded_index = recorded
        .records
        .iter()
        .map(|record| ((&record.role, &record.path), &record.sha256))
        .collect::<BTreeMap<_, _>>();
    let recomputed_index = recomputed
        .records
        .iter()
        .map(|record| ((&record.role, &record.path), &record.sha256))
        .collect::<BTreeMap<_, _>>();
    let mut differences = Vec::new();

    for (key, sha256) in &recomputed_index {
        match recorded_index.get(key) {
            None => differences.push(format!("added {} input `{}`", key.0, key.1)),
            Some(previous) if previous != sha256 => {
                differences.push(format!("changed {} input `{}`", key.0, key.1));
            }
            Some(_) => {}
        }
    }
    for key in recorded_index.keys() {
        if !recomputed_index.contains_key(key) {
            differences.push(format!("removed {} input `{}`", key.0, key.1));
        }
    }
    differences.truncate(limit);

    differences
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OwnerOperationKind {
    Insert,
    Replace,
    Delete,
}

impl OwnerOperationKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Replace => "replace",
            Self::Delete => "delete",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OwnerOperation {
    pub(crate) id: String,
    pub(crate) inventory_owner: String,
    pub(crate) operation: OwnerOperationKind,
    pub(crate) baseline_members: Vec<String>,
    pub(crate) candidate_members: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SupplementalSide {
    Baseline,
    Candidate,
}

impl SupplementalSide {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Candidate => "candidate",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvidenceKind {
    NormalizedWitness,
    ValidationEntry,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClassificationEvidence {
    pub(crate) kind: EvidenceKind,
    pub(crate) value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SupplementalRecord {
    pub(crate) side: SupplementalSide,
    pub(crate) member_id: String,
    pub(crate) declared_owner: String,
    pub(crate) declared_role: Role,
    pub(crate) repository_scope: RepositoryScope,
    pub(crate) invisibility_reason: String,
    pub(crate) classification_evidence: Vec<ClassificationEvidence>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VisibilityTransformation {
    pub(crate) prefix: String,
    pub(crate) baseline_middle: String,
    pub(crate) candidate_middle: String,
    pub(crate) suffix: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VisibilityRewrite {
    pub(crate) baseline_member_id: String,
    pub(crate) candidate_member_id: String,
    pub(crate) recovered_owner: String,
    pub(crate) transformation: VisibilityTransformation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OwnerSpec {
    pub(crate) owners: Vec<OwnerOperation>,
    pub(crate) overlaps: Vec<String>,
    #[serde(default)]
    pub(crate) supplemental: Vec<SupplementalRecord>,
    #[serde(default)]
    pub(crate) visibility_rewrites: Vec<VisibilityRewrite>,
}

/// Check every owner-spec invariant that does not require the captured graph. Member existence and
/// source identity are resolved separately against the state and catalog
pub(crate) fn validate_owner_spec(spec: &OwnerSpec) -> Result<(), String> {
    let mut owner_ids = HashSet::new();
    let mut baseline_members = HashSet::new();
    let mut candidate_members = HashSet::new();

    for owner in &spec.owners {
        if owner.id.is_empty() {
            return Err("Owner operation IDs must be nonempty".to_owned());
        }
        if !owner_ids.insert(owner.id.as_str()) {
            return Err("Owner operation IDs must be unique".to_owned());
        }
        validate_owner_token(&owner.inventory_owner)?;

        let baseline_empty = owner.baseline_members.is_empty();
        let candidate_empty = owner.candidate_members.is_empty();
        let shape_valid = match owner.operation {
            OwnerOperationKind::Insert => baseline_empty && !candidate_empty,
            OwnerOperationKind::Replace => !baseline_empty && !candidate_empty,
            OwnerOperationKind::Delete => !baseline_empty && candidate_empty,
        };
        if !shape_valid {
            return Err(format!(
                "Owner operation `{}` does not match the required `{}` membership shape",
                display_id(&owner.id),
                owner.operation.label()
            ));
        }

        for member in &owner.baseline_members {
            if member.is_empty() {
                return Err("Owner baseline member IDs must be nonempty".to_owned());
            }
            if !baseline_members.insert(member.as_str()) {
                return Err(format!(
                    "Baseline member `{}` is claimed by more than one owner operation",
                    display_id(member)
                ));
            }
        }
        for member in &owner.candidate_members {
            if member.is_empty() {
                return Err("Owner candidate member IDs must be nonempty".to_owned());
            }
            if !candidate_members.insert(member.as_str()) {
                return Err(format!(
                    "Candidate member `{}` is claimed by more than one owner operation",
                    display_id(member)
                ));
            }
        }
    }

    for overlap in &spec.overlaps {
        if overlap.is_empty() {
            return Err("Overlap member IDs must be nonempty".to_owned());
        }
        if !candidate_members.insert(overlap.as_str()) {
            return Err(format!(
                "Overlap `{}` is also claimed as an owner candidate member",
                display_id(overlap)
            ));
        }
    }

    let mut supplemental_keys = HashSet::new();
    for record in &spec.supplemental {
        if record.member_id.is_empty() {
            return Err("Supplemental member IDs must be nonempty".to_owned());
        }
        if !supplemental_keys.insert((record.side, record.member_id.as_str())) {
            return Err(format!(
                "Supplemental member `{}` is declared more than once for one side",
                display_id(&record.member_id)
            ));
        }
        if record.declared_owner.is_empty() {
            return Err("Supplemental records must declare a nonempty owner".to_owned());
        }
        if record.invisibility_reason.trim().is_empty() {
            return Err(format!(
                "Supplemental member `{}` must record why lexical inventory cannot identify it",
                display_id(&record.member_id)
            ));
        }
        let has_witness = record
            .classification_evidence
            .iter()
            .any(|evidence| evidence.kind == EvidenceKind::NormalizedWitness);
        let has_entry = record
            .classification_evidence
            .iter()
            .any(|evidence| evidence.kind == EvidenceKind::ValidationEntry);
        if !has_witness || !has_entry {
            return Err(format!(
                "Supplemental member `{}` requires at least one normalized witness and one validation entry",
                display_id(&record.member_id)
            ));
        }
        for evidence in &record.classification_evidence {
            if evidence.value.is_empty() {
                return Err(
                    "Supplemental classification evidence values must be nonempty".to_owned(),
                );
            }
        }
    }

    let mut rewrite_baselines = HashSet::new();
    let mut rewrite_candidates = HashSet::new();
    for rewrite in &spec.visibility_rewrites {
        if rewrite.baseline_member_id.is_empty() || rewrite.candidate_member_id.is_empty() {
            return Err("Visibility rewrite member IDs must be nonempty".to_owned());
        }
        if !rewrite_baselines.insert(rewrite.baseline_member_id.as_str()) {
            return Err("Visibility rewrite baseline members must be unique".to_owned());
        }
        if !rewrite_candidates.insert(rewrite.candidate_member_id.as_str()) {
            return Err("Visibility rewrite candidate members must be unique".to_owned());
        }
        validate_owner_token(&rewrite.recovered_owner)?;
    }

    Ok(())
}

pub(crate) fn display_id(id: &str) -> String {
    const MAX_DISPLAY_CHARACTERS: usize = 80;
    let mut rendered = String::new();
    for character in id.chars().take(MAX_DISPLAY_CHARACTERS) {
        if character.is_control() {
            rendered.push('\u{fffd}');
        } else {
            rendered.push(character);
        }
    }
    if id.chars().count() > MAX_DISPLAY_CHARACTERS {
        rendered.push('…');
    }

    rendered
}

pub(crate) const MAX_TRANSFORMATION_AFFIX_BYTES: usize = 2000;
pub(crate) const MAX_TRANSFORMATION_EXPANSION: usize = 256;
pub(crate) const MAX_TRANSFORMATION_MIDDLE_BYTES: usize = 200;

struct ExpansionTerm {
    alternatives: Vec<String>,
    optional: bool,
}

fn is_expansion_literal(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Parse one transformation middle under the supported literal-expansion grammar. Every construct
/// outside the grammar refuses rather than being approximated
fn parse_expansion_middle(middle: &str) -> Result<Vec<ExpansionTerm>, String> {
    if middle.is_empty() {
        return Err("A transformation middle must be nonempty".to_owned());
    }
    if middle.len() > MAX_TRANSFORMATION_MIDDLE_BYTES {
        return Err(format!(
            "A transformation middle must not exceed {MAX_TRANSFORMATION_MIDDLE_BYTES} bytes"
        ));
    }

    let bytes = middle.as_bytes();
    let mut terms = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if is_expansion_literal(bytes[index]) {
            let literal = (bytes[index] as char).to_string();
            let optional = bytes.get(index + 1) == Some(&b'?');
            terms.push(ExpansionTerm {
                alternatives: vec![literal],
                optional,
            });
            index += if optional { 2 } else { 1 };
            continue;
        }
        if bytes[index..].starts_with(b"(?:") {
            let start = index + 3;
            let Some(offset) = bytes[start..].iter().position(|byte| *byte == b')') else {
                return Err(
                    "A transformation group must close with `)` inside the middle".to_owned(),
                );
            };
            let body = &middle[start..start + offset];
            if body.contains('(') {
                return Err("Nested transformation groups are unsupported".to_owned());
            }
            let mut alternatives = Vec::new();
            for alternative in body.split('|') {
                if alternative.is_empty() {
                    return Err("Transformation alternatives must be nonempty".to_owned());
                }
                if !alternative.bytes().all(is_expansion_literal) {
                    return Err(
                        "Transformation alternatives accept only `[A-Za-z0-9_]` bytes".to_owned(),
                    );
                }
                alternatives.push(alternative.to_owned());
            }
            index = start + offset + 1;
            let optional = bytes.get(index) == Some(&b'?');
            if optional {
                index += 1;
            }
            terms.push(ExpansionTerm {
                alternatives,
                optional,
            });
            continue;
        }

        return Err(
            "A transformation middle accepts only literals, `X?`, and `(?:…)` literal groups"
                .to_owned(),
        );
    }

    Ok(terms)
}

fn expand_middle(terms: &[ExpansionTerm]) -> Result<BTreeSet<String>, String> {
    let mut expansion = BTreeSet::new();
    expansion.insert(String::new());

    for term in terms {
        let mut next = BTreeSet::new();
        for prefix in &expansion {
            if term.optional {
                next.insert(prefix.clone());
            }
            for alternative in &term.alternatives {
                next.insert(format!("{prefix}{alternative}"));
            }
        }
        if next.len() > MAX_TRANSFORMATION_EXPANSION {
            return Err(format!(
                "A transformation expands to more than {MAX_TRANSFORMATION_EXPANSION} literals"
            ));
        }
        expansion = next;
    }

    Ok(expansion)
}

/// Scan one complete pattern with a bounded construct model, reporting which byte offsets are safe
/// split boundaries. Any construct the model does not represent exactly refuses
fn split_boundaries(pattern: &str) -> Result<Vec<bool>, String> {
    let bytes = pattern.as_bytes();
    let mut safe = vec![false; bytes.len() + 1];
    let mut class_depth = 0_usize;
    let mut group_depth = 0_usize;
    let mut index = 0;

    while index <= bytes.len() {
        safe[index] = class_depth == 0;
        if index == bytes.len() {
            break;
        }
        match bytes[index] {
            b'\\' => {
                if index + 1 >= bytes.len() {
                    return Err("A pattern must not end with a trailing escape".to_owned());
                }
                index += 2;
            }
            b'[' => {
                if class_depth > 0 {
                    return Err("Nested character classes are unsupported".to_owned());
                }
                class_depth = 1;
                index += 1;
            }
            b']' => {
                if class_depth == 0 {
                    return Err(
                        "An unescaped `]` outside a character class is unsupported".to_owned()
                    );
                }
                class_depth = 0;
                index += 1;
            }
            b'(' if class_depth == 0 => {
                if !bytes[index..].starts_with(b"(?:") {
                    return Err(
                        "Only non-capturing `(?:` groups are supported around a transformation"
                            .to_owned(),
                    );
                }
                group_depth += 1;
                index += 3;
            }
            b')' if class_depth == 0 => {
                if group_depth == 0 {
                    return Err("A pattern contains an unbalanced `)`".to_owned());
                }
                group_depth -= 1;
                index += 1;
            }
            _ => index += 1,
        }
    }

    if class_depth != 0 {
        return Err("A pattern contains an unterminated character class".to_owned());
    }
    if group_depth != 0 {
        return Err("A pattern contains an unterminated group".to_owned());
    }

    Ok(safe)
}

/// Verify the supported syntactic transformation invariant between one baseline and one candidate
/// member. This proves only that invariant and never general regex-language equivalence
pub(crate) fn verify_visibility_transformation(
    transformation: &VisibilityTransformation,
    baseline: &str,
    candidate: &str,
) -> Result<(), String> {
    let VisibilityTransformation {
        prefix,
        baseline_middle,
        candidate_middle,
        suffix,
    } = transformation;

    if prefix.len() > MAX_TRANSFORMATION_AFFIX_BYTES
        || suffix.len() > MAX_TRANSFORMATION_AFFIX_BYTES
    {
        return Err(format!(
            "Transformation affixes must not exceed {MAX_TRANSFORMATION_AFFIX_BYTES} bytes"
        ));
    }
    if format!("{prefix}{baseline_middle}{suffix}") != baseline {
        return Err(
            "The transformation affixes and baseline middle do not reconstruct the baseline member"
                .to_owned(),
        );
    }
    if format!("{prefix}{candidate_middle}{suffix}") != candidate {
        return Err(
            "The transformation affixes and candidate middle do not reconstruct the candidate member"
                .to_owned(),
        );
    }
    if let Some(first) = suffix.as_bytes().first()
        && matches!(first, b'*' | b'+' | b'?' | b'{')
    {
        return Err(
            "A transformation suffix must not begin with a quantifier that would bind to the middle"
                .to_owned(),
        );
    }

    for (pattern, middle, side) in [
        (baseline, baseline_middle, "baseline"),
        (candidate, candidate_middle, "candidate"),
    ] {
        let safe = split_boundaries(pattern)?;
        let start = prefix.len();
        let end = start + middle.len();
        if !safe.get(start).copied().unwrap_or(false) || !safe.get(end).copied().unwrap_or(false) {
            return Err(format!(
                "The {side} transformation boundary does not fall on a supported split point"
            ));
        }
    }

    let baseline_expansion = expand_middle(&parse_expansion_middle(baseline_middle)?)?;
    let candidate_expansion = expand_middle(&parse_expansion_middle(candidate_middle)?)?;
    if baseline_expansion != candidate_expansion {
        return Err(
            "The baseline and candidate transformation middles expand to different literal sets"
                .to_owned(),
        );
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResultKind {
    MatcherSuite,
    Comparison,
    LayerDecision,
    OwnerAudit,
    CandidateInventory,
    InventoryQuery,
    DeleteAllAudit,
}

impl ResultKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::MatcherSuite => "matcher_suite",
            Self::Comparison => "comparison",
            Self::LayerDecision => "layer_decision",
            Self::OwnerAudit => "owner_audit",
            Self::CandidateInventory => "candidate_inventory",
            Self::InventoryQuery => "inventory_query",
            Self::DeleteAllAudit => "delete_all_audit",
        }
    }
}

pub(crate) const OUTCOME_PASSED: &str = "passed";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BoundArtifact {
    pub(crate) path: String,
    pub(crate) sha256: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BoundInputs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) manifest_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) catalog_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) settings_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) inventory_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) overlay: Option<BoundArtifact>,
    pub(crate) input_closure: InputClosure,
}

/// Hash-bound reviewed workflow evidence. The bindings establish that a recorded claim refers to the
/// exact graph and inputs. They never prove that the validator ran or make the record authentic
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ValidationResult {
    pub(crate) kind: ResultKind,
    pub(crate) evaluator: String,
    pub(crate) outcome: String,
    pub(crate) bound_inputs: BoundInputs,
    pub(crate) counts: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ValidationEntry {
    pub(crate) id: String,
    pub(crate) kind: ResultKind,
    pub(crate) owner_ids: Vec<String>,
    pub(crate) manifest: BoundArtifact,
    pub(crate) result: BoundArtifact,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) overlay: Option<BoundArtifact>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BundleLineage {
    pub(crate) source_bundle_sha256: String,
    pub(crate) source_settings_sha256: String,
    pub(crate) refreshed_settings_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Bundle {
    pub(crate) baseline: BoundArtifact,
    pub(crate) candidate: BoundArtifact,
    pub(crate) state: BoundArtifact,
    pub(crate) catalog: BoundArtifact,
    pub(crate) owner_spec: BoundArtifact,
    pub(crate) validation: Vec<ValidationEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) lineage: Option<BundleLineage>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ValidationPlanEntry {
    pub(crate) id: String,
    pub(crate) kind: ResultKind,
    pub(crate) manifest: String,
    pub(crate) result: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) overlay: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ValidationPlan {
    pub(crate) results: Vec<ValidationPlanEntry>,
}

/// Redirect exactly the graph-relative paths a refresh reproduced. Any other declared path keeps
/// resolving from its manifest’s parent, so reviewed manifest bytes never need editing
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PathOverlay {
    pub(crate) paths: Vec<String>,
}

pub(crate) const PATH_OVERLAY_FILE: &str = "path-overlay.json";

impl PathOverlay {
    pub(crate) fn redirects(&self, declared: &str) -> bool {
        self.paths.iter().any(|path| path == declared)
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        let mut seen = HashSet::new();
        for path in &self.paths {
            validate_safe_relative_path(path)?;
            if !seen.insert(path.as_str()) {
                return Err(format!("Path overlay entry `{path}` is declared twice"));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BoundPosition {
    pub(crate) bucket: Bucket,
    pub(crate) index: usize,
}

impl BoundPosition {
    pub(crate) fn position(self) -> TerminalPosition {
        TerminalPosition {
            bucket: self.bucket,
            index: self.index,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BoundEntryPosition {
    pub(crate) id: String,
    pub(crate) bucket: Bucket,
    pub(crate) index: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PositionRemap {
    pub(crate) from: BoundPosition,
    pub(crate) to: BoundPosition,
}

/// A transient rebinding applied in memory so a reviewed manifest’s semantic fields stay
/// byte-identical while its snapshot-dependent positions follow a refreshed graph
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ManifestBinding {
    pub(crate) settings_sha256: String,
    #[serde(default)]
    pub(crate) entries: Vec<BoundEntryPosition>,
    #[serde(default)]
    pub(crate) positions: Vec<PositionRemap>,
}

impl ManifestBinding {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !is_valid_sha256(&self.settings_sha256) {
            return Err(
                "A manifest binding settings SHA-256 must be 64 lowercase hexadecimal characters"
                    .to_owned(),
            );
        }
        let mut ids = HashSet::new();
        for entry in &self.entries {
            if entry.id.is_empty() {
                return Err("Manifest binding entry IDs must be nonempty".to_owned());
            }
            if !ids.insert(entry.id.as_str()) {
                return Err("Manifest binding entry IDs must be unique".to_owned());
            }
        }
        let mut sources = HashSet::new();
        for remap in &self.positions {
            if !sources.insert(remap.from) {
                return Err("Manifest binding source positions must be unique".to_owned());
            }
        }

        Ok(())
    }

    pub(crate) fn entry_position(&self, id: &str) -> Option<TerminalPosition> {
        self.entries
            .iter()
            .find(|entry| entry.id == id)
            .map(|entry| TerminalPosition {
                bucket: entry.bucket,
                index: entry.index,
            })
    }

    pub(crate) fn remapped(&self, position: TerminalPosition) -> Option<TerminalPosition> {
        self.positions
            .iter()
            .find(|remap| remap.from.position() == position)
            .map(|remap| remap.to.position())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StatePattern {
    pub(crate) id: String,
    pub(crate) bucket: Bucket,
    pub(crate) source_index: usize,
    pub(crate) case_sensitive: bool,
    pub(crate) sha256: String,
    pub(crate) pattern_file: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StateDocument {
    pub(crate) baseline_file: String,
    pub(crate) baseline_sha256: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) fetch_replay_fields: Vec<FetchPermissionField>,
    pub(crate) scopes: Vec<String>,
    pub(crate) patterns: Vec<StatePattern>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExcludedCandidateRecord {
    pub(crate) bucket: Bucket,
    pub(crate) index: usize,
    pub(crate) sha256: String,
    pub(crate) owner: String,
    pub(crate) witness: String,
    pub(crate) reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RetainedOwnerEntry {
    pub(crate) bucket: Bucket,
    pub(crate) index: usize,
    pub(crate) sha256: String,
    pub(crate) owner_operation_id: String,
    pub(crate) witness: String,
}

/// Pre-seal proof that the deleted owners retain no owned entry in the candidate. Every recomputed
/// lexical hit must be classified once, so a nonzero raw hit count never stands in for the claim
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ZeroOwnerManifest {
    pub(crate) settings_sha256: String,
    pub(crate) inventory_owner: String,
    #[serde(default)]
    pub(crate) excluded_candidates: Vec<ExcludedCandidateRecord>,
    #[serde(default)]
    pub(crate) retained_owner_entries: Vec<RetainedOwnerEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AbsentMember {
    pub(crate) id: String,
    pub(crate) sha256: String,
}

/// Post-promotion delete-all verification. Absence is proved by byte-exact scans over the complete
/// arrays, independently of any remainder comparison
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DeleteAllManifest {
    pub(crate) settings_sha256: String,
    pub(crate) inventory_owner: String,
    pub(crate) bundle_file: String,
    pub(crate) bundle_sha256: String,
    pub(crate) promoted_scopes: Vec<String>,
    pub(crate) deleted_owner_ids: Vec<String>,
    #[serde(default)]
    pub(crate) absent_baseline_members: Vec<AbsentMember>,
    #[serde(default)]
    pub(crate) absent_supplemental_members: Vec<AbsentMember>,
    #[serde(default)]
    pub(crate) retained_exclusions: Vec<ExcludedCandidateRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuditEntryView {
    pub(crate) id: String,
    pub(crate) lexically_invisible: bool,
    pub(crate) position: TerminalPosition,
    pub(crate) witness: Option<String>,
}

/// A bounded read-only view of the ordinary owner-audit manifest. Coverage derivation needs entry
/// identity and position, and the supplemental cross-check adds the invisibility declaration and the
/// witness it infers from, so the strict audit schema stays owned by the audit binary
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuditManifestView {
    pub(crate) settings_sha256: String,
    pub(crate) inventory_owner: String,
    pub(crate) entries: Vec<AuditEntryView>,
    pub(crate) exclusions: Vec<TerminalPosition>,
}

fn view_position(value: &Value) -> Result<TerminalPosition, String> {
    let bucket = value
        .get("bucket")
        .and_then(Value::as_str)
        .and_then(Bucket::parse)
        .ok_or_else(|| "An audit manifest position must declare a valid bucket".to_owned())?;
    let index = value
        .get("index")
        .and_then(Value::as_u64)
        .ok_or_else(|| "An audit manifest position must declare an index".to_owned())?;

    Ok(TerminalPosition {
        bucket,
        index: usize::try_from(index)
            .map_err(|_| "An audit manifest index is out of range".to_owned())?,
    })
}

pub(crate) fn parse_audit_manifest_view(bytes: &[u8]) -> Result<AuditManifestView, String> {
    let manifest: Value = parse_strict_json(bytes, "Owner audit manifest")?;
    let settings_sha256 = declared_string(&manifest, "settings_sha256")
        .ok_or_else(|| "An owner audit manifest must declare `settings_sha256`".to_owned())?;
    let inventory_owner = declared_string(&manifest, "inventory_owner")
        .ok_or_else(|| "An owner audit manifest must declare `inventory_owner`".to_owned())?;

    let mut entries = Vec::new();
    for entry in manifest
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| "An owner audit manifest must declare `entries`".to_owned())?
    {
        let id = declared_string(entry, "id")
            .ok_or_else(|| "An owner audit manifest entry must declare `id`".to_owned())?;
        let lexically_invisible = match entry.get("lexically_invisible") {
            None => false,
            Some(declared) => declared.as_bool().ok_or_else(|| {
                "An owner audit manifest entry must declare a boolean `lexically_invisible`"
                    .to_owned()
            })?,
        };
        let witness = match entry.get("witness") {
            None => None,
            Some(declared) => Some(
                declared
                    .as_str()
                    .ok_or_else(|| {
                        "An owner audit manifest entry must declare a string `witness`".to_owned()
                    })?
                    .to_owned(),
            ),
        };
        entries.push(AuditEntryView {
            id,
            lexically_invisible,
            position: view_position(entry)?,
            witness,
        });
    }

    let mut exclusions = Vec::new();
    if let Some(declared) = manifest
        .get("excluded_candidates")
        .and_then(Value::as_array)
    {
        for exclusion in declared {
            exclusions.push(view_position(exclusion)?);
        }
    }

    Ok(AuditManifestView {
        settings_sha256,
        inventory_owner,
        entries,
        exclusions,
    })
}

/// Compare a recorded closure against an independently recomputed one, reporting bounded path-only
/// differences
pub(crate) fn verify_input_closure(
    recorded: &InputClosure,
    recomputed: &InputClosure,
    limit: usize,
) -> Result<(), String> {
    if input_closure_digest(&recorded.records) != recorded.digest {
        return Err(
            "A recorded validator input closure digest does not match its records".to_owned(),
        );
    }
    if recorded.digest == recomputed.digest {
        return Ok(());
    }
    let differences = describe_closure_difference(recorded, recomputed, limit);
    if differences.is_empty() {
        return Err("A validator input closure no longer matches its recorded digest".to_owned());
    }

    Err(format!(
        "A validator input closure changed after validation: {}",
        differences.join(", ")
    ))
}

pub(crate) const ROLE_BINDING: &str = "binding";
pub(crate) const ROLE_CATALOG: &str = "catalog";
pub(crate) const ROLE_CATALOG_ARTIFACT: &str = "catalog_artifact";
pub(crate) const ROLE_CATALOG_CANDIDATE: &str = "catalog_candidate";
pub(crate) const ROLE_CATALOG_STATE: &str = "catalog_state";
pub(crate) const ROLE_INPUT_FILE: &str = "input_file";
pub(crate) const ROLE_MANIFEST: &str = "manifest";
pub(crate) const ROLE_OVERLAY: &str = "overlay";
pub(crate) const ROLE_PATTERN_FILE: &str = "pattern_file";
pub(crate) const ROLE_SETTINGS: &str = "settings";

/// A loaded `--artifact-root` overlay together with its own identity, so the overlay itself becomes
/// part of every closure it influences
pub(crate) struct ResolvedOverlay {
    pub(crate) root: PathBuf,
    pub(crate) file: PathBuf,
    pub(crate) overlay: PathOverlay,
}

impl ResolvedOverlay {
    pub(crate) fn load(artifact_root: &Path) -> Result<Self, String> {
        let root = path_as_absolute(artifact_root)?;
        let metadata = fs::symlink_metadata(&root)
            .map_err(|error| format!("Failed to inspect the artifact root:\n\n{error}"))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(
                "The artifact root must be an existing directory and not a symbolic link"
                    .to_owned(),
            );
        }
        let file = root.join(PATH_OVERLAY_FILE);
        let bytes = read_regular_file_without_symlinks(&file, "path overlay")?;
        let overlay: PathOverlay = parse_strict_json(&bytes, "Path overlay")?;
        overlay.validate()?;

        Ok(Self {
            root,
            file,
            overlay,
        })
    }
}

/// Shared resolution context. One implementation drives both the evaluator’s reads and the
/// verifier’s recomputation, so a recorded closure and a recomputed closure cannot diverge
pub(crate) struct ClosureContext<'a> {
    pub(crate) overlay: Option<&'a ResolvedOverlay>,
}

impl ClosureContext<'_> {
    /// Resolve one declared manifest-relative path, redirecting it only when the overlay lists its
    /// graph-relative form
    pub(crate) fn resolve(
        &self,
        builder: &InputClosureBuilder,
        base: &Path,
        declared: &str,
    ) -> Result<PathBuf, String> {
        let direct = base.join(validate_safe_relative_path(declared)?);
        let absolute = path_as_absolute(&direct)?;
        let relative = relative_within_root(builder.root(), &absolute)?;
        let Some(overlay) = self.overlay else {
            return Ok(absolute);
        };
        if !overlay.overlay.redirects(&relative) {
            return Ok(absolute);
        }

        resolve_within_root(&overlay.root, &relative)
    }
}

fn record_overlay(
    builder: &mut InputClosureBuilder,
    context: &ClosureContext<'_>,
) -> Result<(), String> {
    let Some(overlay) = context.overlay else {
        return Ok(());
    };
    let bytes = read_regular_file_without_symlinks(&overlay.file, "path overlay")?;
    builder.record(ROLE_OVERLAY, &overlay.file, &bytes)
}

fn parent_of(path: &Path) -> PathBuf {
    path.parent().unwrap_or_else(|| Path::new(".")).to_owned()
}

/// Resolve one declared pattern catalog and every artifact it binds
fn resolve_catalog_closure(
    builder: &mut InputClosureBuilder,
    context: &ClosureContext<'_>,
    base: &Path,
    catalog_file: &str,
    candidate_file: &str,
    state_file: &str,
) -> Result<(), String> {
    let catalog_path = context.resolve(builder, base, catalog_file)?;
    let catalog_bytes = builder.read_recorded(ROLE_CATALOG, &catalog_path, "artifact catalog")?;
    let catalog = parse_artifact_catalog(&catalog_bytes)?;

    let candidate_path = context.resolve(builder, base, candidate_file)?;
    builder.read_recorded(
        ROLE_CATALOG_CANDIDATE,
        &candidate_path,
        "candidate settings",
    )?;
    let state_path = context.resolve(builder, base, state_file)?;
    builder.read_recorded(ROLE_CATALOG_STATE, &state_path, "state manifest")?;

    let catalog_base = parent_of(&catalog_path);
    for pattern in &catalog.patterns {
        let artifact = catalog_base.join(validate_catalog_artifact_path(&pattern.pattern_file)?);
        builder.read_recorded(ROLE_CATALOG_ARTIFACT, &artifact, "catalog pattern")?;
    }

    Ok(())
}

/// Resolve every file a matcher suite reads after overlay redirection
pub(crate) fn resolve_suite_closure(
    builder: &mut InputClosureBuilder,
    context: &ClosureContext<'_>,
    manifest_path: &Path,
) -> Result<(), String> {
    record_overlay(builder, context)?;
    let manifest_bytes = builder.read_recorded(ROLE_MANIFEST, manifest_path, "suite manifest")?;
    let manifest = String::from_utf8(manifest_bytes)
        .map_err(|_| "The suite manifest is not valid UTF-8".to_owned())?;
    let base = parent_of(manifest_path);

    // Every record type splits exactly as the suite evaluator splits it. A file-bearing field is
    // parsed with the same bounded `splitn` arity, so a path containing a tab cannot leave one
    // reader with a field the other silently drops
    for line in manifest.split_terminator('\n') {
        let Some((record_type, _)) = line.split_once('\t') else {
            return Err("A suite manifest record must contain a tab-separated type".to_owned());
        };
        match record_type {
            "pattern" => {
                let fields = line.split('\t').collect::<Vec<_>>();
                if let ["pattern", _, _, _, pattern_file] = fields.as_slice() {
                    let path = context.resolve(builder, &base, pattern_file)?;
                    builder.read_recorded(ROLE_PATTERN_FILE, &path, "suite pattern")?;
                }
            }
            "pattern-catalog" => {
                let fields = line.split('\t').collect::<Vec<_>>();
                if let [
                    "pattern-catalog",
                    _,
                    catalog_file,
                    candidate_file,
                    state_file,
                ] = fields.as_slice()
                {
                    resolve_catalog_closure(
                        builder,
                        context,
                        &base,
                        catalog_file,
                        candidate_file,
                        state_file,
                    )?;
                }
            }
            "pattern-case-file" => {
                let fields = line.splitn(4, '\t').collect::<Vec<_>>();
                if let ["pattern-case-file", _, _, input_file] = fields.as_slice() {
                    let path = context.resolve(builder, &base, input_file)?;
                    builder.read_recorded(ROLE_INPUT_FILE, &path, "suite input")?;
                }
            }
            "decision-case-file" => {
                let fields = line.splitn(3, '\t').collect::<Vec<_>>();
                if let ["decision-case-file", _, input_file] = fields.as_slice() {
                    let path = context.resolve(builder, &base, input_file)?;
                    builder.read_recorded(ROLE_INPUT_FILE, &path, "suite input")?;
                }
            }
            "catalog-pattern" | "decision-case" | "default" | "pattern-case" => {}
            _ => {
                return Err(
                    "A suite manifest declares an unsupported record type, so its input closure cannot be resolved"
                        .to_owned(),
                );
            }
        }
    }

    Ok(())
}

fn declared_string(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_owned)
}

pub(crate) fn layer_tool_from_manifest(manifest: &Value) -> Result<LayerTool, String> {
    let Some(tool) = manifest.get("tool").and_then(Value::as_str) else {
        return Err("A layer manifest must declare string `tool`".to_owned());
    };

    LayerTool::parse(tool)
        .ok_or_else(|| "A layer manifest `tool` must be `fetch` or `terminal`".to_owned())
}

/// Resolve every file a baseline/candidate comparison reads after overlay redirection
pub(crate) fn resolve_comparison_closure(
    builder: &mut InputClosureBuilder,
    context: &ClosureContext<'_>,
    manifest_path: &Path,
) -> Result<(), String> {
    record_overlay(builder, context)?;
    let manifest_bytes =
        builder.read_recorded(ROLE_MANIFEST, manifest_path, "comparison manifest")?;
    let manifest: Value = parse_strict_json(&manifest_bytes, "Comparison manifest")?;
    let base = parent_of(manifest_path);

    if let Some(catalogs) = manifest.get("catalogs").and_then(Value::as_array) {
        for catalog in catalogs {
            let (Some(catalog_file), Some(candidate_file), Some(state_file)) = (
                declared_string(catalog, "catalog_file"),
                declared_string(catalog, "candidate_file"),
                declared_string(catalog, "state_file"),
            ) else {
                return Err(
                    "A comparison catalog declaration is missing a required path".to_owned(),
                );
            };
            resolve_catalog_closure(
                builder,
                context,
                &base,
                &catalog_file,
                &candidate_file,
                &state_file,
            )?;
        }
    }

    for side in ["baseline", "candidate"] {
        let Some(patterns) = manifest
            .get(side)
            .and_then(|set| set.get("patterns"))
            .and_then(Value::as_array)
        else {
            continue;
        };
        for pattern in patterns {
            let Some(pattern_file) = declared_string(pattern, "pattern_file") else {
                continue;
            };
            let path = context.resolve(builder, &base, &pattern_file)?;
            builder.read_recorded(ROLE_PATTERN_FILE, &path, "comparison pattern")?;
        }
    }

    if let Some(cases) = manifest.get("cases").and_then(Value::as_array) {
        for case in cases {
            let Some(input_file) = declared_string(case, "input_file") else {
                continue;
            };
            let path = context.resolve(builder, &base, &input_file)?;
            builder.read_recorded(ROLE_INPUT_FILE, &path, "comparison input")?;
        }
    }

    Ok(())
}

/// Resolve every file a configured-pattern-layer evaluation reads after overlay redirection
pub(crate) fn resolve_layer_closure(
    builder: &mut InputClosureBuilder,
    context: &ClosureContext<'_>,
    manifest_path: &Path,
) -> Result<(), String> {
    record_overlay(builder, context)?;
    let manifest_bytes = builder.read_recorded(ROLE_MANIFEST, manifest_path, "layer manifest")?;
    let manifest: Value = parse_strict_json(&manifest_bytes, "Layer manifest")?;
    layer_tool_from_manifest(&manifest)?;
    let base = parent_of(manifest_path);

    let Some(settings_file) = declared_string(&manifest, "settings_file") else {
        return Err("A layer manifest must declare `settings_file`".to_owned());
    };
    let settings_path = context.resolve(builder, &base, &settings_file)?;
    builder.read_recorded(ROLE_SETTINGS, &settings_path, "layer settings")?;

    for field in ["pattern_cases", "settled_inputs"] {
        if let Some(inputs) = manifest.get(field).and_then(Value::as_array) {
            for input in inputs {
                let Some(input_file) = declared_string(input, "input_file") else {
                    continue;
                };
                let path = context.resolve(builder, &base, &input_file)?;
                builder.read_recorded(ROLE_INPUT_FILE, &path, "layer input")?;
            }
        }
    }

    Ok(())
}

/// Resolve every file an owner audit, zero-owner verification, or delete-all verification reads
pub(crate) fn resolve_audit_closure(
    builder: &mut InputClosureBuilder,
    manifest_path: &Path,
    settings_path: &Path,
    binding_path: Option<&Path>,
) -> Result<(), String> {
    if let Some(binding_path) = binding_path {
        builder.read_recorded(ROLE_BINDING, binding_path, "manifest binding")?;
    }
    builder.read_recorded(ROLE_MANIFEST, manifest_path, "audit manifest")?;
    builder.read_recorded(ROLE_SETTINGS, settings_path, "audit settings")?;

    Ok(())
}

/// Resolve the closure for a manifest-free inventory query
pub(crate) fn resolve_inventory_closure(
    builder: &mut InputClosureBuilder,
    settings_path: &Path,
) -> Result<(), String> {
    builder.read_recorded(ROLE_SETTINGS, settings_path, "inventory settings")?;

    Ok(())
}

pub(crate) fn serialize_pretty_json_bytes<T: Serialize>(
    value: &T,
    description: &str,
) -> Result<Vec<u8>, String> {
    let value = serde_json::to_value(value)
        .map_err(|error| format!("Failed to serialize {description}:\n\n{error}"))?;
    let mut bytes = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    value
        .serialize(&mut serializer)
        .map_err(|error| format!("Failed to serialize {description}:\n\n{error}"))?;
    bytes.push(b'\n');

    Ok(bytes)
}

/// Write one evidence record with a create-new destination. Existing paths are never overwritten
pub(crate) fn write_validation_result(
    path: &Path,
    result: &ValidationResult,
) -> Result<(), String> {
    let bytes = serialize_pretty_json_bytes(result, "validation result")?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            format!(
                "Failed to create validation result `{}`:\n\n{error}",
                path.display()
            )
        })?;
    std::io::Write::write_all(&mut file, &bytes).map_err(|error| {
        format!(
            "Failed to write validation result `{}`:\n\n{error}",
            path.display()
        )
    })
}

pub(crate) fn parse_strict_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    description: &str,
) -> Result<T, String> {
    serde_json::from_slice(bytes).map_err(|error| {
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
    })
}
