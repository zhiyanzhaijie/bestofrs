use dioxus::prelude::*;
use std::collections::BTreeSet;

use crate::IO::repos::list_repo_tag_facets;

use super::{
    append_tag_query, parse_tags_query, query_params_from_filter_sort, remove_tag_query,
    RepoListContext, TagAdviceItem,
};

pub(super) mod skeleton;

#[component]
pub(super) fn RepoListTags() -> Element {
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
                div { class: "flex flex-col gap-3 min-h-[104px]",
                    if !(ctx.active_tags)().is_empty() {
                        div { class: "flex flex-wrap gap-2",
                            for tag in (ctx.active_tags)().iter() {
                                button {
                                    key: "{tag}",
                                    class: "flex items-center gap-1.5 px-3 py-1.5 bg-secondary-2 text-primary border border-secondary-2 rounded-none text-xs font-bold font-mono uppercase tracking-wider hover:cursor-pointer",
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
                                            navigator.push(crate::root::Route::RepoListView {
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
                                    class: "flex items-center gap-1.5 px-3 py-1.5 bg-primary border border-primary-6 text-secondary-5 rounded-none text-xs font-bold font-mono uppercase tracking-wider hover:border-secondary-3 hover:text-secondary-3 hover:cursor-pointer transition-colors",
                                    onclick: {
                                        move |_| {
                                            let query = append_tag_query(&(ctx.active_tags)(), &advice.key);
                                            ctx.active_tags.set(parse_tags_query(Some(&query)));
                                            ctx.current_page.set(1);
                                            let (metric_q, range_q) = query_params_from_filter_sort(
                                                (ctx.filter_type)(),
                                                (ctx.sort_type)(),
                                            );
                                            navigator.push(crate::root::Route::RepoListView {
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
