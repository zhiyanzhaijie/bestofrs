mod deltas;
mod readme;
mod snapshot;

use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, IOCell, RepoAvatar,
};
use crate::components::icons::{GithubIcon, HouseIcon, StarIcon, TagsIcon, UsersRoundIcon};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::ui::avatar::AvatarImageSize;
use crate::IO::repos::get_repo;
use dioxus::prelude::*;
use dioxus_use_js::{use_js, JsValue};
use serde_json::{json, Value};

pub use deltas::DeltasSection;
pub use readme::ReadmeSection;
pub use snapshot::SnapshotSection;

use_js!("src/js/chart_bridge.js"::{create_chart, update_chart});

#[component]
pub fn RepoDetail(owner: String, name: String) -> Element {
    let route_key = format!("{owner}/{name}");
    rsx! { RepoDetailPageContent { key: "{route_key}", owner, name } }
}

#[component]
fn RepoDetailPageContent(owner: String, name: String) -> Element {
    let navigator = use_navigator();
    let mut refresh_tick = use_signal(|| 0u32);
    let mut active_tab = use_signal(|| Some("deltas".to_string()));
    let active_tab_read: ReadSignal<Option<String>> = active_tab.into();

    let repo_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || {
            let _ = refresh_tick();
            get_repo(owner.clone(), name.clone())
        }
    })?;

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
        div { class: "space-y-0",
            GridWrapper {
                grid_type: GridType::Default,
                padding: GridPadding::Lg,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Dot,
                    gradient: GradientDirection::ToBottom,
                },
                section { class: "relative min-h-80 overflow-hidden",
                    div { class: "relative z-10 flex items-center justify-start pb-4",
                        div { class: "flex items-center gap-2",
                            button {
                                class: "border border-primary-6 bg-primary-1 px-4 py-2 text-sm text-secondary-5 transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                                onclick: move |_| navigator.go_back(),
                                "返回"
                            }
                            button {
                                class: "border border-secondary-2 bg-secondary-2 px-4 py-2 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                                onclick: move |_| refresh_tick.with_mut(|v| *v += 1),
                                "刷新"
                            }
                        }
                    }

                    div { class: "relative z-10 flex h-full flex-col gap-6 md:flex-row md:items-stretch md:justify-between",
                        div { class: "flex min-w-0 flex-1 items-start gap-6",
                            // Avatar
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
                                    div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "REPOSITORY / DETAIL" }
                                    h1 { class: "truncate text-3xl font-bold tracking-tight text-secondary-2 md:text-5xl", "{owner}/{name}" }
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
                                            span { class: "font-semibold text-secondary-2", "{date}" }
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
            GridSlashTransition {}
            GridWrapper { padding: GridPadding::Sm,
                section { class: "space-y-8 bg-primary-1",

                    Tabs {
                        class: "space-y-4".to_string(),
                        value: active_tab_read,
                        default_value: "deltas".to_string(),
                        on_value_change: move |value| active_tab.set(Some(value)),
                        TabList {
                            TabTrigger { value: "deltas".to_string(), index: 0usize, "Deltas" }
                            TabTrigger { value: "snapshot".to_string(), index: 1usize, "Snapshot" }
                        }
                        TabContent {
                            value: "deltas".to_string(),
                            index: 0usize,
                            IOCell {
                                DeltasSection {
                                    owner: owner.clone(),
                                    name: name.clone(),
                                    refresh_tick,
                                }
                            }
                        }
                        TabContent {
                            value: "snapshot".to_string(),
                            index: 1usize,
                            IOCell {
                                SnapshotSection {
                                    owner: owner.clone(),
                                    name: name.clone(),
                                    refresh_tick,
                                }
                            }
                        }
                    }

                    IOCell {
                        ReadmeSection {
                            owner,
                            name,
                            refresh_tick,
                        }
                    }
                }
            }
        }
    }
}

fn chart_dom_id(owner: &str, name: &str, suffix: &str) -> String {
    let owner = owner
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    let name = name
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    format!("repo-{owner}-{name}-{suffix}")
}

fn chart_min_width_px(points: usize) -> usize {
    (points.max(12) * 56).clamp(720, 3600)
}

fn short_date_label(input: &str) -> String {
    let parts = input.split('-').collect::<Vec<_>>();
    if parts.len() >= 3 {
        return format!("{}-{}", parts[1], parts[2]);
    }
    input.to_string()
}

