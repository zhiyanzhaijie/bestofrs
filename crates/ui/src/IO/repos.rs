use crate::impls::session::auth::AdminAuth;
use crate::impls::error::api_error;
use crate::impls::state::State;
use crate::types::repos::{BulkUpdateRepoTagResultDto, RepoDto, RepoReadmeDto};
use crate::types::search::SearchResultDto;
use crate::types::snapshot_deltas::SnapshotDeltaDto;
use crate::types::snapshot_deltas_summary::SnapshotDeltasSummaryDto;
use crate::types::snapshots::SnapshotDto;
use crate::types::tags::{ImportTagsResult, TagDto, TagFacetDto, TagImportItem, TagListItemDto};
use dioxus::prelude::*;

use app::prelude::{DurationRange, Page, Pagination};
use app::repo::{
    BulkTagUpdateAction, BulkUpdateRepoTagCommand, ImportTagCommand, ImportTagsCommand,
    ReplaceRepoTagsCommand, RepoListQuery, RepoRankMetric, RepoRankQuery, TagInput,
};
use serde::Deserialize;

#[post("/api/repos", state: State)]
pub async fn list_repos(page: Pagination) -> ServerFnResult<Page<RepoDto>> {
    let app_state = state.0;

    let repos_page = app_state
        .repo
        .query
        .list_with_tags(page, None, Some(RepoRankMetric::Star))
        .await
        .map_err(api_error)?;

    Ok(repos_page.map(RepoDto::from))
}
#[post("/api/repos/query", state: State)]
pub async fn list_repos_with_query(query: RepoListQuery) -> ServerFnResult<Page<RepoDto>> {
    let app_state = state.0;
    let tags = query.tags.as_ref().filter(|tags| !tags.is_empty()).cloned();

    let repos_page = if let Some(tags) = tags {
        app_state
            .repo
            .query
            .list_with_tags(query.page, Some(tags), query.metric)
            .await
            .map_err(api_error)?
    } else if query.metric.is_none() && query.range.is_none() {
        return list_repos(query.page).await;
    } else if let (Some(metric), Some(range)) = (query.metric, query.range) {
        app_state
            .repo
            .query
            .list_ranked_with_tags(RepoRankQuery { metric, range }, query.page)
            .await
            .map_err(api_error)?
    } else {
        return list_repos(query.page).await;
    };
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

#[post("/api/repos/:owner/:name/readme", state: State)]
pub async fn get_repo_readme(owner: String, name: String) -> ServerFnResult<Option<RepoReadmeDto>> {
    let app_state = state.0;

    let readme = app_state
        .repo
        .query
        .get_readme_by_owner_name(&owner, &name)
        .await
        .map_err(api_error)?;

    Ok(readme.map(RepoReadmeDto::from))
}

#[post("/api/repos/:owner/:name/tags/replace", state: State, _auth: AdminAuth)]
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

#[post("/api/tags/import", state: State, _auth: AdminAuth)]
pub async fn import_tags(items: Vec<TagImportItem>) -> ServerFnResult<ImportTagsResult> {
    let app_state = state.0;
    let cmd = ImportTagsCommand {
        items: items
            .into_iter()
            .map(|it| ImportTagCommand {
                label: it.label,
                value: it.value,
                description: it.description,
            })
            .collect(),
    };
    let report = app_state
        .repo
        .command
        .import_tags(cmd)
        .await
        .map_err(api_error)?;

    Ok(ImportTagsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[derive(Debug, Clone, Deserialize)]
struct TagSeedItem {
    label: String,
    value: String,
    #[serde(default)]
    description: Option<String>,
}

#[post("/api/tags/import_json", state: State, _auth: AdminAuth)]
pub async fn import_tags_json(json_text: String) -> ServerFnResult<ImportTagsResult> {
    let app_state = state.0;
    let items: Vec<TagSeedItem> =
        serde_json::from_str(&json_text).map_err(|e| ServerFnError::ServerError {
            code: 400,
            message: format!("invalid json: {e}"),
            details: None,
        })?;

    let cmd = ImportTagsCommand {
        items: items
            .into_iter()
            .map(|it| ImportTagCommand {
                label: it.label,
                value: it.value,
                description: it.description,
            })
            .collect(),
    };
    let report = app_state
        .repo
        .command
        .import_tags(cmd)
        .await
        .map_err(api_error)?;

    Ok(ImportTagsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[post("/api/repos/tags/bulk_update", state: State, _auth: AdminAuth)]
pub async fn bulk_update_repo_tag(
    repo_ids: Vec<String>,
    label: String,
    value: String,
    action: String,
) -> ServerFnResult<BulkUpdateRepoTagResultDto> {
    let app_state = state.0;

    let action = match action.trim().to_lowercase().as_str() {
        "add" => BulkTagUpdateAction::Add,
        "remove" => BulkTagUpdateAction::Remove,
        _ => {
            return Err(ServerFnError::ServerError {
                code: 400,
                message: "invalid action, expected `add` or `remove`".to_string(),
                details: None,
            });
        }
    };

    let result = app_state
        .repo
        .command
        .bulk_update_tag_for_repos(BulkUpdateRepoTagCommand {
            repo_ids,
            tag: TagInput { label, value },
            action,
        })
        .await
        .map_err(api_error)?;

    Ok(BulkUpdateRepoTagResultDto::from(result))
}

#[post("/api/tags", state: State)]
pub async fn list_tags(page: Pagination) -> ServerFnResult<Page<TagDto>> {
    let app_state = state.0;
    let tags_page = app_state
        .repo
        .query
        .list_tags(page)
        .await
        .map_err(api_error)?;
    Ok(tags_page.map(TagDto::from))
}

#[post("/api/tags/list", state: State)]
pub async fn list_tags_with_meta(
    page: Option<u32>,
    page_size: Option<u32>,
    limit: Option<u32>,
    top_n: Option<u32>,
) -> ServerFnResult<Page<TagListItemDto>> {
    let app_state = state.0;
    let page = page.unwrap_or(1).max(1);
    let limit = limit.or(page_size).unwrap_or(Pagination::DEFAULT_LIMIT);
    let pagination = Pagination {
        limit: Some(limit),
        offset: Some(limit.saturating_mul(page.saturating_sub(1))),
    };
    let top_n = top_n.unwrap_or(5).max(1);
    let tags_page = app_state
        .repo
        .query
        .list_tags_with_meta(pagination, top_n)
        .await
        .map_err(api_error)?;
    Ok(tags_page.map(TagListItemDto::from))
}

#[post("/api/tags/list_by_values", state: State)]
pub async fn list_tags_with_meta_by_values(
    values: Vec<String>,
    top_n: Option<u32>,
) -> ServerFnResult<Vec<TagListItemDto>> {
    let app_state = state.0;
    let top_n = top_n.unwrap_or(5).max(1);
    let items = app_state
        .repo
        .query
        .list_tags_with_meta_by_values(values, top_n)
        .await
        .map_err(api_error)?;
    Ok(items.into_iter().map(TagListItemDto::from).collect())
}

#[post("/api/tags/get_by_value", state: State)]
pub async fn get_tag_with_meta_by_value(
    value: String,
    top_n: Option<u32>,
) -> ServerFnResult<Option<TagListItemDto>> {
    let app_state = state.0;
    let top_n = top_n.unwrap_or(5).max(1);
    let item = app_state
        .repo
        .query
        .get_tag_with_meta_by_value(value, top_n)
        .await
        .map_err(api_error)?;
    Ok(item.map(TagListItemDto::from))
}

#[post("/api/tags/search", state: State)]
pub async fn search_tags(key: String, page: Pagination) -> ServerFnResult<Page<TagDto>> {
    let app_state = state.0;
    let tags_page = app_state
        .repo
        .query
        .search_tags_by_key(&key, page)
        .await
        .map_err(api_error)?;
    Ok(tags_page.map(TagDto::from))
}

#[post("/api/tags/create", state: State, _auth: AdminAuth)]
pub async fn create_tag(label: String, value: String) -> ServerFnResult<()> {
    let app_state = state.0;
    app_state
        .repo
        .command
        .create_tag(TagInput { label, value })
        .await
        .map_err(api_error)?;
    Ok(())
}

#[post("/api/tags/update", state: State, _auth: AdminAuth)]
pub async fn update_tag(
    label: String,
    value: String,
    description: Option<String>,
) -> ServerFnResult<()> {
    let app_state = state.0;
    app_state
        .repo
        .command
        .update_tag(label, value, description)
        .await
        .map_err(api_error)?;
    Ok(())
}

#[post("/api/tags/delete", state: State, _auth: AdminAuth)]
pub async fn delete_tag(label: String, value: String) -> ServerFnResult<()> {
    let app_state = state.0;
    app_state
        .repo
        .command
        .delete_tag(TagInput { label, value })
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

#[post("/api/repos/tags/facets", state: State)]
pub async fn list_repo_tag_facets(
    active_tag_values: Vec<String>,
    limit: Option<u32>,
) -> ServerFnResult<Vec<TagFacetDto>> {
    let app_state = state.0;
    let items = app_state
        .repo
        .query
        .list_tag_facets_by_active_values(active_tag_values, limit)
        .await
        .map_err(api_error)?;
    Ok(items.into_iter().map(TagFacetDto::from).collect())
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

#[post("/api/repos/:owner/:name/snapshots/in_duration", state: State)]
pub async fn list_repo_snapshots_in_duration(
    owner: String,
    name: String,
    duration: DurationRange,
) -> ServerFnResult<Page<SnapshotDto>> {
    let app_state = state.0;
    let items_page = app_state
        .snapshot
        .query
        .list_by_owner_name_in_duration(&owner, &name, duration)
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

#[post("/api/repos/:owner/:name/deltas/in_duration", state: State)]
pub async fn list_repo_deltas_in_duration(
    owner: String,
    name: String,
    duration: DurationRange,
) -> ServerFnResult<Page<SnapshotDeltaDto>> {
    let app_state = state.0;

    let items_page = app_state
        .snapshot
        .query
        .list_deltas_by_owner_name_in_duration(&owner, &name, duration)
        .await
        .map_err(api_error)?;
    Ok(items_page.map(SnapshotDeltaDto::from))
}

#[post("/api/repos/:owner/:name/deltas_summary", state: State)]
pub async fn list_repo_deltas_summary(
    owner: String,
    name: String,
) -> ServerFnResult<SnapshotDeltasSummaryDto> {
    let app_state = state.0;
    let summary = app_state
        .snapshot
        .query
        .list_deltas_summary_by_owner_name(&owner, &name)
        .await
        .map_err(api_error)?;
    Ok(SnapshotDeltasSummaryDto::from(summary))
}
