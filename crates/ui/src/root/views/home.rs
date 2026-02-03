use dioxus::prelude::*;

use crate::root::Route;
use crate::types::repos::RepoDto;
use crate::IO::repos::list_repos;
use app::prelude::Pagination;

#[component]
pub fn Home() -> Element {
    rsx! { RepoListContent { title: "Home".to_string(), subtitle: "Repo list".to_string() } }
}

#[component]
pub fn RepoList() -> Element {
    rsx! { RepoListContent { title: "Repo".to_string(), subtitle: "All repos".to_string() } }
}

#[component]
fn RepoListContent(title: String, subtitle: String) -> Element {
    let repos = use_server_future(move || {
        list_repos(Pagination {
            limit: Some(200),
            offset: Some(0),
        })
    })?;

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-6 space-y-6",
            div { class: "space-y-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "{title}" }
                p { class: "text-sm text-secondary-5", "{subtitle}" }
            }

            match repos() {
                Some(Ok(page)) => {
                    let total = page.meta.total;
                    let items = page.items;
                    rsx! {
                    div { class: "text-sm text-secondary-5", "total: {total}" }
                    div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3",
                        for r in items {
                            RepoCard { key: "{r.id}", repo: r }
                        }
                    }
                }
                },
                Some(Err(e)) => rsx! { div { class: "text-sm", "{e}" } },
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading..." } },
            }
        }
    }
}

#[component]
fn RepoCard(repo: RepoDto) -> Element {
    let RepoDto {
        id,
        stars,
        forks,
        last_fetched_at,
        tags,
        ..
    } = repo;

    rsx! {
        Link {
            class: "block rounded-xl border border-primary-6 bg-primary-3 p-4 hover:bg-primary-4",
            to: match id.split_once('/') {
                Some((owner, name)) => Route::RepoDetail {
                    owner: owner.to_string(),
                    name: name.to_string(),
                },
                None => Route::Home {},
            },
            div { class: "space-y-2",
                div { class: "font-semibold truncate", "{id}" }
                div { class: "grid grid-cols-2 gap-2 text-sm",
                    div { class: "text-secondary-5", "stars" }
                    div { class: "text-right font-medium", "{stars}" }
                    div { class: "text-secondary-5", "forks" }
                    div { class: "text-right font-medium", "{forks}" }
                }
                if let Some(last) = last_fetched_at {
                    div { class: "text-xs text-secondary-5 truncate", "last: {last}" }
                }
                if tags.is_empty() {
                    div { class: "text-xs text-secondary-5", "tags: -" }
                } else {
                    div { class: "flex flex-wrap gap-1 text-xs",
                        for tag in tags {
                            span { class: "rounded-full border border-primary-6 bg-primary-2 px-2 py-0.5",
                                "{tag.label}:{tag.value}"
                            }
                        }
                    }
                }
            }
        }
    }
}
