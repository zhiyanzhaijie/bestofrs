use dioxus::prelude::*;
use dioxus_use_js::use_js;
use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridType, GridWrapper,
};

use_js!("src/js/dom_bridge.js"::redirect_to);

#[component]
pub fn Login() -> Element {
    let mut started = use_signal(|| false);

    use_effect(move || {
        if started() {
            return;
        }
        started.set(true);
        spawn(async move {
            let _ = redirect_to::<()>("/api/auth/login/github").await;
        });
    });

    rsx! {
        GridWrapper {
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Slash,
                gradient: GradientDirection::None,
            },
            div { class: "flex min-h-[55vh] items-center justify-center",
                div { class: "w-full max-w-xl border border-secondary-2 bg-primary p-8 shadow-comic",
                    div { class: "space-y-3",
                        h1 { class: "text-3xl font-bold tracking-tight text-secondary-2", "Login" }
                        div { class: "border-l-2 border-primary-6 pl-4 text-sm text-secondary-5", "Redirecting to GitHub..." }
                    }
                }
            }
        }
    }
}
