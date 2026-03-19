use dioxus::prelude::*;

use crate::root::layouts::{UserContext, UserState};
use crate::root::Route;

#[component]
pub fn HeaderNav() -> Element {
    let user_state = use_context::<UserContext>();
    let show_admin = matches!(user_state(), UserState::User(me) if me.role == "Admin");
    let nav_link_class =
        "px-3 py-1 font-mono text-xs tracking-wide text-secondary-5 transition-colors hover:text-secondary-3";

    rsx! {
        nav { class: "flex items-center gap-2 text-sm",
            Link { class: "{nav_link_class}", to: Route::HomeView {}, "HOME" }
            Link {
                class: "{nav_link_class}",
                to: Route::RepoListView {
                    tags: None,
                    metric: None,
                    range: None,
                    page: None,
                    size: None,
                },
                "REPO"
            }
            Link { class: "{nav_link_class}", to: Route::TagListView {}, "TAGS" }
            Link { class: "{nav_link_class}", to: Route::AboutView {}, "ABOUT" }
            if show_admin {
                Link { class: "{nav_link_class}", to: Route::AdminProjectsView {}, "ADMIN" }
            }
        }
    }
}
