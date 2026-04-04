use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::{
    button::{Button, ButtonVariant},
    common::{GridPadding, GridType, GridWrapper},
    icons::{BestOfRSIcon, MenuIcon},
    sheet::{
        Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle,
    },
    skeleton::Skeleton,
    ColorSwitcher, DiscordLink, Footer, FuzzySearch, GithubLink, HeaderNav, LanguageSwitcher,
    ThemeSwitcher, UserProfile,
};
use crate::root::Route;

#[component]
pub fn UserLayout() -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        div { class: "flex min-h-screen w-full flex-col overflow-x-clip",
            header { class: "sticky top-0 z-50 w-full shrink-0 bg-primary px-3 md:px-8",
                GridWrapper {
                    class: "mx-auto max-w-7xl".to_string(),
                    padding: GridPadding::None,
                    div { class: "md:hidden",
                        div { class: "flex h-14 items-center justify-between gap-2 px-3",
                            Link {
                                class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1",
                                to: Route::HomeView {},
                                BestOfRSIcon { height: 36.0 }
                            }
                            div { class: "flex items-center gap-2",
                                FuzzySearch {}
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "button inline-flex h-9 items-center justify-center",
                                    style: "border-radius: 0.5rem;",
                                    aria_label: t!("layout_user_user_layout_open_mobile_menu"),
                                    onclick: move |_| mobile_menu_open.set(true),
                                    MenuIcon { width: 18, height: 18 }
                                }
                            }
                        }
                        Sheet {
                            open: mobile_menu_open(),
                            on_open_change: move |v| mobile_menu_open.set(v),
                            SheetContent {
                                side: SheetSide::Left,
                                class: "w-[18rem] max-w-[90vw] p-0 gap-0".to_string(),
                                SheetHeader {
                                    class: "hidden",
                                    SheetTitle { {t!("layout_user_user_layout_navigation_menu")} }
                                    SheetDescription { {t!("layout_user_user_layout_mobile_navigation_menu")} }
                                }
                                div { class: "flex min-h-0 flex-1 flex-col px-4 pt-4",
                                    div { class: "shrink-0",
                                        Link {
                                            class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1",
                                            to: Route::HomeView {},
                                            BestOfRSIcon { height: 34.0 }
                                        }
                                    }
                                    div { class: "min-h-0 flex-1 overflow-y-auto py-3",
                                        HeaderNav { vertical: true }
                                    }
                                }
                                SheetFooter { class: "border-t border-primary-6 px-4 py-3",
                                    div { class: "flex items-center justify-between gap-3",
                                        UserProfile {}
                                        div { class: "flex items-center gap-3",
                                            ThemeSwitcher {}
                                            LanguageSwitcher {}
                                            ColorSwitcher {}
                                            DiscordLink {}
                                            GithubLink {  }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "hidden h-16 items-center justify-between gap-3 px-4 md:flex",
                        div { class: "flex items-center gap-2",
                            Link { class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1", to: Route::HomeView {},
                                BestOfRSIcon { height: 40.0 }
                            }
                            HeaderNav {}
                        }
                        div { class: "flex items-center gap-3",
                            FuzzySearch {}
                            ThemeSwitcher {}
                            LanguageSwitcher {}
                            ColorSwitcher {}
                            DiscordLink {}
                            GithubLink {}
                            UserProfile {}
                        }
                    }
                }
            }
            main { class: "relative z-10 flex-1 w-full px-3 md:px-8",
                div { class: "mx-auto w-full max-w-7xl",
                    SuspenseBoundary {
                        fallback: move |_: SuspenseContext| {
                            rsx! {
                                Skeleton { class: "skeleton h-full w-full" }
                            }
                        },
                        Outlet::<Route> {}
                    }
                }
            }
            footer { class: "relative z-0 w-full px-3 md:px-8",
                GridWrapper {
                    class: Some("mx-auto max-w-7xl".to_string()),
                    padding: GridPadding::None,
                    is_dot_on: false,
                    grid_type: GridType::End,
                    Footer {}
                }
            }
        }
    }
}
