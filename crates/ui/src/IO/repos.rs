use dioxus::prelude::*;

use crate::impls::error::api_error;
use crate::impls::state::State;
use crate::types::repos::RepoDto;
use crate::types::search::SearchResultDto;
use crate::types::snapshot_deltas::SnapshotDeltaDto;
use crate::types::snapshots::SnapshotDto;
use crate::types::tags::TagDto;

use app::prelude::{Page, Pagination};
use app::repo::{ReplaceRepoTagsCommand, TagInput};

#[post("/api/repos", state: State)]
pub async fn list_repos(page: Pagination) -> ServerFnResult<Page<RepoDto>> {
    let app_state = state.0;

    let repos_page = app_state
        .repo
        .query
        .list_with_tags(page)
        .await
        .map_err(api_error)?;

    Ok(repos_page.map(RepoDto::from))
}

#[post("/api/repos/search", state: State)]
pub async fn search_repos(key: String, page: Pagination) -> ServerFnResult<SearchResultDto> {
    let app_state = state.0;
    let result = app_state
        .repo
        .query
        .search_by_key(&key, page)
        .await
        .map_err(api_error)?;
    Ok(SearchResultDto::from(result))
}

#[post("/api/repos/:owner/:name", state: State)]
pub async fn get_repo(owner: String, name: String) -> ServerFnResult<Option<RepoDto>> {
    let app_state = state.0;
    let repo = app_state
        .repo
        .query
        .get_by_owner_name_with_tags(&owner, &name)
        .await
        .map_err(api_error)?;

    Ok(repo.map(RepoDto::from))
}

#[post("/api/repos/:owner/:name/tags/replace", state: State)]
pub async fn replace_repo_tags(
    owner: String,
    name: String,
    tags: Vec<TagDto>,
) -> ServerFnResult<()> {
    let app_state = state.0;
    let full_name = format!("{owner}/{name}");
    app_state
        .repo
        .command
        .replace_tags_by_repo_id(ReplaceRepoTagsCommand {
            repo_id: full_name,
            tags: tags
                .into_iter()
                .map(|tag| TagInput {
                    label: tag.label,
                    value: tag.value,
                })
                .collect(),
        })
        .await
        .map_err(api_error)?;
    Ok(())
}

#[post("/api/repos/by_label", state: State)]
pub async fn list_repos_by_label(
    label: String,
    value: Option<String>,
    page: Pagination,
) -> ServerFnResult<Page<RepoDto>> {
    let app_state = state.0;
    let repos_page = app_state
        .repo
        .query
        .list_by_label_with_tags(&label, value.as_deref(), page)
        .await
        .map_err(api_error)?;
    Ok(repos_page.map(RepoDto::from))
}

#[post("/api/repos/:owner/:name/snapshots", state: State)]
pub async fn list_repo_snapshots(
    owner: String,
    name: String,
    page: Pagination,
) -> ServerFnResult<Page<SnapshotDto>> {
    let app_state = state.0;

    let items_page = app_state
        .snapshot
        .query
        .list_by_owner_name(&owner, &name, page)
        .await
        .map_err(api_error)?;
    Ok(items_page.map(SnapshotDto::from))
}

#[post("/api/repos/:owner/:name/deltas", state: State)]
pub async fn list_repo_deltas(
    owner: String,
    name: String,
    page: Pagination,
) -> ServerFnResult<Page<SnapshotDeltaDto>> {
    let app_state = state.0;

    let items_page = app_state
        .snapshot
        .query
        .list_deltas_by_owner_name(&owner, &name, page)
        .await
        .map_err(api_error)?;
    Ok(items_page.map(SnapshotDeltaDto::from))
}
