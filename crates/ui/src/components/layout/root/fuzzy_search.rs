use dioxus::prelude::*;
use dioxus_sdk_time::use_debounce;
use dioxus_use_js::use_js;

use crate::components::ui::dialog::{DialogContent, DialogRoot};
use crate::components::ui::input::Input;
use crate::root::Route;
use crate::types::search::{SearchRepoDto, SearchResultDto};
use crate::IO::repos::search_repos;
use app::prelude::Pagination;

use_js!("src/js/dom_bridge.js"::focus_element_by_id);

fn empty_result(page: Pagination) -> SearchResultDto {
    SearchResultDto {
        repos: page.to_page(Vec::new(), 0),
        tags: page.to_page(Vec::new(), 0),
    }
}

fn repo_route(repo: &SearchRepoDto) -> Route {
    match repo.id.split_once('/') {
        Some((owner, name)) => Route::RepoDetailView {
            owner: owner.to_string(),
            name: name.to_string(),
        },
        None => Route::HomeView {},
    }
}

#[component]
pub fn FuzzySearch() -> Element {
    let mut open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let navigator = use_navigator();
    let search_input_id = "fuzzy-search-input";
    let mut close_dialog = move || {
        open.set(false);
        draft.set(String::new());
    };

    let mut go_repo = move |route: Route| {
        close_dialog();
        navigator.push(route);
    };
    let mut go_tag = move |_label: String, value: String| {
        close_dialog();
        navigator.push(Route::RepoListView {
            tags: Some(value),
            metric: None,
            range: None,
            page: None,
            size: None,
        });
    };
    let page = Pagination {
        limit: Some(20),
        offset: Some(0),
    };

    let mut search = use_action({
        let page = page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_result(page));
            }
            search_repos(key, page).await
        }
    });

    let mut debounce_search = use_debounce(std::time::Duration::from_millis(300), move |text: String| {
        if !open() {
            return;
        }
        search.call(text);
    });

    let mut open_dialog = move || {
        if open() {
            return;
        }
        open.set(true);

        let search_input_id = search_input_id.to_string();
        spawn(async move {
            let _ = focus_element_by_id::<()>(search_input_id).await;
        });
    };

    let mut on_draft_change = move |value: String| {
        draft.set(value.clone());
        debounce_search.action(value);
    };

    rsx! {
        div {
            class: "w-full",
            role: "button",
            tabindex: "0",
            onpointerdown: move |e: PointerEvent| {
                e.prevent_default();
            },
            onclick: move |_| open_dialog(),
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Enter || e.key().to_string() == " " {
                    open_dialog();
                }
            },
            Input {
                class: "input w-full",
                readonly: true,
                value: "",
                placeholder: "Search repos / tags",
                aria_label: "Open search dialog",
                children: rsx! {},
            }
        }

        DialogRoot { id: None, open: open(), on_open_change: move |v| {
                open.set(v);
                if !v {
                    close_dialog();
                }
            },
            DialogContent { style: "top: 15%; transform: translate(-50%, 0); max-height: 80vh;",
                Input {
                    id: search_input_id,
                    class: "input w-full",
                    oninput: move |e: FormEvent| on_draft_change(e.value()),
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Enter {
                            search.call(draft());
                        }
                        if e.key() == Key::Escape {
                            close_dialog();
                        }
                    },
                    placeholder: "Search repos / tags",
                    value: draft,
                    aria_label: "Search",
                    children: rsx! {},
                }

                div { class: "flex-1 overflow-y-auto w-full",
                    if draft().trim().is_empty() {
                        div { class: "p-8 text-center text-sm text-secondary-5", "输入后自动搜索（300ms 防抖）" }
                    } else {
                        match search.value() {
                        Some(Ok(res)) => {
                            let repos = res().repos.items.clone();
                            let tags = res().tags.items.clone();
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
                                                {

                                                    repos
                                                        .into_iter()
                                                        .map(|repo| {
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
                                                        })
                                                }
                                            }
                                        }

                                        if !tags.is_empty() {
                                            div { class: "space-y-1",
                                                div { class: "px-2 text-xs font-semibold text-secondary-5 uppercase tracking-wider",
                                                    "Tags"
                                                }
                                                div { class: "flex flex-wrap gap-2 px-2",
                                                    {
                                                        tags.into_iter()
                                                            .map(|tag| {
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
                                                            })
                                                    }
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
                            div { class: "p-8 text-center text-sm text-secondary-5", "输入后自动搜索（300ms 防抖）" }
                        },
                        }
                    }
                }
            }
        }
    }
}
