use crate::components::{
    common::{
        GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
        GridWrapper, SEOHead, SEOProp,
    },
    icons::ArrowRightIcon,
};
use dioxus::prelude::*;

use crate::root::Route;
use action_section::HomeActionSection;
use faq_section::HomeFaqSection;
use rank_panel::HomeRankPanel;

mod action_section;
mod faq_section;
mod rank_panel;
const FERRIS_IMAGE: Asset = asset!(
    "/assets/ferris.gif",
    AssetOptions::builder().with_hash_suffix(false)
);

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        SEOHead {
            data: SEOProp {
                title: "Daily Trends".into(),
                description: "Discover awesome Rust open source projects with daily tracked stars, forks, issues, and ecosystem momentum."
                    .into(),
                keywords: "best of rs, rust projects, rust open source, github, rust trends, rust ranking, community health"
                    .into(),
                canonical_url: "/".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        div { class: "min-h-screen bg-primary flex flex-col items-center selection:bg-secondary-2 selection:text-primary relative transition-colors duration-300",
            div { class: "hero-block absolute top-0 left-1/2 -translate-x-1/2 w-screen h-[65vh] z-0 flex items-center transition-colors duration-300 overflow-visible",
                div { class: "absolute inset-0 z-10 overflow-hidden bg-primary transition-colors duration-300",
                    div { class: "absolute inset-0 z-0",
                        img {
                            src: FERRIS_IMAGE,
                            alt: "Best of Rust Ferris",
                            class: "w-full h-full opacity-50 object-contain mix-blend-multiply pointer-events-none transition-all",
                        }
                    }
                    div {
                        class: "absolute inset-y-0 left-0 w-1/2 z-10",
                        style: "background: linear-gradient(to right, var(--primary-color), color-mix(in oklab, var(--primary-color) 90%, transparent), transparent);",
                    }
                    div {
                        class: "absolute inset-y-0 right-0 w-1/6 z-10",
                        style: "background: linear-gradient(to left, var(--primary-color), color-mix(in oklab, var(--primary-color) 90%, transparent), transparent);",
                    }
                    div {
                        class: "absolute inset-x-0 top-0 h-32 z-10",
                        style: "background: linear-gradient(to bottom, var(--primary-color), transparent);",
                    }
                    div {
                        class: "absolute inset-x-0 bottom-0 h-32 z-10",
                        style: "background: linear-gradient(to top, var(--primary-color), transparent);",
                    }
                }
                div { class: "max-w-7xl mx-auto px-8 relative z-20 w-full",
                    div { class: "max-w-3xl",
                        div { class: "flex items-center gap-3 mb-8",
                            div { class: "w-12 h-[1px] bg-secondary-6" }
                            span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold",
                                "For Rustacean"
                            }
                        }
                        h1 { class: "text-7xl md:text-[120px] font-black font-sans leading-[0.8] tracking-tighter uppercase mb-10 italic text-secondary-1",
                            "Best Of"
                            br {}
                            span { class: "text-secondary-6 not-italic", "Rust" }
                        }
                        h2 { class: "text-xl md:text-2xl text-secondary-4 font-serif italic leading-relaxed max-w-xl mb-12",
                            "The curated archive of Rust's finest. Tracking community health and project velocity, one snapshot at a time."
                        }
                        div { class: "flex items-center gap-8",
                            Link {
                                to: Route::RepoListView {
                                    tags: None,
                                    metric: None,
                                    range: None,
                                    page: None,
                                    size: None,
                                },
                                class: "group flex items-center gap-4 text-5xl font-mono font-bold uppercase tracking-[0.3em] text-secondary-2 hover:text-secondary-6 transition-colors",
                                "GO"
                                span { class: "group-hover:translate-x-2 transition-transform",
                                    ArrowRightIcon { width: 36, height: 36 }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "h-[55vh] w-full" }

            div { class: "w-full max-w-7xl px-8 relative z-30 mt-20 pb-32",
                div { class: "absolute inset-x-8 bottom-24 top-0 bg-screentone opacity-10 pointer-events-none z-0 rounded-[3.5rem]" }
                HomeRankPanel {}
            }

            HomeFaqSection {}

            GridWrapper {
                grid_type: GridType::Inner,
                padding: GridPadding::None,
                background: GridBackground {
                    pattern: GridPattern::Dot,
                    gradient: GradientDirection::ToBottom,
                },
                HomeActionSection {}
            }
            GridSlashTransition {  }
        }
    }
}
