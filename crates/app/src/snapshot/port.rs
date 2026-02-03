use chrono::NaiveDate;

use domain::{RepoId, Snapshot};

use crate::app_error::AppResult;
use crate::common::pagination::{Page, Pagination};

#[async_trait::async_trait]
pub trait SnapshotRepo: Send + Sync {
    async fn insert_daily(&self, snapshot: &Snapshot) -> AppResult<()>;
    async fn insert_daily_many(&self, snapshots: &[Snapshot]) -> AppResult<()>;

    async fn list_by_repo(&self, repo_id: &RepoId, page: Pagination) -> AppResult<Page<Snapshot>>;
}

/// Delta derived from snapshots.
///
/// Stored separately for fast reads (trend charts, top movers, reports).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotDelta {
    pub repo_id: RepoId,
    pub snapshot_date: NaiveDate,

    pub prev_snapshot_date: Option<NaiveDate>,

    pub stars_delta: Option<i64>,
    pub forks_delta: Option<i64>,
    pub open_issues_delta: Option<i64>,
    pub watchers_delta: Option<i64>,
}

#[async_trait::async_trait]
pub trait SnapshotDeltaRepo: Send + Sync {
    async fn upsert(&self, item: &SnapshotDelta) -> AppResult<()>;
    async fn upsert_many(&self, items: &[SnapshotDelta]) -> AppResult<()>;
    async fn upsert_for_date(
        &self,
        repo_ids: &[RepoId],
        snapshot_date: NaiveDate,
    ) -> AppResult<usize>;

    async fn list_by_repo(
        &self,
        repo_id: &RepoId,
        page: Pagination,
    ) -> AppResult<Page<SnapshotDelta>>;
}

pub trait Clock: Send + Sync {
    fn utc_today_ymd(&self) -> chrono::NaiveDate;
    fn utc_now_rfc3339(&self) -> String;
}
