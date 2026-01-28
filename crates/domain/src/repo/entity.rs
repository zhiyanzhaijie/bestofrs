use serde::{Deserialize, Serialize};

use super::{RepoId, Tag};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repo {
    pub id: RepoId,
    pub github_repo_id: Option<i64>,
    pub full_name: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub last_fetched_at: Option<String>,
    pub etag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoWithTags {
    pub repo: Repo,
    pub tags: Vec<Tag>,
}
