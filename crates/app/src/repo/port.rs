use domain::{Repo, RepoId, Tag};

use crate::app_error::AppResult;

use crate::common::pagination::{Page, Pagination};

#[async_trait::async_trait]
pub trait RepoRepo: Send + Sync {
    async fn upsert(&self, repo: &Repo) -> AppResult<()>;
    async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()>;
    async fn get(&self, id: &RepoId) -> AppResult<Option<Repo>>;
    async fn list(&self, page: Pagination) -> AppResult<Page<Repo>>;
}

#[async_trait::async_trait]
pub trait RepoTagRepo: Send + Sync {
    async fn replace_repo_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()>;
    async fn list_by_repo_ids(&self, repo_ids: &[RepoId]) -> AppResult<Vec<(RepoId, Tag)>>;
    async fn list_repo_ids_by_label(
        &self,
        label: &str,
        value: Option<&str>,
        page: Pagination,
    ) -> AppResult<Page<RepoId>>;
    async fn ensure_default_tag_for_repos(&self, repo_ids: &[RepoId]) -> AppResult<()>;
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

#[async_trait::async_trait]
pub trait GithubGateway: Send + Sync {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo>;
}
