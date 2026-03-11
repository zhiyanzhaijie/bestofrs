use dioxus::prelude::*;
use std::collections::BTreeSet;

use crate::components::common::{
    CommonPagination, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper, IOCell, RepoManuscriptCard,
};
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};

use crate::components::skeleton::Skeleton;
use crate::types::repos::RepoDto;
use crate::IO::repos::{list_repo_tag_facets, list_repos_with_query};
use app::prelude::Pagination as PageQuery;
use app::repo::{RepoListQuery, RepoRankMetric, RepoRankTimeRange};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct RepoListMemory {
    list_key: Option<String>,
    anchor_id: Option<String>,
}

fn repo_anchor_id_for_list(repo_id: &str) -> String {
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
enum RepoListHeroType {
    AllProjects,
    SearchResult,
}

#[derive(Clone, PartialEq, Eq)]
struct RepoListCachedPage {
    items: Vec<RepoDto>,
    total_pages: u32,
    current_page: u32,
    hero_type: RepoListHeroType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilterType {
    Total,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortType {
    Star,
    Fork,
    Issue,
    AddTime,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ListSummary {
    from: u64,
    to: u64,
    total: u64,
}

impl ListSummary {
    fn empty() -> Self {
        Self {
            from: 0,
            to: 0,
            total: 0,
        }
    }
}

fn normalize_page_size(size: u32) -> u32 {
    match size {
        20 | 50 | 100 => size,
        _ => 50,
    }
}

fn repo_list_route_from_ctx(ctx: RepoListContext, page: u32, size: u32) -> crate::root::Route {
    let (metric_q, range_q) = query_params_from_filter_sort((ctx.filter_type)(), (ctx.sort_type)());
    crate::root::Route::RepoListView {
        tags: active_tags_to_query(&(ctx.active_tags)()),
        metric: metric_q,
        range: range_q,
        page: Some(page.max(1)),
        size: Some(normalize_page_size(size)),
    }
}

fn repo_list_memory_key(ctx: RepoListContext) -> String {
    format!(
        "{}|{:?}|{:?}|{}|{}",
        active_tags_to_query(&(ctx.active_tags)()).unwrap_or_default(),
        (ctx.filter_type)(),
        (ctx.sort_type)(),
        (ctx.current_page)(),
        (ctx.page_size)()
    )
}

#[component]
fn RepoListCachedFallback() -> Element {
    let ctx = use_context::<RepoListContext>();
    if let Some(cached) = (ctx.last_success)() {
        rsx! {
            RepoListContent {
                items: cached.items,
                total_pages: cached.total_pages,
                current_page: cached.current_page,
                hero_type: cached.hero_type,
            }
        }
    } else {
        rsx! {
            Skeleton { class: "skeleton w-full h-full min-h-[220px] rounded-xl border border-primary-6" }
        }
    }
}

#[component]
fn RepoListContent(
    items: Vec<RepoDto>,
    total_pages: u32,
    current_page: u32,
    hero_type: RepoListHeroType,
) -> Element {
    let mut ctx = use_context::<RepoListContext>();
    let navigator = use_navigator();
    let list_key = repo_list_memory_key(ctx);
    let memory = REPO_LIST_MEMORY.peek().clone();
    let restore_anchor = if memory.list_key.as_deref() == Some(list_key.as_str()) {
        memory.anchor_id.clone()
    } else {
        None
    };
    let rendered_items = items
        .into_iter()
        .map(|repo| {
            let card_anchor = repo_anchor_id_for_list(&repo.id);
            let should_restore = restore_anchor.as_deref() == Some(card_anchor.as_str());
            (repo, should_restore)
        })
        .collect::<Vec<_>>();
    let restore_anchor_for_effect = restore_anchor.clone();
    let mut restore_target = use_signal(|| None::<MountedEvent>);
    let mut restored = use_signal(|| false);

    use_effect(move || {
        if restored() {
            return;
        }
        if let Some(mounted) = restore_target() {
            restored.set(true);
            if restore_anchor_for_effect.is_some() {
                *REPO_LIST_MEMORY.write() = RepoListMemory::default();
            }
            spawn(async move {
                let _ = mounted.scroll_to(ScrollBehavior::Instant).await;
            });
        }
    });
    rsx! {
        div { class: "space-y-8",
            if rendered_items.is_empty() {
                div { class: "flex min-h-[320px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary text-center",
                    span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-5",
                        "NO_DATA"
                    }
                    if hero_type == RepoListHeroType::AllProjects {
                        span { class: "text-sm text-secondary-5", "No repos found" }
                    } else {
                        span { class: "text-sm text-secondary-5",
                            "No repos for selected tag set"
                        }
                    }
                }
            } else {
                div { class: "space-y-4",
                    for (r, should_restore) in rendered_items {
                        if should_restore && !restored() {
                            div {
                                key: "{r.id}",
                                style: "scroll-margin-top: clamp(5rem, 34vh, 20rem);",
                                onmounted: move |evt| {
                                    restore_target.set(Some(evt.clone()));
                                },
                                RepoManuscriptCard {
                                    repo: r,
                                    on_open: {
                                        let list_key = list_key.clone();
                                        move |anchor_id: String| {
                                            *REPO_LIST_MEMORY.write() = RepoListMemory {
                                                list_key: Some(list_key.clone()),
                                                anchor_id: Some(anchor_id),
                                            };
                                        }
                                    },
                                }
                            }
                        } else {
                            RepoManuscriptCard {
                                key: "{r.id}",
                                repo: r,
                                on_open: {
                                    let list_key = list_key.clone();
                                    move |anchor_id: String| {
                                        *REPO_LIST_MEMORY.write() = RepoListMemory {
                                            list_key: Some(list_key.clone()),
                                            anchor_id: Some(anchor_id),
                                        };
                                    }
                                },
                            }
                        }
                    }
                }
            }
            if total_pages > 1 {
                div { class: "pt-2",
                    CommonPagination {
                        current_page,
                        total_pages,
                        on_page_change: move |p| {
                            ctx.current_page.set(p);
                            navigator.replace(repo_list_route_from_ctx(ctx, p, (ctx.page_size)()));
                        },
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct TagAdviceItem {
    key: String,
    count: u64,
}

#[derive(Clone, Copy)]
struct RepoListContext {
    active_tags: Signal<Vec<String>>,
    filter_type: Signal<FilterType>,
    sort_type: Signal<SortType>,
    page_size: Signal<u32>,
    current_page: Signal<u32>,
    summary: Signal<ListSummary>,
    last_success: Signal<Option<RepoListCachedPage>>,
}

fn parse_tags_query(tags: Option<&str>) -> Vec<String> {
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

fn active_tags_to_query(active_tags: &[String]) -> Option<String> {
    if active_tags.is_empty() {
        None
    } else {
        Some(active_tags.join(","))
    }
}

fn parse_filter_type(range: Option<&str>, metric: Option<&str>) -> FilterType {
    let metric_value = metric.unwrap_or_default().trim().to_lowercase();
    if metric_value == "recent" || metric_value == "add_time" || metric_value == "latest" {
        return FilterType::Total;
    }
    match range.unwrap_or_default().trim().to_lowercase().as_str() {
        "daily" | "day" => FilterType::Daily,
        "monthly" | "month" => FilterType::Monthly,
        "weekly" | "week" => FilterType::Weekly,
        _ => FilterType::Total,
    }
}

fn parse_sort_type(metric: Option<&str>) -> SortType {
    match metric.unwrap_or_default().trim().to_lowercase().as_str() {
        "fork" | "forks" => SortType::Fork,
        "issue" | "issues" => SortType::Issue,
        "recent" | "add_time" | "latest" => SortType::AddTime,
        _ => SortType::Star,
    }
}

fn filter_label(filter: FilterType) -> &'static str {
    match filter {
        FilterType::Total => "Total",
        FilterType::Daily => "Daily",
        FilterType::Weekly => "Weekly",
        FilterType::Monthly => "Monthly",
    }
}

fn sort_label(sort: SortType) -> &'static str {
    match sort {
        SortType::Star => "Stars",
        SortType::Fork => "Forks",
        SortType::Issue => "Issues",
        SortType::AddTime => "Update Time",
    }
}

fn sort_metric(sort: SortType) -> RepoRankMetric {
    match sort {
        SortType::Star => RepoRankMetric::Star,
        SortType::Fork => RepoRankMetric::Fork,
        SortType::Issue => RepoRankMetric::Issue,
        SortType::AddTime => RepoRankMetric::Recent,
    }
}

fn filter_range(filter: FilterType) -> RepoRankTimeRange {
    match filter {
        FilterType::Daily => RepoRankTimeRange::Daily,
        FilterType::Weekly => RepoRankTimeRange::Weekly,
        FilterType::Monthly => RepoRankTimeRange::Monthly,
        FilterType::Total => RepoRankTimeRange::Weekly,
    }
}

fn sort_metric_query(sort: SortType) -> &'static str {
    match sort {
        SortType::Star => "star",
        SortType::Fork => "fork",
        SortType::Issue => "issue",
        SortType::AddTime => "recent",
    }
}

fn filter_range_query(filter: FilterType) -> &'static str {
    match filter {
        FilterType::Daily => "daily",
        FilterType::Weekly => "weekly",
        FilterType::Monthly => "monthly",
        FilterType::Total => "weekly",
    }
}

fn query_params_from_filter_sort(
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

fn append_tag_query(active_tags: &[String], append: &str) -> String {
    let mut next = active_tags.to_vec();
    if !next.iter().any(|tag| tag == append) {
        next.push(append.to_string());
    }
    next.join(",")
}

fn remove_tag_query(active_tags: &[String], remove: &str) -> Option<String> {
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

#[component]
pub fn RepoList(
    tags: Option<String>,
    metric: Option<String>,
    range: Option<String>,
    page: Option<u32>,
    size: Option<u32>,
) -> Element {
    let active_tags = use_signal(|| parse_tags_query(tags.as_deref()));
    let filter_type = use_signal(|| parse_filter_type(range.as_deref(), metric.as_deref()));
    let sort_type = use_signal(|| parse_sort_type(metric.as_deref()));
    let page_size = use_signal(|| normalize_page_size(size.unwrap_or(50)));
    let current_page = use_signal(|| page.unwrap_or(1).max(1));
    let summary = use_signal(ListSummary::empty);
    let last_success = use_signal(|| None::<RepoListCachedPage>);

    use_context_provider(|| RepoListContext {
        active_tags,
        filter_type,
        sort_type,
        page_size,
        current_page,
        summary,
        last_success,
    });

    rsx! {
        div { class: "min-h-screen grid grid-rows-[auto_auto_minmax(0,1fr)]",
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
                                RepoListTags {}
                            }
                            RepoListHandler {}
                        }
                    }
                }
            }
            GridSlashTransition {}
            GridWrapper { class: "min-h-0 h-full", padding: GridPadding::Sm,
                IOCell {
                    loading_fallback: rsx! { RepoListCachedFallback {} },
                    RepoListIO {}
                }
            }
        }
    }
}

#[component]
fn RepoMeta() -> Element {
    let ctx = use_context::<RepoListContext>();
    let hero_type = if (ctx.active_tags)().is_empty() {
        RepoListHeroType::AllProjects
    } else {
        RepoListHeroType::SearchResult
    };

    rsx! {
        div { class: "max-w-3xl",
            h1 { class: "text-2xl md:text-3xl font-black font-sans uppercase tracking-tight text-secondary-2 mb-2 flex flex-wrap items-center gap-2",
                if hero_type == RepoListHeroType::SearchResult {
                    "Search Result"
                } else {
                    "All Project"
                }
            }
            p { class: "text-secondary-3 text-sm md:text-base leading-relaxed font-mono italic",
                "A comprehensive catalog of the Rust ecosystem. Monitor growth, track updates, and discover foundational codebases."
            }
        }
    }
}

#[component]
fn RepoListTags() -> Element {
    let mut ctx = use_context::<RepoListContext>();
    let navigator = use_navigator();
    let facets = use_server_future(move || list_repo_tag_facets((ctx.active_tags)(), Some(20)))?;

    match facets() {
        Some(Ok(items)) => {
            let active_set = (ctx.active_tags)()
                .iter()
                .map(|v| v.to_lowercase())
                .collect::<BTreeSet<_>>();
            let advice_tags = items
                .into_iter()
                .filter(|item| !active_set.contains(&item.value.to_lowercase()))
                .map(|item| TagAdviceItem {
                    key: item.value,
                    count: item.count,
                })
                .collect::<Vec<_>>();

            rsx! {
                div { class: "flex flex-col gap-3",
                    if !(ctx.active_tags)().is_empty() {
                        div { class: "flex flex-wrap gap-2",
                            for tag in (ctx.active_tags)().iter() {
                                button {
                                    key: "{tag}",
                                    class: "flex items-center gap-1.5 px-3 py-1.5 bg-secondary-2 text-primary border border-secondary-2 rounded-none text-xs font-bold font-mono uppercase tracking-wider",
                                    onclick: {
                                        let tag = tag.clone();
                                        move |_| {
                                            let next_tags = remove_tag_query(&(ctx.active_tags)(), &tag);
                                            ctx.active_tags.set(parse_tags_query(next_tags.as_deref()));
                                            ctx.current_page.set(1);
                                            let (metric_q, range_q) = query_params_from_filter_sort(
                                                (ctx.filter_type)(),
                                                (ctx.sort_type)(),
                                            );
                                            navigator
                                                .push(crate::root::Route::RepoListView {
                                                    tags: next_tags,
                                                    metric: metric_q,
                                                    range: range_q,
                                                    page: Some(1),
                                                    size: Some((ctx.page_size)()),
                                                });
                                        }
                                    },
                                    "{tag} ×"
                                }
                            }
                        }
                    }
                    if !advice_tags.is_empty() {
                        div { class: "flex flex-wrap gap-2",
                            for advice in advice_tags {
                                button {
                                    key: "{advice.key}",
                                    class: "flex items-center gap-1.5 px-3 py-1.5 bg-primary border border-primary-6 text-secondary-5 rounded-none text-xs font-bold font-mono uppercase tracking-wider hover:border-secondary-3 hover:text-secondary-3 transition-colors",
                                    onclick: {
                                        move |_| {
                                            let query = append_tag_query(&(ctx.active_tags)(), &advice.key);
                                            ctx.active_tags.set(parse_tags_query(Some(&query)));
                                            ctx.current_page.set(1);
                                            let (metric_q, range_q) = query_params_from_filter_sort(
                                                (ctx.filter_type)(),
                                                (ctx.sort_type)(),
                                            );
                                            navigator
                                                .push(crate::root::Route::RepoListView {
                                                    tags: Some(query),
                                                    metric: metric_q,
                                                    range: range_q,
                                                    page: Some(1),
                                                    size: Some((ctx.page_size)()),
                                                });
                                        }
                                    },
                                    "{advice.key} ({advice.count})"
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(_)) => rsx! {},
        None => rsx! {},
    }
}

#[component]
fn RepoListHandler() -> Element {
    let mut ctx = use_context::<RepoListContext>();
    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col md:flex-row items-center justify-between gap-4",
            div { class: "text-xs font-mono tracking-wide text-secondary-5",
                "List "
                span { class: "font-semibold text-secondary-3",
                    "{(ctx.summary)().from}-{(ctx.summary)().to}"
                }
                " of "
                span { class: "font-semibold text-secondary-3", "{(ctx.summary)().total}" }
            }
            div { class: "flex items-center gap-4 w-full md:w-auto",
                Select::<FilterType> {
                    value: Some((ctx.filter_type)()),
                    placeholder: "filter",
                    on_value_change: move |next: Option<FilterType>| {
                        if let Some(next_filter) = next {
                            ctx.filter_type.set(next_filter);
                            ctx.current_page.set(1);
                            navigator
                                .push(repo_list_route_from_ctx(ctx, 1, (ctx.page_size)()));
                        }
                    },
                    SelectTrigger {
                        aria_label: "Select filter",
                        style: "min-width: 9rem;",
                        SelectValue {}
                    }
                    SelectList { aria_label: "Filter options",
                        SelectGroup {
                            SelectGroupLabel { "Filter" }
                            SelectOption::<FilterType> {
                                index: 0usize,
                                value: FilterType::Total,
                                text_value: Some(filter_label(FilterType::Total).to_string()),
                                "{filter_label(FilterType::Total)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 1usize,
                                value: FilterType::Daily,
                                text_value: Some(filter_label(FilterType::Daily).to_string()),
                                "{filter_label(FilterType::Daily)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 2usize,
                                value: FilterType::Weekly,
                                text_value: Some(filter_label(FilterType::Weekly).to_string()),
                                "{filter_label(FilterType::Weekly)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 3usize,
                                value: FilterType::Monthly,
                                text_value: Some(filter_label(FilterType::Monthly).to_string()),
                                "{filter_label(FilterType::Monthly)}"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
                Select::<u32> {
                    value: Some((ctx.page_size)()),
                    placeholder: "page size",
                    on_value_change: move |v: Option<u32>| {
                        if let Some(v) = v {
                            let next_size = normalize_page_size(v);
                            ctx.page_size.set(next_size);
                            ctx.current_page.set(1);
                            navigator.replace(repo_list_route_from_ctx(ctx, 1, next_size));
                        }
                    },
                    SelectTrigger {
                        aria_label: "Select page size",
                        style: "min-width: 7rem;",
                        SelectValue {}
                    }
                    SelectList { aria_label: "Page size options",
                        SelectGroup {
                            SelectGroupLabel { "Page size" }
                            SelectOption::<u32> {
                                index: 0usize,
                                value: 20u32,
                                text_value: Some("20".to_string()),
                                "20"
                                SelectItemIndicator {}
                            }
                            SelectOption::<u32> {
                                index: 1usize,
                                value: 50u32,
                                text_value: Some("50".to_string()),
                                "50"
                                SelectItemIndicator {}
                            }
                            SelectOption::<u32> {
                                index: 2usize,
                                value: 100u32,
                                text_value: Some("100".to_string()),
                                "100"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
                Select::<SortType> {
                    value: Some((ctx.sort_type)()),
                    placeholder: "sort",
                    on_value_change: move |next: Option<SortType>| {
                        if let Some(next_sort) = next {
                            ctx.sort_type.set(next_sort);
                            if next_sort == SortType::AddTime {
                                ctx.filter_type.set(FilterType::Total);
                            }
                            ctx.current_page.set(1);
                            navigator
                                .push(repo_list_route_from_ctx(ctx, 1, (ctx.page_size)()));
                        }
                    },
                    SelectTrigger { aria_label: "Select sort", style: "min-width: 10rem;", SelectValue {} }
                    SelectList { aria_label: "Sort options",
                        SelectGroup {
                            SelectGroupLabel { "Sort" }
                            SelectOption::<SortType> {
                                index: 0usize,
                                value: SortType::Star,
                                text_value: Some(sort_label(SortType::Star).to_string()),
                                "{sort_label(SortType::Star)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 1usize,
                                value: SortType::Fork,
                                text_value: Some(sort_label(SortType::Fork).to_string()),
                                "{sort_label(SortType::Fork)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 2usize,
                                value: SortType::Issue,
                                text_value: Some(sort_label(SortType::Issue).to_string()),
                                "{sort_label(SortType::Issue)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 3usize,
                                value: SortType::AddTime,
                                text_value: Some(sort_label(SortType::AddTime).to_string()),
                                "{sort_label(SortType::AddTime)}"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RepoListIO() -> Element {
    let mut ctx = use_context::<RepoListContext>();
    let repos = use_server_future(move || {
        let active_tags = (ctx.active_tags)();
        let filter_type = (ctx.filter_type)();
        let sort_type = (ctx.sort_type)();
        let limit = (ctx.page_size)();
        let page = (ctx.current_page)().max(1);
        let page_query = PageQuery {
            limit: Some(limit),
            offset: Some(limit.saturating_mul(page.saturating_sub(1))),
        };
        let is_default_query = filter_type == FilterType::Total && sort_type == SortType::Star;
        let query = RepoListQuery {
            page: page_query,
            metric: if is_default_query {
                None
            } else {
                Some(sort_metric(sort_type))
            },
            range: if is_default_query {
                None
            } else {
                Some(filter_range(filter_type))
            },
            tags: (!active_tags.is_empty()).then_some(active_tags),
        };
        async move { list_repos_with_query(query).await }
    })?;

    match repos() {
        Some(Ok(page)) => {
            let meta = page.meta;
            let items = page.items;
            let active_tags = (ctx.active_tags)();
            let hero_type = if active_tags.is_empty() {
                RepoListHeroType::AllProjects
            } else {
                RepoListHeroType::SearchResult
            };

            let visible_total = items.len() as u64;
            let from = if visible_total == 0 {
                0
            } else {
                meta.offset as u64 + 1
            };
            let to = meta.offset as u64 + visible_total;
            let next_summary = ListSummary {
                from,
                to,
                total: meta.total,
            };
            if (ctx.summary)() != next_summary {
                ctx.summary.set(next_summary);
            }
            let total_pages = meta.total_pages;
            let cached_page = RepoListCachedPage {
                items: items.clone(),
                total_pages,
                current_page: (ctx.current_page)(),
                hero_type,
            };
            if (ctx.last_success)().as_ref() != Some(&cached_page) {
                ctx.last_success.set(Some(cached_page));
            }

            rsx! {
                RepoListContent {
                    items,
                    total_pages,
                    current_page: (ctx.current_page)(),
                    hero_type,
                }
            }
        }
        Some(Err(e)) => {
            if (ctx.summary)() != ListSummary::empty() {
                ctx.summary.set(ListSummary::empty());
            }
            ctx.last_success.set(None);
            rsx! {
                div { class: "rounded-lg border border-primary-6 bg-primary-1 p-4 text-sm text-primary-error",
                    "{e}"
                }
            }
        }
        None => rsx! { RepoListCachedFallback {} },
    }
}
