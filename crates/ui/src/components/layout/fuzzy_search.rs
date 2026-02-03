use dioxus::prelude::*;

use crate::components::ui::dialog::{DialogContent, DialogRoot};
use crate::components::ui::input::Input;
use crate::root::Route;
use crate::types::search::{SearchRepoDto, SearchResultDto};
use crate::IO::repos::search_repos;
use app::prelude::Pagination;

fn empty_result(page: Pagination) -> SearchResultDto {
    SearchResultDto {
        repos: page.to_page(Vec::new(), 0),
        tags: page.to_page(Vec::new(), 0),
    }
}

fn repo_route(repo: &SearchRepoDto) -> Route {
    match repo.id.split_once('/') {
        Some((owner, name)) => Route::RepoDetail {
            owner: owner.to_string(),
            name: name.to_string(),
        },
        None => Route::Home {},
    }
}

#[component]
pub fn FuzzySearch() -> Element {
    let mut open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut query = use_signal(String::new);
    let navigator = use_navigator();
    let mut go_repo = move |route: Route| {
        open.set(false);
        navigator.push(route);
    };
    let mut go_tag = move |label: String, value: String| {
        open.set(false);
        // TODO: decide tag route target
        navigator.push(Route::TagList {});
    };
    let page = Pagination {
        limit: Some(20),
        offset: Some(0),
    };

    let mut result = use_server_future({
        let page = page;
        move || {
            let key = query();
            let is_open = open();
            async move {
                if !is_open {
                    return Ok(empty_result(page));
                }
                search_repos(key, page).await
            }
        }
    })?;

    let mut submit = move || {
        query.set(draft());
        result.restart();
    };
    let mut open_dialog = move || {
        open.set(true);
        result.restart();
    };

    rsx! {
        Input {
            onfocusin: move |_| open_dialog(),
            placeholder: "Search repos / tags",
            value: "",
            readonly: true,
            aria_label: "Open search dialog",
            children: rsx! {},
        }

        DialogRoot {
            id: None,
            open: open(),
            on_open_change: move |v| open.set(v),
                DialogContent {
                    style: "top: 15%; transform: translate(-50%, 0); max-height: 80vh;",
                    Input {
                        class: "input w-full",
                        oninput: move |e: FormEvent| draft.set(e.value()),
                        onkeydown: move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                submit();
                            }
                        },
                        placeholder: "Search repos / tags",
                        value: draft,
                        aria_label: "Search",
                        children: rsx! {},
                    }

                    div { class: "flex-1 overflow-y-auto w-full",
                        match result() {
                                Some(Ok(res)) => {
                                    let repos = res.repos.items;
                                    let tags = res.tags.items;
                                    let is_empty = repos.is_empty() && tags.is_empty();

                                    rsx! {
                                        if is_empty {
                                            div { class: "flex flex-col items-center justify-center py-8 text-secondary-5",
                                                "No results found"
                                            }
                                        } else {
                                            div { class: "space-y-4",
                                                if !repos.is_empty() {
                                                    div { class: "space-y-1",
                                                        div { class: "px-2 text-xs font-semibold text-secondary-5 uppercase tracking-wider",
                                                            "Repositories"
                                                        }
                                                        {repos.into_iter().map(|repo| {
                                                            let route = repo_route(&repo);
                                                            let repo_id = repo.id.clone();
                                                            let full_name = repo.full_name.clone();
                                                            rsx! {
                                                                button {
                                                                    key: "{repo_id}",
                                                                    class: "w-full text-left rounded-md px-3 py-2 text-sm hover:bg-primary-3 transition-colors",
                                                                    onclick: move |_| {
                                                                        go_repo(route.clone());
                                                                    },
                                                                    div { class: "font-medium text-secondary-1 truncate", "{repo_id}" }
                                                                    if let Some(full_name) = full_name {
                                                                        div { class: "text-xs text-secondary-5 truncate", "{full_name}" }
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }

                                                if !tags.is_empty() {
                                                    div { class: "space-y-1",
                                                        div { class: "px-2 text-xs font-semibold text-secondary-5 uppercase tracking-wider",
                                                            "Tags"
                                                        }
                                                        div { class: "flex flex-wrap gap-2 px-2",
                                                            {tags.into_iter().map(|tag| {
                                                                let label = tag.label.clone();
                                                                let value = tag.value.clone();
                                                                rsx! {
                                                                    button {
                                                                        class: "rounded-full border border-primary-6 bg-primary-2 px-2.5 py-0.5 text-xs hover:bg-primary-3 transition-colors text-secondary-4",
                                                                        onclick: move |_| {
                                                                            go_tag(label.clone(), value.clone());
                                                                        },
                                                                        "{label}:{value}"
                                                                    }
                                                                }
                                                            })}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(Err(err)) => rsx! {
                                    div { class: "p-4 text-center text-sm text-red-500", "{err}" }
                                },
                                None => rsx! {
                                    div { class: "p-8 text-center text-sm text-secondary-5", "Loading..." }
                                }
                        }
                    }
                }
        }
    }
}
