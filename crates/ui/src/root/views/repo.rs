use dioxus::prelude::*;

use crate::components::IOCell;
use crate::IO::repos::{get_repo, list_repo_deltas, list_repo_snapshots};
use app::prelude::Pagination;

#[component]
pub fn RepoDetail(owner: String, name: String) -> Element {
    rsx! {
        IOCell {
            RepoDetailContent { owner, name }
        }
    }
}

#[component]
fn RepoDetailContent(owner: String, name: String) -> Element {
    let navigator = use_navigator();

    let page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let mut repo_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || get_repo(owner.clone(), name.clone())
    })?;

    let mut snapshots_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || list_repo_snapshots(owner.clone(), name.clone(), page)
    })?;

    let mut deltas_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || list_repo_deltas(owner.clone(), name.clone(), page)
    })?;

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-6 space-y-6",
            div { class: "flex items-center justify-between gap-4",
                div { class: "space-y-1 min-w-0",
                    h1 { class: "text-2xl font-semibold tracking-tight truncate", "{owner}/{name}" }
                    p { class: "text-sm text-secondary-5", "repo detail" }
                }

                div { class: "flex items-center gap-2",
                    button {
                        class: "text-sm text-secondary-5 hover:underline",
                        onclick: move |_| {
                            navigator.go_back();
                        },
                        "返回"
                    }
                    button {
                        class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-4",
                        onclick: move |_| {
                            repo_fut.restart();
                            snapshots_fut.restart();
                            deltas_fut.restart();
                        },
                        "刷新"
                    }
                }
            }

                section { class: "rounded-xl border border-primary-6 bg-primary-3 p-4 space-y-3",
                h2 { class: "text-lg font-semibold", "Repo" }
                match repo_fut() {
                    Some(Ok(Some(repo))) => rsx! {
                        div { class: "grid grid-cols-1 gap-2 text-sm sm:grid-cols-2",
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "id" }
                                span { class: "font-medium", "{repo.id}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "github_repo_id" }
                                span { class: "font-medium", "{repo.github_repo_id:?}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "stars" }
                                span { class: "font-medium", "{repo.stars}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "forks" }
                                span { class: "font-medium", "{repo.forks}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "open_issues" }
                                span { class: "font-medium", "{repo.open_issues}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "watchers" }
                                span { class: "font-medium", "{repo.watchers}" }
                            }
                        }
                        if let Some(full_name) = repo.full_name {
                            div { class: "text-sm text-secondary-5", "full_name: {full_name}" }
                        }
                        if let Some(last) = repo.last_fetched_at {
                            div { class: "text-sm text-secondary-5", "last_fetched_at: {last}" }
                        }
                        if !repo.tags.is_empty() {
                            div { class: "flex flex-wrap gap-1 text-xs",
                                for tag in repo.tags {
                                    span { class: "rounded-full border border-primary-6 bg-primary-2 px-2 py-0.5",
                                        "{tag.label}:{tag.value}"
                                    }
                                }
                            }
                        }
                    },
                    Some(Ok(None)) => rsx! {
                        div { class: "text-sm text-secondary-5", "未找到 repo" }
                    },
                    Some(Err(e)) => Err(e)?,
                    None => rsx! {
                        div { class: "text-sm text-secondary-5", "Loading..." }
                    },
                }
            }

            div { class: "grid grid-cols-1 gap-6 lg:grid-cols-2",
                section { class: "rounded-xl border border-primary-6 bg-primary-3 p-4 space-y-3",
                    h2 { class: "text-lg font-semibold", "Snapshots" }
                    match snapshots_fut() {
                        Some(Ok(page)) => rsx! {
                            div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
                            div { class: "max-h-[520px] overflow-auto space-y-2",
                                for s in page.items {
                                    div {
                                        key: "{s.snapshot_date}",
                                        class: "rounded-md border border-primary-6 bg-primary-2 px-3 py-2 text-sm",
                                        div { class: "flex items-center justify-between",
                                            span { class: "font-medium", "{s.snapshot_date}" }
                                            span { class: "text-secondary-5", "stars: {s.stars}" }
                                        }
                                        div { class: "text-xs text-secondary-5",
                                            "forks: {s.forks} | issues: {s.open_issues} | watchers: {s.watchers}"
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => Err(e)?,
                        None => rsx! {
                            div { class: "text-sm text-secondary-5", "Loading..." }
                        },
                    }
                }

                section { class: "rounded-xl border border-primary-6 bg-primary-3 p-4 space-y-3",
                    h2 { class: "text-lg font-semibold", "Deltas" }
                    match deltas_fut() {
                        Some(Ok(page)) => rsx! {
                            div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
                            div { class: "max-h-[520px] overflow-auto space-y-2",
                                for d in page.items {
                                    div {
                                        key: "{d.snapshot_date}",
                                        class: "rounded-md border border-primary-6 bg-primary-2 px-3 py-2 text-sm",
                                        div { class: "flex items-center justify-between",
                                            span { class: "font-medium", "{d.snapshot_date}" }
                                            span { class: "text-secondary-5", "prev: {d.prev_snapshot_date:?}" }
                                        }
                                        div { class: "text-xs text-secondary-5",
                                            "stars: {d.stars_delta:?} | forks: {d.forks_delta:?} | issues: {d.open_issues_delta:?} | watchers: {d.watchers_delta:?}"
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => Err(e)?,
                        None => rsx! {
                            div { class: "text-sm text-secondary-5", "Loading..." }
                        },
                    }
                }
            }
        }
    }
}
