use domain::Project;

use crate::app_error::AppResult;
use crate::common::pagination::{Page, Pagination};

#[async_trait::async_trait]
pub trait ProjectRepo: Send + Sync {
    async fn upsert(&self, project: &Project) -> AppResult<()>;
    async fn upsert_many(&self, items: &[Project]) -> AppResult<()>;
    async fn list(&self, page: Pagination) -> AppResult<Page<Project>>;
    async fn remove(&self, repo_id: String) -> AppResult<()>;
}
