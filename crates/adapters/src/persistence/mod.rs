mod app_error_impl;
pub mod psql;
pub mod sqlite;

use std::sync::Arc;

use app::app_error::{AppError, AppResult};
use app::project::ProjectRepo;
use app::repo::{RepoRepo, RepoTagRepo};
use app::snapshot::{SnapshotDeltaRepo, SnapshotRepo};

pub struct DbRepos {
    pub project: Arc<dyn ProjectRepo>,
    pub repo: Arc<dyn RepoRepo>,
    pub repo_tag: Arc<dyn RepoTagRepo>,
    pub snapshot: Arc<dyn SnapshotRepo>,
    pub snapshot_delta: Arc<dyn SnapshotDeltaRepo>,
}

#[async_trait::async_trait]
pub trait PersistenceAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_handle(&self, url: &str) -> bool;
    async fn build_repos(&self, database_url: &str) -> AppResult<DbRepos>;
}

pub struct PostgresAdapter;
pub struct SqliteAdapter;

#[async_trait::async_trait]
impl PersistenceAdapter for PostgresAdapter {
    fn name(&self) -> &'static str {
        "postgres"
    }

    fn can_handle(&self, url: &str) -> bool {
        is_postgres_url(url)
    }

    async fn build_repos(&self, database_url: &str) -> AppResult<DbRepos> {
        let pool = psql::connect_and_migrate(database_url).await?;

        let project: Arc<dyn ProjectRepo> = Arc::new(psql::PostgresProjectRepo::new(pool.clone()));
        let repo: Arc<dyn RepoRepo> = Arc::new(psql::PostgresRepoRepo::new(pool.clone()));
        let repo_tag: Arc<dyn RepoTagRepo> = Arc::new(psql::PostgresRepoTagRepo::new(pool.clone()));

        let snapshot_repo = Arc::new(psql::PostgresSnapshotRepo::new(pool));
        let snapshot: Arc<dyn SnapshotRepo> = snapshot_repo.clone();
        let snapshot_delta: Arc<dyn SnapshotDeltaRepo> = snapshot_repo;

        Ok(DbRepos {
            project,
            repo,
            repo_tag,
            snapshot,
            snapshot_delta,
        })
    }
}

#[async_trait::async_trait]
impl PersistenceAdapter for SqliteAdapter {
    fn name(&self) -> &'static str {
        "sqlite"
    }

    fn can_handle(&self, url: &str) -> bool {
        is_sqlite_url(url)
    }

    async fn build_repos(&self, database_url: &str) -> AppResult<DbRepos> {
        let pool = sqlite::connect_and_migrate(database_url).await?;

        let project: Arc<dyn ProjectRepo> = Arc::new(sqlite::SqliteProjectRepo::new(pool.clone()));
        let repo: Arc<dyn RepoRepo> = Arc::new(sqlite::SqliteRepoRepo::new(pool.clone()));
        let repo_tag: Arc<dyn RepoTagRepo> = Arc::new(sqlite::SqliteRepoTagRepo::new(pool.clone()));

        let snapshot_repo = Arc::new(sqlite::SqliteSnapshotRepo::new(pool));
        let snapshot: Arc<dyn SnapshotRepo> = snapshot_repo.clone();
        let snapshot_delta: Arc<dyn SnapshotDeltaRepo> = snapshot_repo;

        Ok(DbRepos {
            project,
            repo,
            repo_tag,
            snapshot,
            snapshot_delta,
        })
    }
}

pub async fn build_repos_by_url(database_url: &str) -> AppResult<DbRepos> {
    let postgres = PostgresAdapter;
    let sqlite = SqliteAdapter;

    let adapters: [&dyn PersistenceAdapter; 2] = [&postgres, &sqlite];
    for adapter in adapters {
        if adapter.can_handle(database_url) {
            return adapter.build_repos(database_url).await;
        }
    }

    Err(AppError::internal(format!(
        "Unsupported database url: {database_url}"
    )))
}

fn is_sqlite_url(url: &str) -> bool {
    url.starts_with("sqlite:")
}

fn is_postgres_url(url: &str) -> bool {
    url.starts_with("postgres://") || url.starts_with("postgresql://")
}
