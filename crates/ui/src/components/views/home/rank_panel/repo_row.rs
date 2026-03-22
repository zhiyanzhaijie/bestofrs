use crate::components::common::RepoAvatar;
use crate::components::common::TagContent;
use crate::components::ui::avatar::AvatarImageSize;
use crate::components::ui::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};
use dioxus::prelude::*;

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
    let route = parse_repo_route(&props.repo.id, props.active_tab);
    let stat_text = stat_value(&props.repo, props.active_tab);
    let accent_color = rainbow_color(props.idx);
    let tag_items = props.repo.tags.iter().take(3).cloned().collect::<Vec<_>>();
    let has_tag_items = !tag_items.is_empty();
    let more_tags = props.repo.tags.len().saturating_sub(3);
    let border_left_style = row_border_style(props.idx);
    let card_style = format!("{border_left_style} --rank-accent: {accent_color};");
    let rank_no_style =
        format!("color: color-mix(in oklab, {accent_color} 46%, var(--secondary-color-6));");
    let avatar_style =
        format!("border-color: color-mix(in oklab, {accent_color} 56%, var(--primary-color-6));");
    let stat_icon_style =
        format!("color: color-mix(in oklab, {accent_color} 84%, var(--secondary-color-2));");
    let tag_chip_style = format!(
        "border-color: color-mix(in oklab, {accent_color} 34%, var(--primary-color-6));\
         background: color-mix(in oklab, {accent_color} 10%, var(--primary-color));"
    );
    let tag_more_style =
        format!("color: color-mix(in oklab, {accent_color} 76%, var(--secondary-color-4));");
    let card_class =
        "rank-card bg-primary border-l-4 border-y border-r border-primary-6 shadow-sm transition-all duration-300 flex items-center p-3 group relative rounded-2xl h-[86px]";
    let detail = rsx! {
        div { class: "{card_class}", style: "{card_style}",
            div { class: "rank-card-number w-10 flex-shrink-0 font-mono font-bold transition-colors text-xl", style: "{rank_no_style}",
                "{(props.idx + 1).to_string()}"
            }
            div { class: "relative mr-6",
                div { class: "rank-card-avatar w-14 h-14 overflow-hidden rounded-full border bg-primary grayscale transition-all duration-500 group-hover:grayscale-0", style: "{avatar_style}",
                    RepoAvatar {
                        repo_id: props.repo.id.clone(),
                        avatar_urls: vec![props.repo.avatar_url.clone()],
                        size: AvatarImageSize::Custom,
                        class: "h-full w-full border-none bg-transparent".to_string(),
                        fallback_class: "flex h-full w-full items-center justify-center border-none bg-primary-2 text-[10px] font-bold text-secondary-4".to_string(),
                    }
                }
            }
            div { class: "flex-grow min-w-0 mr-6",
                h4 { class: "rank-card-title font-black text-sm font-sans uppercase tracking-tight text-secondary-2 transition-colors line-clamp-1",
                    "{props.repo.name}"
                }
                p { class: "text-[11px] text-secondary-5 font-serif italic line-clamp-1 mt-0",
                    "{props.repo.description}"
                }
                div { class: "mt-1.5 flex flex-wrap items-center gap-1 min-h-[18px]",
                    for tag in tag_items {
                        HoverCard {
                            key: "{tag.value}",
                            div {
                                onclick: move |evt| evt.stop_propagation(),
                                HoverCardTrigger {
                                    span {
                                        class: "rank-card-tag inline-flex items-center rounded-full border px-1.5 py-0.5 text-[9px] font-mono uppercase tracking-wide text-secondary-4",
                                        style: "{tag_chip_style}",
                                        "{tag.label}"
                                    }
                                }
                            }
                            HoverCardContent {
                                side: dioxus_primitives::ContentSide::Bottom,
                                div {
                                    onclick: move |evt| evt.stop_propagation(),
                                    TagContent { value: tag.value }
                                }
                            }
                        }
                    }
                    if more_tags > 0 {
                        span { class: "text-[10px] font-mono uppercase tracking-wide", style: "{tag_more_style}",
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
            div { class: "flex flex-col items-end gap-1 flex-shrink-0",
                div { class: "text-sm font-mono font-bold text-secondary-2 flex items-center gap-1.5",
                    "{stat_text}"
                    span { class: "rank-card-icon inline-flex items-center", style: "{stat_icon_style}",
                        {stat_icon_list(props.active_tab)}
                    }
                }
                div { class: "rank-card-tail w-4 h-[1px] bg-primary-6 group-hover:w-8 transition-all duration-500" }
            }
            div { class: "absolute inset-0 bg-screentone opacity-[0.01] pointer-events-none" }
        }
    };
    if let Some(route) = route {
        rsx! {
            Link { to: route, class: "contents", {detail} }
        }
    } else {
        rsx! { {detail} }
    }
}
