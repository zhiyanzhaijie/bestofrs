use dioxus::prelude::*;

use super::gearmap::GearMap;
use crate::{
    components::icons::{FerrisIcon, RustGearIcon},
    root::Route,
};

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: "relative mt-auto min-h-[340px]",
            div { class: "pointer-events-none absolute inset-x-0 bottom-0 h-[460px] overflow-hidden -z-0",
                GearMap {
                    count: 7,
                    class: "text-primary-6/70",
                    height: 460,
                }
            }

            footer {
                class: "relative z-10 overflow-hidden border-t border-dashed border-primary-6 bg-primary/75",
                div { class: "space-y-8 px-6 py-8 md:px-10",
                    div { class: "grid grid-cols-1 gap-8 md:grid-cols-4",
                        div { class: "space-y-3 md:col-span-2",
                            div { class: "flex items-center gap-2",
                                div { class: "h-3 w-3 border border-secondary-2 bg-screentone" }
                                h4 { class: "text-lg font-bold tracking-tight text-secondary-3", "BestOfRust" }
                            }
                            p { class: "max-w-sm text-sm leading-relaxed text-secondary-5",
                                "A curated observation of the Rust ecosystem. Designed with a focus on data clarity and visual calm."
                            }
                            div { class: "flex items-center gap-2",
                                FerrisIcon { height: 34.0 }
                                RustGearIcon { width: 34.0 }
                            }
                        }
                        div { class: "space-y-3",
                            h5 { class: "font-mono text-xs font-bold uppercase tracking-widest text-secondary-4", "Index" }
                            div { class: "space-y-2 text-sm text-secondary-5",
                                Link { class: "block hover:text-secondary-3 hover:underline", to: Route::HomeView {}, "Home" }
                                Link { class: "block hover:text-secondary-3 hover:underline", to: Route::RepoListView {}, "Repo" }
                                Link { class: "block hover:text-secondary-3 hover:underline", to: Route::TagListView {}, "Tag" }
                            }
                        }
                        div { class: "space-y-3",
                            h5 { class: "font-mono text-xs font-bold uppercase tracking-widest text-secondary-4", "System" }
                            div { class: "flex items-center gap-2 text-xs font-mono text-secondary-5",
                                div { class: "h-1.5 w-1.5 rounded-full bg-secondary-success" }
                                span { "Operational" }
                            }
                            div { class: "text-xs font-mono text-secondary-5", "STYLE: MANGA_MANUSCRIPT_V1" }
                        }
                    }

                    div { class: "flex flex-col items-start justify-between gap-2 border-t border-dashed border-primary-6 pt-4 text-xs font-mono text-secondary-5 md:flex-row md:items-center",
                        span { "(C) 2026 OPEN INTELLIGENCE" }
                        span { "CURATED RUST INDEX" }
                    }
                }
            }
        }
    }
}
