mod context;
mod edit_panel;
mod project_table;
mod search;

use dioxus::prelude::*;

use crate::components::common::{
    CommonPagination, GridSlashTransition, GridType, GridWrapper, IOCell,
};
use context::{ProjectPanelMode, ProjectTablePaginationState, ProjectsContext};
use edit_panel::EditPanel;
use project_table::skeleton::ProjectTableSkeleton;
use project_table::ProjectTable;
use search::ProjectsSearch;

#[component]
pub fn Projects() -> Element {
    let refresh = use_signal(|| 0u32);
    let mut search_key = use_signal(String::new);
    let mut panel_mode = use_signal(|| Option::<ProjectPanelMode>::None);
    let edit_panel_tab = use_signal(|| Some("project".to_string()));
    let mut table_pagination = use_signal(ProjectTablePaginationState::default);
    use_context_provider(|| ProjectsContext {
        refresh,
        search_key,
        edit_panel_tab,
        table_pagination,
    });

    rsx! {
        section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",

            GridWrapper { is_dot_on: true, grid_type: GridType::Inner,
                div { class: "space-y-1 mb-10",
                    h2 { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5",
                        "PROJECTS / MANAGEMENT"
                    }
                    p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                        "右侧面板打开时，左侧保留 reponame 与 edit，便于快速切换。"
                    }
                }
                ProjectsSearch {
                    on_add: move |_| panel_mode.set(Some(ProjectPanelMode::Add)),
                    on_search: move |key| search_key.set(key),
                    on_clear: move |_| search_key.set(String::new()),
                }
            }

            GridSlashTransition {}

            div { class: "flex items-start gap-4 overflow-x-auto",
                div { class: if panel_mode().is_some() { "w-[420px] shrink-0 space-y-3" } else { "min-w-0 flex-1 space-y-3" },
                    if table_pagination().total_pages > 1 {
                        div { class: "flex justify-center mb-10",
                            CommonPagination {
                                current_page: table_pagination().current_page,
                                total_pages: table_pagination().total_pages,
                                on_page_change: move |p| table_pagination.with_mut(|state| state.current_page = p),
                            }
                        }
                    }
                    IOCell {
                        loading_fallback: rsx! {
                            ProjectTableSkeleton {}
                        },
                        ProjectTable {
                            panel_open: panel_mode().is_some(),
                            on_edit: move |project| panel_mode.set(Some(ProjectPanelMode::Edit(project))),
                        }
                    }
                }
                if let Some(mode) = panel_mode() {
                    EditPanel { mode, on_close: move |_| panel_mode.set(None) }
                }
            }

        }
    }
}
