use dioxus::prelude::*;

use crate::components::common::RepoAvatar;
use crate::components::icons::{GithubIcon, HouseIcon, StarIcon, TagsIcon, UsersRoundIcon};
use crate::components::ui::avatar::AvatarImageSize;
use crate::IO::repos::get_repo;

use super::RepoDetailContext;

pub(super) mod skeleton;

#[component]
pub(crate) fn MetaSection() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let owner = ctx.owner;
    let name = ctx.name;
    let navigator = use_navigator();

    let repo_fut = use_server_future(move || get_repo(owner(), name()))?;

    let owner = owner();
    let name = name();

    let repo_data = match repo_fut() {
        Some(Ok(Some(r))) => Some(r),
        _ => None,
    };

    let github_url = format!("https://github.com/{owner}/{name}");
    let homepage_url = repo_data.as_ref().and_then(|r| r.homepage_url.clone());
    let avatar_candidates = repo_data
        .as_ref()
        .map(|repo| repo.avatar_urls.clone())
        .unwrap_or_default();

    rsx! {
        section { class: "relative min-h-80 overflow-hidden",
            div { class: "relative z-10 flex h-full flex-col gap-6 md:flex-row md:items-stretch md:justify-between",
                div { class: "flex min-w-0 flex-1 items-start gap-6",
                    div { class: "hidden md:block relative h-24 w-24 shrink-0",
                        div { class: "absolute left-2 top-2 h-24 w-24 border border-primary-6 bg-screentone" }
                        RepoAvatar {
                            repo_id: format!("{owner}/{name}"),
                            avatar_urls: avatar_candidates.clone(),
                            size: AvatarImageSize::Custom,
                            class: "relative z-10 h-24 w-24 border border-primary-6 bg-primary",
                            fallback_class: "relative z-10 flex h-24 w-24 items-center justify-center border border-primary-6 bg-primary-2 text-2xl font-bold text-secondary-4",
                        }
                    }

                    div { class: "space-y-4 min-w-0",
                        div { class: "space-y-2",
                            div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5",
                                "REPOSITORY / DETAIL"
                            }
                            h1 { class: "truncate text-3xl font-bold tracking-tight text-secondary-2 md:text-5xl",
                                "{owner}/{name}"
                            }
                            if let Some(full_name) = repo_data.as_ref().and_then(|r| r.full_name.clone()) {
                                p { class: "text-xs font-mono uppercase tracking-wider text-secondary-5",
                                    "{full_name}"
                                }
                            }
                            if let Some(desc) = repo_data.as_ref().and_then(|r| r.description.clone()) {
                                p { class: "max-w-3xl border-l-2 border-primary-5 pl-3 text-sm leading-relaxed text-secondary-4",
                                    "{desc}"
                                }
                            }
                            if let Some(date) = repo_data.as_ref().and_then(|r| r.last_fetched_at.clone()) {
                                p { class: "pt-2 text-sm text-secondary-5",
                                    "Last Updated on "
                                    span { class: "font-semibold text-secondary-2",
                                        "{date}"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex min-h-[220px] w-full flex-col items-start gap-5 border-l-2 border-primary-6 pl-6 md:w-64 md:justify-between",
                    div { class: "flex w-full flex-col items-start gap-3 text-sm",
                        a {
                            href: "{github_url}",
                            target: "_blank",
                            class: "group flex w-full items-center justify-start gap-2 border border-primary-6 bg-primary-1 px-3 py-2 font-mono text-xs font-semibold uppercase tracking-wider text-secondary-5 transition-all hover:-translate-y-0.5 hover:text-secondary-3 hover:shadow-comic-sm",
                            GithubIcon { width: 16, height: 16 }
                            span { "GitHub" }
                        }
                        if let Some(url) = homepage_url {
                            a {
                                href: "{url}",
                                target: "_blank",
                                class: "group flex w-full items-center justify-start gap-2 border border-primary-6 bg-primary-1 px-3 py-2 font-mono text-xs font-semibold uppercase tracking-wider text-secondary-5 transition-all hover:-translate-y-0.5 hover:text-secondary-3 hover:shadow-comic-sm",
                                HouseIcon { width: 16, height: 16 }
                                span { "Homepage" }
                            }
                        }
                    }
                }
            }

            if let Some(repo) = repo_data {
                div { class: "mt-8 flex flex-wrap gap-4 border-t border-primary-6 pt-8",
                    div { class: "flex items-center gap-2 border border-primary-6 bg-primary-1 px-3 py-1.5 shadow-comic-sm",
                        StarIcon { width: 16, height: 16, class: "text-yellow-500 fill-current" }
                        span { class: "font-bold", "{repo.stars}" }
                        span { class: "text-xs text-secondary-5", "Stars" }
                    }
                    div { class: "flex items-center gap-2 border border-primary-6 bg-primary-1 px-3 py-1.5 shadow-comic-sm",
                        span { class: "font-bold text-secondary-5", "⑂" }
                        span { class: "font-bold", "{repo.forks}" }
                        span { class: "text-xs text-secondary-5", "Forks" }
                    }
                    div { class: "flex items-center gap-2 border border-primary-6 bg-primary-1 px-3 py-1.5 shadow-comic-sm",
                        span { class: "h-2 w-2 rounded-full bg-orange-500" }
                        span { class: "font-bold", "{repo.open_issues}" }
                        span { class: "text-xs text-secondary-5", "Issues" }
                    }
                    div { class: "flex items-center gap-2 border border-primary-6 bg-primary-1 px-3 py-1.5 shadow-comic-sm",
                        UsersRoundIcon { width: 16, height: 16, class: "text-emerald-500" }
                        span { class: "font-bold", "{repo.watchers}" }
                        span { class: "text-xs text-secondary-5", "Watchers" }
                    }
                }

                if !repo.tags.is_empty() {
                    div { class: "mt-4 flex flex-wrap gap-2",
                        for tag in repo.tags.iter() {
                            span { class: "flex items-center gap-1.5 border border-primary-6 bg-primary-1 px-2 py-0.5 text-xs font-medium text-secondary-5",
                                TagsIcon { width: 12, height: 12 }
                                "{tag.label}:{tag.value}"
                            }
                        }
                    }
                }
            }
        }
    }
}
