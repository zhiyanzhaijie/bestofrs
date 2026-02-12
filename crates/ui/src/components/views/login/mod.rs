use dioxus::prelude::*;
use dioxus_use_js::use_js;

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
        div { class: "py-8",
            div { class: "rounded-xl border border-primary-6 bg-primary-2 p-6 space-y-2",
                h1 { class: "text-2xl font-semibold tracking-tight", "Login" }
                div { class: "text-sm text-secondary-5", "Redirecting to GitHub..." }
            }
        }
    }
}
