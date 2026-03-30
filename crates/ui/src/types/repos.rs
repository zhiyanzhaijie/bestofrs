use super::tags::TagDto;
use app::prelude::*;
use app::repo::BulkUpdateRepoTagResult;
use app::repo::GithubReadme;
use domain::{ProjectStatus, Repo, RepoWithTags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoDto {
    pub id: String,
    pub github_repo_id: Option<i64>,
    pub full_name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
    pub avatar_url: Option<String>,
    pub avatar_urls: Vec<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub created_at: String,
    pub last_fetched_at: Option<String>,
    pub project_status: Option<ProjectStatus>,
    pub tags: Vec<TagDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkUpdateRepoTagResultDto {
    pub total: usize,
    pub updated: usize,
    pub skipped: usize,
}

impl From<BulkUpdateRepoTagResult> for BulkUpdateRepoTagResultDto {
    fn from(value: BulkUpdateRepoTagResult) -> Self {
        Self {
            total: value.total,
            updated: value.updated,
            skipped: value.skipped,
        }
    }
}

impl From<Repo> for RepoDto {
    fn from(value: Repo) -> Self {
        let avatar_urls = value.avatar_urls();
        Self {
            id: value.id.to_string(),
            github_repo_id: value.github_repo_id,
            full_name: value.full_name,
            description: value.description,
            homepage_url: value.homepage_url,
            avatar_url: value.avatar_url,
            avatar_urls,
            stars: value.stars,
            forks: value.forks,
            open_issues: value.open_issues,
            watchers: value.watchers,
            created_at: value.created_at,
            last_fetched_at: value.last_fetched_at,
            project_status: None,
            tags: Vec::new(),
        }
    }
}

impl From<RepoWithTags> for RepoDto {
    fn from(value: RepoWithTags) -> Self {
        let tags = value.tags.into_iter().map(TagDto::from).collect();
        let repo = value.repo;
        let avatar_urls = repo.avatar_urls();
        Self {
            id: repo.id.to_string(),
            github_repo_id: repo.github_repo_id,
            full_name: repo.full_name,
            description: repo.description,
            homepage_url: repo.homepage_url,
            avatar_url: repo.avatar_url,
            avatar_urls,
            stars: repo.stars,
            forks: repo.forks,
            open_issues: repo.open_issues,
            watchers: repo.watchers,
            created_at: repo.created_at,
            last_fetched_at: repo.last_fetched_at,
            project_status: None,
            tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoReadmeDto {
    pub content: String,
    pub html_url: Option<String>,
    pub download_url: Option<String>,
}

impl From<GithubReadme> for RepoReadmeDto {
    fn from(value: GithubReadme) -> Self {
        Self {
            content: value.content,
            html_url: value.html_url,
            download_url: value.download_url,
        }
    }
}
