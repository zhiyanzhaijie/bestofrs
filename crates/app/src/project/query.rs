use std::sync::Arc;

use domain::{Project, RepoId};

use crate::app_error::AppResult;
use crate::common::pagination::{Page, Pagination};
use crate::project::ProjectRepo;

#[derive(Clone)]
pub struct ProjectQueryHandler {
    projects: Arc<dyn ProjectRepo>,
}

impl ProjectQueryHandler {
    pub fn new(projects: Arc<dyn ProjectRepo>) -> Self {
        Self { projects }
    }

    pub async fn get(&self, repo_id: &RepoId) -> AppResult<Option<Project>> {
        self.projects.get(repo_id).await
    }

    pub async fn list(&self, page: Pagination) -> AppResult<Page<Project>> {
        self.projects.list(page).await
    }

    pub async fn search_by_key(&self, key: String, page: Pagination) -> AppResult<Page<Project>> {
        self.projects.search_by_key(key, page).await
    }

    pub async fn list_disabled(&self, page: Pagination) -> AppResult<Page<Project>> {
        self.projects.list_disabled(page).await
    }

    pub async fn search_disabled_by_key(
        &self,
        key: String,
        page: Pagination,
    ) -> AppResult<Page<Project>> {
        self.projects.search_disabled_by_key(key, page).await
    }
}
