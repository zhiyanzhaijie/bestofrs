use dioxus::prelude::*;

use crate::components::views::repo::RepoDetail;

#[component]
pub fn RepoDetailView(owner: String, name: String, metric: Option<String>) -> Element {
    rsx! { RepoDetail { owner, name, metric } }
}
