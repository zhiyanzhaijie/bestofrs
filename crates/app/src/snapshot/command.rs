use std::collections::HashSet;
use std::sync::Arc;

use futures::{stream, StreamExt};

use domain::{Repo, Snapshot, SnapshotRecorded};

use crate::app_error::AppResult;
use crate::common::pagination::Pagination;
use crate::project::{ProjectQueryHandler, ProjectStatusExt};
use crate::repo::{
    GithubGateway, RepoCommandHandler, RepoGithubLookupExt,
    RepoGithubLookupKey, RepoRepo,
};
use crate::snapshot::{Clock, SnapshotEventHandler, SnapshotRepo};

#[derive(Clone)]
pub struct SnapshotCommandHandler {
    snapshots: Arc<dyn SnapshotRepo>,
    event_handler: SnapshotEventHandler,
}

impl SnapshotCommandHandler {
    pub fn new(snapshots: Arc<dyn SnapshotRepo>, event_handler: SnapshotEventHandler) -> Self {
        Self {
            snapshots,
            event_handler,
        }
    }
    pub async fn insert_daily(&self, snapshot: &Snapshot) -> AppResult<()> {
        self.insert_daily_many(std::slice::from_ref(snapshot)).await
    }

    pub async fn insert_daily_many(&self, snapshots: &[Snapshot]) -> AppResult<()> {
        if snapshots.is_empty() {
            return Ok(());
        }

        self.snapshots.insert_daily_many(snapshots).await?;

        let events = snapshots
            .iter()
            .map(|s| SnapshotRecorded {
                repo_id: s.repo_id.clone(),
                snapshot_date: s.snapshot_date,
            })
            .collect::<Vec<_>>();

        self.event_handler
            .handle_snapshots_recorded(&events)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestDailySnapshotsResult {
    pub projects: usize,
    pub repos_upserted: usize,
    pub snapshots_inserted: usize,
    pub failures: Vec<IngestDailySnapshotFailure>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestDailySnapshotFailure {
    pub repo_id: String,
    pub lookup_key: String,
    pub error: String,
}

/// Command use case: ingest snapshots for all projects for today's date.
///
/// Note: This currently coordinates across contexts (project/repo/snapshot) which is fine for an
/// application-layer use case.
#[derive(Clone)]
pub struct IngestDailySnapshots {
    project_query: ProjectQueryHandler,
    repo_command: RepoCommandHandler,
    repos: Arc<dyn RepoRepo>,
    snapshot_command: SnapshotCommandHandler,
    github: Arc<dyn GithubGateway>,
    clock: Arc<dyn Clock>,
}

impl IngestDailySnapshots {
    pub fn new(
        project_query: ProjectQueryHandler,
        repo_command: RepoCommandHandler,
        repos: Arc<dyn RepoRepo>,
        snapshot_command: SnapshotCommandHandler,
        github: Arc<dyn GithubGateway>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            project_query,
            repo_command,
            repos,
            snapshot_command,
            github,
            clock,
        }
    }

    pub async fn execute(&self) -> AppResult<IngestDailySnapshotsResult> {
        const FETCH_CONCURRENCY: usize = 16;
        let mut projects = Vec::new();
        let mut offset = 0u32;
        loop {
            let projects_page = self
                .project_query
                .list(Pagination {
                    limit: Some(Pagination::MAX_LIMIT),
                    offset: Some(offset),
                })
                .await?;
            let total = projects_page.meta.total;
            let page_items = projects_page.items;
            if page_items.is_empty() {
                break;
            }
            offset = offset.saturating_add(page_items.len() as u32);
            projects.extend(page_items);
            if offset as u64 >= total {
                break;
            }
        }

        let projects_total = projects.len();
        let projects = projects
            .into_iter()
            .filter(|project| !project.is_disabled())
            .collect::<Vec<_>>();

        let today = self.clock.utc_today_ymd();
        let fetched_at = self.clock.utc_now_rfc3339();

        let project_ids = projects.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
        let existing_repos = self.repos.list_by_ids(&project_ids).await?;
        let fetch_items = projects
            .into_iter()
            .map(|project| {
                let lookup_key = existing_repos
                    .iter()
                    .find(|repo| repo.id == project.id)
                    .map(|repo| repo.github_lookup_key())
                    .unwrap_or_else(|| RepoGithubLookupKey::from_repo_id(project.id.as_str()));
                (project, lookup_key)
            })
            .collect::<Vec<_>>();

        let github = self.github.clone();
        let fetched = stream::iter(fetch_items.into_iter())
            .map(|(project, lookup_key)| {
                let github = github.clone();
                let fetched_at = fetched_at.clone();
                async move {
                    let lookup_key_display = format!("{lookup_key:?}");
                    let repo_id = project.id.as_str().to_string();

                    match github.fetch_repo_by_lookup_key(&lookup_key).await {
                        Ok(repo) => {
                            let homepage_url = Self::resolve_homepage_url(
                                repo.homepage.as_deref(),
                                project.url.as_deref(),
                            );
                            let avatar_url = Self::resolve_avatar_url(
                                project.id.as_str(),
                                project.avatar_url.as_deref(),
                                repo.owner_avatar_url.as_deref(),
                            );

                            let domain_repo = Repo {
                                id: project.id,
                                github_repo_id: Some(repo.id),
                                full_name: Some(repo.full_name),
                                description: repo.description,
                                homepage_url,
                                avatar_url,
                                stars: repo.stargazers_count,
                                forks: repo.forks_count,
                                open_issues: repo.open_issues_count,
                                watchers: repo.subscribers_count,
                                created_at: fetched_at.clone(),
                                last_fetched_at: Some(fetched_at.clone()),
                                etag: None,
                            };

                            let snapshot = Snapshot {
                                repo_id: domain_repo.id.clone(),
                                snapshot_date: today,
                                stars: domain_repo.stars,
                                forks: domain_repo.forks,
                                open_issues: domain_repo.open_issues,
                                watchers: domain_repo.watchers,
                                fetched_at: fetched_at.clone(),
                            };

                            Ok((domain_repo, snapshot))
                        }
                        Err(err) => Err(IngestDailySnapshotFailure {
                            repo_id,
                            lookup_key: lookup_key_display,
                            error: err.to_string(),
                        }),
                    }
                }
            })
            .buffer_unordered(FETCH_CONCURRENCY)
            .collect::<Vec<_>>()
            .await;

        let mut failures = Vec::new();
        let mut repos = Vec::new();
        let mut snapshots = Vec::new();
        for item in fetched {
            match item {
                Ok((repo, snapshot)) => {
                    repos.push(repo);
                    snapshots.push(snapshot);
                }
                Err(failure) => failures.push(failure),
            }
        }

        self.repo_command.upsert_many(&repos).await?;
        let repo_ids = repos.iter().map(|repo| repo.id.clone()).collect::<Vec<_>>();
        let existing_repo_ids = self.repos.find_existing_ids(&repo_ids).await?;
        let existing_repo_ids = existing_repo_ids
            .into_iter()
            .map(|id| id.as_str().to_string())
            .collect::<HashSet<_>>();
        let snapshots = snapshots
            .into_iter()
            .filter(|snapshot| existing_repo_ids.contains(snapshot.repo_id.as_str()))
            .collect::<Vec<_>>();
        self.snapshot_command.insert_daily_many(&snapshots).await?;

        let repos_upserted = repos.len();
        let snapshots_inserted = snapshots.len();

        Ok(IngestDailySnapshotsResult {
            projects: projects_total,
            repos_upserted,
            snapshots_inserted,
            failures,
        })
    }

    fn resolve_homepage_url(github_homepage: Option<&str>, project_url: Option<&str>) -> Option<String> {
        github_homepage
            .and_then(Self::normalize_url)
            .or_else(|| project_url.and_then(Self::normalize_url))
    }

    fn resolve_avatar_url(
        repo_id: &str,
        project_avatar_url: Option<&str>,
        owner_avatar_url: Option<&str>,
    ) -> Option<String> {
        project_avatar_url
            .and_then(Self::normalize_url)
            .or_else(|| owner_avatar_url.and_then(Self::normalize_url))
            .or_else(|| {
                let owner = repo_id.split('/').next()?;
                if owner.is_empty() {
                    return None;
                }
                Some(format!("https://github.com/{owner}.png"))
            })
    }

    fn normalize_url(value: &str) -> Option<String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(trimmed.to_string())
    }
}
