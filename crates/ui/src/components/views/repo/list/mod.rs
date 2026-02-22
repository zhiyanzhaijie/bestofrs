use dioxus::prelude::*;

use crate::components::common::{
    CommonPagination, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper, IOCell, RepoManuscriptCard,
};
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use crate::IO::repos::list_repos;
use app::prelude::Pagination as PageQuery;

#[component]
pub fn RepoList() -> Element {
    let mut page_size = use_signal(|| 50u32);
    let mut current_page = use_signal(|| 1u32);

    rsx! {
        div { class: "space-y-0",
            GridWrapper {
                grid_type: GridType::Default,
                padding: GridPadding::Sm,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Grid,
                    gradient: GradientDirection::ToBottom,
                },
                section { class: "relative overflow-hidden bg-transparent h-120",
                    div { class: "relative z-10 space-y-8",
                        div { class: "inline-flex items-center gap-2 border border-primary-6 bg-transparent px-2 py-1 font-mono text-xs font-semibold tracking-wide text-secondary-5",
                            "VOL. 2026 / ISSUE #01"
                        }
                        div { class: "flex flex-col gap-6 lg:flex-row lg:items-end lg:justify-between",
                            div { class: "max-w-3xl space-y-4",
                                h1 { class: "text-4xl font-bold tracking-tight text-secondary-2 md:text-6xl",
                                    "Weekly "
                                    span { class: "text-secondary-6", "Curated" }
                                    " List"
                                }
                                p { class: "border-l-2 border-primary-6 pl-5 text-base leading-relaxed text-secondary-4",
                                    "Observing the fastest growing projects in the Rust ecosystem. Data aggregated from GitHub activity and filtered for quality."
                                }
                            }
                            div { class: "flex flex-wrap items-center gap-3",
                                button { class: "border border-primary-6 bg-primary-1 px-5 py-2.5 text-sm font-medium text-secondary-4 transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                                    "Filter"
                                }
                                button { class: "border border-secondary-2 bg-secondary-2 px-5 py-2.5 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                                    "Sort: Popular"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-wrap items-center justify-between gap-3 border-b border-dashed border-primary-6 pb-4",
                        div { class: "text-sm font-mono tracking-wider text-secondary-5",
                            "INDEX / REPOSITORIES"
                        }
                        div { class: "flex items-center gap-3",
                            span { class: "text-sm font-medium text-secondary-5", "page size" }
                            Select::<u32> {
                                value: Some(page_size()),
                                placeholder: "page size",
                                on_value_change: move |v: Option<u32>| {
                                    if let Some(v) = v {
                                        page_size.set(v);
                                        current_page.set(1);
                                    }
                                },
                                SelectTrigger {
                                    aria_label: "Select page size",
                                    style: "min-width: 7rem;",
                                    SelectValue {}
                                }
                                SelectList { aria_label: "Page size options",
                                    SelectGroup {
                                        SelectGroupLabel { "Page size" }
                                        SelectOption::<u32> {
                                            index: 0usize,
                                            value: 20u32,
                                            text_value: Some("20".to_string()),
                                            "20"
                                            SelectItemIndicator {}
                                        }
                                        SelectOption::<u32> {
                                            index: 1usize,
                                            value: 50u32,
                                            text_value: Some("50".to_string()),
                                            "50"
                                            SelectItemIndicator {}
                                        }
                                        SelectOption::<u32> {
                                            index: 2usize,
                                            value: 100u32,
                                            text_value: Some("100".to_string()),
                                            "100"
                                            SelectItemIndicator {}
                                        }
                                    }
                                }
                            }
                        }
                    }

                }
            }

            GridSlashTransition {  }

            GridWrapper {
                padding: GridPadding::Sm,
                section { class: "space-y-6 bg-primary-1",
                    IOCell {
                        RepoListIO { page_size, current_page }
                    }
                }
            }
        }
    }
}

#[component]
fn RepoListIO(mut page_size: Signal<u32>, mut current_page: Signal<u32>) -> Element {
    let repos = use_server_future(move || {
        let limit = page_size();
        let page = current_page().max(1);
        list_repos(PageQuery {
            limit: Some(limit),
            offset: Some(limit.saturating_mul(page.saturating_sub(1))),
        })
    })?;

    match repos() {
        Some(Ok(page)) => {
            let meta = page.meta;
            let total = meta.total;
            let total_pages = meta.total_pages;
            let items = page.items;

            let from = if total == 0 {
                0
            } else {
                meta.offset as u64 + 1
            };
            let to = meta.offset as u64 + items.len() as u64;

            rsx! {
                div { class: "space-y-8",
                    div { class: "flex items-center justify-between gap-4 border border-primary-6 bg-primary px-4 py-3",
                        div { class: "text-xs font-mono tracking-wide text-secondary-5",
                            "ENTRIES: "
                            span { class: "font-semibold text-secondary-3", "{total}" }
                        }
                        if total > 0 {
                            div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                "RANGE: "
                                span { class: "font-semibold text-secondary-3", "{from}-{to}" }
                            }
                        }
                    }

                    if items.is_empty() {
                        div { class: "flex min-h-[320px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary text-center",
                            span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-5",
                                "NO_DATA"
                            }
                            span { class: "text-sm text-secondary-5", "No repos found" }
                        }
                    } else {
                        div { class: "space-y-4",
                            for r in items {
                                RepoManuscriptCard { key: "{r.id}", repo: r }
                            }
                        }
                    }

                    if total_pages > 1 {
                        div { class: "pt-2",
                            CommonPagination {
                                current_page: current_page(),
                                total_pages,
                                on_page_change: move |p| current_page.set(p),
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => rsx! {
            div { class: "rounded-lg border border-primary-6 bg-primary-1 p-4 text-sm text-primary-error",
                "{e}"
            }
        },
        None => rsx! {},
    }
}
