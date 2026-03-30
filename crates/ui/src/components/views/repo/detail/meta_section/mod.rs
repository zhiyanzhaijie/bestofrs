use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::common::{
    GradientDirection, GridBackground, GridLineType, GridPadding, GridPattern, GridType,
    GridWrapper, RepoAvatar, SEOHead, SEOProp,
};
use crate::components::icons::{
    ArrowRightIcon, CircleDotIcon, GitForkIcon, GithubIcon, HouseIcon, StarIcon, UsersRoundIcon,
};
use crate::components::ui::avatar::AvatarImageSize;
use crate::impls::datetime::format_utc_ymd_hms;
use crate::root::Route;
use crate::IO::repos::get_repo;
use domain::ProjectStatus;

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

    let Some(Ok(Some(repo_data))) = repo_fut() else {
        return rsx! { skeleton::MetaSectionSkeleton {} };
    };

    let github_url = format!("https://github.com/{owner}/{name}");
    let homepage_url = repo_data.homepage_url.clone();
    let is_disabled = matches!(repo_data.project_status, Some(ProjectStatus::Disabled));
    let description = repo_data.description.clone();
    let last_fetched_at = repo_data.last_fetched_at.clone();

    // SEO part
    let seo_description = repo_data.description.clone().unwrap_or_else(|| {
            t!(
                "view_repo_detail_meta_seo_description_fallback",
                owner: owner.clone(),
                name: name.clone()
            )
            .to_string()
        });

    let seo_keywords = {
        let tag_values = repo_data
            .tags
            .iter()
            .map(|t| t.value.clone())
            .collect::<Vec<_>>()
            .join(", ");
        if tag_values.is_empty() {
            "best of rs, rust repository, github rust, rust trends, community health".to_string()
        } else {
            format!("best of rs, rust repository, {}", tag_values)
        }
    };
    let canonical_url = format!("/repo/{owner}/{name}");
    let avatar_candidates = repo_data.avatar_urls.clone();

    rsx! {
        SEOHead {
            data: SEOProp {
                title: format!("{owner}/{name}"),
                description: seo_description,
                keywords: seo_keywords,
                canonical_url,
                og_type: "article".into(),
                ..Default::default()
            },
        }
        section { class: "relative min-h-80 overflow-hidden",
            div { class: "relative z-10 flex flex-col gap-6 md:gap-10",
                div { class: "grid grid-cols-1 lg:grid-cols-12",
                    GridWrapper {
                        class: "lg:col-span-9".to_string(),
                        padding: GridPadding::Sm,
                        is_dot_on: false,
                        div { class: "flex min-w-0 flex-col gap-5 md:gap-8",
                            div { class: "flex flex-col items-start gap-5 md:flex-row md:gap-8",
                                div { class: "relative h-24 w-24 shrink-0 self-center md:h-40 md:w-40 md:self-auto",
                                    div { class: "absolute left-2 top-2 h-full w-full border-2 border-primary-6 bg-screentone" }
                                    div { class: "relative z-10 h-24 w-24 border-4 border-primary-6 bg-primary shadow-comic md:h-40 md:w-40",
                                        RepoAvatar {
                                            repo_id: format!("{owner}/{name}"),
                                            avatar_urls: avatar_candidates.clone(),
                                            size: AvatarImageSize::Custom,
                                            class: "h-24 w-24 bg-transparent md:h-40 md:w-40",
                                            fallback_class: "flex h-24 w-24 items-center justify-center bg-primary-2 text-2xl font-black text-secondary-3 md:h-40 md:w-40 md:text-3xl",
                                        }
                                    }
                                }
                                div { class: "flex min-w-0 flex-1 flex-col items-center gap-4 text-center md:items-start md:text-left",
                                    div {
                                        div { class: "flex flex-wrap items-center justify-center gap-x-3 gap-y-2 md:justify-start",
                                            h1 { class: "break-all text-3xl leading-none font-black tracking-tighter text-secondary-2 uppercase md:text-7xl",
                                                "{name}"
                                            }
                                            if is_disabled {
                                                span { class: "inline-flex items-center border border-secondary-2 bg-secondary-2 px-2 py-1 font-mono text-[10px] font-bold tracking-[0.18em] text-primary uppercase md:px-3 md:py-1.5 md:text-xs",
                                                    "Disabled"
                                                }
                                            }
                                        }
                                        div { class: "mt-1.5 text-center font-mono text-[10px] font-bold tracking-widest text-grid-accent uppercase md:mt-2 md:text-left md:text-sm",
                                            "{owner}/{name}"
                                        }
                                    }
                                    if let Some(desc) = description {
                                        p { class: "w-full max-w-3xl text-left font-mono text-xs leading-relaxed text-secondary-4 md:text-base",
                                            span { class: "mr-2 font-bold text-grid-accent",
                                                ">"
                                            }
                                            "{desc}"
                                        }
                                    }
                                }
                            }

                            if let Some(date) = last_fetched_at {
                                div { class: "font-mono text-[9px] tracking-widest text-secondary-6 uppercase md:text-[10px]",
                                    "{t!(\"view_repo_detail_meta_last_updated_prefix\")} // {format_utc_ymd_hms(&date)}"
                                }
                            }

                            GridWrapper {
                                grid_type: GridType::Inner,
                                line_type: GridLineType::Dashed,
                                is_dot_on: false,
                                padding: GridPadding::None,
                                background: GridBackground {
                                    pattern: GridPattern::Slash,
                                    gradient: GradientDirection::None,
                                },
                                div { class: "grid grid-cols-2 gap-2 px-2 py-2 md:grid-cols-4",
                                    div { class: "flex h-12 cursor-default items-center justify-center gap-1.5 px-1 py-1 md:h-14 md:gap-2 md:px-2",
                                        StarIcon {
                                            width: 24,
                                            height: 24,
                                            class: "text-grid-accent fill-current",
                                        }
                                        div { class: "flex min-w-0 flex-col leading-none",
                                            span { class: "mb-0.5 text-base font-black text-secondary-2 md:text-xl",
                                                "{repo_data.stars}"
                                            }
                                            span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                "Stars"
                                            }
                                        }
                                    }
                                    div { class: "flex h-12 cursor-default items-center justify-center gap-1.5 px-1 py-1 md:h-14 md:gap-2 md:px-2",
                                        GitForkIcon {
                                            width: 24,
                                            height: 24,
                                            class: "text-grid-accent",
                                        }
                                        div { class: "flex min-w-0 flex-col leading-none",
                                            span { class: "mb-0.5 text-base font-black text-secondary-2 md:text-xl",
                                                "{repo_data.forks}"
                                            }
                                            span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                "Forks"
                                            }
                                        }
                                    }
                                    div { class: "flex h-12 cursor-default items-center justify-center gap-1.5 px-1 py-1 md:h-14 md:gap-2 md:px-2",
                                        CircleDotIcon {
                                            width: 24,
                                            height: 24,
                                            class: "text-grid-accent",
                                        }
                                        div { class: "flex min-w-0 flex-col leading-none",
                                            span { class: "mb-0.5 text-base font-black text-secondary-2 md:text-xl",
                                                "{repo_data.open_issues}"
                                            }
                                            span { class: "font-mono text-[9px] font-bold tracking-widest text-secondary-5 uppercase",
                                                "Issues"
                                            }
                                        }
                                    }
                                    div { class: "flex h-12 cursor-default items-center justify-center gap-1.5 px-1 py-1 md:h-14 md:gap-2 md:px-2",
                                        UsersRoundIcon {
                                            width: 24,
                                            height: 24,
                                            class: "text-grid-accent",
                                        }
                                        div { class: "flex min-w-0 flex-col leading-none",
                                            span { class: "mb-0.5 text-base font-black text-secondary-2 md:text-xl",
                                                "{repo_data.watchers}"
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
                    GridWrapper {
                        class: "lg:col-span-3".to_string(),
                        padding: GridPadding::Sm,
                        line_type: GridLineType::None,
                        div { class: "flex flex-col gap-4 md:gap-6",
                            div { class: "font-mono text-[10px] font-bold tracking-widest text-secondary-5 uppercase md:text-xs",
                                {t!("view_repo_detail_meta_links_and_tags")}
                            }
                            div { class: "flex flex-col gap-3 md:gap-5",
                                a {
                                    href: "{github_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "relative group block w-[calc(100%-8px)] mb-[8px] md:w-[calc(100%-10px)] md:mb-[10px]",
                                    div { class: "absolute inset-0 rounded-full border border-primary-6 bg-primary-1 translate-x-[8px] translate-y-[8px] transition-all duration-300 group-hover:border-focused-border md:border-2 md:translate-x-[10px] md:translate-y-[10px]" }
                                    div { class: "relative flex w-full items-center justify-between gap-2 rounded-full border-2 border-secondary-2 bg-primary px-4 py-2 text-secondary-2 transition-all duration-300 ease-out group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] group-hover:bg-secondary-2 group-hover:text-primary md:gap-3 md:border-4 md:px-8 md:py-3",
                                        span { class: "flex items-center gap-2 md:gap-3",
                                            GithubIcon { width: 16, height: 16 }
                                            span { class: "font-black font-sans text-xs uppercase tracking-[0.12em] italic md:text-sm md:tracking-[0.2em]",
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
                                        class: "relative group block w-[calc(100%-8px)] mb-[8px] md:w-[calc(100%-10px)] md:mb-[10px]",
                                        div { class: "absolute inset-0 rounded-full border border-primary-6 bg-primary-1 translate-x-[8px] translate-y-[8px] transition-all duration-300 group-hover:border-focused-border md:border-2 md:translate-x-[10px] md:translate-y-[10px]" }
                                        div { class: "relative flex w-full items-center justify-between gap-2 rounded-full border-2 border-secondary-2 bg-primary px-4 py-2 text-secondary-2 transition-all duration-300 ease-out group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] group-hover:bg-secondary-2 group-hover:text-primary md:gap-3 md:border-4 md:px-8 md:py-3",
                                            span { class: "flex items-center gap-2 md:gap-3",
                                                HouseIcon { width: 16, height: 16 }
                                                span { class: "font-black font-sans text-xs uppercase tracking-[0.12em] italic md:text-sm md:tracking-[0.2em]",
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

                            if !repo_data.tags.is_empty() {
                                div { class: "flex flex-wrap gap-1.5 pt-1 md:gap-2",
                                    for tag in repo_data.tags.iter() {
                                        Link {
                                            key: "{tag.label}:{tag.value}",
                                            to: Route::RepoListView {
                                                tags: Some(tag.value.clone()),
                                                metric: None,
                                                range: None,
                                                page: None,
                                                size: None,
                                            },
                                            class: "group inline-flex items-center border border-primary-6 bg-primary-1 px-2 py-1 font-mono text-[10px] font-bold tracking-wider text-secondary-2 lowercase transition-colors hover:bg-grid-accent hover:text-primary-1 md:px-2.5 md:py-1.5 md:text-xs",
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
