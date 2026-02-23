use dioxus::prelude::*;
use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridType, GridWrapper,
};

#[component]
pub fn Home() -> Element {
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
                    div { class: "relative z-10 space-y-6",
                        div { class: "inline-flex items-center gap-2 border border-primary-6 bg-primary-1 px-2 py-1 font-mono text-xs font-semibold tracking-wide text-secondary-5",
                            "MANIFEST / HOME"
                        }
                        h1 { class: "text-4xl font-bold tracking-tight text-secondary-2 md:text-6xl",
                            "BestOfRust "
                            span { class: "text-secondary-6", "Manuscript" }
                        }
                        p { class: "max-w-3xl border-l-2 border-primary-6 pl-5 text-base leading-relaxed text-secondary-4",
                            "A curated observation of the Rust ecosystem. Start from the repository index, browse by tags, and inspect project trends in detail."
                        }
                        div { class: "flex flex-wrap gap-3 pt-2",
                            Link { class: "border border-secondary-2 bg-secondary-2 px-5 py-2.5 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm", to: crate::root::Route::RepoListView {}, "Open Index" }
                            Link { class: "border border-primary-6 bg-primary-1 px-5 py-2.5 text-sm font-medium text-secondary-4 transition-all hover:-translate-y-0.5 hover:shadow-comic-sm", to: crate::root::Route::TagListView {}, "Browse Tags" }
                        }
                    }
                }
            }
        }
    }
}
