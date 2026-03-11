mod detail;

use dioxus::prelude::*;

use crate::components::views::repo::RepoList;

pub use detail::RepoDetailView;

#[component]
pub fn RepoListView(
    tags: Option<String>,
    metric: Option<String>,
    range: Option<String>,
    page: Option<u32>,
    size: Option<u32>,
) -> Element {
    rsx! { RepoList { tags, metric, range, page, size } }
}
