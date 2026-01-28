use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::IO::repos::list_repos;
use app::prelude::Pagination;

#[derive(Clone, PartialEq, Eq)]
struct TagItem {
    label: String,
    value: String,
    count: usize,
}

#[component]
pub fn TagList() -> Element {
    let repos = use_server_future(move || {
        list_repos(Pagination {
            limit: Some(200),
            offset: Some(0),
        })
    })?;

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-6 space-y-6",
            div { class: "space-y-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Tags" }
                p { class: "text-sm text-secondary-5", "Tag list" }
            }

            match repos() {
                Some(Ok(page)) => {
                    let total_repos = page.meta.total;
                    let mut tag_map: BTreeMap<(String, String), usize> = BTreeMap::new();
                    for repo in page.items {
                        for tag in repo.tags {
                            let key = (tag.label, tag.value);
                            let entry = tag_map.entry(key).or_insert(0);
                            *entry += 1;
                        }
                    }
                    let mut tags = Vec::with_capacity(tag_map.len());
                    for ((label, value), count) in tag_map {
                        tags.push(TagItem { label, value, count });
                    }
                    let total_tags = tags.len();
                    rsx! {
                        div { class: "text-sm text-secondary-5",
                            "repos: {total_repos} | tags: {total_tags}"
                        }
                        if tags.is_empty() {
                            div { class: "text-sm text-secondary-5", "no tags" }
                        } else {
                            div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3",
                                for tag in tags {
                                    div {
                                        key: "{tag.label}:{tag.value}",
                                        class: "rounded-xl border border-primary-6 bg-primary-3 p-4",
                                        div { class: "flex items-center justify-between gap-2",
                                            div { class: "font-semibold truncate", "{tag.label}:{tag.value}" }
                                            div { class: "text-sm text-secondary-5", "{tag.count}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => rsx! { div { class: "text-sm", "{e}" } },
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading..." } },
            }
        }
    }
}
