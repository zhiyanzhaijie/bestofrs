use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridType, GridWrapper,
};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();
    use_effect(move || {
        navigator.replace(NavigationTarget::<String>::External(
            "/api/auth/login/github".to_string(),
        ));
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
            div { class: "flex min-h-[calc(100svh-4rem-340px)] w-full items-center justify-center",
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
