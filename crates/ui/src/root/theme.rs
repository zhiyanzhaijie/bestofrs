use dioxus::prelude::*;
use dioxus_use_js::use_js;

const COOKIE_NAME: &str = "bestofrs_theme";
const GRID_COOKIE_NAME: &str = "bestofrs_grid_theme";
const CHANNEL_NAME: &str = "bestofrs-theme";
const SEEDED_KEY: &str = "__bestofrs_theme_seeded";
const CHANNEL_KEY: &str = "__bestofrs_theme_channel";

use_js!("src/js/theme_bridge.js"::{
    js_seed_theme,
    js_seed_grid_theme,
    js_is_dark_mode,
    js_set_theme,
    js_toggle_theme,
    js_set_grid_theme
});

pub fn theme_seed() {
    spawn(async move {
        let _ = js_seed_theme::<()>(COOKIE_NAME, CHANNEL_NAME, SEEDED_KEY, CHANNEL_KEY).await;
        let _ = js_seed_grid_theme::<()>(GRID_COOKIE_NAME).await;
    });
}

pub async fn is_dark_mode() -> bool {
    js_is_dark_mode::<bool>().await.unwrap_or(false)
}

pub fn set_theme(dark_mode: bool) {
    let theme = if dark_mode { "dark" } else { "light" };
    spawn(async move {
        let _ = js_set_theme::<()>(COOKIE_NAME, CHANNEL_NAME, CHANNEL_KEY, theme).await;
    });
}

pub fn toggle_theme() {
    spawn(async move {
        let _ = js_toggle_theme::<()>(COOKIE_NAME, CHANNEL_NAME, CHANNEL_KEY).await;
    });
}

pub fn set_grid_theme(theme: &str) {
    let theme = theme.to_string();
    spawn(async move {
        let _ = js_set_grid_theme::<()>(GRID_COOKIE_NAME, theme).await;
    });
}