fn build_trend_chart_config(
    labels: Vec<String>,
    stars_series: Vec<i64>,
    forks_series: Vec<i64>,
    issues_series: Vec<i64>,
    watchers_series: Vec<i64>,
) -> Value {
    json!({
        "type": "line",
        "data": {
            "labels": labels,
            "datasets": [
                {
                    "label": "stars",
                    "data": stars_series,
                    "borderColor": "#2563eb",
                    "backgroundColor": "rgba(37, 99, 235, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                },
                {
                    "label": "forks",
                    "data": forks_series,
                    "borderColor": "#7c3aed",
                    "backgroundColor": "rgba(124, 58, 237, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                },
                {
                    "label": "open_issues",
                    "data": issues_series,
                    "borderColor": "#f59e0b",
                    "backgroundColor": "rgba(245, 158, 11, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                },
                {
                    "label": "watchers",
                    "data": watchers_series,
                    "borderColor": "#10b981",
                    "backgroundColor": "rgba(16, 185, 129, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                }
            ]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "interaction": { "mode": "index", "intersect": false },
            "plugins": {
                "legend": { "position": "bottom" },
                "tooltip": { "mode": "index", "intersect": false }
            },
            "scales": {
                "x": {
                    "ticks": {
                        "autoSkip": true,
                        "maxTicksLimit": 12,
                        "maxRotation": 45
                    }
                },
                "y": {
                    "ticks": {
                        "maxTicksLimit": 8
                    }
                }
            }
        }
    })
}

fn build_delta_chart_config(
    labels: Vec<String>,
    stars_deltas: Vec<i64>,
    forks_deltas: Vec<i64>,
    issues_deltas: Vec<i64>,
) -> Value {
    json!({
        "type": "bar",
        "data": {
            "labels": labels,
            "datasets": [
                {
                    "label": "stars_delta",
                    "data": stars_deltas,
                    "backgroundColor": "rgba(16, 185, 129, 0.7)",
                    "borderColor": "#10b981",
                    "borderWidth": 1
                },
                {
                    "label": "forks_delta",
                    "data": forks_deltas,
                    "backgroundColor": "rgba(124, 58, 237, 0.7)",
                    "borderColor": "#7c3aed",
                    "borderWidth": 1
                },
                {
                    "label": "open_issues_delta",
                    "data": issues_deltas,
                    "backgroundColor": "rgba(245, 158, 11, 0.7)",
                    "borderColor": "#f59e0b",
                    "borderWidth": 1
                }
            ]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "interaction": { "mode": "index", "intersect": false },
            "plugins": {
                "legend": { "position": "bottom" },
                "tooltip": { "mode": "index", "intersect": false }
            },
            "scales": {
                "x": {
                    "ticks": {
                        "autoSkip": true,
                        "maxTicksLimit": 14,
                        "maxRotation": 45
                    }
                },
                "y": {
                    "ticks": {
                        "maxTicksLimit": 8
                    }
                }
            }
        }
    })
}

#[component]
fn ChartJsCanvas(chart_id: String, config: Value, height_px: u32, min_width_px: u32) -> Element {
    let chart_ref = use_signal(|| None::<JsValue>);
    let last_config_key = use_signal(String::new);
    let chart_id_for_effect = chart_id.clone();

    use_effect(move || {
        let chart_id = chart_id_for_effect.clone();
        let config = config.clone();
        let config_key = config.to_string();
        let mut chart_ref = chart_ref;
        let mut last_config_key = last_config_key;

        spawn(async move {
            if let Some(chart) = chart_ref() {
                if last_config_key() == config_key {
                    return;
                }
                if let Err(err) = update_chart::<()>(&chart, &config).await {
                    error!("failed to update chart: {err}");
                } else {
                    last_config_key.set(config_key);
                }
                return;
            }
            match create_chart(&chart_id, &config).await {
                Ok(chart) => {
                    chart_ref.set(Some(chart));
                    last_config_key.set(config_key);
                }
                Err(err) => error!("failed to create chart: {err}"),
            }
        });
    });

    rsx! {
        div { class: "overflow-x-auto",
            div {
                class: "min-w-full",
                style: "min-width: {min_width_px}px; height: {height_px}px;",
                canvas {
                    id: "{chart_id}",
                    style: "width: 100%; height: 100%;",
                }
            }
        }
    }
}
