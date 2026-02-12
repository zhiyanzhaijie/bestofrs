mod ingest_daily_snapshots_control;
mod project_management;

use dioxus::prelude::*;

use crate::components::common::IOCell;

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
        div { class: "py-8 space-y-6",
            div { class: "space-y-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Admin" }
                p { class: "text-sm text-secondary-5",
                    "项目管理 / 本地手动触发 ingest（生产环境默认禁止）"
                }
            }

            div { class: "grid grid-cols-1 gap-6 lg:grid-cols-2",
                ProjectManagement {}
                IngestDailySnapshotsControl {}
            }
        }
    }
}
