use crate::types::auth::MeDto;
use crate::{
    components::{icons, skeleton::Skeleton, toast::ToastProvider, FuzzySearch, UserProfile},
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
            header { class: "border-b border-primary-6 bg-primary-2",
                div { class: "mx-auto max-w-6xl px-4 py-3 flex items-center justify-between",
                    div { class: "flex items-center gap-4",
                        Link { class: "font-semibold", to: Route::Home {}, "bestofrs" }
                        nav { class: "flex items-center gap-3 text-sm",
                            Link { class: "text-secondary-5 hover:underline", to: Route::Home {}, "Home" }
                            Link { class: "text-secondary-5 hover:underline", to: Route::RepoList {}, "Repo" }
                            Link { class: "text-secondary-5 hover:underline", to: Route::TagList {}, "Tag" }
                            Link { class: "text-secondary-5 hover:underline", to: Route::Admin {}, "Admin" }
                        }
                    }
                    div { class: "flex items-center gap-3",
                        FuzzySearch {}
                        button {
                            class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 p-2 text-secondary-5 hover:bg-primary-4",
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

            main { class: "min-h-screen",
                SuspenseBoundary {
                    fallback: move |_: SuspenseContext| {
                        rsx! {
                            div { class: "mx-auto max-w-6xl px-4 py-6",
                                Skeleton { class: "w-full h-[420px]" }
                            }
                        }
                    },
                    Outlet::<Route> {}
                }
            }
        }
    }
}
