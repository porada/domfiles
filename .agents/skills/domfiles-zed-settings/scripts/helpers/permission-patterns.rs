use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
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

pub(crate) const ARTIFACT_CATALOG_VERSION: u64 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactCatalog {
    pub(crate) version: u64,
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
    if catalog.version != ARTIFACT_CATALOG_VERSION {
        return Err(format!(
            "Unsupported artifact catalog schema version. Expected `{ARTIFACT_CATALOG_VERSION}`, received `{}`",
            catalog.version
        ));
    }
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
    if catalog.patterns.is_empty() {
        return Err("Artifact catalog must contain at least one pattern".to_owned());
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

pub(crate) fn parse_artifact_catalog(bytes: &[u8]) -> Result<ArtifactCatalog, String> {
    let catalog: ArtifactCatalog = serde_json::from_slice(bytes).map_err(|error| {
        let summary = match error.classify() {
            serde_json::error::Category::Data => {
                "Artifact catalog JSON data does not match the required schema"
            }
            serde_json::error::Category::Eof => {
                "Artifact catalog JSON ends before a complete value"
            }
            serde_json::error::Category::Io => "Failed to read artifact catalog JSON",
            serde_json::error::Category::Syntax => "Artifact catalog JSON syntax is invalid",
        };
        format!(
            "{summary} at line {}, column {}",
            error.line(),
            error.column()
        )
    })?;
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

pub(crate) fn load_artifact_catalog(path: &Path) -> Result<LoadedArtifactCatalog, String> {
    let catalog_path = path_as_absolute(path)?;
    let bytes = read_regular_file_without_symlinks(&catalog_path, "artifact catalog")?;
    let document = parse_artifact_catalog(&bytes)?;
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
