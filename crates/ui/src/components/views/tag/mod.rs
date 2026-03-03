use dioxus::prelude::*;

use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, RepoAvatar,
};
use crate::components::ui::avatar::AvatarImageSize;
use crate::IO::repos::list_tags_with_meta;
use crate::root::Route;
fn parse_owner_name(repo_id: &str) -> Option<(String, String)> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((owner.to_string(), name.to_string()))
}

#[component]
pub fn TagList() -> Element {
    let mut page_size = use_signal(|| 20u32);
    let mut current_page = use_signal(|| 1u32);
    let tags = use_server_future(move || {
        list_tags_with_meta(Some(current_page()), Some(page_size()), None, Some(5))
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
                        h1 { class: "text-4xl font-bold tracking-tight text-secondary-2", "Tags" }
                        p { class: "max-w-2xl text-sm leading-relaxed text-secondary-5",
                            "Tag index with metadata and top repositories. Click a tag to open filtered repo list."
                        }
                    }
                }
            }
            GridSlashTransition {}

            GridWrapper { padding: GridPadding::Sm,
                section { class: "space-y-6 bg-primary-1",
                    div { class: "flex items-center gap-3",
                        span { class: "text-sm font-medium text-secondary-5", "page size" }
                        select {
                            class: "border border-primary-6 bg-primary px-2 py-1 text-sm text-secondary-4",
                            value: "{page_size()}",
                            onchange: move |e| {
                                if let Ok(v) = e.value().parse::<u32>() {
                                    page_size.set(v);
                                    current_page.set(1);
                                }
                            },
                            option { value: "10", "10" }
                            option { value: "20", "20" }
                            option { value: "50", "50" }
                        }
                    }

                    match tags() {
                        Some(Ok(page)) => {
                            rsx! {
                                div { class: "flex items-center justify-between gap-3 border border-primary-6 bg-primary px-4 py-3",
                                    div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                        "TAGS: "
                                        span { class: "font-semibold text-secondary-3", "{page.meta.total}" }
                                    }
                                    div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                        "PAGE: "
                                        span { class: "font-semibold text-secondary-3", "{page.meta.current_page}/{page.meta.total_pages.max(1)}" }
                                    }
                                }

                                if page.items.is_empty() {
                                    div { class: "flex min-h-[220px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary text-center",
                                        span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-5", "NO_DATA" }
                                        span { class: "text-sm text-secondary-5", "No tags found" }
                                    }
                                } else {
                                    div { class: "space-y-4",
                                        for tag in page.items {
                                            article { key: "{tag.label}:{tag.value}", class: "space-y-3 border border-primary-6 bg-primary px-4 py-3",
                                                div { class: "flex items-center justify-between gap-2",
                                                    Link {
                                                        class: "border border-secondary-2 bg-secondary-2 px-2.5 py-1 text-xs font-medium text-primary shadow-comic-sm",
                                                        to: Route::RepoListView { tags: Some(tag.value.clone()) },
                                                        "{tag.label}:{tag.value}"
                                                    }
                                                    span { class: "text-xs font-mono text-secondary-5",
                                                        "REPOS {tag.repos_total}"
                                                    }
                                                }
                                                if let Some(description) = tag.description.clone() {
                                                    p { class: "text-sm text-secondary-5", "{description}" }
                                                }
                                                if !tag.top_repos.is_empty() {
                                                    div { class: "space-y-2",
                                                        div { class: "text-xs font-mono tracking-wide text-secondary-5", "TOP REPOS" }
                                                        div { class: "flex flex-wrap gap-2",
                                                            for repo in tag.top_repos {
                                                                if let Some((owner, name)) = parse_owner_name(&repo.repo_id) {
                                                                    Link {
                                                                        key: "{repo.repo_id}",
                                                                        class: "flex items-center gap-2 border border-primary-6 bg-primary-1 px-2 py-1 hover:bg-primary-3",
                                                                        to: Route::RepoDetailView { owner, name },
                                                                        div { class: "h-6 w-6 shrink-0",
                                                                            RepoAvatar {
                                                                                repo_id: repo.repo_id.clone(),
                                                                                avatar_urls: repo.avatar_urls.clone(),
                                                                                class: "h-6 w-6 border border-primary-6 bg-primary".to_string(),
                                                                                fallback_class: "flex h-6 w-6 items-center justify-center border border-primary-6 bg-primary-2 text-[10px] font-bold text-secondary-4".to_string(),
                                                                                size: AvatarImageSize::Small,
                                                                            }
                                                                        }
                                                                        span { class: "text-xs text-secondary-4", "{repo.repo_id}" }
                                                                    }
                                                                } else {
                                                                    div { key: "{repo.repo_id}", class: "flex items-center gap-2 border border-primary-6 bg-primary-1 px-2 py-1",
                                                                        div { class: "h-6 w-6 shrink-0",
                                                                            RepoAvatar {
                                                                                repo_id: repo.repo_id.clone(),
                                                                                avatar_urls: repo.avatar_urls.clone(),
                                                                                class: "h-6 w-6 border border-primary-6 bg-primary".to_string(),
                                                                                fallback_class: "flex h-6 w-6 items-center justify-center border border-primary-6 bg-primary-2 text-[10px] font-bold text-secondary-4".to_string(),
                                                                                size: AvatarImageSize::Small,
                                                                            }
                                                                        }
                                                                        span { class: "text-xs text-secondary-4", "{repo.repo_id}" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
