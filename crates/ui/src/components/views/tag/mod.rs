use dioxus::prelude::*;

use crate::components::common::{
    CommonPagination, GradientDirection, GridBackground, GridPadding, GridPattern, GridType,
    GridWrapper, IOCell,
};
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use crate::components::skeleton::Skeleton;
use crate::types::tags::TagListItemDto;
use crate::IO::repos::list_tags_with_meta;

mod mini_repo_card;
mod tag_row;

use tag_row::TagRow;

#[derive(Clone, PartialEq, Eq)]
struct TagRouteItem {
    label: String,
    value: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct TagListSummary {
    from: u64,
    to: u64,
    total: u64,
}

impl TagListSummary {
    fn empty() -> Self {
        Self {
            from: 0,
            to: 0,
            total: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct TagListContext {
    route_tags: Signal<Vec<TagRouteItem>>,
    menu_hovered: Signal<bool>,
    page_size: Signal<u32>,
    current_page: Signal<u32>,
    summary: Signal<TagListSummary>,
    last_success: Signal<Option<TagListCachedPage>>,
}

#[derive(Clone, PartialEq, Eq)]
struct TagListCachedPage {
    items: Vec<TagListItemDto>,
    offset: u32,
    total_pages: u32,
    current_page: u32,
    page_size: u32,
}

#[component]
fn TagRouteMenu() -> Element {
    let mut ctx = use_context::<TagListContext>();
    let hovered = (ctx.menu_hovered)();
    let route_tags = (ctx.route_tags)();

    rsx! {
        div {
            class: "fixed right-6 top-20 z-30 pointer-events-auto w-64 h-[calc(100vh-5rem)]",
            onmouseenter: move |_| ctx.menu_hovered.set(true),
            onmouseleave: move |_| ctx.menu_hovered.set(false),
            div { class: "relative text-right",
                div { class: if hovered { "mb-2 inline-flex h-10 w-10 items-center justify-center text-[var(--grid-accent)] transition-colors" } else { "mb-2 inline-flex h-10 w-10 items-center justify-center text-secondary-6 transition-colors" },
                    svg {
                        width: "18",
                        height: "18",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        line {
                            x1: "4",
                            y1: "7",
                            x2: "20",
                            y2: "7",
                        }
                        line {
                            x1: "4",
                            y1: "12",
                            x2: "20",
                            y2: "12",
                        }
                        line {
                            x1: "4",
                            y1: "17",
                            x2: "20",
                            y2: "17",
                        }
                    }
                }
                div { class: if hovered { "absolute right-0 top-10 w-64 max-h-[calc(100vh-7.5rem)] overflow-auto space-y-1 pr-1 opacity-100 transition-opacity duration-150" } else { "pointer-events-none absolute right-0 top-10 w-64 max-h-0 overflow-hidden opacity-0 transition-opacity duration-150" },
                    if hovered {
                        for item in route_tags {
                            a {
                                key: "{item.value}",
                                href: "#{item.value}",
                                class: "block px-2 py-1 text-sm font-mono text-secondary-6 hover:text-[var(--grid-accent)] transition-colors",
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TagList() -> Element {
    let mut page_size = use_signal(|| 20u32);
    let mut current_page = use_signal(|| 1u32);
    let menu_hovered = use_signal(|| false);
    let route_tags = use_signal(Vec::<TagRouteItem>::new);
    let summary = use_signal(TagListSummary::empty);
    let last_success = use_signal(|| None::<TagListCachedPage>);

    use_context_provider(|| TagListContext {
        route_tags,
        menu_hovered,
        page_size,
        current_page,
        summary,
        last_success,
    });

    rsx! {
        TagRouteMenu {}
        GridWrapper {
            class: "min-h-screen",
            bg_class: "opacity-60",
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Dot,
                gradient: GradientDirection::ToBottom,
            },
            section { class: "relative overflow-visible min-h-screen grid grid-rows-[auto_minmax(0,1fr)] gap-6",
                div { class: "relative",
                    div { class: "flex flex-col items-center text-center gap-4",
                        div { class: "max-w-3xl",
                            h1 { class: "text-2xl md:text-3xl font-black font-sans uppercase tracking-tight text-secondary-2 mb-2",
                                "All Tags"
                            }
                            h2 { class: "text-secondary-3 text-sm md:text-base leading-relaxed font-mono italic font-normal",
                                "A comprehensive tag index of the Rust ecosystem."
                            }
                        }
                        div { class: "flex items-center gap-4 justify-center",
                            div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                "List "
                                span {
                                    class: "font-semibold",
                                    style: "color: var(--grid-accent);",
                                    "{summary().from}-{summary().to}"
                                }
                                " of "
                                span {
                                    class: "font-semibold",
                                    style: "color: var(--grid-accent);",
                                    "{summary().total}"
                                }
                            }
                            div { class: "flex items-center",
                                Select::<u32> {
                                    value: Some(page_size()),
                                    placeholder: "page size",
                                    on_value_change: move |v: Option<u32>| {
                                        if let Some(v) = v {
                                            if v != page_size() {
                                                page_size.set(v);
                                            }
                                            current_page.set(1);
                                        }
                                    },
                                    SelectTrigger {
                                        aria_label: "Select page size",
                                        style: "min-width: 7rem;",
                                        SelectValue {}
                                    }
                                    SelectList { aria_label: "Page size options",
                                        SelectGroup {
                                            SelectGroupLabel { "Page size" }
                                            SelectOption::<u32> {
                                                index: 0usize,
                                                value: 10u32,
                                                text_value: Some("10".to_string()),
                                                "10"
                                                SelectItemIndicator {}
                                            }
                                            SelectOption::<u32> {
                                                index: 1usize,
                                                value: 20u32,
                                                text_value: Some("20".to_string()),
                                                "20"
                                                SelectItemIndicator {}
                                            }
                                            SelectOption::<u32> {
                                                index: 2usize,
                                                value: 50u32,
                                                text_value: Some("50".to_string()),
                                                "50"
                                                SelectItemIndicator {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                IOCell {
                    loading_fallback: rsx! {
                        TagListCachedFallback {}
                    },
                    TagListIO {}
                }
            }
        }
    }
}

#[component]
fn TagListIO() -> Element {
    let mut ctx = use_context::<TagListContext>();
    let tags = use_server_future(move || {
        list_tags_with_meta(
            Some((ctx.current_page)()),
            Some((ctx.page_size)()),
            None,
            Some(5),
        )
    })?;

    match tags() {
        Some(Ok(page)) => {
            let meta = page.meta;
            let items = page.items;
            let visible_total = items.len() as u64;
            let from = if visible_total == 0 {
                0
            } else {
                meta.offset as u64 + 1
            };
            let to = meta.offset as u64 + visible_total;
            let next_summary = TagListSummary {
                from,
                to,
                total: meta.total,
            };
            if (ctx.summary)() != next_summary {
                ctx.summary.set(next_summary);
            }

            let next_route_tags = items
                .iter()
                .map(|tag| TagRouteItem {
                    label: tag.value.clone(),
                    value: tag.value.clone(),
                })
                .collect::<Vec<_>>();
            if (ctx.route_tags)() != next_route_tags {
                ctx.route_tags.set(next_route_tags);
            }

            let cached_page = TagListCachedPage {
                items: items.clone(),
                offset: meta.offset,
                total_pages: meta.total_pages,
                current_page: (ctx.current_page)(),
                page_size: (ctx.page_size)(),
            };
            if (ctx.last_success)().as_ref() != Some(&cached_page) {
                ctx.last_success.set(Some(cached_page));
            }

            rsx! {
                TagListContent {
                    items,
                    offset: meta.offset,
                    total_pages: meta.total_pages,
                    current_page: (ctx.current_page)(),
                    page_size: (ctx.page_size)(),
                }
            }
        }
        Some(Err(e)) => {
            if (ctx.summary)() != TagListSummary::empty() {
                ctx.summary.set(TagListSummary::empty());
            }
            if !(ctx.route_tags)().is_empty() {
                ctx.route_tags.set(Vec::new());
            }
            ctx.last_success.set(None);
            rsx! {
                div { class: "border border-primary-error bg-primary p-4 text-sm text-primary-error",
                    "{e}"
                }
            }
        }
        None => rsx! {
            TagListCachedFallback {}
        },
    }
}

#[component]
fn TagListCachedFallback() -> Element {
    let ctx = use_context::<TagListContext>();
    if let Some(cached) = (ctx.last_success)() {
        rsx! {
            TagListContent {
                items: cached.items,
                offset: cached.offset,
                total_pages: cached.total_pages,
                current_page: cached.current_page,
                page_size: cached.page_size,
            }
        }
    } else {
        rsx! {
            Skeleton { class: "skeleton w-full h-full min-h-[220px] rounded-xl border border-primary-6" }
        }
    }
}

#[component]
fn TagListContent(
    items: Vec<TagListItemDto>,
    offset: u32,
    total_pages: u32,
    current_page: u32,
    page_size: u32,
) -> Element {
    let mut ctx = use_context::<TagListContext>();
    rsx! {
        if items.is_empty() {
            div { class: "flex min-h-[220px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary-1 text-center",
                span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-6",
                    "NO_DATA"
                }
                span { class: "text-sm text-secondary-6", "No tags found" }
            }
        } else {
            div {
                for (index , tag) in items.into_iter().enumerate() {
                    TagRow {
                        key: "{tag.label}:{tag.value}:{offset}:{index}",
                        tag,
                        index,
                        current_page,
                        page_size,
                    }
                }
            }
        }
        if total_pages > 1 {
            div { class: "pt-2",
                CommonPagination {
                    current_page,
                    total_pages,
                    on_page_change: move |p| ctx.current_page.set(p),
                }
            }
        }
    }
}
