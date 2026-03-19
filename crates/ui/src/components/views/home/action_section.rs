use dioxus::prelude::*;

use crate::components::icons::{ArrowRightIcon, StarIcon};

#[component]
pub(super) fn HomeActionSection() -> Element {
    rsx! {
        div { class: "w-full max-w-7xl px-8 mb-24 relative z-10",
            div { class: "border-t border-primary-6 pt-16 flex flex-col md:flex-row md:items-center md:justify-between gap-8",
                div { class: "space-y-3",
                    div { class: "flex items-center gap-3",
                        div { class: "w-8 h-[1px] bg-secondary-6" }
                        span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold", "Action" }
                    }
                    h4 { class: "text-3xl md:text-4xl font-black font-sans uppercase tracking-tighter italic text-secondary-1 leading-none",
                        "Support "
                        span { class: "text-secondary-6 not-italic", "\"Best Of RS\"" }
                    }
                    p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed max-w-2xl",
                        "If Best of RS is useful to you, star the project or help us add more great repositories."
                    }
                }
                div { class: "flex flex-wrap items-center gap-5",
                    a {
                        href: "https://github.com/zhiyanzhaijie/bestofrs",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "Star On Github" }
                            span { class: "group-hover:translate-x-1 transition-transform",  StarIcon {  } }
                        }
                    }
                    a {
                        href: "https://github.com/zhiyanzhaijie/bestofrs/issues/new",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "Recommend One" }
                            span { class: "group-hover:translate-x-1 transition-transform", ArrowRightIcon {  } }
                        }
                    }
                }
            }
        }
    }
}
