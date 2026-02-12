use dioxus::prelude::*;

use app::prelude::Pagination as PageQuery;

use crate::components::common::{CommonPagination, RepoRow};
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use crate::IO::repos::list_repos;

#[component]
pub fn RepoListContent(title: String, subtitle: String) -> Element {
    let mut page_size = use_signal(|| 50u32);
    let mut current_page = use_signal(|| 1u32);

    let repos = use_server_future(move || {
        let limit = page_size();
        let page = current_page().max(1);
        list_repos(PageQuery {
            limit: Some(limit),
            offset: Some(limit.saturating_mul(page.saturating_sub(1))),
        })
    })?;

    rsx! {
        div { class: "py-8 space-y-6",
            div { class: "rounded-xl border border-primary-6 bg-primary-2 px-6 py-6 space-y-6",
                div { class: "space-y-2",
                    h1 { class: "text-2xl font-semibold tracking-tight text-secondary-4", "{title}" }
                    p { class: "text-sm text-secondary-5", "{subtitle}" }
                }

                match repos() {
                    Some(Ok(page)) => {
                        let meta = page.meta;
                        let total = meta.total;
                        let total_pages = meta.total_pages;
                        let items = page.items;

                        let from = if total == 0 { 0 } else { meta.offset as u64 + 1 };
                        let to = meta.offset as u64 + items.len() as u64;

                        rsx! {
                            div { class: "flex flex-col gap-4 rounded-lg border border-primary-6 bg-primary-1 px-4 py-3 sm:flex-row sm:items-center sm:justify-between",
                                div { class: "text-sm font-medium text-secondary-4",
                                    "total: {total}"
                                    if total > 0 {
                                        span { class: "ml-2 text-secondary-5", "({from}-{to})" }
                                    }
                                }

                                div { class: "flex items-center gap-3",
                                    span { class: "text-sm font-medium text-secondary-5", "page size" }
                                    Select::<u32> {
                                        value: Some(page_size()),
                                        placeholder: "page size",
                                        on_value_change: move |v: Option<u32>| {
                                            if let Some(v) = v {
                                                page_size.set(v);
                                                current_page.set(1);
                                            }
                                        },
                                        SelectTrigger { aria_label: "Select page size", style: "min-width: 7rem;",
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
                                }
                            }

                            if items.is_empty() {
                                div { class: "text-center py-10 text-secondary-5", "No repos found" }
                            } else {
                                div { class: "space-y-4",
                                    for r in items {
                                        RepoRow { key: "{r.id}", repo: r }
                                    }
                                }
                            }

                            if total_pages > 1 {
                                CommonPagination {
                                    current_page: current_page(),
                                    total_pages,
                                    on_page_change: move |p| current_page.set(p),
                                }
                            }
                        }
                    }
                    Some(Err(e)) => rsx! {
                        div { class: "rounded-lg border border-primary-6 bg-primary-1 p-4 text-sm text-primary-error", "{e}" }
                    },
                    None => rsx! { div { class: "text-center py-10 text-secondary-5", "Loading..." } },
                }
            }
        }
    }
}

#[component]
pub fn RepoList() -> Element {
    rsx! { RepoListContent { title: "Repo".to_string(), subtitle: "All repos".to_string() } }
}
