use async_trait::async_trait;
use domain::{Repo, RepoId, Tag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::app_error::AppResult;

use crate::common::pagination::{Page, Pagination};
use crate::repo::{RepoGithubLookupKey, RepoSearchResult};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoRankMetric {
    Star,
    Fork,
    Issue,
    Recent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RepoRankTimeRange {
    #[default]
    All,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoRankQuery {
    pub metric: RepoRankMetric,
    pub range: RepoRankTimeRange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoListQuery {
    pub page: Pagination,
    pub metric: Option<RepoRankMetric>,
    pub range: Option<RepoRankTimeRange>,
    pub tags: Option<Vec<String>>,
}

#[async_trait]
pub trait RepoRepo: Send + Sync {
    async fn upsert(&self, repo: &Repo) -> AppResult<()>;
    async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()>;
    async fn get(&self, id: &RepoId) -> AppResult<Option<Repo>>;
    async fn find_existing_ids(&self, ids: &[RepoId]) -> AppResult<Vec<RepoId>>;
    async fn find_existing_github_repo_ids(&self, ids: &[i64]) -> AppResult<Vec<i64>>;
    async fn list_by_ids(&self, ids: &[RepoId]) -> AppResult<Vec<Repo>>;
    async fn list_repos(&self, page: Pagination) -> AppResult<Page<Repo>>;
    async fn list_ranked(&self, query: RepoRankQuery, page: Pagination) -> AppResult<Page<Repo>>;
    async fn search_repos_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Repo>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoTagFacet {
    pub value: String,
    pub count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoTagTopRepo {
    pub repo_id: String,
    pub avatar_urls: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoTagListItem {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub repos_total: u64,
    pub top_repos: Vec<RepoTagTopRepo>,
}

#[async_trait]
pub trait RepoTagRepo: Send + Sync {
    async fn replace_repo_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()>;
    async fn replace_repo_tags_bulk(&self, items: &[(RepoId, Vec<Tag>)]) -> AppResult<()>;
    async fn upsert_tag(&self, tag: &Tag) -> AppResult<()>;
    async fn update_tag(&self, tag: &Tag) -> AppResult<()>;
    async fn delete_tag(&self, tag: &Tag) -> AppResult<()>;
    async fn list_by_repo_ids(&self, repo_ids: &[RepoId]) -> AppResult<Vec<(RepoId, Tag)>>;
    async fn find_tags_by_values(&self, values: &[String]) -> AppResult<Vec<Tag>>;
    async fn list_repo_ids_without_tags(&self, page: Pagination) -> AppResult<Page<RepoId>>;
    async fn list_repo_ids_by_label(
        &self,
        label: &str,
        value: Option<&str>,
        page: Pagination,
    ) -> AppResult<Page<RepoId>>;
    async fn list_tags(&self, page: Pagination) -> AppResult<Page<Tag>>;
    async fn search_tags_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Tag>>;
    async fn count_repos_by_tags(&self, tags: &[Tag]) -> AppResult<HashMap<(String, String), u64>>;
    async fn list_tags_with_meta(
        &self,
        page: Pagination,
        top_n: u32,
    ) -> AppResult<Page<RepoTagListItem>>;
    async fn list_tags_with_meta_by_values(
        &self,
        values: &[String],
        top_n: u32,
    ) -> AppResult<Vec<RepoTagListItem>>;
    async fn list_tag_facets_by_active_values(
        &self,
        active_values: &[String],
        limit: Option<u32>,
    ) -> AppResult<Vec<RepoTagFacet>>;
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
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub owner_avatar_url: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubLatestPushedRepoInfo {
    pub id: i64,
    pub full_name: String,
    pub stargazers_count: i64,
    pub created_at: String,
    pub pushed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubLatestPushedRepoSearchResult {
    pub requested_limit: usize,
    pub upstream_total_count: Option<u64>,
    pub fetched_raw_count: usize,
    pub unique_count: usize,
    pub items: Vec<GithubLatestPushedRepoInfo>,
}

#[async_trait]
pub trait GithubGateway: Send + Sync {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo>;
    async fn fetch_repo_by_github_id(&self, github_repo_id: i64) -> AppResult<GithubRepoInfo>;
    async fn search_recently_pushed_repos(
        &self,
        limit: usize,
    ) -> AppResult<GithubLatestPushedRepoSearchResult>;
    async fn fetch_repo_by_lookup_key(
        &self,
        key: &RepoGithubLookupKey,
    ) -> AppResult<GithubRepoInfo> {
        match key {
            RepoGithubLookupKey::GithubRepoId(github_repo_id) => {
                self.fetch_repo_by_github_id(*github_repo_id).await
            }
            RepoGithubLookupKey::RepoFullName(full_name) => self.fetch_repo(full_name).await,
        }
    }
    async fn fetch_readme(&self, full_name: &str) -> AppResult<Option<GithubReadme>>;
}
