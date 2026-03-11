use dioxus::prelude::*;

use crate::components::{
    common::{GridPadding, GridType, GridWrapper},
    dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
    icons,
    skeleton::Skeleton,
    Footer, FuzzySearch, HeaderNav, UserProfile,
};
use crate::root::theme::{set_grid_theme, toggle_theme};
use crate::root::Route;

const GRID_THEMES: [(&str, &str); 7] = [
    ("red", "#e8473c"),
    ("orange", "#f28c1b"),
    ("yellow", "#d4b100"),
    ("green", "#2fa84f"),
    ("cyan", "#1aa6a6"),
    ("blue", "#1aa6a6"),
    ("purple", "#8756c9"),
];

#[component]
pub fn UserLayout() -> Element {
    let mut is_dark = use_signal(|| false);

    use_effect(move || {
        let mut is_dark = is_dark;
        spawn(async move {
            let value = crate::root::theme::is_dark_mode().await;
            is_dark.set(value);
        });
    });

    rsx! {
        header { class: "sticky top-0 z-50 bg-primary px-[max(12px,calc((100vw-80rem)/2))] md:px-[max(32px,calc((100vw-80rem)/2))]",
            GridWrapper { padding: GridPadding::None, is_dot_on: true, grid_type: GridType::Default,
                div { class: "flex h-16 items-center justify-between gap-3 px-3 md:px-4",
                    div { class: "flex items-center gap-6",
                        Link { class: "flex items-center gap-2 border border-transparent px-2 py-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1", to: Route::HomeView {},
                            icons::BestOfRSIcon { height: 40.0 }
                        }
                        HeaderNav {}
                    }
                    div { class: "flex items-center gap-3",
                        FuzzySearch {}
                        DropdownMenu {
                            DropdownMenuTrigger {
                                aria_label: "Select grid theme",
                                style: "padding:0; width:1.6rem; height:1.6rem; border-radius:9999px; background:transparent; box-shadow:none; display:flex; align-items:center; justify-content:center;",
                                span {
                                    class: "block h-3 w-3 rounded-full",
                                    style: "background-color: var(--grid-accent);",
                                }
                            }
                            DropdownMenuContent {
                                style: "min-width: auto;",
                                div { class: "flex flex-col items-center gap-1 p-1",
                                    for (idx, (theme, color)) in GRID_THEMES.iter().enumerate() {
                                        DropdownMenuItem::<String> {
                                            key: "{theme}",
                                            index: idx,
                                            value: theme.to_string(),
                                            aria_label: "Set grid theme {theme}",
                                            on_select: move |value: String| {
                                                set_grid_theme(value.as_str());
                                            },
                                            span {
                                                class: "block h-3 w-3 rounded-full",
                                                style: "background-color: {color};",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        button {
                            class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 p-2 text-secondary-5 transition-colors hover:bg-primary-3 hover:text-secondary-4",
                            onclick: move |_| {
                                toggle_theme();
                                is_dark.set(!is_dark());
                            },
                            aria_label: "Toggle theme",
                            if is_dark() {
                                icons::MoonIcon { size: 18 }
                            } else {
                                icons::SunIcon { size: 18 }
                            }
                        }
                        UserProfile {}
                    }
                }
            }
        }
        main { class: "relative z-10 min-h-[calc(100svh-4rem)] overflow-x-hidden",
            div { class: "mx-auto w-[calc(100%-24px)] max-w-7xl md:w-[calc(100%-64px)]",
                SuspenseBoundary {
                    fallback: move |_: SuspenseContext| {
                        rsx! {
                            Skeleton { class: "skeleton h-[calc(100svh-4rem)] w-full" }
                        }
                    },
                    Outlet::<Route> {}
                }
            }
        }
        footer { class: "relative z-50 mx-auto w-[calc(100%-24px)] max-w-7xl md:w-[calc(100%-64px)]",
            GridWrapper { padding: GridPadding::None, is_dot_on: false, grid_type: GridType::End,
                Footer {}
            }
        }
    }
}
