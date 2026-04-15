use crate::components::common::{SEOHead, SEOProp};
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::collections::BTreeSet;

use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, IOCell,
};
use crate::types::repos::RepoDto;
use app::repo::{RepoRankMetric, RepoRankTimeRange};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct RepoListMemory {
    list_key: Option<String>,
    anchor_id: Option<String>,
}
pub(super) fn repo_anchor_id_for_list(repo_id: &str) -> String {
    let normalized = repo_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();
    format!("repo-{normalized}")
}

static REPO_LIST_MEMORY: GlobalSignal<RepoListMemory> = Signal::global(RepoListMemory::default);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RepoListHeroType {
    AllProjects,
    SearchResult,
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct RepoListCachedPage {
    items: Vec<RepoDto>,
    total_pages: u32,
    current_page: u32,
    hero_type: RepoListHeroType,
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct RepoListTagsCachedState {
    advice_tags: Vec<TagAdviceItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FilterType {
    Total,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SortType {
    Star,
    Fork,
    Issue,
    AddTime,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct ListSummary {
    from: u64,
    to: u64,
    total: u64,
}

impl ListSummary {
    pub(super) fn empty() -> Self {
        Self {
            from: 0,
            to: 0,
            total: 0,
        }
    }
}

pub(super) fn normalize_page_size(size: u32) -> u32 {
    match size {
        20 | 50 | 100 => size,
        _ => 50,
    }
}

pub(super) fn repo_list_route_from_ctx(
    ctx: RepoListContext,
    page: u32,
    size: u32,
) -> crate::root::Route {
    let (metric_q, range_q) = query_params_from_filter_sort((ctx.filter_type)(), (ctx.sort_type)());
    crate::root::Route::RepoListView {
        tags: active_tags_to_query(&(ctx.active_tags)()),
        metric: metric_q,
        range: range_q,
        page: Some(page.max(1)),
        size: Some(normalize_page_size(size)),
    }
}

pub(super) fn repo_list_memory_key(ctx: RepoListContext) -> String {
    format!(
        "{}|{:?}|{:?}|{}|{}",
        active_tags_to_query(&(ctx.active_tags)()).unwrap_or_default(),
        (ctx.filter_type)(),
        (ctx.sort_type)(),
        (ctx.current_page)(),
        (ctx.page_size)()
    )
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct TagAdviceItem {
    key: String,
    count: u64,
}

#[derive(Clone, Copy)]
pub(super) struct RepoListContext {
    active_tags: ReadSignal<Vec<String>>,
    filter_type: ReadSignal<FilterType>,
    sort_type: ReadSignal<SortType>,
    page_size: ReadSignal<u32>,
    current_page: ReadSignal<u32>,
    summary: Signal<ListSummary>,
    last_success: Signal<Option<RepoListCachedPage>>,
    last_success_tags: Signal<Option<RepoListTagsCachedState>>,
}

pub(super) fn parse_tags_query(tags: Option<&str>) -> Vec<String> {
    let mut dedup = BTreeSet::new();
    let mut result = Vec::new();
    if let Some(raw) = tags {
        for tag in raw.split(',') {
            let trimmed = tag.trim();
            if trimmed.is_empty() {
                continue;
            }
            if dedup.insert(trimmed.to_string()) {
                result.push(trimmed.to_string());
            }
        }
    }
    result
}

pub(super) fn active_tags_to_query(active_tags: &[String]) -> Option<String> {
    if active_tags.is_empty() {
        None
    } else {
        Some(active_tags.join(","))
    }
}

pub(super) fn parse_filter_type(range: Option<&str>, metric: Option<&str>) -> FilterType {
    let metric_value = metric.unwrap_or_default().trim().to_lowercase();
    if metric_value == "recent" || metric_value == "add_time" || metric_value == "latest" {
        return FilterType::Total;
    }
    match range.unwrap_or_default().trim().to_lowercase().as_str() {
        "all" | "total" => FilterType::Total,
        "daily" | "day" => FilterType::Daily,
        "monthly" | "month" => FilterType::Monthly,
        "weekly" | "week" => FilterType::Weekly,
        _ => FilterType::Total,
    }
}

pub(super) fn parse_sort_type(metric: Option<&str>) -> SortType {
    match metric.unwrap_or_default().trim().to_lowercase().as_str() {
        "fork" | "forks" => SortType::Fork,
        "issue" | "issues" => SortType::Issue,
        "recent" | "add_time" | "latest" => SortType::AddTime,
        _ => SortType::Star,
    }
}

pub(super) fn sort_metric(sort: SortType) -> RepoRankMetric {
    match sort {
        SortType::Star => RepoRankMetric::Star,
        SortType::Fork => RepoRankMetric::Fork,
        SortType::Issue => RepoRankMetric::Issue,
        SortType::AddTime => RepoRankMetric::Recent,
    }
}

pub(super) fn filter_range(filter: FilterType) -> RepoRankTimeRange {
    match filter {
        FilterType::Total => RepoRankTimeRange::All,
        FilterType::Daily => RepoRankTimeRange::Daily,
        FilterType::Weekly => RepoRankTimeRange::Weekly,
        FilterType::Monthly => RepoRankTimeRange::Monthly,
    }
}

pub(super) fn sort_metric_query(sort: SortType) -> &'static str {
    match sort {
        SortType::Star => "star",
        SortType::Fork => "fork",
        SortType::Issue => "issue",
        SortType::AddTime => "recent",
    }
}

pub(super) fn filter_range_query(filter: FilterType) -> &'static str {
    match filter {
        FilterType::Total => "all",
        FilterType::Daily => "daily",
        FilterType::Weekly => "weekly",
        FilterType::Monthly => "monthly",
    }
}

pub(super) fn query_params_from_filter_sort(
    filter: FilterType,
    sort: SortType,
) -> (Option<String>, Option<String>) {
    if filter == FilterType::Total && sort == SortType::Star {
        (None, None)
    } else {
        (
            Some(sort_metric_query(sort).to_string()),
            Some(filter_range_query(filter).to_string()),
        )
    }
}

pub(super) fn append_tag_query(active_tags: &[String], append: &str) -> String {
    let mut next = active_tags.to_vec();
    if !next.iter().any(|tag| tag == append) {
        next.push(append.to_string());
    }
    next.join(",")
}

pub(super) fn remove_tag_query(active_tags: &[String], remove: &str) -> Option<String> {
    let next = active_tags
        .iter()
        .filter(|tag| tag.as_str() != remove)
        .cloned()
        .collect::<Vec<_>>();
    if next.is_empty() {
        None
    } else {
        Some(next.join(","))
    }
}

mod repo_list;
mod repo_list_handler;
mod repo_list_tags;
mod repo_meta;
use repo_list::{skeleton::RepoListCachedFallback, RepoListIO};
use repo_list_handler::RepoListHandler;
use repo_list_tags::{skeleton::RepoListTagsCachedFallback, RepoListTags};
use repo_meta::RepoMeta;

#[component]
pub fn RepoList(
    tags: ReadSignal<Option<String>>,
    metric: ReadSignal<Option<String>>,
    range: ReadSignal<Option<String>>,
    page: ReadSignal<Option<u32>>,
    size: ReadSignal<Option<u32>>,
) -> Element {
    let active_tags = use_memo(move || parse_tags_query(tags().as_deref()));
    let filter_type = use_memo(move || parse_filter_type(range().as_deref(), metric().as_deref()));
    let sort_type = use_memo(move || parse_sort_type(metric().as_deref()));
    let page_size = use_memo(move || normalize_page_size(size().unwrap_or(50)));
    let current_page = use_memo(move || page().unwrap_or(1).max(1));
    let active_tags: ReadSignal<Vec<String>> = active_tags.into();
    let filter_type: ReadSignal<FilterType> = filter_type.into();
    let sort_type: ReadSignal<SortType> = sort_type.into();
    let page_size: ReadSignal<u32> = page_size.into();
    let current_page: ReadSignal<u32> = current_page.into();
    let summary = use_signal(ListSummary::empty);
    let last_success = use_signal(|| None::<RepoListCachedPage>);
    let last_success_tags = use_signal(|| None::<RepoListTagsCachedState>);

    use_context_provider(|| RepoListContext {
        active_tags,
        filter_type,
        sort_type,
        page_size,
        current_page,
        summary,
        last_success,
        last_success_tags,
    });

    rsx! {
        SEOHead {
            data: SEOProp {
                title: t!("view_repo_list_seo_title").to_string(),
                description: t!("view_repo_list_seo_description").to_string(),
                keywords: t!("view_repo_list_seo_keywords").to_string(),
                canonical_url: "/repo".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        GridWrapper {
            grid_type: GridType::Default,
            padding: GridPadding::Sm,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Grid,
                gradient: GradientDirection::ToBottom,
            },
            section { class: "relative bg-transparent",
                div { class: "relative z-10 flex flex-col gap-2",
                    RepoMeta {}
                    div { class: "flex flex-col gap-6 pt-6",
                        IOCell {
                            loading_fallback: rsx! { RepoListTagsCachedFallback {} },
                            RepoListTags {}
                        }
                        RepoListHandler {}
                    }
                }
            }
        }
        GridSlashTransition {}
        GridWrapper { class: "min-h-screen h-full", padding: GridPadding::Sm,
            IOCell {
                loading_fallback: rsx! { RepoListCachedFallback {} },
                RepoListIO {}
            }
        }
    }
}
