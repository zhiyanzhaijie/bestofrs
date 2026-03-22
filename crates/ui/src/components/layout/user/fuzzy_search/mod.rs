use dioxus::prelude::*;
use dioxus_sdk_time::use_debounce;
use dioxus_use_js::use_js;

use crate::components::icons::{CommandIcon, SearchIcon};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{DialogContent, DialogRoot};
use crate::components::ui::input::Input;
use crate::root::Route;
use crate::types::search::{SearchRepoDto, SearchResultDto};
use crate::types::tags::TagDto;
use crate::IO::repos::search_repos;
use app::prelude::Pagination;

use repo_row::RepoRow;
use skeleton::{FuzzySearchCachedFallback, FuzzySearchIdleFallback};
use tag_row::TagRow;

mod repo_row;
mod skeleton;
mod tag_row;

use_js!("src/js/dom_bridge.js"::mount_fuzzy_search_hotkey);

fn empty_result(page: Pagination) -> SearchResultDto {
    SearchResultDto {
        repos: page.to_page(Vec::new(), 0),
        tags: page.to_page(Vec::new(), 0),
    }
}
#[derive(Clone, PartialEq, Eq)]
pub(super) struct FuzzySearchCachedResult {
    pub(super) repos: Vec<SearchRepoDto>,
    pub(super) tags: Vec<TagDto>,
}

fn repo_route(repo: &SearchRepoDto) -> Route {
    match repo.id.split_once('/') {
        Some((owner, name)) => Route::RepoDetailView {
            owner: owner.to_string(),
            name: name.to_string(),
            metric: None,
        },
        None => Route::HomeView {},
    }
}

#[derive(Clone, PartialEq)]
enum FuzzySearchResultState {
    Loading,
    Ready {
        repos: Vec<SearchRepoDto>,
        tags: Vec<TagDto>,
    },
    Error(String),
}

#[derive(Props, Clone, PartialEq)]
struct FuzzySearchResultIOProps {
    draft: String,
    state: FuzzySearchResultState,
    cached: Option<FuzzySearchCachedResult>,
    on_repo_select: Callback<Route>,
    on_tag_select: Callback<(String, String)>,
}

#[component]
pub(super) fn FuzzySearchResultList(
    repos: Vec<SearchRepoDto>,
    tags: Vec<TagDto>,
    on_repo_select: Callback<Route>,
    on_tag_select: Callback<(String, String)>,
) -> Element {
    let mut active_tab = use_signal(|| Some("repos".to_string()));
    let active_tab_read: ReadSignal<Option<String>> = active_tab.into();

    rsx! {
        Tabs {
            class: "flex h-full min-h-0 flex-col gap-2".to_string(),
            variant: TabsVariant::Ghost,
            value: active_tab_read,
            default_value: "repos".to_string(),
            on_value_change: move |value| active_tab.set(Some(value)),
            TabList {
                TabTrigger { value: "repos".to_string(), index: 0usize, "Repos ({repos.len()})" }
                TabTrigger { value: "tags".to_string(), index: 1usize, "Tags ({tags.len()})" }
            }
            TabContent {
                value: "repos".to_string(),
                index: 0usize,
                class: "p-0 flex min-h-0 flex-1".to_string(),
                ul { class: "min-h-0 flex-1 overflow-y-auto space-y-1",
                    {
                        repos
                            .into_iter()
                            .map(|repo| {
                                let route = repo_route(&repo);
                                rsx! {
                                    RepoRow {
                                        key: "{repo.id}",
                                        repo,
                                        route,
                                        on_select: on_repo_select,
                                    }
                                }
                            })
                    }
                }
            }
            TabContent {
                value: "tags".to_string(),
                index: 1usize,
                class: "p-0 flex min-h-0 flex-1".to_string(),
                ul { class: "min-h-0 flex-1 overflow-y-auto space-y-1",
                    {
                        tags.into_iter()
                            .map(|tag| {
                                rsx! {
                                    TagRow { key: "{tag.label}:{tag.value}", tag, on_select: on_tag_select }
                                }
                            })
                    }
                }
            }
        }
    }
}

