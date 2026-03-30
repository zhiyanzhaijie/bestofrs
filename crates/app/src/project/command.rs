use futures::{stream, StreamExt};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use domain::{Project, ProjectCreated, ProjectStatus, ProjectUpdated, Repo, RepoId, Tag};
use serde::{Deserialize, Serialize};

use crate::app_error::AppResult;
use crate::project::{ProjectEventHandler, ProjectRepo};
use crate::repo::{GithubGateway, RepoGithubLookupKey, RepoRepo, RepoTagRepo};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportProjectCommand {
    pub repo_id: String,
    pub name: String,
    pub slug: String,
    pub description: String,

    pub url: Option<String>,
    pub avatar_url: Option<String>,

    pub status: ProjectStatus,
    pub twitter: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportProjectsCommand {
    pub items: Vec<ImportProjectCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportProjectsReport {
    pub total: usize,
    pub upserted: usize,
    pub skipped_invalid: usize,
    pub failed_upsert: usize,

    #[serde(default)]
    pub invalid_examples: Vec<String>,

    #[serde(default)]
    pub error_examples: Vec<String>,
}

impl ImportProjectsReport {
    fn new(total: usize) -> Self {
        Self {
            total,
            upserted: 0,
            skipped_invalid: 0,
            failed_upsert: 0,
            invalid_examples: Vec::new(),
            error_examples: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoveProjectCommand {
    pub repo_id: String,
}

#[derive(Clone)]
pub struct ProjectCommandHandler {
    projects: Arc<dyn ProjectRepo>,
    repos: Arc<dyn RepoRepo>,
    repo_tags: Arc<dyn RepoTagRepo>,
    github: Arc<dyn GithubGateway>,
    event_handler: ProjectEventHandler,
}

impl ProjectCommandHandler {
    pub fn new(
        projects: Arc<dyn ProjectRepo>,
        repos: Arc<dyn RepoRepo>,
        repo_tags: Arc<dyn RepoTagRepo>,
        github: Arc<dyn GithubGateway>,
        event_handler: ProjectEventHandler,
    ) -> Self {
        Self {
            projects,
            repos,
            repo_tags,
            github,
            event_handler,
        }
    }

    pub async fn import_projects(
        &self,
        cmd: ImportProjectsCommand,
    ) -> AppResult<ImportProjectsReport> {
        const MAX_INVALID_EXAMPLES: usize = 20;
        const MAX_ERROR_EXAMPLES: usize = 20;

        let mut report = ImportProjectsReport::new(cmd.items.len());

        let mut projects = Vec::new();
        let mut input_tags_by_repo_id: HashMap<String, Option<Vec<String>>> = HashMap::new();

        for item in cmd.items {
            let repo_id = match RepoId::parse(&item.repo_id) {
                Ok(v) => v,
                Err(_) => {
                    report.skipped_invalid += 1;
                    if report.invalid_examples.len() < MAX_INVALID_EXAMPLES {
                        report.invalid_examples.push(item.repo_id);
                    }
                    continue;
                }
            };
            let normalized_tags = item.tags.map(Self::normalize_tag_values);
            input_tags_by_repo_id.insert(repo_id.as_str().to_string(), normalized_tags);
            projects.push(Project {
                id: repo_id,
                name: item.name,
                slug: item.slug,
                description: item.description,
                url: item.url,
                avatar_url: item.avatar_url,
                status: item.status,
                twitter: item.twitter,
            });
        }

        if !projects.is_empty() {
            const FETCH_CONCURRENCY: usize = 16;
            let github = self.github.clone();
            let fetched = stream::iter(projects.iter().cloned())
                .map(|project| {
                    let github = github.clone();
                    async move {
                        let github_repo = github
                            .fetch_repo_by_lookup_key(&RepoGithubLookupKey::from_repo_id(
                                project.id.as_str(),
                            ))
                            .await
                            .map_err(|err| {
                                format!(
                                    "invalid repo_id `{}`: github lookup failed: {err}",
                                    project.id.as_str()
                                )
                            })?;
                        Ok::<(Project, Repo), String>((
                            project.clone(),
                            Repo {
                                id: project.id,
                                github_repo_id: Some(github_repo.id),
                                full_name: Some(github_repo.full_name),
                                description: github_repo.description,
                                homepage_url: github_repo.homepage,
                                avatar_url: github_repo.owner_avatar_url,
                                stars: github_repo.stargazers_count,
                                forks: github_repo.forks_count,
                                open_issues: github_repo.open_issues_count,
                                watchers: github_repo.subscribers_count,
                                created_at: chrono::Utc::now().to_rfc3339(),
                                last_fetched_at: None,
                                etag: None,
                            },
                        ))
                    }
                })
                .buffer_unordered(FETCH_CONCURRENCY)
                .collect::<Vec<_>>()
                .await;
            let mut upsert_projects = Vec::new();
            let mut repos = Vec::new();
            for item in fetched {
                match item {
                    Ok((project, repo)) => {
                        upsert_projects.push(project);
                        repos.push(repo);
                    }
                    Err(err) => {
                        report.failed_upsert += 1;
                        if report.error_examples.len() < MAX_ERROR_EXAMPLES {
                            report.error_examples.push(err);
                        }
                    }
                }
            }
            if !upsert_projects.is_empty() {
                self.projects.upsert_many(&upsert_projects).await?;
            }
            let events = repos
                .iter()
                .cloned()
                .map(|repo| ProjectCreated { repo })
                .collect::<Vec<_>>();
            self.event_handler.handle_projects_created(&events).await?;
            let mut all_requested_values = Vec::new();
            for project in &upsert_projects {
                if let Some(Some(values)) = input_tags_by_repo_id.get(project.id.as_str()) {
                    all_requested_values.extend(values.iter().cloned());
                }
            }
            let all_requested_values = Self::normalize_tag_values(all_requested_values);
            let matched_tags = self
                .repo_tags
                .find_tags_by_values(&all_requested_values)
                .await?;
            let mut matched_tag_by_value = HashMap::new();
            for matched_tag in matched_tags {
                matched_tag_by_value.insert(matched_tag.value.as_str().to_string(), matched_tag);
            }
            let repo_ids = upsert_projects
                .iter()
                .map(|project| project.id.clone())
                .collect::<Vec<_>>();
            let existing_repo_ids = self.repos.find_existing_ids(&repo_ids).await?;
            let existing_repo_ids = existing_repo_ids
                .into_iter()
                .map(|id| id.as_str().to_string())
                .collect::<HashSet<_>>();
            let mut replace_items = Vec::new();
            for project in &upsert_projects {
                if !existing_repo_ids.contains(project.id.as_str()) {
                    continue;
                }
                let Some(Some(values)) = input_tags_by_repo_id.get(project.id.as_str()) else {
                    continue;
                };
                let resolved_tags = values
                    .iter()
                    .filter_map(|value| matched_tag_by_value.get(value).cloned())
                    .collect::<Vec<Tag>>();
                replace_items.push((project.id.clone(), resolved_tags));
            }
            if !replace_items.is_empty() {
                self.repo_tags
                    .replace_repo_tags_bulk(&replace_items)
                    .await?;
            }
            report.upserted = upsert_projects.len();
        }
        Ok(report)
    }

    pub async fn update_projects(
        &self,
        cmd: ImportProjectsCommand,
    ) -> AppResult<ImportProjectsReport> {
        const MAX_INVALID_EXAMPLES: usize = 20;
        let mut report = ImportProjectsReport::new(cmd.items.len());
        let mut projects = Vec::new();
        for item in cmd.items {
            let repo_id = match RepoId::parse(&item.repo_id) {
                Ok(v) => v,
                Err(_) => {
                    report.skipped_invalid += 1;
                    if report.invalid_examples.len() < MAX_INVALID_EXAMPLES {
                        report.invalid_examples.push(item.repo_id);
                    }
                    continue;
                }
            };
            projects.push(Project {
                id: repo_id,
                name: item.name,
                slug: item.slug,
                description: item.description,
                url: item.url,
                avatar_url: item.avatar_url,
                status: item.status,
                twitter: item.twitter,
            });
        }
        if !projects.is_empty() {
            self.projects.update_many(&projects).await?;
            let events = projects
                .iter()
                .cloned()
                .map(|project| ProjectUpdated { project })
                .collect::<Vec<_>>();
            self.event_handler.handle_projects_updated(&events).await?;
            report.upserted = projects.len();
        }
        Ok(report)
    }

    pub async fn remove_project(&self, cmd: RemoveProjectCommand) -> AppResult<()> {
        let repo_id = RepoId::parse(&cmd.repo_id)?;
        self.projects.remove(repo_id.to_string()).await
    }

    fn normalize_tag_values(values: Vec<String>) -> Vec<String> {
        let mut dedup = HashSet::new();
        let mut out = Vec::new();
        for value in values {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            let value = value.to_string();
            if dedup.insert(value.clone()) {
                out.push(value);
            }
        }
        out
    }
}
