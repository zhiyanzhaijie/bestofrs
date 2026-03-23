use dioxus::prelude::*;

use super::gearmap::GearMap;
use crate::{
    components::common::IOCell,
    components::icons::{BORSFerrisIcon, DioxusIcon, HeartIcon},
    root::Route,
};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "relative mt-auto h-full min-h-[340px] overflow-hidden bg-transparent",
            div { class: "pointer-events-none absolute inset-x-0 bottom-0 h-[460px] overflow-hidden -z-0",
                IOCell {
                    loading_fallback: rsx! {
                        div { class: "h-full w-full hidden" }
                    },
                    GearMap {
                        count: 7,
                        class: "text-secondary-6/60",
                        height: 460,
                    }
                }
            }
            div { class: "relative z-10 flex h-full min-h-[340px] flex-col px-6 py-8 md:px-10",
                div { class: "grid flex-1 grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-4",
                    section { class: "space-y-1 text-center lg:col-span-2 lg:text-left",
                        div { class: "flex flex-col items-center gap-1 text-grid-accent lg:flex-row lg:items-center lg:gap-1",
                            BORSFerrisIcon { width: 68.0 }
                            h3 { class: "text-3xl font-extrabold",
                                span { style: "color: #f28c1b;", "B" }
                                span { style: "color: #d4b100;", "E" }
                                span { style: "color: #2fa84f;", "S" }
                                span { style: "color: #1aa6a6;", "T" }
                                span { " " }
                                span { style: "color: #2f6fd4;", "O" }
                                span { style: "color: #8756c9;", "F" }
                                span { " " }
                                span { style: "color: #e8473c;", "R" }
                                span { style: "color: #e8473c;", "S" }
                            }
                        }
                        p { class: "mx-auto max-w-sm text-sm leading-relaxed text-secondary-5 lg:mx-0",
                            "Collect cool things of the Rust ecosystem."
                        }
                    }
                    div { class: "grid grid-cols-1 gap-8 sm:grid-cols-2 lg:col-span-2 lg:grid-cols-2",
                        nav {
                            class: "space-y-3",
                            aria_label: "Footer Index",
                            h5 { class: "font-mono text-xs font-bold uppercase tracking-widest text-secondary-4",
                                "Index"
                            }
                            div { class: "space-y-2 text-sm text-secondary-5",
                                Link {
                                    class: "block hover:text-secondary-3 hover:underline",
                                    to: Route::HomeView {},
                                    "Home"
                                }
                                Link {
                                    class: "block hover:text-secondary-3 hover:underline",
                                    to: Route::RepoListView {
                                        tags: None,
                                        metric: None,
                                        range: None,
                                        page: None,
                                        size: None,
                                    },
                                    "Repos"
                                }
                                Link {
                                    class: "block hover:text-secondary-3 hover:underline",
                                    to: Route::TagListView {},
                                    "Tag"
                                }
                                Link {
                                    class: "block hover:text-secondary-3 hover:underline",
                                    to: Route::AboutView {},
                                    "About"
                                }
                            }
                        }
                        section { class: "space-y-3",
                            h5 { class: "font-mono text-xs font-bold uppercase tracking-widest text-secondary-4",
                                "System"
                            }
                            div { class: "flex items-center gap-2 text-xs font-mono text-secondary-5",
                                div { class: "h-1.5 w-1.5 rounded-full bg-secondary-success" }
                                span { "Operational" }
                            }
                        }
                    }
                }
                div { class: "mt-8 flex items-center justify-center border-t border-dashed border-primary-6 pt-4 text-xs font-mono text-secondary-5",
                    div { class: "flex flex-wrap items-center justify-center gap-2 text-center",
                        span { "Made with" }
                        a {
                            class: "inline-flex items-center gap-1 text-secondary-4 underline-offset-2 transition-colors hover:text-secondary-2 hover:underline",
                            href: "https://dioxuslabs.com/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            DioxusIcon { width: 18.0, height: 18.0 }
                            span { "Dioxus" }
                        }
                        span { "and" }
                        HeartIcon {
                            width: 18,
                            height: 18,
                            style: "color: var(--primary-error-color);",
                        }
                        span { "by" }
                        a {
                            class: "text-secondary-4 underline-offset-2 transition-colors hover:text-secondary-2 hover:underline",
                            href: "https://github.com/zhiyanzhaijie",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "zhiyanzhaijie"
                        }
                        span { "for Rustacean" }
                    }
                }
            }
        }
    }
}
