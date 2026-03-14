mod chart;
mod deltas_section;
mod meta_section;
mod readme_section;
mod snapshot_section;

use crate::components::common::{
    CommonBreadcrumb, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper, IOCell,
};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};

use deltas_section::skeleton::DeltasSectionSkeleton;
use dioxus::prelude::*;

use meta_section::skeleton::MetaSectionSkeleton;
use meta_section::MetaSection;
use readme_section::skeleton::ReadmeSectionSkeleton;
use snapshot_section::skeleton::SnapshotSectionSkeleton;

use chart::{
    build_delta_chart_config, build_trend_chart_config, chart_dom_id, short_date_label,
    ChartJsCanvas,
};
use deltas_section::DeltasSection;
use readme_section::ReadmeSection;
use snapshot_section::SnapshotSection;

#[derive(Clone, Copy)]
pub(super) struct RepoDetailContext {
    pub(super) owner: ReadSignal<String>,
    pub(super) name: ReadSignal<String>,
}

#[component]
pub fn RepoDetail(owner: ReadSignal<String>, name: ReadSignal<String>) -> Element {
    let mut active_tab = use_signal(|| Some("deltas".to_string()));
    let active_tab_read: ReadSignal<Option<String>> = active_tab.into();
    use_context_provider(|| RepoDetailContext { owner, name });

    rsx! {
        div { class: "space-y-0",
            GridWrapper {
                is_dot_on: true,
                padding: GridPadding::Sm,
                CommonBreadcrumb {  }
            }
            GridWrapper {
                grid_type: GridType::Default,
                padding: GridPadding::Lg,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Dot,
                    gradient: GradientDirection::ToBottom,
                },
                IOCell {
                    loading_fallback: rsx! { MetaSectionSkeleton {} },
                    MetaSection {}
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
                                loading_fallback: rsx! { DeltasSectionSkeleton {} },
                                DeltasSection {}
                            }
                        }
                        TabContent {
                            value: "snapshot".to_string(),
                            index: 1usize,
                            IOCell {
                                loading_fallback: rsx! { SnapshotSectionSkeleton {} },
                                SnapshotSection {}
                            }
                        }
                    }

                    IOCell {
                        loading_fallback: rsx! {
                            ReadmeSectionSkeleton {}
                        },
                        ReadmeSection {}
                    }
                }
            }
        }
    }
}
