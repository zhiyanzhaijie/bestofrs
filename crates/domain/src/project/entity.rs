use serde::{Deserialize, Serialize};

use crate::RepoId;

/// A curated tracking entry.
///
/// In this project, a `Project` always corresponds 1:1 to a GitHub repository.
/// Therefore the identity of a project is the repository id (`owner/name`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: RepoId,

    pub name: String,
    pub slug: String,
    pub description: String,
    pub url: Option<String>,
    pub avatar_url: Option<String>,
    pub status: ProjectStatus,
    pub twitter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectStatus {
    Active,
    Disabled,
    Unknown,
}
