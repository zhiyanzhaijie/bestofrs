use crate::types::auth::MeDto;
use crate::{
    components::{
        dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger},
        common::{GridPadding, GridType, GridWrapper},
        icons,
        skeleton::Skeleton,
        toast::ToastProvider,
        Footer, FuzzySearch, HeaderNav, ScrollProgress, UserProfile,
    },
    root::theme::{is_dark_mode, set_grid_theme, theme_seed, toggle_theme},
    root::Route,
    IO::auth::me,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum UserState {
    Loading,
    Guest,
    User(MeDto),
    Error(String),
}

pub type UserContext = Signal<UserState>;
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
pub fn RootLayout() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut user_state: UserContext = use_signal(|| UserState::Loading);
    use_context_provider(|| user_state);
    let me_fut = use_server_future(move || me())?;

    use_effect(move || {
        theme_seed();
        let mut is_dark = is_dark;
        spawn(async move {
            let value = is_dark_mode().await;
            is_dark.set(value);
        });
    });

    use_effect(move || {
        if !matches!(user_state(), UserState::Loading) {
            return;
        }
        match me_fut() {
            Some(Ok(Some(me))) => user_state.set(UserState::User(me)),
            Some(Ok(None)) => user_state.set(UserState::Guest),
            Some(Err(err)) => user_state.set(UserState::Error(err.to_string())),
            None => {}
        }
    });

    rsx! {
        ToastProvider {
            div { class: "min-h-screen overflow-x-hidden bg-primary text-secondary-5",
                ScrollProgress {}
                div { class: "relative mx-auto flex min-h-screen w-[calc(100%-24px)] max-w-7xl flex-col shadow-[0_0_50px_rgba(0,0,0,0.02)] md:w-[calc(100%-64px)]",
                    header { class: "fixed top-0 left-0 right-0 z-50 mx-auto w-[calc(100%-24px)] max-w-7xl bg-transparent md:w-[calc(100%-64px)]",
                        GridWrapper { padding: GridPadding::None, is_dot_on: true, grid_type: GridType::Default,
                            div { class: "relative",
                                div { class: "flex h-16 items-center justify-between gap-3 bg-primary/95 px-3 md:px-4",
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
                    }

                    main { class: "relative z-10 flex-1 pt-16",
                        SuspenseBoundary {
                            fallback: move |_: SuspenseContext| {
                                rsx! {
                                    div { class: "py-8 min-h-[60vh]",
                                        Skeleton { class: "skeleton h-full min-h-[420px] w-full rounded-xl border border-primary-6" }
                                    }
                                }
                            },
                            Outlet::<Route> {}
                        }
                    }

                    GridWrapper { padding: GridPadding::None, is_dot_on: false, grid_type: GridType::End,
                        Footer {}
                    }
                }
            }
        }
    }
}
