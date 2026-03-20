use dioxus::prelude::*;

const DOTLOTTIE_WC_CDN: &str = "https://unpkg.com/@lottiefiles/dotlottie-wc/dist/dotlottie-wc.js";

#[component]
pub fn LottieWebComp(
    src: String,
) -> Element {
    let style = "width: 100%; height: 100%; display: block;".to_string();

    rsx! {
        document::Script { src: DOTLOTTIE_WC_CDN, r#type: "module" }
        dotlottie-wc {
            src,
            autoplay: true,
            loop: false,
            style,
        }
    }
}
