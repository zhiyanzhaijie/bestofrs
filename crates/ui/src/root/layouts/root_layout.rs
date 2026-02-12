use crate::types::auth::MeDto;
use crate::{
    components::{
        icons, skeleton::Skeleton, toast::ToastProvider, Footer, FuzzySearch, HeaderNav,
        ScrollProgress, UserProfile,
    },
    root::theme::{is_dark_mode, theme_seed, toggle_theme},
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
            div {
                class: "min-h-screen bg-primary-1 text-secondary-5 flex flex-col",
                style: "--header-height: 6rem;",
                header {
                    class: "fixed inset-x-0 top-0 z-50 border-b border-primary-6 bg-primary-2/85 shadow-sm",
                    div { class: "relative",
                        ScrollProgress {}
                        div { class: "mx-auto flex max-w-6xl items-center justify-between px-4 py-3",
                            div { class: "flex items-center gap-6",
                                Link { class: "font-semibold tracking-tight text-secondary-4", to: Route::HomeView {}, "bestofrs" }
                                HeaderNav {}
                            }
                            div { class: "flex items-center gap-3",
                                FuzzySearch {}
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

                main {
                    class: "mx-auto w-full max-w-6xl flex-1 px-4 pb-10 pt-[var(--header-height)]",
                    SuspenseBoundary {
                        fallback: move |_: SuspenseContext| {
                            rsx! {
                                div { class: "py-8 h-[calc(100dvh-var(--header-height))]",
                                    Skeleton { class: "skeleton h-full min-h-[420px] w-full rounded-xl border border-primary-6" }
                                }
                            }
                        },
                        Outlet::<Route> {}
                    }
                }

                Footer {}
            }
        }
    }
}
