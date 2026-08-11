use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
        .unwrap_or("Regex compilation failed")
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
