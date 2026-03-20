use dioxus::prelude::*;

use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};

use super::TagListContext;

#[component]
pub(super) fn TagMetaHandler() -> Element {
    let mut ctx = use_context::<TagListContext>();

    rsx! {
        div { class: "flex flex-col items-center text-center gap-4",
            div { class: "max-w-3xl",
                h1 { class: "text-3xl md:text-6xl font-black font-sans uppercase tracking-tight text-secondary-2 mb-2",
                    "All Tags"
                }
                h2 { class: "text-secondary-3 text-sm md:text-base leading-relaxed font-mono italic font-normal",
                    "A comprehensive tag index of the Rust ecosystem."
                }
            }
            div { class: "flex items-center gap-4 justify-center",
                div { class: "text-xs font-mono tracking-wide text-secondary-5",
                    "List "
                    span { class: "font-semibold text-grid-accent",
                        "{(ctx.summary)().from}-{(ctx.summary)().to}"
                    }
                    " of "
                    span { class: "font-semibold text-grid-accent", "{(ctx.summary)().total}" }
                }
                div { class: "flex items-center",
                    Select::<u32> {
                        value: Some((ctx.page_size)()),
                        placeholder: "page size",
                        on_value_change: move |v: Option<u32>| {
                            if let Some(v) = v {
                                if v != (ctx.page_size)() {
                                    ctx.page_size.set(v);
                                }
                                ctx.current_page.set(1);
                            }
                        },
                        SelectTrigger {
                            aria_label: "Select page size",
                            style: "min-width: 7rem;",
                            SelectValue {}
                        }
                        SelectList { aria_label: "Page size options",
                            SelectGroup {
                                SelectGroupLabel { "Page size" }
                                SelectOption::<u32> {
                                    index: 0usize,
                                    value: 10u32,
                                    text_value: Some("10".to_string()),
                                    "10"
                                    SelectItemIndicator {}
                                }
                                SelectOption::<u32> {
                                    index: 1usize,
                                    value: 20u32,
                                    text_value: Some("20".to_string()),
                                    "20"
                                    SelectItemIndicator {}
                                }
                                SelectOption::<u32> {
                                    index: 2usize,
                                    value: 50u32,
                                    text_value: Some("50".to_string()),
                                    "50"
                                    SelectItemIndicator {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
