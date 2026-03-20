use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

use super::super::TagListContext;
use super::TagListContent;

#[component]
pub(crate) fn TagListCachedFallback() -> Element {
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
