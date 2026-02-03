use super::db_err;
use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::snapshot::{SnapshotDelta, SnapshotDeltaRepo, SnapshotRepo};
use domain::{RepoId, Snapshot};
use sqlx::{QueryBuilder, Sqlite};

#[derive(Debug, sqlx::FromRow)]
struct SnapshotDb {
    repo_id: String,
    snapshot_date: chrono::NaiveDate,
    stars: i64,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    fetched_at: String,
}

impl From<SnapshotDb> for Snapshot {
    fn from(db: SnapshotDb) -> Self {
        Self {
            repo_id: RepoId::new_unchecked(db.repo_id),
            snapshot_date: db.snapshot_date,
            stars: db.stars,
            forks: db.forks,
            open_issues: db.open_issues,
            watchers: db.watchers,
            fetched_at: db.fetched_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SnapshotDeltaDb {
    repo_id: String,
    snapshot_date: chrono::NaiveDate,
    prev_snapshot_date: Option<chrono::NaiveDate>,
    stars_delta: Option<i64>,
    forks_delta: Option<i64>,
    open_issues_delta: Option<i64>,
    watchers_delta: Option<i64>,
}

impl From<SnapshotDeltaDb> for SnapshotDelta {
    fn from(db: SnapshotDeltaDb) -> Self {
        Self {
            repo_id: RepoId::new_unchecked(db.repo_id),
            snapshot_date: db.snapshot_date,
            prev_snapshot_date: db.prev_snapshot_date,
            stars_delta: db.stars_delta,
            forks_delta: db.forks_delta,
            open_issues_delta: db.open_issues_delta,
            watchers_delta: db.watchers_delta,
        }
    }
}

#[derive(Clone)]
pub struct SqliteSnapshotRepo {
    pool: sqlx::SqlitePool,
}

impl SqliteSnapshotRepo {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SnapshotRepo for SqliteSnapshotRepo {
    async fn insert_daily(&self, snapshot: &Snapshot) -> AppResult<()> {
        self.insert_daily_many(std::slice::from_ref(snapshot)).await
    }

    async fn insert_daily_many(&self, snapshots: &[Snapshot]) -> AppResult<()> {
        if snapshots.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            INSERT INTO snapshots (
              repo_id, snapshot_date,
              stars, forks, open_issues, watchers,
              fetched_at
            )
            "#,
        );

        builder.push_values(snapshots, |mut b, s| {
            b.push_bind(s.repo_id.as_str())
                .push_bind(s.snapshot_date)
                .push_bind(s.stars)
                .push_bind(s.forks)
                .push_bind(s.open_issues)
                .push_bind(s.watchers)
                .push_bind(&s.fetched_at);
        });

        builder.push(
            r#"
            ON CONFLICT(repo_id, snapshot_date) DO NOTHING
            "#,
        );

        builder.build().execute(&mut *tx).await.map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;

        Ok(())
    }

    async fn list_by_repo(&self, repo_id: &RepoId, page: Pagination) -> AppResult<Page<Snapshot>> {
        let limit = page.limit();
        let offset = page.offset();

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM snapshots WHERE repo_id = ?")
            .bind(repo_id.as_str())
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;
        let rows: Vec<SnapshotDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id, snapshot_date,
              stars, forks, open_issues, watchers,
              fetched_at
            FROM snapshots
            WHERE repo_id = ?
            ORDER BY snapshot_date DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(repo_id.as_str())
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }
}

#[async_trait::async_trait]
impl SnapshotDeltaRepo for SqliteSnapshotRepo {
    async fn upsert(&self, item: &SnapshotDelta) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(item)).await
    }

    async fn upsert_many(&self, items: &[SnapshotDelta]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            INSERT INTO snapshot_deltas (
              repo_id, snapshot_date,
              prev_snapshot_date,
              stars_delta, forks_delta, open_issues_delta, watchers_delta,
              updated_at
            )
            "#,
        );

        builder.push_values(items, |mut b, d| {
            b.push_bind(d.repo_id.as_str())
                .push_bind(d.snapshot_date)
                .push_bind(d.prev_snapshot_date)
                .push_bind(d.stars_delta)
                .push_bind(d.forks_delta)
                .push_bind(d.open_issues_delta)
                .push_bind(d.watchers_delta)
                .push("datetime('now')");
        });

        builder.push(
            r#"
            ON CONFLICT(repo_id, snapshot_date) DO UPDATE SET
              prev_snapshot_date = excluded.prev_snapshot_date,
              stars_delta = excluded.stars_delta,
              forks_delta = excluded.forks_delta,
              open_issues_delta = excluded.open_issues_delta,
              watchers_delta = excluded.watchers_delta,
              updated_at = excluded.updated_at
            "#,
        );

        builder.build().execute(&mut *tx).await.map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;

        Ok(())
    }

