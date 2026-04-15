use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::repo::{RepoRankMetric, RepoRankQuery, RepoRankTimeRange, RepoRepo};
use async_trait::async_trait;
use chrono::Duration;
use domain::{Repo, RepoId};
use sqlx::{Postgres, QueryBuilder};

use super::db_err;

#[derive(Debug, sqlx::FromRow)]
struct RepoDb {
    id: String,
    github_repo_id: Option<i64>,
    full_name: Option<String>,
    description: Option<String>,
    homepage_url: Option<String>,
    avatar_url: Option<String>,
    stars: i64,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    created_at: String,
    last_fetched_at: Option<String>,
    etag: Option<String>,
}

impl From<RepoDb> for Repo {
    fn from(db: RepoDb) -> Self {
        Self {
            id: RepoId::new_unchecked(db.id),
            github_repo_id: db.github_repo_id,
            full_name: db.full_name,
            description: db.description,
            homepage_url: db.homepage_url,
            avatar_url: db.avatar_url,
            stars: db.stars,
            forks: db.forks,
            open_issues: db.open_issues,
            watchers: db.watchers,
            created_at: db.created_at,
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

#[async_trait]
impl RepoRepo for PostgresRepoRepo {
    async fn upsert(&self, repo: &Repo) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(repo)).await
    }

    async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()> {
        if repos.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let mut insert_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO repos (
              id, github_repo_id, full_name, description,
              homepage_url, avatar_url,
              stars, forks, open_issues, watchers,
              last_fetched_at, etag,
              updated_at
            )
            "#,
        );

        insert_builder.push_values(repos, |mut b, r| {
            b.push_bind(r.id.as_str())
                .push_bind(r.github_repo_id)
                .push_bind(&r.full_name)
                .push_bind(&r.description)
                .push_bind(&r.homepage_url)
                .push_bind(&r.avatar_url)
                .push_bind(r.stars)
                .push_bind(r.forks)
                .push_bind(r.open_issues)
                .push_bind(r.watchers)
                .push_bind(&r.last_fetched_at)
                .push_bind(&r.etag)
                .push("NOW()");
        });

        insert_builder.push(
            r#"
            ON CONFLICT DO NOTHING
            "#,
        );

        insert_builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        let mut update_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            UPDATE repos AS r
            SET
              github_repo_id = incoming.github_repo_id,
              full_name = incoming.full_name,
              description = incoming.description,
              homepage_url = incoming.homepage_url,
              avatar_url = incoming.avatar_url,
              stars = incoming.stars,
              forks = incoming.forks,
              open_issues = incoming.open_issues,
              watchers = incoming.watchers,
              last_fetched_at = incoming.last_fetched_at,
              etag = incoming.etag,
              updated_at = NOW()
            FROM (
            "#,
        );

        update_builder.push_values(repos, |mut b, r| {
            b.push_bind(r.id.as_str())
                .push_bind(r.github_repo_id)
                .push_bind(&r.full_name)
                .push_bind(&r.description)
                .push_bind(&r.homepage_url)
                .push_bind(&r.avatar_url)
                .push_bind(r.stars)
                .push_bind(r.forks)
                .push_bind(r.open_issues)
                .push_bind(r.watchers)
                .push_bind(&r.last_fetched_at)
                .push_bind(&r.etag);
        });

        update_builder.push(
            r#"
            ) AS incoming(id, github_repo_id, full_name, description, homepage_url, avatar_url, stars, forks, open_issues, watchers, last_fetched_at, etag)
            WHERE r.id = incoming.id
              AND (
                incoming.github_repo_id IS NULL
                OR NOT EXISTS (
                  SELECT 1
                  FROM repos r2
                  WHERE r2.id <> incoming.id
                    AND r2.github_repo_id = incoming.github_repo_id
                )
              )
            "#,
        );

        update_builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;

