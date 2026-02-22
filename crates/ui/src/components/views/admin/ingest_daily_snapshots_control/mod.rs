use dioxus::prelude::*;

use crate::IO::admin::run_ingest_daily_snapshots;
use app::prelude::IngestDailySnapshotsResult;

#[component]
pub fn IngestDailySnapshotsControl() -> Element {
    let mut run_nonce = use_signal(|| 0u32);

    let run_result = use_server_future(move || {
        let n = run_nonce();
        async move {
            if n == 0 {
                Ok::<Option<IngestDailySnapshotsResult>, ServerFnError>(None)
            } else {
                run_ingest_daily_snapshots().await.map(Some)
            }
        }
    })?;

    rsx! {
        section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "JOBS / SNAPSHOT INGEST" }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Ingest Daily Snapshots" }
                p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                    "用于本地开发：手动触发一次 ingest（production 环境会返回 403）。"
                }
            }

            button {
                class: "inline-flex items-center justify-center border border-secondary-2 bg-secondary-2 px-4 py-2 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                onclick: move |_| run_nonce.with_mut(|v| *v += 1),
                "Run once"
            }
            div { class: "pt-2 border-t border-dashed border-primary-6",
                match run_result() {
                    Some(Ok(Some(res))) => rsx! {
                        div { class: "grid grid-cols-1 gap-2 text-sm",
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "projects" }
                                span { class: "font-medium", "{res.projects}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "repos_upserted" }
                                span { class: "font-medium", "{res.repos_upserted}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "snapshots_inserted" }
                                span { class: "font-medium", "{res.snapshots_inserted}" }
                            }
                        }
                    },
                    Some(Ok(None)) => rsx! {
                        div { class: "text-sm text-secondary-5", "尚未运行" }
                    },
                    Some(Err(e)) => Err(e)?,
                    None => rsx! {
                        div { class: "text-sm text-secondary-5", "Running..." }
                    },
                }
            }
        }
    }
}
