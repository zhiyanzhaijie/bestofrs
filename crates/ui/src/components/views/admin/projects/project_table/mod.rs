pub(crate) mod skeleton;

use dioxus::prelude::*;

use crate::components::button::ButtonVariant;
use crate::components::icons::{GithubIcon, TrashIcon, WrenchIcon};
use crate::components::ui::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::ui::switch::{Switch, SwitchThumb};
use app::prelude::Pagination;

use crate::components::ui::button::Button;
use crate::types::projects::ProjectDto;
use crate::IO::projects::{
    list_disabled_projects, list_projects, remove_project, search_disabled_projects,
    search_projects,
};

use super::context::ProjectsContext;

#[derive(Props, Clone, PartialEq)]
pub(super) struct ProjectTableProps {
    pub panel_open: bool,
    pub on_edit: Callback<ProjectDto>,
}

#[component]
pub(super) fn ProjectTable(props: ProjectTableProps) -> Element {
    let mut refresh = use_context::<ProjectsContext>().refresh;
    let search_key = use_context::<ProjectsContext>().search_key;
    let mut table_pagination = use_context::<ProjectsContext>().table_pagination;
    let mut show_disabled_only = use_signal(|| false);
    let mut last_search_key = use_signal(String::new);
    let mut last_show_disabled_only = use_signal(|| false);
    let mut remove_pending = use_signal(|| false);
    let mut table_message = use_signal(|| Option::<String>::None);
    let mut delete_confirm_open = use_signal(|| false);
    let mut delete_target_repo_id = use_signal(|| Option::<String>::None);
    let mut cached_items = use_signal(Vec::<ProjectDto>::new);
    let mut has_loaded_once = use_signal(|| false);

    use_effect(move || {
        let mut should_reset_items = false;
        let key = search_key();
        if last_search_key() != key {
            last_search_key.set(key);
            table_pagination.with_mut(|p| p.current_page = 1);
            should_reset_items = true;
        }

        let show_disabled = show_disabled_only();
        if last_show_disabled_only() != show_disabled {
            last_show_disabled_only.set(show_disabled);
            table_pagination.with_mut(|p| p.current_page = 1);
            should_reset_items = true;
        }

        if should_reset_items {
            cached_items.set(Vec::new());
            has_loaded_once.set(false);
        }
    });

    let projects_page = use_server_future(move || {
        let _ = refresh();
        let key = search_key().trim().to_string();
        let show_disabled = show_disabled_only();
        let pagination_state = table_pagination();
        let page_size = pagination_state.page_size;
        let current_page = pagination_state.current_page.max(1);
        let pagination = Pagination {
            limit: Some(page_size),
            offset: Some(page_size.saturating_mul(current_page.saturating_sub(1))),
        };
        async move {
            if show_disabled {
                if key.is_empty() {
                    list_disabled_projects(pagination).await
                } else {
                    search_disabled_projects(key, pagination).await
                }
            } else if key.is_empty() {
                list_projects(pagination).await
            } else {
                search_projects(key, pagination).await
            }
        }
    })?;

    let page_data = projects_page();
    if let Some(Ok(page)) = page_data.as_ref() {
        if cached_items() != page.items {
            cached_items.set(page.items.clone());
        }
        has_loaded_once.set(true);
        let next_total_pages = page.meta.total_pages;
        let next_total_items = page.meta.total;
        let current = table_pagination();
        if current.total_pages != next_total_pages || current.total_items != next_total_items {
            table_pagination.with_mut(|p| {
                p.total_pages = next_total_pages;
                p.total_items = next_total_items;
            });
        }
    }
    let total_items = table_pagination().total_items;
    let search_on = !search_key().trim().is_empty();
    let display_items = match page_data.as_ref() {
        Some(Ok(page)) => page.items.clone(),
        _ => cached_items(),
    };

    rsx! {
        div { class: "overflow-auto rounded-md border border-primary-6 bg-primary-1",
            div { class: "flex items-center justify-between gap-3 px-3 py-2",
                div { class: "text-xs text-secondary-5", "{total_items} items" }
                label { class: "flex items-center gap-2 text-xs text-secondary-5",
                    "disabled"
                    Switch {
                        checked: show_disabled_only(),
                        aria_label: "Show disabled projects only",
                        on_checked_change: move |next| show_disabled_only.set(next),
                        SwitchThumb {}
                    }
                }
            }
            table { class: "min-w-full text-sm",
                thead { class: "border-b border-primary-6 bg-primary",
                    tr {
                        th { class: "px-3 py-2 text-left font-medium text-secondary-5", "Name" }
                        if !props.panel_open {
                            th { class: "px-3 py-2 text-left font-medium text-secondary-5", "REPO-ID" }
                            th { class: "px-3 py-2 text-left font-medium text-secondary-5", "SLUG" }
                        }
                        th { class: "px-3 py-2 text-right font-medium text-secondary-5", "ACTIONS" }
                    }
                }
                tbody {
                    match page_data {
                        Some(Err(err)) => rsx! {
                            tr { td { class: "px-3 py-6 text-center text-primary-error", colspan: if props.panel_open { "2" } else { "4" }, "{err}" } }
                        },
                        Some(Ok(_)) => {
                            if display_items.is_empty() {
                                rsx! {
                                    tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if props.panel_open { "2" } else { "4" }, if search_on { "无搜索结果" } else { "暂无数据" } } }
                                }
                            } else {
                                rsx! {
                                    for item in display_items.clone() {
                                        tr { key: "{item.id}", class: "border-b border-primary-6 last:border-b-0",
                                            td { class: "px-3 py-2", "{item.name}" }
                                            if !props.panel_open {
                                                td { class: "px-3 py-2 font-mono text-xs text-secondary-5", "{item.repo_id}" }
                                                td { class: "px-3 py-2 text-secondary-5", "{item.slug}" }
                                            }
                                            td { class: "px-3 py-2",
                                                div { class: "flex justify-end gap-2",
                                                    a {
                                                        class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs hover:bg-primary-3",
                                                        href: "https://github.com/{item.repo_id}",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        GithubIcon { width: 14, height: 14 }
                                                    }
                                                    Button {
                                                        variant: ButtonVariant::Secondary,
                                                        class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                        disabled: remove_pending(),
                                                        onclick: {
                                                            let project = item.clone();
                                                            move |_: MouseEvent| props.on_edit.call(project.clone())
                                                        },
                                                        WrenchIcon { width: 14, height: 14 }
                                                    }
                                                    if !props.panel_open {
                                                        Button {
                                                            variant: ButtonVariant::Destructive,
                                                            class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                                            disabled: remove_pending(),
                                                            onclick: {
                                                                let repo_id = item.repo_id.clone();
                                                                move |_: MouseEvent| {
                                                                    delete_target_repo_id.set(Some(repo_id.clone()));
                                                                    delete_confirm_open.set(true);
                                                                }
                                                            },
                                                            TrashIcon { width: 14, height: 14 }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        None => rsx! {
                            if !has_loaded_once() {
                                tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if props.panel_open { "2" } else { "4" }, "Loading..." } }
                            } else if display_items.is_empty() {
                                tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if props.panel_open { "2" } else { "4" }, if search_on { "无搜索结果" } else { "暂无数据" } } }
                            } else {
                                for item in display_items.clone() {
                                    tr { key: "{item.id}", class: "border-b border-primary-6 last:border-b-0",
                                        td { class: "px-3 py-2", "{item.name}" }
                                        if !props.panel_open {
                                            td { class: "px-3 py-2 font-mono text-xs text-secondary-5", "{item.repo_id}" }
                                            td { class: "px-3 py-2 text-secondary-5", "{item.slug}" }
                                        }
                                        td { class: "px-3 py-2",
                                            div { class: "flex justify-end gap-2",
                                                a {
                                                    class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs hover:bg-primary-3",
                                                    href: "https://github.com/{item.repo_id}",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    GithubIcon { width: 14, height: 14 }
                                                }
                                                Button {
                                                    variant: ButtonVariant::Secondary,
                                                    class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                    disabled: remove_pending(),
                                                    onclick: {
                                                        let project = item.clone();
                                                        move |_: MouseEvent| props.on_edit.call(project.clone())
                                                    },
                                                    WrenchIcon { width: 14, height: 14 }
                                                }
                                                if !props.panel_open {
                                                    Button {
                                                        variant: ButtonVariant::Destructive,
                                                        class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                                        disabled: remove_pending(),
                                                        onclick: {
                                                            let repo_id = item.repo_id.clone();
                                                            move |_: MouseEvent| {
                                                                delete_target_repo_id.set(Some(repo_id.clone()));
                                                                delete_confirm_open.set(true);
                                                            }
                                                        },
                                                        TrashIcon { width: 14, height: 14 }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
        if remove_pending() {
            div { class: "text-xs text-secondary-5", "删除中..." }
        }
        if let Some(msg) = table_message() {
            div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
        }
        AlertDialogRoot {
            open: delete_confirm_open(),
            on_open_change: move |v| delete_confirm_open.set(v),
            AlertDialogContent {
                AlertDialogTitle { "确认删除 Project" }
                AlertDialogDescription {
                    if let Some(repo_id) = delete_target_repo_id() {
                        "将删除 project: {repo_id}，此操作不可撤销。"
                    } else {
                        "此操作不可撤销。"
                    }
                }
                AlertDialogActions {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        on_click: move |_| {
                            let Some(target_repo_id) = delete_target_repo_id() else {
                                delete_confirm_open.set(false);
                                return;
                            };
                            delete_confirm_open.set(false);
                            delete_target_repo_id.set(None);
                            *remove_pending.write() = true;
                            *table_message.write() = None;
                            spawn(async move {
                                match remove_project(target_repo_id.clone()).await {
                                    Ok(()) => {
                                        *table_message.write() = Some(format!("已删除 project: {target_repo_id}"));
                                        refresh.with_mut(|v| *v += 1);
                                    }
                                    Err(err) => *table_message.write() = Some(err.to_string()),
                                }
                                *remove_pending.write() = false;
                            });
                        },
                        "Confirm"
                    }
                }
            }
        }
    }
}
