use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::root::Route;
use crate::types::repos::RepoDto;

#[component]
pub fn RepoRow(repo: RepoDto) -> Element {
    let RepoDto {
        id,
        stars,
        last_fetched_at,
        tags,
        full_name,
        homepage_url,
        avatar_url,
        ..
    } = repo;

    let (owner, name) = id.split_once('/').unwrap_or(("", &id));
    let display_name = full_name.as_deref().unwrap_or(&id);
    let github_url = if owner.is_empty() {
        format!("https://github.com/{id}")
    } else {
        format!("https://github.com/{owner}/{name}")
    };
    let homepage = homepage_url.as_deref().and_then(normalize_url);
    let favicon = homepage
        .as_deref()
        .map(|v| format!("{}/favicon.ico", v.trim_end_matches('/')));
    let owner_png = if owner.is_empty() {
        None
    } else {
        Some(format!("https://github.com/{owner}.png"))
    };
    let avatar_candidates = [favicon, avatar_url, owner_png]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let avatar_candidates_for_error = avatar_candidates.clone();
    let mut avatar_index = use_signal(|| 0usize);
    let avatar_fallback = name
        .chars()
        .next()
        .unwrap_or('R')
        .to_ascii_uppercase()
        .to_string();

    let route = if owner.is_empty() {
        Route::HomeView {}
    } else {
        Route::RepoDetailView {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    };

    rsx! {
        article { class: "flex gap-4 rounded-xl border border-primary-6 bg-primary-2 px-5 py-4",
            div { class: "shrink-0",
                Avatar {
                    size: AvatarImageSize::Small,
                    on_error: move |_| {
                        let next = avatar_index() + 1;
                        if next < avatar_candidates_for_error.len() {
                            avatar_index.set(next);
                        }
                    },
                    if let Some(src) = avatar_candidates.get(avatar_index()).cloned() {
                        AvatarImage {
                            src: src.clone(),
                            alt: "{display_name} avatar",
                        }
                    }
                    AvatarFallback { "{avatar_fallback}" }
                }
            }

            div { class: "min-w-0 flex-1 space-y-2",
                div { class: "flex items-center gap-2",
                    h3 { class: "truncate text-base font-semibold text-secondary-4", "{display_name}" }
                    if !tags.is_empty() {
                        div { class: "hidden items-center gap-1 sm:flex",
                            for tag in tags.iter().take(3) {
                                span { class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-0.5 text-xs text-secondary-5",
                                    "{tag.label}"
                                }
                            }
                        }
                    }
                }

                div { class: "text-sm text-secondary-5", "Source: {github_url}" }
                if let Some(homepage) = homepage {
                    div { class: "text-sm text-secondary-5",
                        "Homepage: "
                        a { class: "hover:underline", href: "{homepage}", target: "_blank", "{homepage}" }
                    }
                }
                div { class: "text-xs text-secondary-5",
                    if let Some(last) = last_fetched_at {
                        "Updated: {last}"
                    } else {
                        "Updated: -"
                    }
                }
            }

            div { class: "shrink-0 space-y-2 text-right",
                div { class: "text-sm font-semibold text-secondary-4", "{stars} stars" }
                Link {
                    class: "text-sm text-secondary-5 hover:text-secondary-4 hover:underline",
                    to: route,
                    "View"
                }
            }
        }
    }
}

fn normalize_url(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}