#[component]
fn FuzzySearchResultIO(props: FuzzySearchResultIOProps) -> Element {
    if props.draft.trim().is_empty() {
        return rsx! {
            FuzzySearchIdleFallback {}
        };
    }

    match props.state {
        FuzzySearchResultState::Loading => rsx! {
            FuzzySearchCachedFallback {
                cached: props.cached,
                on_repo_select: props.on_repo_select,
                on_tag_select: props.on_tag_select,
            }
        },
        FuzzySearchResultState::Error(error) => rsx! {
            div { class: "p-4 text-center text-sm text-red-500", "{error}" }
        },
        FuzzySearchResultState::Ready { repos, tags } => rsx! {
            FuzzySearchResultList {
                repos,
                tags,
                on_repo_select: props.on_repo_select,
                on_tag_select: props.on_tag_select,
            }
        },
    }
}

#[component]
pub fn FuzzySearch() -> Element {
    let mut open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut last_success = use_signal(|| None::<FuzzySearchCachedResult>);
    let navigator = use_navigator();
    let search_trigger_id = "fuzzy-search-trigger";

    let mut close_dialog = move || {
        open.set(false);
        draft.set(String::new());
    };

    let mut go_repo = move |route: Route| {
        close_dialog();
        navigator.push(route);
    };
    let mut go_tag = move |pair: (String, String)| {
        close_dialog();
        navigator.push(Route::RepoListView {
            tags: Some(pair.1),
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

    let mut debounce_search = use_debounce(
        std::time::Duration::from_millis(300),
        move |text: String| {
            if !open() {
                return;
            }
            search.call(text);
        },
    );

    let mut open_dialog = move || {
        if open() {
            return;
        }
        open.set(true);
    };

    let mut on_draft_change = move |value: String| {
        draft.set(value.clone());
        debounce_search.action(value);
    };

    use_effect(move || {
        let search_trigger_id = search_trigger_id.to_string();
        spawn(async move {
            let _ = mount_fuzzy_search_hotkey::<()>(search_trigger_id).await;
        });
    });

    let result_state = match search.value() {
        Some(Ok(res)) => FuzzySearchResultState::Ready {
            repos: res().repos.items.clone(),
            tags: res().tags.items.clone(),
        },
        Some(Err(err)) => FuzzySearchResultState::Error(err.to_string()),
        None => FuzzySearchResultState::Loading,
    };

    match &result_state {
        FuzzySearchResultState::Ready { repos, tags } => {
            let cached = FuzzySearchCachedResult {
                repos: repos.clone(),
                tags: tags.clone(),
            };
            if (last_success)().as_ref() != Some(&cached) {
                last_success.set(Some(cached));
            }
        }
        FuzzySearchResultState::Error(_) => {
            if (last_success)().is_some() {
                last_success.set(None);
            }
        }
        FuzzySearchResultState::Loading => {}
    }

    rsx! {
        div { class: "inline-flex items-center gap-2 text-sm text-secondary-4",
            Button {
                id: search_trigger_id,
                variant: ButtonVariant::Outline,
                class: "button inline-flex h-10 w-full items-center justify-center !rounded-lg border-primary-6 !px-2",
                onclick: move |_| {
                    open_dialog();
                },
                span { class: "inline-flex w-full items-center justify-center gap-2",
                    SearchIcon { width: 16, height: 16 }
                    span { class: "inline-flex items-center px-2 py-1 rounded-md gap-1 bg-secondary-6/40",
                        CommandIcon { width: 16, height: 16 }
                        "K"
                    }
                }
            }
        }

        DialogRoot {
            id: None,
            open: open(),
            on_open_change: move |v| {
                open.set(v);
                if !v {
                    close_dialog();
                }
            },
            DialogContent { style: "top: 15%; transform: translate(-50%, 0); height: min(80vh, 34rem); max-height: min(80vh, 34rem); overflow: hidden;",
                Input {
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
                    placeholder: "Search Projects and Tags...",
                    value: draft,
                    aria_label: "Search",
                    children: rsx! {},
                }

                div { class: "flex-1 min-h-0 w-full overflow-hidden",
                    FuzzySearchResultIO {
                        draft: draft(),
                        state: result_state,
                        cached: (last_success)(),
                        on_repo_select: move |route| {
                            go_repo(route);
                        },
                        on_tag_select: move |pair| {
                            go_tag(pair);
                        },
                    }
                }
            }
        }
    }
}
