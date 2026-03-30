use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::project::{ProjectRepo, parse_project_status, project_status_value};
use domain::{Project, ProjectStatus, RepoId};
use sqlx::{Postgres, QueryBuilder};

use super::db_err;

#[derive(Debug, sqlx::FromRow)]
struct ProjectDb {
    repo_id: String,
    name: String,
    slug: String,
    description: String,
    url: Option<String>,
    avatar_url: Option<String>,
    status: Option<String>,
    twitter: Option<String>,
}

impl From<ProjectDb> for Project {
    fn from(db: ProjectDb) -> Self {
        Self {
            id: RepoId::new_unchecked(db.repo_id),
            name: db.name,
            slug: db.slug,
            description: db.description,
            url: db.url,
            avatar_url: db.avatar_url,
            status: parse_project_status(db.status.as_deref()),
            twitter: db.twitter,
        }
    }
}

#[derive(Clone)]
pub struct PostgresProjectRepo {
    pool: sqlx::PgPool,
}

impl PostgresProjectRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ProjectRepo for PostgresProjectRepo {
    async fn get(&self, repo_id: &RepoId) -> AppResult<Option<Project>> {
        let row: Option<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, twitter
            FROM projects
            WHERE repo_id = $1
            "#,
        )
        .bind(repo_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(Into::into))
    }

    async fn upsert(&self, project: &Project) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(project)).await
    }

    async fn upsert_many(&self, items: &[Project]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let mut insert_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO projects (
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, twitter,
              updated_at
            )
            "#,
        );

        insert_builder.push_values(items, |mut b, p| {
            b.push_bind(p.id.as_str())
                .push_bind(&p.name)
                .push_bind(&p.slug)
                .push_bind(&p.description)
                .push_bind(&p.url)
                .push_bind(&p.avatar_url)
                .push_bind(project_status_value(p.status))
                .push_bind(&p.twitter)
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
            UPDATE projects AS p
            SET
              name = incoming.name,
              slug = incoming.slug,
              description = incoming.description,
              url = incoming.url,
              avatar_url = incoming.avatar_url,
              status = incoming.status,
              twitter = incoming.twitter,
              updated_at = NOW()
            FROM (
            "#,
        );

        update_builder.push_values(items, |mut b, p| {
            b.push_bind(p.id.as_str())
                .push_bind(&p.name)
                .push_bind(&p.slug)
                .push_bind(&p.description)
                .push_bind(&p.url)
                .push_bind(&p.avatar_url)
                .push_bind(project_status_value(p.status))
                .push_bind(&p.twitter);
        });

        update_builder.push(
            r#"
            ) AS incoming(repo_id, name, slug, description, url, avatar_url, status, twitter)
            WHERE p.repo_id = incoming.repo_id
              AND NOT EXISTS (
                SELECT 1
                FROM projects p2
                WHERE p2.repo_id <> incoming.repo_id
                  AND (p2.name = incoming.name OR p2.slug = incoming.slug)
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

    async fn update_many(&self, items: &[Project]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let mut update_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            UPDATE projects AS p
            SET
              name = incoming.name,
              slug = incoming.slug,
              description = incoming.description,
              url = incoming.url,
              avatar_url = incoming.avatar_url,
              status = incoming.status,
              twitter = incoming.twitter,
              updated_at = NOW()
            FROM (
            "#,
        );
        update_builder.push_values(items, |mut b, p| {
            b.push_bind(p.id.as_str())
                .push_bind(&p.name)
                .push_bind(&p.slug)
                .push_bind(&p.description)
                .push_bind(&p.url)
                .push_bind(&p.avatar_url)
                .push_bind(project_status_value(p.status))
                .push_bind(&p.twitter);
        });
        update_builder.push(
            r#"
            ) AS incoming(repo_id, name, slug, description, url, avatar_url, status, twitter)
            WHERE p.repo_id = incoming.repo_id
              AND NOT EXISTS (
                SELECT 1
                FROM projects p2
                WHERE p2.repo_id <> incoming.repo_id
                  AND (p2.name = incoming.name OR p2.slug = incoming.slug)
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

    async fn list(&self, page: Pagination) -> AppResult<Page<Project>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let rows: Vec<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, twitter
            FROM projects
            ORDER BY name ASC
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

    async fn search_by_key(&self, key: String, page: Pagination) -> AppResult<Page<Project>> {
        let limit = page.limit();
        let offset = page.offset();
        let key = key.trim();
        if key.is_empty() {
            return Ok(page.to_page(Vec::new(), 0));
        }
        let pattern = format!("%{key}%");

        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM projects
            WHERE repo_id ILIKE $1 OR name ILIKE $1 OR slug ILIKE $1 OR description ILIKE $1
            "#,
        )
        .bind(&pattern)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, twitter
            FROM projects
            WHERE repo_id ILIKE $1 OR name ILIKE $1 OR slug ILIKE $1 OR description ILIKE $1
            ORDER BY name ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn list_disabled(&self, page: Pagination) -> AppResult<Page<Project>> {
        let limit = page.limit();
        let offset = page.offset();
        let disabled_status = project_status_value(ProjectStatus::Disabled);
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE status = $1")
            .bind(disabled_status)
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let rows: Vec<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, twitter
            FROM projects
            WHERE status = $1
            ORDER BY name ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(disabled_status)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn search_disabled_by_key(
        &self,
        key: String,
        page: Pagination,
    ) -> AppResult<Page<Project>> {
        let limit = page.limit();
        let offset = page.offset();
        let key = key.trim();
        if key.is_empty() {
            return Ok(page.to_page(Vec::new(), 0));
        }
        let pattern = format!("%{key}%");
        let disabled_status = project_status_value(ProjectStatus::Disabled);

        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM projects
            WHERE status = $1
              AND (repo_id ILIKE $2 OR name ILIKE $2 OR slug ILIKE $2 OR description ILIKE $2)
            "#,
        )
        .bind(disabled_status)
        .bind(&pattern)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, twitter
            FROM projects
            WHERE status = $1
              AND (repo_id ILIKE $2 OR name ILIKE $2 OR slug ILIKE $2 OR description ILIKE $2)
            ORDER BY name ASC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(disabled_status)
        .bind(&pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn remove(&self, repo_id: String) -> AppResult<()> {
        sqlx::query("DELETE FROM projects WHERE repo_id = $1")
            .bind(repo_id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}
