use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::common::IOCell;
use crate::components::icons::{
    ArrowRightIcon, CalendarDaysIcon, CircleDotIcon, GitForkIcon, RustDEVIcon, StarIcon,
};
use crate::impls::datetime::format_utc_ymd_hm;
use crate::root::Route;
use crate::types::repos::RepoDto;
use app::repo::{RepoRankMetric as RankType, RepoRankTimeRange as TimeRange};
use rank_panel_list::{skeleton::RankPanelListSkeleton, HomeRankPanelList};
use rank_tab_item::HomeRankTabItem;
use time_range_button::HomeTimeRangeButton;

mod rank_panel_list;
mod rank_tab_item;
mod repo_row;
mod time_range_button;

fn rank_metric_query(metric: RankType) -> &'static str {
    match metric {
        RankType::Star => "star",
        RankType::Fork => "fork",
        RankType::Issue => "issue",
        RankType::Recent => "recent",
    }
}

pub(super) fn stat_icon_tab(tab: RankType) -> Element {
    stat_icon_with_size(tab, 32)
}

pub(super) fn stat_icon_list(tab: RankType) -> Element {
    stat_icon_with_size(tab, 12)
}

pub(super) fn stat_icon_mobile_tab(tab: RankType) -> Element {
    stat_icon_with_size(tab, 20)
}

fn rank_range_query(range: TimeRange) -> &'static str {
    match range {
        TimeRange::Daily => "daily",
        TimeRange::Weekly => "weekly",
        TimeRange::Monthly => "monthly",
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct HomeRankTag {
    pub(super) label: String,
    pub(super) value: String,
}

#[derive(Clone, PartialEq, Eq)]
struct HomeRankRepo {
    id: String,
    name: String,
    description: String,
    tags: Vec<HomeRankTag>,
    avatar_url: String,
    stars: i64,
    forks: i64,
    issues: i64,
    created_at: String,
}

pub(super) fn row_border_style(index: usize) -> String {
    let accent = rainbow_color(index);
    format!(
        "border-left-color: color-mix(in oklab, {accent} 86%, var(--secondary-color-2));\
         box-shadow: inset 0 0 0 1px color-mix(in oklab, {accent} 22%, var(--primary-color-6));\
         background: linear-gradient(90deg, color-mix(in oklab, {accent} 8%, var(--primary-color)) 0%, var(--primary-color) 28%);"
    )
}

pub(super) fn rainbow_color(index: usize) -> &'static str {
    match index % 7 {
        0 => "#ef4444",
        1 => "#f97316",
        2 => "#eab308",
        3 => "#22c55e",
        4 => "#3b82f6",
        5 => "#6366f1",
        _ => "#a855f7",
    }
}

