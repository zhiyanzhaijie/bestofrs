use dioxus::prelude::*;

use app::prelude::Pagination as PageQuery;
use app::repo::{RepoListQuery, RepoRankTimeRange};

use crate::IO::repos::list_repos_with_query;

mod repo_list_content;
pub(super) mod skeleton;

use super::{
    filter_range, sort_metric, FilterType, ListSummary, RepoListCachedPage, RepoListContext,
    RepoListHeroType, SortType,
};
use repo_list_content::RepoListContent;

#[component]
pub(super) fn RepoListIO() -> Element {
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
        let query = RepoListQuery {
            page: page_query,
            metric: Some(sort_metric(sort_type)),
            range: Some(
                if sort_type == SortType::AddTime || filter_type == FilterType::Total {
                    RepoRankTimeRange::All
                } else {
                    filter_range(filter_type)
                },
            ),
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
        None => rsx! { skeleton::RepoListCachedFallback {} },
    }
}
