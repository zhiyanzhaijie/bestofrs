use dioxus::prelude::*;

use crate::components::{
    common::{GridPadding, GridType, GridWrapper},
    icons::{self, BestOfRSIcon},
    skeleton::Skeleton,
    ColorSwitcher, Footer, FuzzySearch, HeaderNav, ThemeSwitcher, UserProfile,
};
use crate::root::Route;

#[component]
pub fn UserLayout() -> Element {
    rsx! {
        div { class: "flex min-h-screen w-full flex-col overflow-x-clip",
            header { class: "sticky top-0 z-50 w-full shrink-0 bg-primary px-3 md:px-8",
                GridWrapper {
                    class: "mx-auto max-w-7xl".to_string(),
                    padding: GridPadding::None,
                    div { class: "flex h-16 px-4 items-center justify-between gap-3",
                        div { class: "flex items-center gap-2",
                            Link { class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1", to: Route::HomeView {},
                                BestOfRSIcon { height: 40.0 }
                            }
                            HeaderNav {}
                        }
                        div { class: "flex items-center gap-3",
                            FuzzySearch {}
                            ColorSwitcher {}
                            ThemeSwitcher {}
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
            footer { class: "relative z-50 w-full px-3 md:px-8",
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