#[component]
pub(super) fn HomeRankPanel() -> Element {
    let mut active_tab = use_signal(|| RankType::Star);
    let mut time_range = use_signal(|| TimeRange::Daily);

    rsx! {
        div { class: "relative z-10 flex min-h-0 flex-col overflow-visible rounded-3xl border-2 border-primary-6 bg-primary shadow-2xl transition-colors duration-300 md:rounded-[3rem] lg:min-h-[600px] lg:flex-row lg:rounded-[3.5rem] lg:border-x-4",
            div { class: "flex w-full flex-col self-stretch overflow-hidden rounded-t-[calc(1.5rem-2px)] border-b border-primary-6 bg-primary p-3 md:rounded-t-[calc(3rem-2px)] md:p-4 lg:w-[260px] lg:flex-none lg:rounded-t-none lg:rounded-l-[calc(3.5rem-4px)] lg:border-b-0 lg:border-r",
                div { class: "mb-2 flex items-center justify-center gap-2 p-2 text-center md:p-4",
                    RustDEVIcon {  width: 40.0, height: 40.0 }
                    div {
                        h3 { class: "font-sans text-xl font-black uppercase tracking-tighter text-secondary-1 md:text-2xl",
                            "Ranking"
                        }
                        p { class: "mt-1 text-[9px] font-bold font-mono uppercase tracking-widest text-secondary-6 md:text-[10px]",
                            "in metri"
                        }
                    }
                }
                div { class: "grid flex-grow grid-cols-4 gap-2 overflow-visible md:flex md:flex-col",
                    for tab in [RankType::Star, RankType::Fork, RankType::Issue, RankType::Recent] {
                        HomeRankTabItem {
                            tab,
                            active_tab: active_tab(),
                            on_select: move |_| active_tab.set(tab),
                        }
                    }
                }
                div { class: "mt-2 hidden border-t border-primary-6 p-2 md:mt-auto md:block md:p-6",
                    div { class: "flex items-center gap-2 text-[8px] font-mono text-secondary-5 uppercase tracking-widest",
                        div { class: "h-2 w-2 rounded-full bg-secondary-6 animate-pulse" }
                        {t!("view_home_rank_panel_status_active")}
                    }
                }
            }
            div { class: "mx-2 my-3 flex min-w-0 flex-1 flex-col self-stretch rounded-2xl bg-primary-1/60 px-3 py-3 md:mx-4 md:my-5 md:rounded-[2.2rem] md:px-5 lg:my-6 lg:rounded-[2.5rem] lg:px-6",
                div { class: "mb-4 flex flex-col items-stretch gap-3 border-b-2 border-primary-6 pb-5 md:mb-6 md:items-start md:gap-4 md:pb-8 xl:flex-row xl:items-center xl:justify-between",
                    div { class: "w-full md:flex md:flex-wrap md:items-center md:gap-6",
                        if active_tab() != RankType::Recent {
                            div { class: "grid w-full grid-cols-3 gap-2 md:flex md:flex-wrap md:items-center md:gap-4",
                                for range in [TimeRange::Daily, TimeRange::Weekly, TimeRange::Monthly] {
                                    HomeTimeRangeButton {
                                        range,
                                        active: time_range() == range,
                                        onclick: move |_| time_range.set(range),
                                    }
                                }
                            }
                        } else {
                            div { class: "relative group",
                                div { class: "absolute inset-0 translate-x-[6px] translate-y-[6px] rounded-md border border-primary-6 bg-primary-1 md:translate-x-[10px] md:translate-y-[10px] md:rounded-full md:border-2" }
                                div { class: "relative translate-x-[2px] translate-y-[2px] rounded-md border-2 border-secondary-2 bg-secondary-2 px-3 py-1.5 font-sans text-[10px] font-black italic uppercase tracking-[0.1em] text-primary shadow-[0_0_14px_color-mix(in_oklab,var(--grid-accent)_20%,transparent)] md:translate-x-[3.82px] md:translate-y-[3.82px] md:rounded-full md:border-4 md:px-8 md:py-3 md:text-sm md:tracking-[0.2em] md:shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_24%,transparent)]",
                                    {t!("view_home_rank_panel_recent_badge")}
                                }
                            }
                        }
                    }
                    div { class: "w-full md:w-auto",
                        Link {
                            to: Route::RepoListView {
                                tags: None,
                                metric: Some(rank_metric_query(active_tab()).to_string()),
                                range: Some(rank_range_query(time_range()).to_string()),
                                page: None,
                                size: None,
                            },
                            class: "relative group block w-full md:inline-block md:w-auto",
                            div { class: "absolute inset-0 translate-x-[8px] translate-y-[8px] rounded-full border-2 border-primary-6 bg-primary-1 transition-all duration-300 group-hover:border-focused-border md:translate-x-[10px] md:translate-y-[10px]" }
                            div { class: "relative flex w-full items-center justify-center gap-2 rounded-full border-4 border-secondary-2 bg-primary px-4 py-2 text-secondary-2 transition-all duration-300 ease-out group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] md:w-auto md:gap-3 md:px-8 md:py-3",
                                ArrowRightIcon {  }
                            }
                        }
                    }
                }
                IOCell {
                    loading_fallback: rsx! {
                        RankPanelListSkeleton {}
                    },
                    HomeRankPanelList { active_tab, time_range }
                }
            }
        }
    }
}

