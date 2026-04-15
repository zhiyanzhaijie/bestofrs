mod app_error_impl;
pub mod psql;

use std::sync::Arc;

use app::app_error::{AppError, AppResult};
use app::backup::DatabaseBackupPort;
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

pub struct DbRuntime {
    pub repos: DbRepos,
    pub backup: Arc<dyn DatabaseBackupPort>,
}

#[async_trait::async_trait]
pub trait PersistenceBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_handle(&self, url: &str) -> bool;
    async fn build_runtime(&self, database_url: &str, backup_dir: &str) -> AppResult<DbRuntime>;
}

pub async fn build_repos_by_url(database_url: &str) -> AppResult<DbRepos> {
    let runtime = build_runtime_by_url(database_url, "./backups").await?;
    Ok(runtime.repos)
}

pub async fn build_backup_by_url(
    database_url: &str,
    backup_dir: &str,
) -> AppResult<Arc<dyn DatabaseBackupPort>> {
    let runtime = build_runtime_by_url(database_url, backup_dir).await?;
    Ok(runtime.backup)
}

pub async fn build_runtime_by_url(database_url: &str, backup_dir: &str) -> AppResult<DbRuntime> {
    let postgres = psql::PostgresBackend;
    let backends: [&dyn PersistenceBackend; 1] = [&postgres];
    for backend in backends {
        if backend.can_handle(database_url) {
            return backend.build_runtime(database_url, backup_dir).await;
        }
    }

    Err(AppError::internal(format!(
        "Unsupported database url: {database_url}"
    )))
}
