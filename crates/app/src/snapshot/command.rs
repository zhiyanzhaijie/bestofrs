use std::sync::Arc;

use futures::{stream, StreamExt, TryStreamExt};

use domain::{Repo, Snapshot, SnapshotRecorded};

use crate::app_error::AppResult;
use crate::common::pagination::Pagination;
use crate::project::ProjectQueryHandler;
use crate::repo::{GithubGateway, RepoCommandHandler};
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
}

/// Command use case: ingest snapshots for all projects for today's date.
///
/// Note: This currently coordinates across contexts (project/repo/snapshot) which is fine for an
/// application-layer use case.
#[derive(Clone)]
pub struct IngestDailySnapshots {
    project_query: ProjectQueryHandler,
    repo_command: RepoCommandHandler,
    snapshot_command: SnapshotCommandHandler,
    github: Arc<dyn GithubGateway>,
    clock: Arc<dyn Clock>,
}

impl IngestDailySnapshots {
    pub fn new(
        project_query: ProjectQueryHandler,
        repo_command: RepoCommandHandler,
        snapshot_command: SnapshotCommandHandler,
        github: Arc<dyn GithubGateway>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            project_query,
            repo_command,
            snapshot_command,
            github,
            clock,
        }
    }

    pub async fn execute(&self) -> AppResult<IngestDailySnapshotsResult> {
        const FETCH_CONCURRENCY: usize = 16;
        let projects_page = self
            .project_query
            .list(Pagination {
                limit: Some(10_000),
                offset: Some(0),
            })
            .await?;
        let projects = projects_page.items;

        let today = self.clock.utc_today_ymd();
        let fetched_at = self.clock.utc_now_rfc3339();

        let github = self.github.clone();
        let fetched_at = fetched_at.clone();
        let today = today.clone();

        let fetched: Vec<(Repo, Snapshot)> = stream::iter(projects.iter())
            .map(|p| {
                let github = github.clone();
                let fetched_at = fetched_at.clone();
                let today = today.clone();
                async move {
                    let repo = github.fetch_repo(p.id.as_str()).await?;

                    let domain_repo = Repo {
                        id: p.id.clone(),
                        github_repo_id: Some(repo.id),
                        full_name: Some(repo.full_name),
                        stars: repo.stargazers_count,
                        forks: repo.forks_count,
                        open_issues: repo.open_issues_count,
                        watchers: repo.subscribers_count,
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

                    Ok::<(Repo, Snapshot), crate::app_error::AppError>((domain_repo, snapshot))
                }
            })
            .buffer_unordered(FETCH_CONCURRENCY)
            .try_collect()
            .await?;

        let mut repos = Vec::with_capacity(fetched.len());
        let mut snapshots = Vec::with_capacity(fetched.len());
        for (repo, snapshot) in fetched {
            repos.push(repo);
            snapshots.push(snapshot);
        }

        self.repo_command.upsert_many(&repos).await?;
        self.snapshot_command.insert_daily_many(&snapshots).await?;

        let repos_upserted = repos.len();
        let snapshots_inserted = snapshots.len();

        Ok(IngestDailySnapshotsResult {
            projects: projects.len(),
            repos_upserted,
            snapshots_inserted,
        })
    }
}