pub(super) fn map_rank_repo(repo: RepoDto) -> HomeRankRepo {
    let name = repo.full_name.clone().unwrap_or_else(|| repo.id.clone());
    let description = repo
        .description
        .clone()
        .unwrap_or_else(|| t!("view_home_rank_panel_no_description").to_string());
    let avatar_url = repo
        .avatar_url
        .clone()
        .or_else(|| repo.avatar_urls.first().cloned())
        .unwrap_or_else(|| fallback_avatar(&repo.id));

    HomeRankRepo {
        id: repo.id,
        name,
        description,
        tags: repo
            .tags
            .iter()
            .map(|tag| HomeRankTag {
                label: if tag.label.is_empty() {
                    tag.value.clone()
                } else {
                    tag.label.clone()
                },
                value: tag.value.clone(),
            })
            .collect(),
        avatar_url,
        stars: repo.stars,
        forks: repo.forks,
        issues: repo.open_issues,
        created_at: repo.created_at,
    }
}

fn fallback_avatar(repo_id: &str) -> String {
    if let Some((owner, _)) = repo_id.split_once('/') {
        if !owner.is_empty() {
            return format!("https://github.com/{owner}.png");
        }
    }
    "https://github.com/github.png".to_string()
}

pub(super) fn parse_repo_route(repo_id: &str, metric_tab: RankType) -> Option<Route> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    let metric = match metric_tab {
        RankType::Star => Some("star".to_string()),
        RankType::Fork => Some("fork".to_string()),
        RankType::Issue => Some("issue".to_string()),
        RankType::Recent => None,
    };
    Some(Route::RepoDetailView {
        owner: owner.to_string(),
        name: name.to_string(),
        metric,
    })
}

pub(super) fn rank_title(tab: RankType) -> String {
    match tab {
        RankType::Star => "star".to_string(),
        RankType::Fork => "fork".to_string(),
        RankType::Issue => "issue".to_string(),
        RankType::Recent => t!("view_home_rank_panel_metric_recent").to_string(),
    }
}

pub(super) fn rank_desc(tab: RankType) -> String {
    match tab {
        RankType::Star => t!("view_home_rank_panel_desc_star").to_string(),
        RankType::Fork => t!("view_home_rank_panel_desc_fork").to_string(),
        RankType::Issue => t!("view_home_rank_panel_desc_issue").to_string(),
        RankType::Recent => t!("view_home_rank_panel_desc_recent").to_string(),
    }
}

pub(super) fn time_range_text(range: TimeRange) -> String {
    match range {
        TimeRange::Daily => t!("view_home_rank_panel_range_daily").to_string(),
        TimeRange::Weekly => t!("view_home_rank_panel_range_weekly").to_string(),
        TimeRange::Monthly => t!("view_home_rank_panel_range_monthly").to_string(),
    }
}

pub(super) fn stat_value(repo: &HomeRankRepo, tab: RankType) -> String {
    match tab {
        RankType::Star => repo.stars.to_string(),
        RankType::Fork => repo.forks.to_string(),
        RankType::Issue => repo.issues.to_string(),
        RankType::Recent => format_utc_ymd_hm(&repo.created_at),
    }
}

fn stat_icon_with_size(tab: RankType, size: u32) -> Element {
    match tab {
        RankType::Star => rsx! { StarIcon { width: size, height: size } },
        RankType::Fork => rsx! { GitForkIcon { width: size, height: size } },
        RankType::Issue => rsx! { CircleDotIcon { width: size, height: size } },
        RankType::Recent => rsx! { CalendarDaysIcon { width: size, height: size } },
    }
}