        Ok(())
    }

    async fn get(&self, id: &RepoId) -> AppResult<Option<Repo>> {
        let row: Option<RepoDb> = sqlx::query_as(
            r#"
            SELECT
              id, github_repo_id, full_name, description,
              homepage_url, avatar_url,
              stars, forks, open_issues, watchers,
              created_at::TEXT AS created_at,
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
    async fn find_existing_ids(&self, ids: &[RepoId]) -> AppResult<Vec<RepoId>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT id
            FROM repos
            WHERE id IN (
            "#,
        );
        let mut separated = qb.separated(", ");
        for id in ids {
            separated.push_bind(id.as_str());
        }
        qb.push(
            r#"
            )
            "#,
        );
        let rows: Vec<(String,)> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|(id,)| RepoId::new_unchecked(id))
            .collect())
    }

    async fn find_existing_github_repo_ids(&self, ids: &[i64]) -> AppResult<Vec<i64>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT github_repo_id
            FROM repos
            WHERE github_repo_id IS NOT NULL
              AND github_repo_id IN (
            "#,
        );
        let mut separated = qb.separated(", ");
        for id in ids {
            separated.push_bind(id);
        }
        qb.push(
            r#"
            )
            "#,
        );
        let rows: Vec<(i64,)> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    async fn list_by_ids(&self, ids: &[RepoId]) -> AppResult<Vec<Repo>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
              id, github_repo_id, full_name, description,
              homepage_url, avatar_url,
              stars, forks, open_issues, watchers,
              created_at::TEXT AS created_at,
              last_fetched_at, etag
            FROM repos
            WHERE id IN (
            "#,
        );
        let mut separated = qb.separated(", ");
        for id in ids {
            separated.push_bind(id.as_str());
        }
        qb.push(
            r#"
            )
            "#,
        );

        let rows: Vec<RepoDb> = qb
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_repos(&self, page: Pagination) -> AppResult<Page<Repo>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM repos")
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let rows: Vec<RepoDb> = sqlx::query_as(
            r#"
            SELECT
              id, github_repo_id, full_name, description,
              homepage_url, avatar_url,
              stars, forks, open_issues, watchers,
              created_at::TEXT AS created_at,
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

    async fn list_ranked(&self, query: RepoRankQuery, page: Pagination) -> AppResult<Page<Repo>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM repos")
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        if query.metric == RepoRankMetric::Recent {
            let rows: Vec<RepoDb> = sqlx::query_as(
                r#"
                SELECT
                  id, github_repo_id, full_name, description,
                  homepage_url, avatar_url,
                  stars, forks, open_issues, watchers,
                  created_at::TEXT AS created_at,
                  last_fetched_at, etag
                FROM repos
                ORDER BY created_at DESC, stars DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
            let items = rows.into_iter().map(Into::into).collect();
            return Ok(page.to_page(items, total as u64));
        }

        let anchor_date: Option<chrono::NaiveDate> =
            sqlx::query_scalar("SELECT MAX(snapshot_date) FROM snapshot_deltas")
                .fetch_one(&self.pool)
                .await
                .map_err(db_err)?;

        let rows: Vec<RepoDb> = if let Some(anchor_date) = anchor_date {
            let order_expr = match query.metric {
                RepoRankMetric::Star => "stars",
                RepoRankMetric::Fork => "forks",
                RepoRankMetric::Issue => "open_issues",
                RepoRankMetric::Recent => "r.created_at",
            };
            if query.range == RepoRankTimeRange::All {
                let sql = format!(
                    r#"
                    SELECT
                      r.id, r.github_repo_id, r.full_name, r.description,
                      r.homepage_url, r.avatar_url,
                      COALESCE(SUM(d.stars_delta), 0)::BIGINT AS stars,
                      COALESCE(SUM(d.forks_delta), 0)::BIGINT AS forks,
                      ABS(COALESCE(SUM(d.open_issues_delta), 0))::BIGINT AS open_issues,
                      r.watchers,
                      r.created_at::TEXT AS created_at,
                      r.last_fetched_at, r.etag
                    FROM repos r
                    LEFT JOIN snapshot_deltas d
                      ON d.repo_id = r.id
                    GROUP BY
                      r.id, r.github_repo_id, r.full_name, r.description,
                      r.homepage_url, r.avatar_url,
                      r.watchers, r.created_at,
                      r.last_fetched_at, r.etag
                    ORDER BY {order_expr} DESC, r.stars DESC
                    LIMIT $1 OFFSET $2
                    "#
                );
                sqlx::query_as(&sql)
                    .bind(limit as i64)
                    .bind(offset as i64)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(db_err)?
            } else {
                let window_days = match query.range {
                    RepoRankTimeRange::Daily => 1,
                    RepoRankTimeRange::Weekly => 7,
                    RepoRankTimeRange::Monthly => 30,
                    RepoRankTimeRange::All => unreachable!(),
                };
                let range_start = anchor_date - Duration::days((window_days - 1) as i64);
                let sql = format!(
                    r#"
                    SELECT
                      r.id, r.github_repo_id, r.full_name, r.description,
                      r.homepage_url, r.avatar_url,
                      COALESCE(SUM(d.stars_delta), 0)::BIGINT AS stars,
                      COALESCE(SUM(d.forks_delta), 0)::BIGINT AS forks,
                      ABS(COALESCE(SUM(d.open_issues_delta), 0))::BIGINT AS open_issues,
                      r.watchers,
                      r.created_at::TEXT AS created_at,
                      r.last_fetched_at, r.etag
                    FROM repos r
                    LEFT JOIN snapshot_deltas d
                      ON d.repo_id = r.id
                     AND d.snapshot_date >= $1
                     AND d.snapshot_date <= $2
                    GROUP BY
                      r.id, r.github_repo_id, r.full_name, r.description,
                      r.homepage_url, r.avatar_url,
                      r.watchers, r.created_at,
                      r.last_fetched_at, r.etag
                    ORDER BY {order_expr} DESC, r.stars DESC
                    LIMIT $3 OFFSET $4
                    "#
                );
                sqlx::query_as(&sql)
                    .bind(range_start)
                    .bind(anchor_date)
                    .bind(limit as i64)
                    .bind(offset as i64)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(db_err)?
            }
        } else {
            let fallback_order = match query.metric {
                RepoRankMetric::Star => "stars",
                RepoRankMetric::Fork => "forks",
                RepoRankMetric::Issue => "open_issues",
                RepoRankMetric::Recent => "created_at",
            };
            let sql = format!(
                r#"
                SELECT
                  id, github_repo_id, full_name, description,
                  homepage_url, avatar_url,
                  0 AS stars, 0 AS forks, 0 AS open_issues, watchers,
                  created_at::TEXT AS created_at,
                  last_fetched_at, etag
                FROM repos
                ORDER BY {fallback_order} DESC, stars DESC
                LIMIT $1 OFFSET $2
                "#
            );
            sqlx::query_as(&sql)
                .bind(limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await
                .map_err(db_err)?
        };

        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn search_repos_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Repo>> {
        let key = format!("%{key}%");
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT r.id)
            FROM repos r
            LEFT JOIN projects p ON p.repo_id = r.id
            WHERE r.id ILIKE $1 OR r.full_name ILIKE $1 OR p.description ILIKE $1
            "#,
        )
        .bind(&key)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<RepoDb> = sqlx::query_as(
            r#"
            SELECT
              r.id, r.github_repo_id, r.full_name, r.description,
              r.homepage_url, r.avatar_url,
              r.stars, r.forks, r.open_issues, r.watchers,
              r.created_at::TEXT AS created_at,
              r.last_fetched_at, r.etag
            FROM repos r
            LEFT JOIN projects p ON p.repo_id = r.id
            WHERE r.id ILIKE $1 OR r.full_name ILIKE $1 OR p.description ILIKE $1
            ORDER BY r.stars DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&key)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }
}
