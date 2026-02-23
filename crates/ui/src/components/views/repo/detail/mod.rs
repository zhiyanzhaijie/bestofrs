mod deltas;
mod meta;
mod readme;
mod snapshot;

use dioxus::prelude::*;
use dioxus_use_js::{JsValue, use_js};
use serde_json::{Value, json};
use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, IOCell,
};
use crate::IO::repos::get_repo;

pub use deltas::DeltasSection;
pub use meta::RepoMetaSection;
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

    let repo_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || {
            let _ = refresh_tick();
            get_repo(owner.clone(), name.clone())
        }
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
                    div { class: "relative z-10 flex flex-col gap-6 md:flex-row md:items-end md:justify-between",
                        div { class: "space-y-2 min-w-0",
                            div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "REPOSITORY / DETAIL" }
                            h1 { class: "truncate text-3xl font-bold tracking-tight text-secondary-2 md:text-5xl", "{owner}/{name}" }
                            p { class: "text-sm text-secondary-5", "Snapshot trends, deltas and README." }
                        }

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
                }
            }
            GridSlashTransition {}
            GridWrapper { padding: GridPadding::Sm,
                section { class: "space-y-8 bg-primary-1",

                    RepoMetaSection {
                        repo_fut,
                        owner: owner.clone(),
                        name: name.clone(),
                    }

                    IOCell {
                        SnapshotSection {
                            owner: owner.clone(),
                            name: name.clone(),
                            refresh_tick,
                        }
                    }

                    IOCell {
                        DeltasSection {
                            owner: owner.clone(),
                            name: name.clone(),
                            refresh_tick,
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

fn build_delta_chart_config(labels: Vec<String>, stars_deltas: Vec<i64>) -> Value {
    let delta_colors = stars_deltas
        .iter()
        .map(|value| {
            if *value >= 0 {
                "rgba(16, 185, 129, 0.7)"
            } else {
                "rgba(239, 68, 68, 0.7)"
            }
        })
        .collect::<Vec<_>>();

    json!({
        "type": "bar",
        "data": {
            "labels": labels,
            "datasets": [
                {
                    "label": "stars_delta",
                    "data": stars_deltas,
                    "backgroundColor": delta_colors,
                    "borderWidth": 0
                }
            ]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "plugins": {
                "legend": { "display": false }
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
