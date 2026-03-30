use crate::impls::session::auth::AdminAuth;
use crate::types::projects::{ImportProjectsResult, ProjectDto, ProjectImportItem};
use dioxus::prelude::*;

use crate::impls::error::api_error;
use crate::impls::state::State;

use app::prelude::{Page, Pagination};
use app::project::{ImportProjectCommand, ImportProjectsCommand, RemoveProjectCommand};
use domain::ProjectStatus;
use serde::Deserialize;

#[post("/api/projects", state: State)]
pub async fn list_projects(page: Pagination) -> ServerFnResult<Page<ProjectDto>> {
    let app_state = state.0;
    let projects_page = app_state
        .project
        .query
        .list(page)
        .await
        .map_err(api_error)?;

    Ok(projects_page.map(ProjectDto::from))
}

#[post("/api/projects/search", state: State)]
pub async fn search_projects(key: String, page: Pagination) -> ServerFnResult<Page<ProjectDto>> {
    let app_state = state.0;
    let projects_page = app_state
        .project
        .query
        .search_by_key(key, page)
        .await
        .map_err(api_error)?;

    Ok(projects_page.map(ProjectDto::from))
}

#[post("/api/projects/disabled", state: State)]
pub async fn list_disabled_projects(page: Pagination) -> ServerFnResult<Page<ProjectDto>> {
    let app_state = state.0;
    let projects_page = app_state
        .project
        .query
        .list_disabled(page)
        .await
        .map_err(api_error)?;

    Ok(projects_page.map(ProjectDto::from))
}

#[post("/api/projects/search_disabled", state: State)]
pub async fn search_disabled_projects(
    key: String,
    page: Pagination,
) -> ServerFnResult<Page<ProjectDto>> {
    let app_state = state.0;
    let projects_page = app_state
        .project
        .query
        .search_disabled_by_key(key, page)
        .await
        .map_err(api_error)?;

    Ok(projects_page.map(ProjectDto::from))
}

#[post("/api/projects/import", state: State, _auth: AdminAuth)]
pub async fn import_projects(
    items: Vec<ProjectImportItem>,
) -> ServerFnResult<ImportProjectsResult> {
    let app_state = state.0;

    let cmd = ImportProjectsCommand {
        items: items
            .into_iter()
            .map(|it| ImportProjectCommand {
                repo_id: it.repo_id,
                name: it.name,
                slug: it.slug,
                description: it.description,
                url: it.url,
                avatar_url: it.avatar_url,
                status: it.status,
                twitter: it.twitter,
                tags: it.tags,
            })
            .collect(),
    };

    let report = app_state
        .project
        .command
        .import_projects(cmd)
        .await
        .map_err(api_error)?;

    Ok(ImportProjectsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[post("/api/projects/update", state: State, _auth: AdminAuth)]
pub async fn update_projects(
    items: Vec<ProjectImportItem>,
) -> ServerFnResult<ImportProjectsResult> {
    let app_state = state.0;
    let cmd = ImportProjectsCommand {
        items: items
            .into_iter()
            .map(|it| ImportProjectCommand {
                repo_id: it.repo_id,
                name: it.name,
                slug: it.slug,
                description: it.description,
                url: it.url,
                avatar_url: it.avatar_url,
                status: it.status,
                twitter: it.twitter,
                tags: None,
            })
            .collect(),
    };
    let report = app_state
        .project
        .command
        .update_projects(cmd)
        .await
        .map_err(api_error)?;
    Ok(ImportProjectsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectSeedItem {
    name: String,
    full_name: String,
    #[serde(default)]
    tags: Option<Vec<String>>,
}

#[post("/api/projects/import_json", state: State, _auth: AdminAuth)]
pub async fn import_projects_json(json_text: String) -> ServerFnResult<ImportProjectsResult> {
    let app_state = state.0;

    let items: Vec<ProjectSeedItem> =
        serde_json::from_str(&json_text).map_err(|e| ServerFnError::ServerError {
            code: 400,
            message: format!("invalid json: {e}"),
            details: None,
        })?;

    let cmd = ImportProjectsCommand {
        items: items
            .into_iter()
            .map(|it| ImportProjectCommand {
                repo_id: it.full_name,
                name: it.name.clone(),
                slug: it.name,
                description: String::new(),
                url: None,
                avatar_url: None,
                status: ProjectStatus::Unknown,
                twitter: None,
                tags: it.tags,
            })
            .collect(),
    };

    let report = app_state
        .project
        .command
        .import_projects(cmd)
        .await
        .map_err(api_error)?;

    Ok(ImportProjectsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[post("/api/projects/remove", state: State, _auth: AdminAuth)]
pub async fn remove_project(repo_id: String) -> ServerFnResult<()> {
    let app_state = state.0;

    app_state
        .project
        .command
        .remove_project(RemoveProjectCommand { repo_id })
        .await
        .map_err(api_error)?;

    Ok(())
}
