use std::sync::Arc;

use adapters::persistence;
use app::app_error::{AppError, AppResult};
use app::auth::{OAuth2AuthorizationCodePkcePort, OAuth2ResourceOwnerPort};
use app::prelude::{
    AuthCommandHandler, IngestDailySnapshots, ProjectCommandHandler, ProjectQueryHandler,
    RepoCommandHandler, RepoQueryHandler, SnapshotCommandHandler, SnapshotEventHandler,
    SnapshotQueryHandler,
};

use crate::config::Config as AppConfig;
use adapters::auth::{ConfigRolePolicy, GithubOAuthAdapter};
use adapters::clock::SystemClock;
use adapters::github::GithubClient;

pub struct ProjectState {
    pub query: ProjectQueryHandler,
    pub command: ProjectCommandHandler,
}

pub struct RepoState {
    pub query: RepoQueryHandler,
    pub command: RepoCommandHandler,
}

pub struct SnapshotState {
    pub query: SnapshotQueryHandler,
    pub command: SnapshotCommandHandler,
}

pub struct AuthState {
    pub command: AuthCommandHandler,
}

pub struct AppContainer {
    pub config: AppConfig,

    pub auth: AuthState,

    pub project: ProjectState,
    pub repo: RepoState,
    pub snapshot: SnapshotState,

    pub ingest_daily_snapshots: IngestDailySnapshots,
}

/// The server/runtime state injected into HTTP handlers.
///
/// For now we reuse `AppContainer` directly as the state type.
pub type AppState = AppContainer;

pub async fn init_app_container() -> AppResult<AppContainer> {
    let config = AppConfig::load().map_err(AppError::internal)?;

    let repos = persistence::build_repos_by_url(&config.database.url).await?;

    let project = ProjectState {
        query: ProjectQueryHandler::new(repos.project.clone()),
        command: ProjectCommandHandler::new(repos.project.clone()),
    };

    let github = Arc::new(GithubClient::new(Some(config.server.github_token.clone()))?);

    let repo = RepoState {
        query: RepoQueryHandler::new(repos.repo.clone(), repos.repo_tag.clone(), github.clone()),
        command: RepoCommandHandler::new(repos.repo.clone(), repos.repo_tag.clone()),
    };

    let snapshot_query =
        SnapshotQueryHandler::new(repos.snapshot.clone(), repos.snapshot_delta.clone());
    let snapshot_event_handler = SnapshotEventHandler::new(repos.snapshot_delta);

    let snapshot = SnapshotState {
        query: snapshot_query,
        command: SnapshotCommandHandler::new(repos.snapshot, snapshot_event_handler),
    };

    let provider = Arc::new(GithubOAuthAdapter::new(
        config.auth.github_client_id.clone(),
        config.auth.github_client_secret.clone(),
        config.auth.github_redirect_url.clone(),
    )?);

    let oauth: Arc<dyn OAuth2AuthorizationCodePkcePort> = provider.clone();
    let resource_owner: Arc<dyn OAuth2ResourceOwnerPort> = provider.clone();

    let role_policy = Arc::new(ConfigRolePolicy::new(config.auth.admin_github_ids.clone()));

    let auth = AuthState {
        command: AuthCommandHandler::new(oauth, resource_owner, role_policy),
    };

    let clock = Arc::new(SystemClock);

    let ingest_daily_snapshots = IngestDailySnapshots::new(
        project.query.clone(),
        repo.command.clone(),
        snapshot.command.clone(),
        github,
        clock,
    );

    Ok(AppContainer {
        config,
        auth,
        project,
        repo,
        snapshot,
        ingest_daily_snapshots,
    })
}
