use dioxus::prelude::*;
use std::collections::BTreeMap;

use super::{apply_metric_visibility, TrendContext};
use crate::types::snapshots::SnapshotDto;
use crate::IO::repos::list_repo_snapshots_in_duration;
use app::prelude::{DurationRange, Page};

use super::super::{
    build_trend_chart_config, chart_dom_id, short_date_label, ChartJsCanvas, RepoDetailContext,
};

pub(super) mod skeleton;

#[component]
pub(crate) fn SnapshotContent() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let snapshot_timeframe = trend_ctx.snapshot_timeframe;
    let monthly_fut = use_server_future(move || {
        list_repo_snapshots_in_duration((ctx.owner)(), (ctx.name)(), DurationRange::Monthly)
    })?;
    let yearly_fut = use_server_future(move || {
        list_repo_snapshots_in_duration((ctx.owner)(), (ctx.name)(), DurationRange::Yearly)
    })?;

    let selected = if snapshot_timeframe() == "yearly" {
        yearly_fut()
    } else {
        monthly_fut()
    };

    match selected {
        Some(Ok(page)) => rsx! { SnapshotChartContent { page: page.clone() } },
        Some(Err(e)) => Err(e)?,
        None => rsx! { skeleton::SnapshotContentSkeleton {} },
    }
}

#[component]
fn SnapshotChartContent(page: Page<SnapshotDto>) -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let metric = trend_ctx.metric;
    let snapshot_timeframe = trend_ctx.snapshot_timeframe;
    let active_tab = trend_ctx.active_tab;

    let chart_id_memo = use_memo({
        let owner = ctx.owner;
        let name = ctx.name;
        move || chart_dom_id(&owner(), &name(), "trend")
    });

    let chart_config_memo = use_memo({
        let page = page.clone();
        let metric = metric;
        let snapshot_timeframe = snapshot_timeframe;
        move || {
            let current_metric = metric();
            let current_timeframe = snapshot_timeframe();
            let sorted_items = page.items.clone();
            let (labels, stars_series, forks_series, issues_series, watchers_series) =
                if current_timeframe == "yearly" {
                    let mut groups: BTreeMap<String, (i64, i64, i64, i64, i64)> = BTreeMap::new();
                    for item in &sorted_items {
                        let month_key = item
                            .snapshot_date
                            .get(0..7)
                            .unwrap_or(&item.snapshot_date)
                            .to_string();
                        let entry = groups.entry(month_key).or_insert((0, 0, 0, 0, 0));
                        entry.0 += item.stars;
                        entry.1 += item.forks;
                        entry.2 += item.open_issues;
                        entry.3 += item.watchers;
                        entry.4 += 1;
                    }
                    let grouped = groups.into_iter().collect::<Vec<_>>();
                    let start = grouped.len().saturating_sub(12);
                    let recent = grouped[start..].to_vec();
                    let labels = recent
                        .iter()
                        .map(|(month, _)| month.clone())
                        .collect::<Vec<_>>();
                    let stars_series = recent
                        .iter()
                        .map(|(_, (stars, _, _, _, count))| stars / *count)
                        .collect::<Vec<_>>();
                    let forks_series = recent
                        .iter()
                        .map(|(_, (_, forks, _, _, count))| forks / *count)
                        .collect::<Vec<_>>();
                    let issues_series = recent
                        .iter()
                        .map(|(_, (_, _, issues, _, count))| issues / *count)
                        .collect::<Vec<_>>();
                    let watchers_series = recent
                        .iter()
                        .map(|(_, (_, _, _, watchers, count))| watchers / *count)
                        .collect::<Vec<_>>();
                    (
                        labels,
                        stars_series,
                        forks_series,
                        issues_series,
                        watchers_series,
                    )
                } else {
                    let labels = sorted_items
                        .iter()
                        .map(|item| short_date_label(&item.snapshot_date))
                        .collect::<Vec<_>>();
                    let stars_series = sorted_items
                        .iter()
                        .map(|item| item.stars)
                        .collect::<Vec<_>>();
                    let forks_series = sorted_items
                        .iter()
                        .map(|item| item.forks)
                        .collect::<Vec<_>>();
                    let issues_series = sorted_items
                        .iter()
                        .map(|item| item.open_issues)
                        .collect::<Vec<_>>();
                    let watchers_series = sorted_items
                        .iter()
                        .map(|item| item.watchers)
                        .collect::<Vec<_>>();
                    (
                        labels,
                        stars_series,
                        forks_series,
                        issues_series,
                        watchers_series,
                    )
                };

            let config = build_trend_chart_config(
                labels,
                stars_series,
                forks_series,
                issues_series,
                watchers_series,
            );
            apply_metric_visibility(config, &current_metric, "snapshot")
        }
    });

    let id: ReadSignal<String> = chart_id_memo.into();
    let chart_config: ReadSignal<serde_json::Value> = chart_config_memo.into();
    let is_active = use_memo(move || active_tab().as_deref() == Some("snapshot"));
    let is_active: ReadSignal<bool> = is_active.into();

    rsx! {
        div { class: "flex h-full flex-col gap-2",
            if page.items.is_empty() {
                div { class: "text-sm text-secondary-5", "No snapshot data" }
            } else {
                div { class: "min-h-0 flex-1 border border-primary-6 bg-primary-1 p-3",
                    ChartJsCanvas {
                        id,
                        config: chart_config,
                        active: is_active,
                    }
                }
            }
        }
    }
}
