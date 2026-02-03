use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::project::ProjectRepo;
use domain::{Project, RepoId};
use sqlx::{Postgres, QueryBuilder};

use super::db_err;

#[derive(Debug, sqlx::FromRow)]
struct ProjectDb {
    repo_id: String,
    name: String,
    slug: String,
    description: String,
    override_description: bool,
    url: Option<String>,
    override_url: bool,
    status: Option<String>,
    logo: Option<String>,
    twitter: Option<String>,
    comments: Option<String>,
}

impl From<ProjectDb> for Project {
    fn from(db: ProjectDb) -> Self {
        Self {
            id: RepoId::new_unchecked(db.repo_id),
            name: db.name,
            slug: db.slug,
            description: db.description,
            override_description: db.override_description,
            url: db.url,
            override_url: db.override_url,
            status: db.status,
            logo: db.logo,
            twitter: db.twitter,
            comments: db.comments,
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
    async fn upsert(&self, project: &Project) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(project)).await
    }

    async fn upsert_many(&self, items: &[Project]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO projects (
              repo_id,
              name, slug, description,
              override_description,
              url, override_url,
              status, logo, twitter, comments,
              updated_at
            )
            "#,
        );

        builder.push_values(items, |mut b, p| {
            b.push_bind(p.id.as_str())
                .push_bind(&p.name)
                .push_bind(&p.slug)
                .push_bind(&p.description)
                .push_bind(p.override_description)
                .push_bind(&p.url)
                .push_bind(p.override_url)
                .push_bind(&p.status)
                .push_bind(&p.logo)
                .push_bind(&p.twitter)
                .push_bind(&p.comments)
                .push("NOW()");
        });

        builder.push(
            r#"
            ON CONFLICT(repo_id) DO UPDATE SET
              name = excluded.name,
              slug = excluded.slug,
              description = excluded.description,
              override_description = excluded.override_description,
              url = excluded.url,
              override_url = excluded.override_url,
              status = excluded.status,
              logo = excluded.logo,
              twitter = excluded.twitter,
              comments = excluded.comments,
              updated_at = excluded.updated_at
            "#,
        );

        builder.build().execute(&mut *tx).await.map_err(db_err)?;

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
              override_description,
              url, override_url,
              status, logo, twitter, comments
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

    async fn remove(&self, repo_id: String) -> AppResult<()> {
        sqlx::query("DELETE FROM projects WHERE repo_id = $1")
            .bind(repo_id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}
