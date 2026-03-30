use crate::app_error::AppResult;
use crate::repo::{RepoCommandHandler, RepoProjectOverrideExt, RepoRepo};
use domain::{ProjectCreated, ProjectUpdated, Repo};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProjectEventHandler {
    repo_command: RepoCommandHandler,
    repos: Arc<dyn RepoRepo>,
}

impl ProjectEventHandler {
    pub fn new(
        repo_command: RepoCommandHandler,
        repos: Arc<dyn RepoRepo>,
    ) -> Self {
        Self {
            repo_command,
            repos,
        }
    }

    pub async fn handle_project_created(&self, event: &ProjectCreated) -> AppResult<()> {
        self.handle_projects_created(std::slice::from_ref(event))
            .await
    }

    pub async fn handle_projects_created(&self, events: &[ProjectCreated]) -> AppResult<()> {
        if events.is_empty() {
            return Ok(());
        }
        let repos = events
            .iter()
            .map(|event| event.repo.clone())
            .collect::<Vec<Repo>>();
        self.repo_command.upsert_many(&repos).await
    }

    pub async fn handle_project_updated(&self, event: &ProjectUpdated) -> AppResult<()> {
        self.handle_projects_updated(std::slice::from_ref(event))
            .await
    }

    pub async fn handle_projects_updated(&self, events: &[ProjectUpdated]) -> AppResult<()> {
        if events.is_empty() {
            return Ok(());
        }

        let projects = events
            .iter()
            .map(|event| event.project.clone())
            .collect::<Vec<_>>();
        let project_ids = projects
            .iter()
            .map(|project| project.id.clone())
            .collect::<Vec<_>>();
        let existing_repos = self.repos.list_by_ids(&project_ids).await?;
        if existing_repos.is_empty() {
            return Ok(());
        }

        let fetch_items = projects
            .into_iter()
            .filter_map(|project| {
                let repo = existing_repos
                    .iter()
                    .find(|repo| repo.id == project.id)
                    .cloned()?;
                Some((project, repo))
            })
            .collect::<Vec<_>>();

        let synced_repos = fetch_items
            .into_iter()
            .map(|(project, repo)| repo.with_project_overrides(&project))
            .collect::<Vec<Repo>>();

        if synced_repos.is_empty() {
            return Ok(());
        }
        self.repo_command.upsert_many(&synced_repos).await
    }
}
