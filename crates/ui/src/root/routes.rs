use super::views::{Admin, Home, Layout, RepoDetail, RepoList, TagList};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/repo")]
        RepoList {},
        #[route("/tag")]
        TagList {},
        #[route("/admin")]
        Admin {},
        #[route("/repo/:owner/:name")]
        RepoDetail { owner: String, name: String },
}
