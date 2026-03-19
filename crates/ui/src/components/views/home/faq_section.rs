use dioxus::prelude::*;

#[component]
pub(super) fn HomeFaqSection() -> Element {
    rsx! {
        div { class: "w-full max-w-7xl px-8 mb-32 relative z-10",
            div { class: "pt-20",
                div { class: "flex items-center gap-3 mb-8",
                    div { class: "w-8 h-[1px] bg-secondary-6" }
                    span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold", "FAQ" }
                }
                h3 { class: "text-4xl md:text-5xl font-black font-sans uppercase tracking-tighter italic text-secondary-1 leading-none mb-14",
                    "Frequently Asked"
                    br {}
                    span { class: "text-secondary-6 not-italic", "Questions" }
                }
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-12 gap-y-16",
                    HomeFaqItem {
                        index: 1,
                        question: "How are projects ranked?",
                        answer: "Projects are ranked based on their GitHub growth on dimension like Star, Fork and Issue, over the selected period (daily, weekly, monthly). This highlights what's currently trending in the Rust community.",
                    }
                    HomeFaqItem {
                        index: 2,
                        question: "How to add a project?",
                        answer: "If you know a great Rust project that's missing, use the recommend link in the archive pages. We curate the list to ensure high quality and relevance.",
                    }
                    HomeFaqItem {
                        index: 3,
                        question: "Is this official?",
                        answer: "Best of Rust is a community-driven project inspired by Best of JS. It is not an official Rust Foundation project.",
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HomeFaqItemProps {
    index: usize,
    question: &'static str,
    answer: &'static str,
}

#[component]
fn HomeFaqItem(props: HomeFaqItemProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            h4 { class: "font-black font-sans uppercase tracking-tight text-secondary-2 flex items-center gap-2",
                span { class: "text-secondary-6 font-mono text-xs", "Q{props.index}." }
                "{props.question}"
            }
            p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed",
                "{props.answer}"
            }
        }
    }
}
