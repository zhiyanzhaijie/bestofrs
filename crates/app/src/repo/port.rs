use async_trait::async_trait;
use domain::{Repo, RepoId, Tag};

use crate::app_error::AppResult;

use crate::common::pagination::{Page, Pagination};
use crate::repo::RepoSearchResult;

#[async_trait]
pub trait RepoRepo: Send + Sync {
    async fn upsert(&self, repo: &Repo) -> AppResult<()>;
    async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()>;
    async fn get(&self, id: &RepoId) -> AppResult<Option<Repo>>;
    async fn list(&self, page: Pagination) -> AppResult<Page<Repo>>;
    async fn search_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Repo>>;
}

#[async_trait]
pub trait RepoTagRepo: Send + Sync {
    async fn replace_repo_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()>;
    async fn list_by_repo_ids(&self, repo_ids: &[RepoId]) -> AppResult<Vec<(RepoId, Tag)>>;
    async fn list_repo_ids_by_label(
        &self,
        label: &str,
        value: Option<&str>,
        page: Pagination,
    ) -> AppResult<Page<RepoId>>;
    async fn list_tags(&self, page: Pagination) -> AppResult<Page<Tag>>;
    async fn search_tags_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Tag>>;
    async fn ensure_default_tag_for_repos(&self, repo_ids: &[RepoId]) -> AppResult<()>;
}

#[async_trait]
pub trait RepoSearchCache: Send + Sync {
    async fn get(&self, key: &str, page: Pagination) -> AppResult<Option<RepoSearchResult>>;
    async fn set(&self, key: &str, page: Pagination, value: &RepoSearchResult) -> AppResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubRepoInfo {
    pub id: i64,
    pub full_name: String,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    pub subscribers_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubReadme {
    pub content: String,
    pub html_url: Option<String>,
    pub download_url: Option<String>,
}

#[async_trait]
pub trait GithubGateway: Send + Sync {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo>;
    async fn fetch_readme(&self, full_name: &str) -> AppResult<Option<GithubReadme>>;
}
