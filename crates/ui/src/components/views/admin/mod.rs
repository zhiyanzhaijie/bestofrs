mod ingest_daily_snapshots_control;
mod project_management;

use dioxus::prelude::*;
use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper, IOCell,
};

use self::ingest_daily_snapshots_control::IngestDailySnapshotsControl;
use self::project_management::ProjectManagement;

#[component]
pub fn Admin() -> Element {
    rsx! {
        IOCell { AdminContent {} }
    }
}

#[component]
fn AdminContent() -> Element {
    rsx! {
        div { class: "space-y-0",
            GridWrapper {
                grid_type: GridType::Default,
                padding: GridPadding::Sm,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Slash,
                    gradient: GradientDirection::None,
                },
                div { class: "space-y-3 bg-primary",
                    div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "ADMIN / CONTROL PANEL" }
                    h1 { class: "text-3xl font-bold tracking-tight text-secondary-2", "Admin" }
                    p { class: "border-l-2 border-primary-6 pl-4 text-sm text-secondary-5",
                        "项目管理 / 本地手动触发 ingest（生产环境默认禁止）"
                    }
                }
            }
            GridSlashTransition {}
            GridWrapper { padding: GridPadding::Sm,
                div { class: "grid grid-cols-1 gap-6 bg-primary-1 lg:grid-cols-2",
                    ProjectManagement {}
                    IngestDailySnapshotsControl {}
                }
            }
        }
    }
}
