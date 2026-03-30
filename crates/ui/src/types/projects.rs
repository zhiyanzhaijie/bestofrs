use domain::{Project, ProjectStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectDto {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub repo_id: String,
    pub url: Option<String>,
    pub avatar_url: Option<String>,
    pub status: ProjectStatus,
    pub twitter: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl From<Project> for ProjectDto {
    fn from(value: Project) -> Self {
        let repo_id = value.id.to_string();
        Self {
            // Keep `id` for UI keys/backward-compat; it matches repo_id.
            id: repo_id.clone(),
            name: value.name,
            slug: value.slug,
            description: value.description,
            repo_id,
            url: value.url,
            avatar_url: value.avatar_url,
            status: value.status,
            twitter: value.twitter,
            tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectImportItem {
    pub id: Option<String>,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub repo_id: String,

    pub url: Option<String>,
    pub avatar_url: Option<String>,

    pub status: ProjectStatus,
    pub twitter: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportProjectsResult {
    pub total: usize,
    pub upserted: usize,
    pub skipped_invalid: usize,
    pub failed_upsert: usize,

    #[serde(default)]
    pub invalid_examples: Vec<String>,

    #[serde(default)]
    pub error_examples: Vec<String>,
}
