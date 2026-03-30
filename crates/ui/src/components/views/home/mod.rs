use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, SEOHead, SEOProp,
};
use dioxus::prelude::*;
use dioxus_i18n::t;

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
                title: t!("view_home_seo_title").to_string(),
                description: t!("view_home_seo_description").to_string(),
                keywords: t!("view_home_seo_keywords").to_string(),
                canonical_url: "/".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        div { class: "min-h-screen bg-primary flex flex-col items-center relative transition-colors duration-300",
            div { class: "hero-block absolute top-0 left-1/2 -translate-x-1/2 w-screen h-[40vh] sm:h-[46vh] md:h-[46vh] lg:h-[75vh] z-0 flex items-center transition-colors duration-300 overflow-visible",
                div { class: "absolute inset-0 z-10 overflow-hidden bg-primary transition-colors duration-300",
                    div { class: "absolute inset-0 z-0 flex items-center justify-center",
                        img {
                            src: FERRIS_IMAGE,
                            alt: "Best of Rust Ferris",
                            class: "h-[130%] w-[150vw] lg:w-full lg:max-w-full max-w-none opacity-50 object-contain mix-blend-multiply pointer-events-none transition-all",
                        }
                    }
                    div {
                        class: "absolute inset-y-0 left-0 w-1/2 z-10",
                        style: "background: linear-gradient(to right, color-mix(in oklab, var(--primary-color-3) 100%, transparent), color-mix(in oklab, var(--primary-color-2) 62%, transparent), transparent);",
                    }
                    div {
                        class: "absolute inset-y-0 right-0 md:w-1/6 z-10",
                        style: "background: linear-gradient(to left, color-mix(in oklab, var(--primary-color-3) 88%, transparent), color-mix(in oklab, var(--primary-color-2) 42%, transparent), transparent);",
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
                div { class: "max-w-7xl mx-auto px-3 sm:px-4 md:px-8 relative z-20 w-full",
                    div { class: "max-w-3xl px-3 sm:px-4 md:px-6",
                        div { class: "flex items-center gap-2 mb-4 sm:mb-6 md:mb-8",
                            div { class: "w-8 sm:w-10 md:w-12 h-[1px] bg-secondary-6" }
                            span { class: "font-mono text-[9px] sm:text-[10px] tracking-[0.35em] sm:tracking-[0.5em] uppercase text-secondary-6 font-bold",
                                {t!("view_home_hero_for_rustacean")}
                            }
                        }
                        h1 { class: "text-5xl sm:text-6xl md:text-[120px] font-black font-sans leading-[0.8] tracking-tighter uppercase mb-5 sm:mb-7 md:mb-10 italic text-secondary-1",
                            "Best Of"
                            br {}
                            span { class: "text-secondary-6 not-italic", "Rust" }
                        }
                        h2 { class: "text-base sm:text-lg md:text-2xl text-secondary-4 font-sans italic leading-relaxed max-w-xl mb-6 sm:mb-8 md:mb-12",
                            {t!("view_home_hero_subtitle")}
                        }
                    }
                }
            }

            div { class: "h-[34vh] sm:h-[24vh] md:h-[40vh] lg:h-[65vh] w-full" }

            div { class: "w-full max-w-7xl px-0 md:px-8 relative z-30 mt-8 sm:mt-10 md:mt-20 pb-16 sm:pb-20 md:pb-32",
                div { class: "absolute inset-x-3 sm:inset-x-4 md:inset-x-8 bottom-8 sm:bottom-12 md:bottom-24 top-0 bg-screentone opacity-10 pointer-events-none z-0 rounded-[2rem] md:rounded-[3.5rem]" }
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
            GridSlashTransition {}
        }
    }
}
