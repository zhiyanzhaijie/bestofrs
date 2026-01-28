use serde::{Deserialize, Serialize};

/// Repository identifier in the form `owner/name`.
///
/// This is used as a cross-aggregate reference from other contexts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RepoId(String);

impl RepoId {
    /// Construct without validation.
    ///
    /// Prefer `parse` when the input is user-controlled.
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate and construct a RepoId.
    pub fn parse(value: &str) -> Result<Self, RepoIdInvalidError> {
        let value = value.trim();
        let mut it = value.split('/');
        let owner = it.next().unwrap_or_default();
        let name = it.next().unwrap_or_default();

        // Must be exactly one slash and non-empty parts.
        if owner.is_empty() || name.is_empty() || it.next().is_some() {
            return Err(RepoIdInvalidError {
                value: value.to_string(),
            });
        }

        Ok(Self(value.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoIdInvalidError {
    value: String,
}

impl std::fmt::Display for RepoIdInvalidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Include context in the message for easier debugging/logging.
        write!(
            f,
            "repo: invalid input: invalid repo id: '{}', expected 'owner/name'",
            self.value
        )
    }
}

impl std::error::Error for RepoIdInvalidError {}

impl crate::error::DomainError for RepoIdInvalidError {}

impl std::fmt::Display for RepoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for RepoId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag {
    pub label: TagLabel,
    pub value: TagValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagLabel(String);

impl TagLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagValue(String);

impl TagValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
