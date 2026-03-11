use dioxus::prelude::*;

use crate::components::common::IOCell;
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

fn rank_range_query(range: TimeRange) -> &'static str {
    match range {
        TimeRange::Daily => "daily",
        TimeRange::Weekly => "weekly",
        TimeRange::Monthly => "monthly",
    }
}

#[derive(Clone, PartialEq, Eq)]
struct HomeRankRepo {
    id: String,
    name: String,
    description: String,
    tags: Vec<String>,
    avatar_url: String,
    stars: i64,
    forks: i64,
    issues: i64,
    updated_at: String,
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
    let mut time_range = use_signal(|| TimeRange::Weekly);

    rsx! {
        div { class: "bg-primary border border-2 border-x-4 border-primary-6 shadow-2xl rounded-[3.5rem] overflow-hidden flex flex-col lg:flex-row min-h-[600px] transition-colors duration-300 relative z-10",
            div { class: "w-full lg:w-[260px] flex flex-col bg-primary border-r border-primary-6 self-stretch p-4",
                div { class: "p-4 mb-2",
                    h3 { class: "text-2xl font-black font-sans uppercase tracking-tighter italic text-secondary-1", "Rankings" }
                    p { class: "text-[10px] font-mono text-secondary-6 uppercase tracking-widest mt-1 font-bold", "Metric_Select" }
                }
                div { class: "flex flex-col flex-grow gap-2 overflow-hidden",
                    for (idx, tab) in [RankType::Star, RankType::Fork, RankType::Issue, RankType::Recent].into_iter().enumerate() {
                        HomeRankTabItem {
                            idx,
                            tab,
                            active_tab: active_tab(),
                            on_select: move |_| active_tab.set(tab),
                        }
                    }
                }
                div { class: "mt-auto p-6 border-t border-primary-6",
                    div { class: "flex items-center gap-2 text-[8px] font-mono text-secondary-5 uppercase tracking-widest",
                        div { class: "w-1 h-1 bg-secondary-6 rounded-full animate-pulse" }
                        "Active"
                    }
                }
            }
            div { class: "w-full lg:flex-grow px-5 md:px-6 bg-primary-1/60 flex flex-col self-stretch mx-4 my-6 rounded-[2.5rem]",
                div { class: "flex flex-col xl:flex-row items-start xl:items-center justify-between mb-6 pb-4 border-b-2 border-primary-6 gap-4",
                    div { class: "flex flex-wrap items-center gap-6",
                        if active_tab() != RankType::Recent {
                            for range in [TimeRange::Daily, TimeRange::Weekly, TimeRange::Monthly] {
                                HomeTimeRangeButton {
                                    range,
                                    active: time_range() == range,
                                    onclick: move |_| time_range.set(range),
                                }
                            }
                        } else {
                            div { class: "relative group",
                                div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px]" }
                                div { class: "relative px-8 py-3 rounded-full text-sm font-black font-sans uppercase tracking-[0.2em] italic bg-secondary-2 text-primary border-4 border-secondary-2 translate-x-[3.82px] translate-y-[3.82px] shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_24%,transparent)]",
                                    "Latest_Transmissions"
                                }
                            }
                        }
                    }
                    Link {
                        to: Route::RepoListView {
                            tags: None,
                            metric: Some(rank_metric_query(active_tab()).to_string()),
                            range: Some(rank_range_query(time_range()).to_string()),
                            page: None,
                            size: None,
                        },
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "View_All" }
                            span { class: "group-hover:translate-x-1 transition-transform", "→" }
                        }
                    }
                }
                IOCell {
                    loading_fallback: rsx! { RankPanelListSkeleton {} },
                    HomeRankPanelList {
                        active_tab,
                        time_range,
                    }
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
        .unwrap_or_else(|| "No description".to_string());
    let avatar_url = repo
        .avatar_url
        .clone()
        .or_else(|| repo.avatar_urls.first().cloned())
        .unwrap_or_else(|| fallback_avatar(&repo.id));
    let updated_at = repo
        .last_fetched_at
        .clone()
        .unwrap_or_else(|| "1970-01-01".to_string());
    HomeRankRepo {
        id: repo.id,
        name,
        description,
        tags: repo
            .tags
            .iter()
            .map(|tag| {
                if tag.label.is_empty() {
                    tag.value.clone()
                } else {
                    tag.label.clone()
                }
            })
            .collect(),
        avatar_url,
        stars: repo.stars,
        forks: repo.forks,
        issues: repo.open_issues,
        updated_at,
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

pub(super) fn parse_repo_route(repo_id: &str) -> Option<Route> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some(Route::RepoDetailView {
        owner: owner.to_string(),
        name: name.to_string(),
    })
}

pub(super) fn rank_title(tab: RankType) -> &'static str {
    match tab {
        RankType::Star => "stars",
        RankType::Fork => "forks",
        RankType::Issue => "issues",
        RankType::Recent => "Recent",
    }
}

pub(super) fn rank_desc(tab: RankType) -> &'static str {
    match tab {
        RankType::Star => {
            "Highest community validation and visibility. The gold standard of Rust excellence."
        }
        RankType::Fork => {
            "Most active foundations for extension. High-utility codebases built for growth."
        }
        RankType::Issue => {
            "High-velocity development environments. Active problem-solving and iteration."
        }
        RankType::Recent => {
            "Freshly tracked transmissions. The latest additions to the manuscript archive."
        }
    }
}

pub(super) fn time_range_text(range: TimeRange) -> &'static str {
    match range {
        TimeRange::Daily => "daily",
        TimeRange::Weekly => "weekly",
        TimeRange::Monthly => "monthly",
    }
}

pub(super) fn stat_value(repo: &HomeRankRepo, tab: RankType) -> String {
    match tab {
        RankType::Star => repo.stars.to_string(),
        RankType::Fork => repo.forks.to_string(),
        RankType::Issue => repo.issues.to_string(),
        RankType::Recent => short_date(&repo.updated_at),
    }
}

fn short_date(value: &str) -> String {
    let cutoff = value.find('T').unwrap_or(value.len());
    value[..cutoff].to_string()
}

pub(super) fn stat_icon(tab: RankType) -> &'static str {
    match tab {
        RankType::Star => "★",
        RankType::Fork => "⑂",
        RankType::Issue => "!",
        RankType::Recent => "↻",
    }
}
