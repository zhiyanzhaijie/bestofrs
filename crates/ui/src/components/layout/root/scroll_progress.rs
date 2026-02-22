use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/scroll_progress.js"::mount_scroll_progress);

#[component]
pub fn ScrollProgress() -> Element {
    use_effect(move || {
        spawn(async move {
            let _ = mount_scroll_progress::<()>().await;
        });
    });

    rsx! {
        div { class: "pointer-events-none fixed left-0 top-0 z-[70] h-0.5 w-screen bg-primary-6/50",
            div {
                id: "root-scroll-progress",
                class: "h-full w-0 bg-secondary-3 transition-[width] duration-100 ease-out",
                role: "progressbar",
                aria_label: "Page scroll progress",
                aria_valuemin: "0",
                aria_valuemax: "100",
                aria_valuenow: "0",
            }
        }
    }
}
