use dioxus::prelude::*;
use dioxus_sdk_time::use_interval;

#[component]
pub fn TypingText(
    text: String,
    active: bool,
    #[props(default = 30)] speed_ms: u64,
    #[props(default = true)] show_cursor: bool,
    #[props(default = "ml-1 inline-block animate-pulse".to_string())] cursor_class: String,
) -> Element {
    let mut typed_len = use_signal(|| 0usize);
    let text_for_tick = text.clone();
    let total_len = text.chars().count();
    use_interval(std::time::Duration::from_millis(speed_ms.max(1)), move |_| {
        if active {
            let next = typed_len().saturating_add(1);
            typed_len.set(next.min(text_for_tick.chars().count()));
        } else if typed_len() != 0 {
            typed_len.set(0);
        }
    });
    let typed = text.chars().take(typed_len().min(total_len)).collect::<String>();

    rsx! {
        span {
            "{typed}"
            if show_cursor && typed_len() < total_len {
                span { class: "{cursor_class}", "_" }
            }
        }
    }
}
