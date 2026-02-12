use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::components::common::RepoRow;
use crate::IO::repos::list_repos;
use app::prelude::Pagination;

#[derive(Clone, PartialEq, Eq)]
struct TagItem {
    key: String,
    label: String,
    value: String,
    count: usize,
}

#[component]
pub fn TagList() -> Element {
    let mut selected_tag = use_signal(|| Option::<String>::None);

    let repos = use_server_future(move || {
        list_repos(Pagination {
            limit: Some(200),
            offset: Some(0),
        })
    })?;

    rsx! {
        div { class: "py-8 space-y-6",
            div { class: "space-y-1 rounded-xl border border-primary-6 bg-primary-2 p-5",
                h1 { class: "text-2xl font-semibold tracking-tight", "Tags" }
                p { class: "text-sm text-secondary-5", "Tag list" }
            }

            match repos() {
                Some(Ok(page)) => {
                    let total_repos = page.meta.total;
                    let repos = page.items;

                    let mut tag_map: BTreeMap<(String, String), usize> = BTreeMap::new();
                    for repo in &repos {
                        for tag in &repo.tags {
                            let key = (tag.label.clone(), tag.value.clone());
                            let entry = tag_map.entry(key).or_insert(0);
                            *entry += 1;
                        }
                    }

                    let mut tags = Vec::with_capacity(tag_map.len());
                    for ((label, value), count) in tag_map {
                        tags.push(TagItem {
                            key: format!("{label}:{value}"),
                            label,
                            value,
                            count,
                        });
                    }

                    let total_tags = tags.len();
                    let selected = selected_tag();

                    let mut filtered_repos = Vec::new();
                    for repo in repos {
                        let keep = match &selected {
                            Some(tag_key) => repo
                                .tags
                                .iter()
                                .any(|t| format!("{}:{}", t.label, t.value) == *tag_key),
                            None => true,
                        };
                        if keep {
                            filtered_repos.push(repo);
                        }
                    }

                    rsx! {
                        div { class: "text-sm text-secondary-5",
                            "repos: {total_repos} | tags: {total_tags}"
                        }

                        div { class: "rounded-xl border border-primary-6 bg-primary-2 p-4 space-y-3",
                            div { class: "text-sm font-medium text-secondary-4", "Tag filters" }
                            div { class: "flex flex-wrap gap-2",
                                button {
                                    class: if selected.is_none() {
                                        "rounded-md border border-primary-6 bg-primary-1 px-3 py-1 text-xs font-medium text-secondary-4"
                                    } else {
                                        "rounded-md border border-primary-6 bg-primary-2 px-3 py-1 text-xs text-secondary-5 hover:bg-primary-1"
                                    },
                                    onclick: move |_| selected_tag.set(None),
                                    "All"
                                }

                                for tag in tags {
                                    button {
                                        key: "{tag.key}",
                                        class: if selected.as_ref() == Some(&tag.key) {
                                            "rounded-md border border-primary-6 bg-primary-1 px-3 py-1 text-xs font-medium text-secondary-4"
                                        } else {
                                            "rounded-md border border-primary-6 bg-primary-2 px-3 py-1 text-xs text-secondary-5 hover:bg-primary-1"
                                        },
                                        onclick: move |_| selected_tag.set(Some(tag.key.clone())),
                                        "{tag.label}:{tag.value} ({tag.count})"
                                    }
                                }
                            }
                        }

                        if filtered_repos.is_empty() {
                            div { class: "rounded-xl border border-primary-6 bg-primary-2 p-8 text-center text-sm text-secondary-5",
                                "no repos for selected tag"
                            }
                        } else {
                            div { class: "space-y-4",
                                for repo in filtered_repos {
                                    RepoRow { key: "{repo.id}", repo }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => rsx! {
                    div { class: "rounded-lg border border-primary-6 bg-primary-1 p-4 text-sm text-primary-error", "{e}" }
                },
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading..." } },
            }
        }
    }
}
