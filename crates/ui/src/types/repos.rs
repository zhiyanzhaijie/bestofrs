use super::tags::TagDto;
use app::repo::GithubReadme;
use domain::{Repo, RepoWithTags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoDto {
    pub id: String,
    pub github_repo_id: Option<i64>,
    pub full_name: Option<String>,
    pub homepage_url: Option<String>,
    pub avatar_url: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub last_fetched_at: Option<String>,
    pub tags: Vec<TagDto>,
}

impl From<Repo> for RepoDto {
    fn from(value: Repo) -> Self {
        Self {
            id: value.id.to_string(),
            github_repo_id: value.github_repo_id,
            full_name: value.full_name,
            homepage_url: value.homepage_url,
            avatar_url: value.avatar_url,
            stars: value.stars,
            forks: value.forks,
            open_issues: value.open_issues,
            watchers: value.watchers,
            last_fetched_at: value.last_fetched_at,
            tags: Vec::new(),
        }
    }
}

impl From<RepoWithTags> for RepoDto {
    fn from(value: RepoWithTags) -> Self {
        let tags = value.tags.into_iter().map(TagDto::from).collect();
        let repo = value.repo;
        Self {
            id: repo.id.to_string(),
            github_repo_id: repo.github_repo_id,
            full_name: repo.full_name,
            homepage_url: repo.homepage_url,
            avatar_url: repo.avatar_url,
            stars: repo.stars,
            forks: repo.forks,
            open_issues: repo.open_issues,
            watchers: repo.watchers,
            last_fetched_at: repo.last_fetched_at,
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
