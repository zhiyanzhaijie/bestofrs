use crate::components::common::RepoAvatar;
use crate::components::icons::{ArrowRightIcon, GithubIcon, HouseIcon, StarIcon};
use crate::components::ui::avatar::AvatarImageSize;
use crate::root::Route;
use crate::types::repos::RepoDto;
use dioxus::prelude::*;

#[component]
pub fn RepoManuscriptCard(
    repo: RepoDto,
    on_open: Option<EventHandler<String>>,
) -> Element {
    let RepoDto {
        id,
        stars,
        forks,
        description,
        homepage_url,
        avatar_urls,
        tags,
        ..
    } = repo;

    let navigator = use_navigator();
    let (owner, name) = id.split_once('/').unwrap_or(("", &id));
    let github_url = if owner.is_empty() {
        format!("https://github.com/{id}")
    } else {
        format!("https://github.com/{owner}/{name}")
    };
    let homepage = homepage_url.as_deref().and_then(normalize_url);

    let route = if owner.is_empty() {
        Route::HomeView {}
    } else {
        Route::RepoDetailView {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    };
    let anchor_id = repo_anchor_id(&id);

    rsx! {
        article {
            id: "{anchor_id}",
            class: "group relative flex min-h-[120px] cursor-pointer flex-col border border-primary-6 [box-shadow:4px_4px_0_0_color-mix(in_oklab,var(--primary-color-6)_70%,transparent)] transition-all duration-200 hover:-translate-y-0.5 hover:bg-primary-1 hover:[border-color:color-mix(in_oklab,var(--grid-accent)_78%,var(--secondary-color-2))] hover:[box-shadow:8px_8px_0_0_color-mix(in_oklab,var(--grid-accent)_72%,transparent)] md:flex-row",
            onclick: move |_| {
                navigator.push(route.clone());
                if let Some(on_open) = on_open.as_ref() {
                    on_open.call(anchor_id.clone());
                }
            },
            div { class: "relative flex shrink-0 items-center gap-3 p-4 md:w-56",
                div { class: "relative h-16 w-16 shrink-0",
                    div { class: "absolute left-1 top-1 h-16 w-16 border border-primary-6 bg-screentone transition-all duration-200 group-hover:left-2 group-hover:top-2 group-hover:[border-color:color-mix(in_oklab,var(--grid-accent)_72%,var(--primary-color-6))] group-hover:[background-color:color-mix(in_oklab,var(--grid-accent)_18%,var(--primary-color))]" }
                    RepoAvatar {
                        repo_id: id.clone(),
                        avatar_urls: avatar_urls.clone(),
                        size: AvatarImageSize::Large,
                        class: "relative z-10 h-16 w-16 border border-primary-6 bg-primary grayscale contrast-125 transition-all group-hover:grayscale-0",
                        fallback_class: "relative z-10 flex h-16 w-16 items-center justify-center border border-primary-6 bg-primary-2 font-bold text-secondary-4",
                    }
                }
                div { class: "flex min-w-0 flex-1 flex-col justify-center gap-2 pl-2",
                    div {
                        h3 { class: "break-words text-base font-bold leading-tight text-secondary-6 transition-colors group-hover:text-secondary-2", "{name}" }
                        p { class: "mt-0.5 text-[10px] font-mono text-secondary-5", "@{owner}" }
                    }
                    div { class: "relative z-20 flex items-center gap-2",
                            a {
                                href: "{github_url}",
                                class: "text-secondary-5 transition-colors hover:text-secondary-3",
                                target: "_blank",
                                onclick: move |evt| evt.stop_propagation(),
                                GithubIcon { width: 14, height: 14 }
                            }
                            if let Some(homepage) = homepage.clone() {
                                a {
                                    href: "{homepage}",
                                    class: "text-secondary-5 transition-colors hover:text-secondary-3",
                                    target: "_blank",
                                    onclick: move |evt| evt.stop_propagation(),
                                    HouseIcon { width: 14, height: 14 }
                                }
                            }
                    }
                }
            }
            div { class: "flex shrink-0 flex-row items-center justify-between gap-2 p-4 md:w-36 md:flex-col md:items-end",
                div { class: "flex w-full flex-col items-end gap-1 text-xs font-mono text-secondary-5",
                    div { class: "flex w-full items-center justify-end gap-2",
                        span { class: "font-medium", "{stars}" }
                        StarIcon { width: 12, height: 12, class: "text-secondary-4" }
                    }
                    div { class: "flex w-full items-center justify-end gap-2",
                        span { "{forks}" }
                        span { class: "text-secondary-4", "⑂" }
                    }
                    div { class: "mt-0.5 flex w-full items-center justify-end gap-2 font-bold text-secondary-3",
                        span { "VIEW" }
                        ArrowRightIcon { width: 12, height: 12 }
                    }
                }
                div { class: "mt-auto w-full border-t border-primary-5 pt-2 text-right text-[10px] font-mono text-secondary-5",
                    "{owner}"
                }
            }
            div { class: "flex min-w-0 flex-grow flex-col justify-between p-4",
                p { class: "mb-3 line-clamp-2 text-sm leading-relaxed text-secondary-4",
                    "{description.clone().unwrap_or_else(|| \"No description\".to_string())}"
                }
                div { class: "flex flex-wrap justify-start gap-x-2 gap-y-2",
                    for tag in tags.iter().take(6) {
                        span { class: "border-b border-primary-5 pb-0.5 font-mono text-[10px] uppercase tracking-wider text-secondary-5 transition-colors group-hover:border-secondary-6 group-hover:text-secondary-6",
                            "#{tag.label}"
                        }
                    }
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

fn repo_anchor_id(repo_id: &str) -> String {
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

