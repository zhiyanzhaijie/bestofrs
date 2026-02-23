use dioxus::prelude::*;

use crate::IO::repos::list_repo_snapshots;
use app::prelude::Pagination;

use super::{ChartJsCanvas, build_trend_chart_config, chart_dom_id, chart_min_width_px, short_date_label};

#[component]
pub fn SnapshotSection(owner: String, name: String, refresh_tick: Signal<u32>) -> Element {
    let page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let snapshots_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || {
            let _ = refresh_tick();
            list_repo_snapshots(owner.clone(), name.clone(), page)
        }
    })?;

    rsx! {
        section { class: "space-y-5 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                h2 { class: "text-lg font-semibold", "Trend" }
                p { class: "text-sm text-secondary-5", "Snapshots timeline" }
            }

            match snapshots_fut() {
                Some(Ok(page)) => {
                    let labels = page
                        .items
                        .iter()
                        .map(|item| short_date_label(&item.snapshot_date))
                        .collect::<Vec<_>>();
                    let stars_series = page.items.iter().map(|item| item.stars).collect::<Vec<_>>();
                    let forks_series = page.items.iter().map(|item| item.forks).collect::<Vec<_>>();
                    let issues_series = page.items.iter().map(|item| item.open_issues).collect::<Vec<_>>();
                    let watchers_series = page.items.iter().map(|item| item.watchers).collect::<Vec<_>>();

                    let trend_chart_id = chart_dom_id(&owner, &name, "trend");
                    let trend_chart_width = chart_min_width_px(labels.len()) as u32;
                    let trend_config = build_trend_chart_config(
                        labels,
                        stars_series,
                        forks_series,
                        issues_series,
                        watchers_series,
                    );

                    rsx! {
                        div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
                        if page.items.is_empty() {
                            div { class: "text-sm text-secondary-5", "No snapshot data" }
                        } else {
                            div { class: "space-y-3",
                                div { class: "border border-primary-6 bg-primary-1 p-3",
                                    ChartJsCanvas {
                                        chart_id: trend_chart_id,
                                        config: trend_config,
                                        height_px: 320,
                                        min_width_px: trend_chart_width,
                                    }
                                }

                                div { class: "max-h-[240px] space-y-2 overflow-auto",
                                    for s in page.items {
                                        div {
                                            key: "{s.snapshot_date}",
                                            class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm",
                                            div { class: "font-medium", "{s.snapshot_date}" }
                                            div { class: "text-xs text-secondary-5",
                                                "stars: {s.stars} | forks: {s.forks} | issues: {s.open_issues} | watchers: {s.watchers}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading timeline..." } },
            }
        }
    }
}
