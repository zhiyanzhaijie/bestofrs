use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

use crate::components::{button::Button, skeleton::Skeleton};

#[derive(Props, Clone, PartialEq)]
pub struct IOCellProps {
    children: Element,
    #[props(default)]
    loading_fallback: Option<Element>,
    #[props(default)]
    error_fallback: Option<Callback<ErrorContext, Element>>,
}

#[component]
pub fn IOCell(props: IOCellProps) -> Element {
    rsx! {
        div { class: "h-full min-h-0",
            ErrorBoundary {
                handle_error: move |error: ErrorContext| {
                    if let Some(error_handler) = props.error_fallback {
                        error_handler.call(error)
                    } else {
                        let toast = use_toast();
                        toast.info(format!("{:?}", error), ToastOptions::new().permanent(false));
                        rsx! {
                            div { class: "iocell-error",
                                "Something went wrong"
                                Button { onclick: move |_| error.clear_errors(), "Retry" }
                            }
                        }
                    }
                },
                SuspenseBoundary {
                    fallback: move |_: SuspenseContext| {
                        rsx! {
                            div { class: "h-full min-h-0",
                                {props
                                    .loading_fallback
                                    .clone()
                                    .unwrap_or_else(|| {
                                        rsx! {
                                            Skeleton { class: "skeleton w-full h-full min-h-[24px] rounded-xl border border-primary-6" }
                                        }
                                    })}
                            }
                        }
                    },
                    {props.children}
                }
            }
        }
    }
}