    async fn upsert_for_date(
        &self,
        repo_ids: &[RepoId],
        snapshot_date: chrono::NaiveDate,
    ) -> AppResult<usize> {
        if repo_ids.is_empty() {
            return Ok(0);
        }

        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            INSERT INTO snapshot_deltas (
              repo_id, snapshot_date,
              prev_snapshot_date,
              stars_delta, forks_delta, open_issues_delta, watchers_delta,
              updated_at
            )
            SELECT
              s.repo_id,
              s.snapshot_date,
              prev.snapshot_date,
              CASE WHEN prev.snapshot_date IS NULL THEN NULL ELSE s.stars - prev.stars END,
              CASE WHEN prev.snapshot_date IS NULL THEN NULL ELSE s.forks - prev.forks END,
              CASE WHEN prev.snapshot_date IS NULL THEN NULL ELSE s.open_issues - prev.open_issues END,
              CASE WHEN prev.snapshot_date IS NULL THEN NULL ELSE s.watchers - prev.watchers END,
              datetime('now')
            FROM snapshots s
            LEFT JOIN snapshots prev
              ON prev.repo_id = s.repo_id
             AND prev.snapshot_date = (
                SELECT s2.snapshot_date
                FROM snapshots s2
                WHERE s2.repo_id = s.repo_id
                  AND s2.snapshot_date < s.snapshot_date
                ORDER BY s2.snapshot_date DESC
                LIMIT 1
             )
            WHERE s.snapshot_date = 
            "#,
        );

        builder.push_bind(snapshot_date);
        builder.push(" AND s.repo_id IN (");
        let mut separated = builder.separated(", ");
        for id in repo_ids {
            separated.push_bind(id.as_str());
        }
        builder.push(") ");

        builder.push(
            r#"
            ON CONFLICT(repo_id, snapshot_date) DO UPDATE SET
              prev_snapshot_date = excluded.prev_snapshot_date,
              stars_delta = excluded.stars_delta,
              forks_delta = excluded.forks_delta,
              open_issues_delta = excluded.open_issues_delta,
              watchers_delta = excluded.watchers_delta,
              updated_at = excluded.updated_at
            "#,
        );

        let result = builder.build().execute(&self.pool).await.map_err(db_err)?;

        Ok(result.rows_affected() as usize)
    }

    async fn list_by_repo(
        &self,
        repo_id: &RepoId,
        page: Pagination,
    ) -> AppResult<Page<SnapshotDelta>> {
        let limit = page.limit();
        let offset = page.offset();

        let total: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM snapshot_deltas WHERE repo_id = ?")
                .bind(repo_id.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(db_err)?;
        let rows: Vec<SnapshotDeltaDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id, snapshot_date,
              prev_snapshot_date,
              stars_delta, forks_delta, open_issues_delta, watchers_delta
            FROM snapshot_deltas
            WHERE repo_id = ?
            ORDER BY snapshot_date DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(repo_id.as_str())
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }
}
