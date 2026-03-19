use dioxus::prelude::*;

use super::{rank_desc, rank_title, stat_icon_tab, RankType};

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeRankTabItemProps {
    tab: RankType,
    active_tab: RankType,
    on_select: EventHandler<MouseEvent>,
}

#[component]
pub(super) fn HomeRankTabItem(props: HomeRankTabItemProps) -> Element {
    let is_active = props.active_tab == props.tab;
    rsx! {
        div {
            class: "rounded-r-2xl transition-all duration-500 relative flex flex-col flex-grow",
            class: if is_active { "bg-primary-1 shadow-sm" } else { "bg-transparent hover:bg-primary-1/40" },
            if is_active {
                div { class: "absolute left-0 top-0 bottom-0 w-1 bg-secondary-6" }
            }
            button {
                onclick: move |e| props.on_select.call(e),
                class: "w-full px-6 py-6 flex items-center justify-between group text-left relative z-10 hover:cursor-pointer",
                div { class: "flex items-center gap-2",
                    span {
                        class: "inline-flex items-center gap-2 text-lg font-black font-sans uppercase tracking-widest transition-colors",
                        class: if is_active { "text-secondary-2" } else { "text-secondary-5 group-hover:text-secondary-2" },
                        span { class: "inline-flex items-center", {stat_icon_tab(props.tab)} }
                        "{rank_title(props.tab)}"
                    }
                }
                span {
                    class: "transition-all duration-300 text-3xl",
                    class: if is_active { "rotate-90 text-secondary-6" } else { "text-primary-6" },
                    "›"
                }
            }
            div {
                class: "px-6 flex-grow flex items-start overflow-hidden transition-all duration-700 ease-in-out",
                class: if is_active { "pb-12" } else { "max-h-0 opacity-0" },
                div { class: "relative pt-2",
                    p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed pl-4",
                        "{rank_desc(props.tab)}"
                    }
                }
            }
        }
    }
}
