use domain::{Project, RepoId};

use crate::app_error::AppResult;
use crate::common::pagination::{Page, Pagination};

#[async_trait::async_trait]
pub trait ProjectRepo: Send + Sync {
    async fn get(&self, repo_id: &RepoId) -> AppResult<Option<Project>>;
    async fn upsert(&self, project: &Project) -> AppResult<()>;
    async fn upsert_many(&self, items: &[Project]) -> AppResult<()>;
    async fn update_many(&self, items: &[Project]) -> AppResult<()>;
    async fn list(&self, page: Pagination) -> AppResult<Page<Project>>;
    async fn search_by_key(&self, key: String, page: Pagination) -> AppResult<Page<Project>>;
    async fn list_disabled(&self, page: Pagination) -> AppResult<Page<Project>>;
    async fn search_disabled_by_key(&self, key: String, page: Pagination)
        -> AppResult<Page<Project>>;
    async fn remove(&self, repo_id: String) -> AppResult<()>;
}
