use crate::components::common::RepoAvatar;
use crate::components::common::TagContent;
use crate::components::ui::avatar::AvatarImageSize;
use crate::components::ui::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};
use crate::root::Route;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

use super::{
    parse_repo_route, rainbow_color, row_border_style, stat_icon_list, stat_value, HomeRankRepo,
    RankType,
};

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeRankRepoRowProps {
    idx: usize,
    repo: HomeRankRepo,
    active_tab: RankType,
}

#[component]
pub(super) fn HomeRankRepoRow(props: HomeRankRepoRowProps) -> Element {
    let route = parse_repo_route(&props.repo.id, props.active_tab).unwrap_or(Route::HomeView {});
    let navigator = use_navigator();
    let stat_text = stat_value(&props.repo, props.active_tab);
    let accent_color = rainbow_color(props.idx);
    let tag_items = props.repo.tags.iter().take(3).cloned().collect::<Vec<_>>();
    let has_tag_items = !tag_items.is_empty();
    let more_tags = props.repo.tags.len().saturating_sub(3);

    rsx! {
        div {
            class: "rank-card relative flex items-start rounded-xl border-l-4 border-y border-r border-primary-6 bg-primary p-2 shadow-sm transition-all duration-300 group md:h-[86px] md:items-center md:rounded-2xl md:p-3",
            class: "hover:cursor-pointer",
            style: "{row_border_style(props.idx)} --rank-accent: {accent_color};",
            onclick: move |_| {
                navigator.push(route.clone());
            },
            div {
                class: "rank-card-number w-6 flex-shrink-0 font-mono text-sm font-bold transition-colors md:w-10 md:text-xl",
                style: "color: color-mix(in oklab, {accent_color} 46%, var(--secondary-color-6));",
                "{(props.idx + 1).to_string()}"
            }
            div { class: "relative mr-2 hidden md:block md:mr-6",
                div {
                    class: "rank-card-avatar h-10 w-10 overflow-hidden rounded-full border bg-primary grayscale transition-all duration-500 group-hover:grayscale-0 sm:h-12 sm:w-12 md:h-14 md:w-14",
                    style: "border-color: color-mix(in oklab, {accent_color} 56%, var(--primary-color-6));",
                    RepoAvatar {
                        repo_id: props.repo.id.clone(),
                        avatar_urls: vec![props.repo.avatar_url.clone()],
                        size: AvatarImageSize::Custom,
                        class: "h-full w-full border-none bg-transparent".to_string(),
                        fallback_class: "flex h-full w-full items-center justify-center border-none bg-primary-2 text-[10px] font-bold text-secondary-4"
                            .to_string(),
                    }
                }
            }
            div { class: "mr-2 min-w-0 flex-grow md:mr-6",
                h4 { class: "rank-card-title line-clamp-1 font-sans text-[11px] font-black uppercase tracking-tight text-secondary-2 transition-colors sm:text-xs md:text-sm",
                    "{props.repo.name}"
                }
                p { class: "mt-0 line-clamp-2 font-serif text-[10px] italic text-secondary-5 md:line-clamp-1 md:text-[11px]",
                    "{props.repo.description}"
                }
                div { class: "mt-1.5 hidden min-h-[18px] flex-wrap items-center gap-1 sm:flex",
                    for tag in tag_items {
                        HoverCard { key: "{tag.value}",
                            HoverCardTrigger {
                                button {
                                    r#type: "button",
                                    class: "rank-card-tag inline-flex items-center rounded-full border px-1.5 py-0.5 text-[9px] font-mono uppercase tracking-wide text-secondary-4",
                                    style: "border-color: color-mix(in oklab, {accent_color} 34%, var(--primary-color-6)); background: color-mix(in oklab, {accent_color} 10%, var(--primary-color));",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        evt.prevent_default();
                                    },
                                    "{tag.label}"
                                }
                            }
                            HoverCardContent { side: ContentSide::Bottom,
                                div {
                                    class: "w-full",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        evt.prevent_default();
                                    },
                                    TagContent { value: tag.value }
                                }
                            }
                        }
                    }
                    if more_tags > 0 {
                        span {
                            class: "text-[10px] font-mono uppercase tracking-wide",
                            style: "color: color-mix(in oklab, {accent_color} 76%, var(--secondary-color-4));",
                            "+{more_tags}"
                        }
                    }
                    if !has_tag_items && more_tags == 0 {
                        span { class: "inline-flex items-center rounded-full px-1.5 py-0.5 text-[9px] font-mono uppercase tracking-wide opacity-0 pointer-events-none select-none",
                            "placeholder"
                        }
                    }
                }
            }
            div { class: "flex flex-shrink-0 flex-col items-end gap-1",
                div { class: "flex max-w-[82px] items-center gap-1 font-mono text-[10px] font-bold text-secondary-2 sm:max-w-[110px] sm:gap-1.5 sm:text-xs md:max-w-none md:text-sm",
                    span { class: "block max-w-[62px] truncate sm:max-w-[84px] md:max-w-none",
                        "{stat_text}"
                    }
                    span {
                        class: "rank-card-icon inline-flex items-center",
                        style: "color: color-mix(in oklab, {accent_color} 84%, var(--secondary-color-2));",
                        {stat_icon_list(props.active_tab)}
                    }
                }
                div { class: "rank-card-tail h-[1px] w-3 bg-primary-6 transition-all duration-500 group-hover:w-6 md:w-4 md:group-hover:w-8" }
            }
            div { class: "absolute inset-0 bg-screentone opacity-[0.01] pointer-events-none" }
        }
    }
}
