use dioxus::prelude::*;

use crate::root::Route;
use crate::root::layouts::{UserContext, UserState};

#[component]
pub fn AdminLayout() -> Element {
    let navigator = use_navigator();
    let mut redirected = use_signal(|| false);
    let user_state = use_context::<UserContext>();
    let is_admin = matches!(user_state(), UserState::User(me) if me.role == "Admin");

    use_effect(move || {
        if redirected() {
            return;
        }

        let unauthorized = match user_state() {
            UserState::User(me) => me.role != "Admin",
            UserState::Loading => false,
            UserState::Guest | UserState::Error(_) => true,
        };

        if unauthorized {
            redirected.set(true);
            navigator.replace(Route::Home {});
        }
    });

    rsx! {
        if is_admin {
            Outlet::<Route> {}
        } else if matches!(user_state(), UserState::Loading) {
            div { class: "mx-auto max-w-6xl px-4 py-6 text-sm text-secondary-5",
                "Loading..."
            }
        } else {
            div { class: "mx-auto max-w-6xl px-4 py-6 text-sm text-secondary-5",
                "Redirecting..."
            }
        }
    }
}
