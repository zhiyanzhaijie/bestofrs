use dioxus::prelude::*;

use crate::components::common::{
    GradientDirection, GridBackground, GridLineType, GridPadding, GridPattern, GridType,
    GridWrapper, RepoAvatar,
};
use crate::components::icons::{
    ArrowRightIcon, CircleDotIcon, GitForkIcon, GithubIcon, HouseIcon, StarIcon, UsersRoundIcon,
};
use crate::components::ui::avatar::AvatarImageSize;
use crate::root::Route;
use crate::IO::repos::get_repo;

use super::RepoDetailContext;

pub(super) mod skeleton;

#[component]
pub(crate) fn MetaSection() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let owner = ctx.owner;
    let name = ctx.name;

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
            div { class: "relative z-10 flex flex-col gap-10",
                div { class: "grid grid-cols-1 lg:grid-cols-12",
                    GridWrapper {
                        class: "lg:col-span-9".to_string(),
                        padding: GridPadding::Sm,
                        is_dot_on: false,
                        div { class: "flex min-w-0 flex-col gap-8",
                            div { class: "flex flex-col items-start gap-8 md:flex-row",
                                div { class: "relative h-32 w-32 shrink-0 md:h-40 md:w-40",
                                    div { class: "absolute left-2 top-2 h-full w-full border-2 border-primary-6 bg-screentone" }
                                    RepoAvatar {
                                        repo_id: format!("{owner}/{name}"),
                                        avatar_urls: avatar_candidates.clone(),
                                        size: AvatarImageSize::Custom,
                                        class: "relative z-10 h-32 w-32 border-2 border-primary-6 bg-primary shadow-comic md:h-40 md:w-40",
                                        fallback_class: "relative z-10 flex h-32 w-32 items-center justify-center border-2 border-primary-6 bg-primary-2 text-3xl font-black text-secondary-3 md:h-40 md:w-40",
                                    }
                                }
                                div { class: "flex min-w-0 flex-1 flex-col gap-4",
                                    div {
                                        h1 { class: "break-all text-5xl leading-none font-black tracking-tighter text-secondary-2 uppercase md:text-7xl",
                                            "{name}"
                                        }
                                        div { class: "mt-2 font-mono text-xs font-bold tracking-widest text-grid-accent uppercase md:text-sm",
                                            "{owner}/{name}"
                                        }
                                    }
                                    if let Some(desc) = repo_data.as_ref().and_then(|r| r.description.clone()) {
                                        p { class: "max-w-3xl font-mono text-sm leading-relaxed text-secondary-4 md:text-base",
                                            span { class: "mr-2 font-bold text-grid-accent",
                                                ">"
                                            }
                                            "{desc}"
                                        }
                                    }
                                }
                            }

                            if let Some(date) = repo_data.as_ref().and_then(|r| r.last_fetched_at.clone()) {
                                div { class: "font-mono text-[10px] tracking-widest text-secondary-6 uppercase",
                                    "last updated // {date}"
                                }
                            }

                            if let Some(repo) = repo_data.as_ref() {
                                GridWrapper {
                                    grid_type: GridType::Inner,
                                    line_type: GridLineType::Dashed,
                                    is_dot_on: false,
                                    padding: GridPadding::None,
                                    background: GridBackground {
                                        pattern: GridPattern::Slash,
                                        gradient: GradientDirection::None,
                                    },
                                    div { class: "grid grid-cols-4 gap-2 px-2 py-2",
                                        div { class: "flex h-14 cursor-default items-center justify-center gap-2 px-2 py-1",
                                            StarIcon {
                                                width: 36,
                                                height: 36,
                                                class: "text-grid-accent fill-current",
                                            }
                                            div { class: "flex min-w-0 flex-col leading-none",
                                                span { class: "mb-0.5 text-xl font-black text-secondary-2",
                                                    "{repo.stars}"
                                                }
                                                span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                    "Stars"
                                                }
                                            }
                                        }
                                        div { class: "flex h-14 cursor-default items-center justify-center gap-2 px-2 py-1",
                                            GitForkIcon {
                                                width: 36,
                                                height: 36,
                                                class: "text-grid-accent",
                                            }
                                            div { class: "flex min-w-0 flex-col leading-none",
                                                span { class: "mb-0.5 text-xl font-black text-secondary-2",
                                                    "{repo.forks}"
                                                }
                                                span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                    "Forks"
                                                }
                                            }
                                        }
                                        div { class: "flex h-14 cursor-default items-center justify-center gap-2 px-2 py-1",
                                            CircleDotIcon {
                                                width: 36,
                                                height: 36,
                                                class: "text-grid-accent",
                                            }
                                            div { class: "flex min-w-0 flex-col leading-none",
                                                span { class: "mb-0.5 text-xl font-black text-secondary-2",
                                                    "{repo.open_issues}"
                                                }
                                                span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                    "Issues"
                                                }
                                            }
                                        }
                                        div { class: "flex h-14 cursor-default items-center justify-center gap-2 px-2 py-1",
                                            UsersRoundIcon {
                                                width: 36,
                                                height: 36,
                                                class: "text-grid-accent",
                                            }
                                            div { class: "flex min-w-0 flex-col leading-none",
                                                span { class: "mb-0.5 text-xl font-black text-secondary-2",
                                                    "{repo.watchers}"
                                                }
                                                span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                    "Watchers"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }


                    }
                    GridWrapper {
                        class: "lg:col-span-3".to_string(),
                        padding: GridPadding::Sm,
                        line_type: GridLineType::None,
                        div { class: "flex flex-col gap-6",
                            div { class: "font-mono text-xs font-bold tracking-widest text-secondary-5 uppercase",
                                "Links & Tags"
                            }
                            div { class: "flex flex-col gap-5",
                                a {
                                    href: "{github_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "relative group block w-[calc(100%-10px)] mb-[10px]",
                                    div { class: "absolute inset-0 rounded-full border-2 border-primary-6 bg-primary-1 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                                    div { class: "relative flex w-full items-center justify-between gap-3 rounded-full border-4 border-secondary-2 bg-primary px-8 py-3 text-secondary-2 transition-all duration-300 ease-out group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] group-hover:bg-secondary-2 group-hover:text-primary",
                                        span { class: "flex items-center gap-3",
                                            GithubIcon { width: 16, height: 16 }
                                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic",
                                                "GitHub"
                                            }
                                        }
                                        ArrowRightIcon {
                                            width: 16,
                                            height: 16,
                                            class: "transition-transform group-hover:translate-x-1",
                                        }
                                    }
                                }
                                if let Some(url) = homepage_url {
                                    a {
                                        href: "{url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "relative group block w-[calc(100%-10px)] mb-[10px]",
                                        div { class: "absolute inset-0 rounded-full border-2 border-primary-6 bg-primary-1 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                                        div { class: "relative flex w-full items-center justify-between gap-3 rounded-full border-4 border-secondary-2 bg-primary px-8 py-3 text-secondary-2 transition-all duration-300 ease-out group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] group-hover:bg-secondary-2 group-hover:text-primary",
                                            span { class: "flex items-center gap-3",
                                                HouseIcon { width: 16, height: 16 }
                                                span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic",
                                                    "Homepage"
                                                }
                                            }
                                            ArrowRightIcon {
                                                width: 16,
                                                height: 16,
                                                class: "transition-transform group-hover:translate-x-1",
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(repo) = repo_data.as_ref() {
                                if !repo.tags.is_empty() {
                                    div { class: "flex flex-wrap gap-2 pt-1",
                                        for tag in repo.tags.iter() {
                                            Link {
                                                key: "{tag.label}:{tag.value}",
                                                to: Route::RepoListView {
                                                    tags: Some(tag.value.clone()),
                                                    metric: None,
                                                    range: None,
                                                    page: None,
                                                    size: None,
                                                },
                                                class: "group inline-flex items-center border border-primary-6 bg-primary-1 px-2.5 py-1.5 font-mono text-xs font-bold tracking-wider text-secondary-2 lowercase transition-colors hover:bg-grid-accent hover:text-primary-1",
                                                span { class: "mr-1.5 text-grid-accent transition-colors group-hover:text-primary-1",
                                                    "#"
                                                }
                                                "{tag.value}"
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
