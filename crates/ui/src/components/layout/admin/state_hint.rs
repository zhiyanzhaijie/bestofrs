use dioxus::prelude::*;

#[component]
pub fn AdminStateHint(message: String) -> Element {
    rsx! {
        div { class: "py-8 text-sm text-secondary-5",
            "{message}"
        }
    }
}
