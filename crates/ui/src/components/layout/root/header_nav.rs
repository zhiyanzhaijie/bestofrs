use dioxus::prelude::*;

use crate::root::layouts::{UserContext, UserState};
use crate::root::Route;

#[component]
pub fn HeaderNav() -> Element {
    let user_state = use_context::<UserContext>();
    let show_admin = matches!(user_state(), UserState::User(me) if me.role == "Admin");

    rsx! {
        nav { class: "flex items-center gap-2 text-sm",
            Link { class: "border border-transparent px-3 py-1 font-mono text-xs tracking-wide text-secondary-5 transition-all hover:border-primary-6 hover:bg-primary-1 hover:text-secondary-3", to: Route::HomeView {}, "HOME" }
            Link {
                class: "border border-transparent px-3 py-1 font-mono text-xs tracking-wide text-secondary-5 transition-all hover:border-primary-6 hover:bg-primary-1 hover:text-secondary-3",
                to: Route::RepoListView {
                    tags: None,
                    metric: None,
                    range: None,
                    page: None,
                    size: None,
                },
                "REPO"
            }
            Link { class: "border border-transparent px-3 py-1 font-mono text-xs tracking-wide text-secondary-5 transition-all hover:border-primary-6 hover:bg-primary-1 hover:text-secondary-3", to: Route::TagListView {}, "TAGS" }
            if show_admin {
                Link { class: "border border-transparent px-3 py-1 font-mono text-xs tracking-wide text-secondary-5 transition-all hover:border-primary-6 hover:bg-primary-1 hover:text-secondary-3", to: Route::AdminProjectsView {}, "ADMIN" }
            }
        }
    }
}
