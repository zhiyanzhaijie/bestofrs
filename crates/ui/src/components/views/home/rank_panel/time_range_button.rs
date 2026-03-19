use dioxus::prelude::*;

use super::{time_range_text, TimeRange};

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeTimeRangeButtonProps {
    range: TimeRange,
    active: bool,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub(super) fn HomeTimeRangeButton(props: HomeTimeRangeButtonProps) -> Element {
    rsx! {
        div { class: "relative group",
            div {
                class: "absolute inset-0 rounded-full border-2 transition-all duration-300 translate-x-[10px] translate-y-[10px]",
                class: if props.active { "border-focused-border" } else { "bg-primary-1 border-primary-6 group-hover:border-focused-border" }
            }
            button {
                onclick: move |e| props.onclick.call(e),
                class: "relative px-8 py-3 rounded-full text-sm font-black font-sans uppercase tracking-[0.2em] italic border-4 transition-all duration-300 ease-out hover:cursor-pointer",
                class: if props.active {
                    "bg-secondary-2 border-secondary-2 text-primary translate-x-[3.82px] translate-y-[3.82px] shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_30%,transparent)]"
                } else {
                    "bg-primary border-secondary-2 text-secondary-2 hover:border-focused-border hover:text-secondary-6 hover:translate-x-[3.82px] hover:translate-y-[3.82px] hover:shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_22%,transparent)]"
                },
                "{time_range_text(props.range)}"
            }
        }
    }
}
