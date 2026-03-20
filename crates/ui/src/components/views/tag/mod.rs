use dioxus::prelude::*;

use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridType, GridWrapper, IOCell,
    SEOHead, SEOProp,
};
use crate::components::icons::{FerrisFisherIcon, FisherWaveIcon};
use crate::types::tags::TagListItemDto;

mod meta_handler;
mod tab_menu;
mod tag_list;

use meta_handler::TagMetaHandler;
use tab_menu::TagRouteMenu;
use tag_list::{skeleton::TagListCachedFallback, TagListIO};

#[derive(Clone, PartialEq, Eq)]
pub(super) struct TagRouteItem {
    pub(super) label: String,
    pub(super) value: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct TagListSummary {
    pub(super) from: u64,
    pub(super) to: u64,
    pub(super) total: u64,
}

impl TagListSummary {
    pub(super) fn empty() -> Self {
        Self {
            from: 0,
            to: 0,
            total: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct TagListContext {
    pub(super) route_tags: Signal<Vec<TagRouteItem>>,
    pub(super) menu_hovered: Signal<bool>,
    pub(super) page_size: Signal<u32>,
    pub(super) current_page: Signal<u32>,
    pub(super) summary: Signal<TagListSummary>,
    pub(super) last_success: Signal<Option<TagListCachedPage>>,
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct TagListCachedPage {
    pub(super) items: Vec<TagListItemDto>,
    pub(super) offset: u32,
    pub(super) total_pages: u32,
    pub(super) current_page: u32,
    pub(super) page_size: u32,
}

#[component]
pub fn TagList() -> Element {
    let page_size = use_signal(|| 20u32);
    let current_page = use_signal(|| 1u32);
    let menu_hovered = use_signal(|| false);
    let route_tags = use_signal(Vec::<TagRouteItem>::new);
    let summary = use_signal(TagListSummary::empty);
    let last_success = use_signal(|| None::<TagListCachedPage>);

    use_context_provider(|| TagListContext {
        route_tags,
        menu_hovered,
        page_size,
        current_page,
        summary,
        last_success,
    });

    rsx! {
        SEOHead {
            data: SEOProp {
                title: "Tag Trends".into(),
                description: "Explore a comprehensive tag index of the Rust ecosystem and discover curated projects by category on Best of RS."
                    .into(),
                keywords: "best of rs, rust tags, rust categories, rust group rank".into(),
                canonical_url: "/tag".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        TagRouteMenu {}
        GridWrapper {
            class: "text-grid-accent",
            grid_type: GridType::End,
            padding: GridPadding::None,
            GridWrapper {
                class: "!absolute bottom-0 w-full h-5 overflow-y-visible pointer-events-none",
                grid_type: GridType::End,
                padding: GridPadding::None,
                is_dot_on: false,
                background: GridBackground {
                    pattern: GridPattern::Dot,
                    gradient: GradientDirection::None,
                },
                div { class: "absolute inset-0 bg-gradient-to-b from-grid-accent/30 via-grid-accent/20 to-grid-accent/10" }
                div { class: "absolute bottom-5 w-full overflow-hidden text-grid-accent/24 pointer-events-none",
                    FisherWaveIcon { class: "h-8 md:h-12" }
                }
            }
            FerrisFisherIcon {}
            div { class: "absolute w-full mx-auto top-40", TagMetaHandler {} }
        }
        GridWrapper {
            class: "min-h-screen",
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Dot,
                gradient: GradientDirection::ToBottom,
            },
            section { class: "relative overflow-visible min-h-screen grid grid-rows-[auto_minmax(0,1fr)] gap-6",
                IOCell {
                    loading_fallback: rsx! {
                        TagListCachedFallback {}
                    },
                    TagListIO {}
                }
            }
        }
    }
}
