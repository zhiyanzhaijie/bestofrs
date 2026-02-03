use app::repo::RepoSearchResult;
use app::prelude::Page;
use domain::Repo;
use serde::{Deserialize, Serialize};

use super::tags::TagDto;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchRepoDto {
    pub id: String,
    pub full_name: Option<String>,
    pub github_repo_id: Option<i64>,
}

impl From<Repo> for SearchRepoDto {
    fn from(value: Repo) -> Self {
        Self {
            id: value.id.to_string(),
            full_name: value.full_name,
            github_repo_id: value.github_repo_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResultDto {
    pub repos: Page<SearchRepoDto>,
    pub tags: Page<TagDto>,
}

impl From<RepoSearchResult> for SearchResultDto {
    fn from(value: RepoSearchResult) -> Self {
        Self {
            repos: value.repos.map(SearchRepoDto::from),
            tags: value.tags.map(TagDto::from),
        }
    }
}
