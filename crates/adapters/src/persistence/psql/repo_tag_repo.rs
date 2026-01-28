use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::repo::RepoTagRepo;
use domain::{RepoId, Tag, TagLabel, TagValue};
use sqlx::{Postgres, QueryBuilder};

use super::db_err;

const DEFAULT_TAG_LABEL: &str = "UNTAG";
const DEFAULT_TAG_VALUE: &str = "untagged";

fn tag_id(label: &str, value: &str) -> String {
    format!("tag:{label}:{value}")
}

#[derive(Debug, sqlx::FromRow)]
struct RepoTagRow {
    repo_id: String,
    label: String,
    value: String,
}

impl RepoTagRow {
    fn into_pair(self) -> (RepoId, Tag) {
        let repo_id = RepoId::new_unchecked(self.repo_id);
        let tag = Tag {
            label: TagLabel::new(self.label),
            value: TagValue::new(self.value),
        };
        (repo_id, tag)
    }
}

#[derive(Clone)]
pub struct PostgresRepoTagRepo {
    pool: sqlx::PgPool,
}

impl PostgresRepoTagRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RepoTagRepo for PostgresRepoTagRepo {
    async fn replace_repo_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        sqlx::query("DELETE FROM repo_tag_map WHERE repo_id = $1")
            .bind(repo_id.as_str())
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        if tags.is_empty() {
            tx.commit().await.map_err(db_err)?;
            return Ok(());
        }

        let mut tag_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO tags (id, label, value) ");
        tag_builder.push_values(tags, |mut b, tag| {
            let id = tag_id(tag.label.as_str(), tag.value.as_str());
            b.push_bind(id)
                .push_bind(tag.label.as_str())
                .push_bind(tag.value.as_str());
        });
        tag_builder.push(
            " ON CONFLICT(id) DO UPDATE SET label = excluded.label, value = excluded.value",
        );
        tag_builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        let mut map_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO repo_tag_map (repo_id, tag_id, source) ");
        map_builder.push_values(tags, |mut b, tag| {
            let id = tag_id(tag.label.as_str(), tag.value.as_str());
            b.push_bind(repo_id.as_str())
                .push_bind(id)
                .push_bind("manual");
        });
        map_builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn list_by_repo_ids(&self, repo_ids: &[RepoId]) -> AppResult<Vec<(RepoId, Tag)>> {
        if repo_ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT m.repo_id, t.label, t.value FROM repo_tag_map m \
             JOIN tags t ON t.id = m.tag_id WHERE m.repo_id IN (",
        );
        let mut first = true;
        for repo_id in repo_ids {
            if !first {
                builder.push(", ");
            }
            first = false;
            builder.push_bind(repo_id.as_str());
        }
        builder.push(")");
        let rows: Vec<RepoTagRow> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(RepoTagRow::into_pair).collect())
    }

    async fn list_repo_ids_by_label(
        &self,
        label: &str,
        value: Option<&str>,
        page: Pagination,
    ) -> AppResult<Page<RepoId>> {
        let mut count_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT COUNT(DISTINCT m.repo_id) FROM repo_tag_map m \
             JOIN tags t ON t.id = m.tag_id WHERE t.label = ",
        );
        count_builder.push_bind(label);
        if let Some(value) = value {
            count_builder.push(" AND t.value = ");
            count_builder.push_bind(value);
        }
        let total: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let limit = page.limit();
        let offset = page.offset();
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT DISTINCT m.repo_id FROM repo_tag_map m \
             JOIN tags t ON t.id = m.tag_id WHERE t.label = ",
        );
        builder.push_bind(label);
        if let Some(value) = value {
            builder.push(" AND t.value = ");
            builder.push_bind(value);
        }
        builder.push(" ORDER BY m.repo_id LIMIT ");
        builder.push_bind(limit as i64);
        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);
        let rows: Vec<(String,)> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        let items = rows
            .into_iter()
            .map(|(repo_id,)| RepoId::new_unchecked(repo_id))
            .collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn ensure_default_tag_for_repos(&self, repo_ids: &[RepoId]) -> AppResult<()> {
        if repo_ids.is_empty() {
            return Ok(());
        }
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let default_id = tag_id(DEFAULT_TAG_LABEL, DEFAULT_TAG_VALUE);
        sqlx::query("INSERT INTO tags (id, label, value) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING")
            .bind(&default_id)
            .bind(DEFAULT_TAG_LABEL)
            .bind(DEFAULT_TAG_VALUE)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO repo_tag_map (repo_id, tag_id, source) \
             SELECT r.id, ",
        );
        builder.push_bind(&default_id);
        builder.push(", 'system' FROM repos r WHERE r.id IN (");
        let mut first = true;
        for repo_id in repo_ids {
            if !first {
                builder.push(", ");
            }
            first = false;
            builder.push_bind(repo_id.as_str());
        }
        builder.push(") AND NOT EXISTS (SELECT 1 FROM repo_tag_map m WHERE m.repo_id = r.id)");
        builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }
}
