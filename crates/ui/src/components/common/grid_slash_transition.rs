use dioxus::prelude::*;

use super::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridType, GridWrapper,
};

#[component]
pub fn GridSlashTransition() -> Element {
    rsx! {
        GridWrapper {
            class: Some("h-4".to_string()),
            padding: GridPadding::None,
            is_dot_on: true,
            grid_type: GridType::Default,
            background: GridBackground {
                pattern: GridPattern::Slash,
                gradient: GradientDirection::None,
            },
            div { class: "h-full w-full" }
        }
    }
}
