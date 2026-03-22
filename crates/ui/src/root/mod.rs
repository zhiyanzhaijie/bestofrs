pub mod layouts;
mod routes;
pub mod theme;

use dioxus::prelude::*;
pub use routes::Route;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENT_CSS: Asset = asset!("/assets/dx-components-theme.css");
const CHART_JS_CDN: &str = "https://cdn.jsdelivr.net/npm/chart.js@4.5.1/dist/chart.umd.min.js";

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DX_COMPONENT_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }
        document::Script {
            src: CHART_JS_CDN,
            async: true,
            defer: true,
        }

        Router::<Route> {}
    }
}
