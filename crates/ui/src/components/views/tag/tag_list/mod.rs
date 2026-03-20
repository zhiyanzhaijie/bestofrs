use dioxus::prelude::*;

use super::{TagListCachedPage, TagListContext, TagListSummary, TagRouteItem};
use crate::components::common::CommonPagination;
use crate::types::tags::TagListItemDto;
use crate::IO::repos::list_tags_with_meta;
use tag_row::TagRow;

pub(super) mod skeleton;
mod tag_row;

#[component]
pub(super) fn TagListIO() -> Element {
    let mut ctx = use_context::<TagListContext>();
    let tags = use_server_future(move || {
        list_tags_with_meta(
            Some((ctx.current_page)()),
            Some((ctx.page_size)()),
            None,
            Some(10),
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
        None => rsx! { skeleton::TagListCachedFallback {} },
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
                for (index, tag) in items.into_iter().enumerate() {
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
