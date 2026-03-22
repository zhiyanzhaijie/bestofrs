mod about;
mod admin;
mod home;
mod login;
mod repo;
mod tag;

use super::layouts::{AdminLayout, RootLayout, UserLayout};
use about::AboutView;
use admin::{AdminJobView, AdminProjectsView, AdminTagsView};
use dioxus::prelude::*;
use home::HomeView;
use login::LoginView;
use repo::{RepoDetailView, RepoListView};
use tag::TagListView;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[layout(AdminLayout)]
            #[nest("/admin")]
                #[redirect("/", || Route::AdminProjectsView {})]
                #[route("/projects")]
                AdminProjectsView {},
                #[route("/tags")]
                AdminTagsView {},
                #[route("/job")]
                AdminJobView {},
            #[end_nest]
        #[end_layout]
        #[layout(UserLayout)]
            #[route("/")]
            HomeView {},
            #[nest("/repo")]
                #[route("/?:tags&:metric&:range&:page&:size")]
                RepoListView {
                    tags: Option<String>,
                    metric: Option<String>,
                    range: Option<String>,
                    page: Option<u32>,
                    size: Option<u32>,
                },
                #[route("/:owner/:name?:metric")]
                RepoDetailView {
                    owner: String,
                    name: String,
                    metric: Option<String>,
                },
            #[end_nest]
            #[route("/tag")]
            TagListView {},
            #[route("/login")]
            LoginView {},
            #[route("/about")]
            AboutView {}
}

impl Route {
    pub fn is_auth(&self) -> bool {
        matches!(
            self,
            Route::AdminProjectsView {}
                | Route::AdminTagsView {}
                | Route::AdminJobView {}
                | Route::LoginView {}
        )
    }
}
