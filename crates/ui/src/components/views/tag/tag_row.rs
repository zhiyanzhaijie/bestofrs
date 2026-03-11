use dioxus::prelude::*;

use crate::root::Route;
use crate::types::tags::TagListItemDto;

use super::mini_repo_card::MiniRepoCard;

#[component]
pub fn TagRow(
    tag: TagListItemDto,
    index: usize,
    current_page: u32,
    page_size: u32,
) -> Element {
    let outlined = tag
        .label
        .split(':')
        .next()
        .unwrap_or(tag.label.as_str())
        .to_uppercase();
    let description = tag
        .description
        .clone()
        .unwrap_or_else(|| "System classification pending.".to_string());

    let rank = (current_page.saturating_sub(1) as usize) * page_size as usize + index + 1;
    let rank_text = format!("{:03}", rank);
    let population = format!("{:03}", tag.repos_total);

    rsx! {
        article {
            id: "{tag.value}",
            class: "group relative overflow-hidden px-8 py-12 last:border-0",
            style: "scroll-margin-top: 80px;",
            div {
                class: "relative z-0 mb-[-30px] font-mono whitespace-nowrap text-5xl font-bold leading-none tracking-wide opacity-50 md:mb-[-60px] md:text-[120px]",
                style: "-webkit-text-stroke: 2px color-mix(in oklab, var(--secondary-color) 22%, transparent); color: transparent;",
                "{outlined}"
            }
            div { class: "relative z-10 flex px-20 flex-col gap-6",
                div { class: "flex items-center gap-4",
                    span {
                        class: "text-[12px] font-mono font-bold uppercase tracking-[0.6em]",
                        style: "color: var(--grid-accent);",
                        "#{tag.value}"
                    }
                    div { class: "h-[1px] flex-grow", style: "background-color: color-mix(in oklab, var(--grid-accent) 30%, transparent);" }
                }
                p {
                    class: "max-w-3xl border-l-4 pl-10 py-2 font-serif text-2xl italic leading-relaxed text-secondary-4 opacity-50",
                    style: "border-left-color: color-mix(in oklab, var(--grid-accent) 40%, transparent);",
                    "{description}"
                }
                div { class: "pt-0",
                    div { class: "flex items-stretch h-24 gap-10",
                        div { class: "flex items-stretch gap-10 self-stretch",
                            div { class: "flex h-full flex-col",
                                span { class: "text-[10px] font-mono uppercase tracking-widest text-secondary-6",
                                    "Rank"
                                }
                                div { class: "flex flex-1 items-center",
                                    span { class: "text-3xl font-mono font-bold text-secondary-4", "{rank_text}" }
                                }
                            }
                            div { class: "h-full w-px bg-primary-6" }
                            div { class: "flex h-full flex-col",
                                span { class: "text-[10px] font-mono uppercase tracking-widest text-secondary-6",
                                    "Count"
                                }
                                div { class: "flex flex-1 items-center",
                                    span { class: "text-3xl font-mono font-bold text-secondary-1", "{population}" }
                                }
                            }
                        }
                        div { class: "min-w-0 flex-1 self-stretch flex flex-col",
                            div { class: "mb-2 flex items-center gap-4",
                                div { class: "text-[10px] font-mono uppercase tracking-[0.4em] text-secondary-6",
                                    "TOP10"
                                }
                                div { class: "h-px flex-grow border-t border-dashed border-primary-6" }
                            }
                            div { class: "flex flex-1 flex-wrap content-center items-center justify-start gap-2",
                                for (repo_idx, repo) in tag.top_repos.into_iter().take(10).enumerate() {
                                    MiniRepoCard { key: "{repo.repo_id}:{repo_idx}", repo }
                                }
                            }
                        }
                        Link {
                            class: "ml-auto self-start border px-3 py-1 text-xs font-mono tracking-wider transition-colors",
                            style: "border-color: var(--grid-accent); color: var(--grid-accent); background-color: color-mix(in oklab, var(--grid-accent) 10%, transparent);",
                            to: Route::RepoListView {
                                tags: Some(tag.value.clone()),
                                metric: None,
                                range: None,
                                page: None,
                                size: None,
                            },
                            "OPEN"
                        }
                    }
                }
            }
        }
    }
}
