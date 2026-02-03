use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::repo::RepoRepo;
use domain::{Repo, RepoId};
use sqlx::{Postgres, QueryBuilder};

use super::db_err;

#[derive(Debug, sqlx::FromRow)]
struct RepoDb {
    id: String,
    github_repo_id: Option<i64>,
    full_name: Option<String>,
    stars: i64,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    last_fetched_at: Option<String>,
    etag: Option<String>,
}

impl From<RepoDb> for Repo {
    fn from(db: RepoDb) -> Self {
        Self {
            id: RepoId::new_unchecked(db.id),
            github_repo_id: db.github_repo_id,
            full_name: db.full_name,
            stars: db.stars,
            forks: db.forks,
            open_issues: db.open_issues,
            watchers: db.watchers,
            last_fetched_at: db.last_fetched_at,
            etag: db.etag,
        }
    }
}

#[derive(Clone)]
pub struct PostgresRepoRepo {
    pool: sqlx::PgPool,
}

impl PostgresRepoRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RepoRepo for PostgresRepoRepo {
    async fn upsert(&self, repo: &Repo) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(repo)).await
    }

    async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()> {
        if repos.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO repos (
              id, github_repo_id, full_name,
              stars, forks, open_issues, watchers,
              last_fetched_at, etag,
              updated_at
            )
            "#,
        );

        builder.push_values(repos, |mut b, r| {
            b.push_bind(r.id.as_str())
                .push_bind(r.github_repo_id)
                .push_bind(&r.full_name)
                .push_bind(r.stars)
                .push_bind(r.forks)
                .push_bind(r.open_issues)
                .push_bind(r.watchers)
                .push_bind(&r.last_fetched_at)
                .push_bind(&r.etag)
                .push("NOW()");
        });

        builder.push(
            r#"
            ON CONFLICT(id) DO UPDATE SET
              github_repo_id = excluded.github_repo_id,
              full_name = excluded.full_name,
              stars = excluded.stars,
              forks = excluded.forks,
              open_issues = excluded.open_issues,
              watchers = excluded.watchers,
              last_fetched_at = excluded.last_fetched_at,
              etag = excluded.etag,
              updated_at = excluded.updated_at
            "#,
        );

        builder.build().execute(&mut *tx).await.map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;

        Ok(())
    }

    async fn get(&self, id: &RepoId) -> AppResult<Option<Repo>> {
        let row: Option<RepoDb> = sqlx::query_as(
            r#"
            SELECT
              id, github_repo_id, full_name,
              stars, forks, open_issues, watchers,
              last_fetched_at, etag
            FROM repos
            WHERE id = $1
            "#,
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(Into::into))
    }

    async fn list(&self, page: Pagination) -> AppResult<Page<Repo>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM repos")
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let rows: Vec<RepoDb> = sqlx::query_as(
            r#"
            SELECT
              id, github_repo_id, full_name,
              stars, forks, open_issues, watchers,
              last_fetched_at, etag
            FROM repos
            ORDER BY stars DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }
}
