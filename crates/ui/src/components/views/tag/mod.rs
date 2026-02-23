use std::collections::BTreeMap;

use dioxus::prelude::*;
use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, RepoManuscriptCard,
};
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
        div { class: "space-y-0",
            GridWrapper {
                grid_type: GridType::Default,
                padding: GridPadding::Lg,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Dot,
                    gradient: GradientDirection::ToBottom,
                },
                section { class: "relative overflow-hidden",
                    div { class: "relative z-10 space-y-3",
                        h1 { class: "text-4xl font-bold tracking-tight text-secondary-2", "Archives" }
                        p { class: "max-w-2xl text-sm leading-relaxed text-secondary-5",
                            "Browse the collection by category. The index is organized by primary domain applications."
                        }
                    }
                }
            }
            GridSlashTransition {}

            GridWrapper { padding: GridPadding::Sm,
                section { class: "space-y-8 bg-primary-1",
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
                                div { class: "flex items-center justify-between gap-3 border border-primary-6 bg-primary px-4 py-3",
                                    div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                        "REPOS: "
                                        span { class: "font-semibold text-secondary-3", "{total_repos}" }
                                    }
                                    div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                        "TAGS: "
                                        span { class: "font-semibold text-secondary-3", "{total_tags}" }
                                    }
                                }

                        div { class: "space-y-3 border border-dashed border-primary-6 bg-hatch p-4",
                            div { class: "text-sm font-mono font-semibold tracking-wide text-secondary-4", "Tag filters" }
                            div { class: "flex flex-wrap gap-2",
                                button {
                                    class: if selected.is_none() {
                                        "border border-secondary-2 bg-secondary-2 px-3 py-1 text-xs font-medium text-primary shadow-comic-sm"
                                    } else {
                                        "border border-primary-6 bg-primary px-3 py-1 text-xs text-secondary-5 hover:bg-primary-1"
                                    },
                                    onclick: move |_| selected_tag.set(None),
                                    "All"
                                }

                                for tag in tags {
                                    button {
                                        key: "{tag.key}",
                                        class: if selected.as_ref() == Some(&tag.key) {
                                            "border border-secondary-2 bg-secondary-2 px-3 py-1 text-xs font-medium text-primary shadow-comic-sm"
                                        } else {
                                            "border border-primary-6 bg-primary px-3 py-1 text-xs text-secondary-5 hover:bg-primary-1"
                                        },
                                        onclick: move |_| selected_tag.set(Some(tag.key.clone())),
                                        "{tag.label}:{tag.value} ({tag.count})"
                                    }
                                }
                            }
                        }

                        if filtered_repos.is_empty() {
                            div { class: "flex min-h-[320px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary text-center",
                                span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-5", "NO_DATA" }
                                span { class: "text-sm text-secondary-5", "No repos for selected tag" }
                            }
                        } else {
                            div { class: "space-y-4",
                                for repo in filtered_repos {
                                    RepoManuscriptCard { key: "{repo.id}", repo }
                                }
                            }
                        }
                            }
                        }
                        Some(Err(e)) => rsx! {
                            div { class: "border border-primary-error bg-primary p-4 text-sm text-primary-error", "{e}" }
                        },
                        None => rsx! { div { class: "text-sm text-secondary-5", "Loading..." } },
                    }
                }
            }
        }
    }
}
