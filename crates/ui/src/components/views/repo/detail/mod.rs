mod chart;
mod meta_section;
mod readme_section;
mod trend_section;

use crate::components::common::{
    CommonBreadcrumb, GradientDirection, GridBackground, GridLineType, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper, IOCell,
};
use dioxus::prelude::*;

use meta_section::skeleton::MetaSectionSkeleton;
use meta_section::MetaSection;
use readme_section::skeleton::ReadmeSectionSkeleton;
use readme_section::ReadmeSection;
use trend_section::TrendSection;

use chart::{
    build_delta_chart_config, build_trend_chart_config, chart_dom_id, short_date_label,
    ChartJsCanvas,
};

#[derive(Clone, Copy)]
pub(super) struct RepoDetailContext {
    pub(super) owner: ReadSignal<String>,
    pub(super) name: ReadSignal<String>,
}

#[component]
pub fn RepoDetail(
    owner: ReadSignal<String>,
    name: ReadSignal<String>,
    metric: ReadSignal<Option<String>>,
) -> Element {
    use_context_provider(|| RepoDetailContext { owner, name });

    rsx! {
        div { class: "space-y-0",
            GridWrapper { is_dot_on: true, padding: GridPadding::Sm, CommonBreadcrumb {} }
            GridWrapper {
                padding: GridPadding::None,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Dot,
                    gradient: GradientDirection::ToBottom,
                },
                IOCell {
                    loading_fallback: rsx! {
                        MetaSectionSkeleton {}
                    },
                    MetaSection {}
                }
            }
            GridSlashTransition {}
            GridWrapper { padding: GridPadding::Sm,
                section { class: "space-y-8",
                    TrendSection { initial_metric: metric }
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
