use dioxus::prelude::*;

use crate::components::icons::MenuIcon;

use super::TagListContext;

#[component]
pub(super) fn TagRouteMenu() -> Element {
    let mut ctx = use_context::<TagListContext>();
    let hovered = (ctx.menu_hovered)();
    let route_tags = (ctx.route_tags)();

    rsx! {
        div {
            class: "fixed right-6 top-20 z-30 pointer-events-auto w-64 h-[calc(100vh-5rem)]",
            onmouseenter: move |_| ctx.menu_hovered.set(true),
            onmouseleave: move |_| ctx.menu_hovered.set(false),
            div { class: "relative text-right",
                div { class: if hovered { "mb-2 inline-flex h-10 w-10 items-center justify-center text-grid-accent transition-colors" } else { "mb-2 inline-flex h-10 w-10 items-center justify-center text-secondary-6 transition-colors" },
                    MenuIcon { width: 18, height: 18 }
                }
                div { class: if hovered { "absolute right-0 top-10 w-64 max-h-[calc(100vh-7.5rem)] overflow-auto space-y-1 pr-1 opacity-100 transition-opacity duration-150" } else { "pointer-events-none absolute right-0 top-10 w-64 max-h-0 overflow-hidden opacity-0 transition-opacity duration-150" },
                    if hovered {
                        for item in route_tags {
                            a {
                                key: "{item.value}",
                                href: "#{item.value}",
                                class: "block px-2 py-1 text-sm font-mono text-secondary-6 hover:text-grid-accent transition-colors",
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
