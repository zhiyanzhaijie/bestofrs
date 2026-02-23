use dioxus::prelude::*;

use crate::IO::repos::list_repo_deltas;
use app::prelude::Pagination;

use super::{ChartJsCanvas, build_delta_chart_config, chart_dom_id, chart_min_width_px, short_date_label};

#[component]
pub fn DeltasSection(owner: String, name: String, refresh_tick: Signal<u32>) -> Element {
    let page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let deltas_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || {
            let _ = refresh_tick();
            list_repo_deltas(owner.clone(), name.clone(), page)
        }
    })?;

    rsx! {
        section { class: "space-y-4 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            h3 { class: "text-sm font-semibold text-secondary-4", "Daily deltas" }
            match deltas_fut() {
                Some(Ok(page)) => {
                    let labels = page
                        .items
                        .iter()
                        .map(|item| short_date_label(&item.snapshot_date))
                        .collect::<Vec<_>>();
                    let stars_deltas = page
                        .items
                        .iter()
                        .map(|item| item.stars_delta.unwrap_or(0))
                        .collect::<Vec<_>>();
                    let delta_chart_id = chart_dom_id(&owner, &name, "delta");
                    let delta_chart_width = chart_min_width_px(labels.len()) as u32;
                    let delta_config = build_delta_chart_config(labels, stars_deltas);

                    rsx! {
                        div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
                        if page.items.is_empty() {
                            div { class: "text-sm text-secondary-5", "No delta data" }
                        } else {
                            div { class: "space-y-3",
                                div { class: "border border-primary-6 bg-primary-1 p-3",
                                    ChartJsCanvas {
                                        chart_id: delta_chart_id,
                                        config: delta_config,
                                        height_px: 280,
                                        min_width_px: delta_chart_width,
                                    }
                                }

                                div { class: "max-h-[240px] space-y-2 overflow-auto",
                                    for d in page.items {
                                        div {
                                            key: "{d.snapshot_date}",
                                            class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm",
                                            div { class: "flex items-center justify-between",
                                                span { class: "font-medium", "{d.snapshot_date}" }
                                                span {
                                                    class: if d.stars_delta.unwrap_or(0) >= 0 { "text-green-600" } else { "text-red-600" },
                                                    if d.stars_delta.unwrap_or(0) >= 0 {
                                                        "+{d.stars_delta.unwrap_or(0)}"
                                                    } else {
                                                        "{d.stars_delta.unwrap_or(0)}"
                                                    }
                                                }
                                            }
                                            div { class: "text-xs text-secondary-5",
                                                "forks: {d.forks_delta:?} | issues: {d.open_issues_delta:?} | watchers: {d.watchers_delta:?}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading deltas..." } },
            }
        }
    }
}
