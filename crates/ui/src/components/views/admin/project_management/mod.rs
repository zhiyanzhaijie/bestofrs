use dioxus::prelude::*;

use crate::types::projects::ProjectImportItem;
use crate::IO::projects::{import_projects, import_projects_json, list_projects, remove_project};
use app::prelude::Pagination;

#[component]
pub fn ProjectManagement() -> Element {
    let mut refresh = use_signal(|| 0u32);

    let projects = use_server_future(move || {
        let _ = refresh();
        list_projects(Pagination {
            limit: Some(500),
            offset: Some(0),
        })
    })?;

    let mut repo_id = use_signal(String::new);
    let mut name = use_signal(String::new);
    let mut slug = use_signal(String::new);
    let mut description = use_signal(String::new);

    let mut submitting = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);
    let mut json_file = use_signal(|| Option::<dioxus_elements::FileData>::None);
    let mut json_file_name = use_signal(String::new);

    rsx! {
        section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "PROJECTS / MANAGEMENT" }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Project 管理" }
                p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                    "列表查看 + 单条添加（底层复用 /api/projects/import）"
                }
            }

            div { class: "space-y-3",
                div { class: "space-y-2",
                    input {
                        class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "repo_id (owner/name)",
                        value: repo_id,
                        oninput: move |e| *repo_id.write() = e.value(),
                    }
                    input {
                        class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "name",
                        value: name,
                        oninput: move |e| *name.write() = e.value(),
                    }
                    input {
                        class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "slug",
                        value: slug,
                        oninput: move |e| *slug.write() = e.value(),
                    }
                    textarea {
                        class: "w-full min-h-[96px] rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "description",
                        value: description,
                        oninput: move |e| *description.write() = e.value(),
                    }
                }

                div { class: "border border-primary-6 bg-primary-1 p-4 space-y-2",
                    div { class: "text-sm font-medium", "通过 JSON 文件批量导入" }
                    div { class: "text-xs text-secondary-5",
                        "JSON 格式: [{{\"name\": \"xxx\", \"full_name\": \"owner/name\"}}, ...]"
                    }
                    input {
                        r#type: "file",
                        accept: ".json,application/json",
                        class: "block w-full text-sm text-secondary-5 file:mr-3 file:rounded-md file:border-0 file:bg-primary-3 file:px-3 file:py-2 file:text-sm file:font-medium file:text-secondary-6 hover:file:bg-primary-4",
                        disabled: submitting(),
                        onchange: move |e| {
                            let files = e.files();
                            let Some(file) = files.into_iter().next() else {
                                return;
                            };
                            *json_file.write() = Some(file.clone());
                            *json_file_name.write() = file.name();
                            *message.write() = None;
                        },
                    }
                    if !json_file_name().is_empty() {
                        div { class: "text-xs text-secondary-5",
                            "已选择: {json_file_name}"
                        }
                    }
                    button {
                        class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-xs font-medium text-secondary-5 hover:bg-primary-3 hover:text-secondary-4 disabled:cursor-not-allowed disabled:opacity-50",
                        disabled: submitting() || json_file().is_none(),
                        onclick: move |_| {
                            let Some(file) = json_file() else {
                                *message.write() = Some("请先选择 JSON 文件".to_string());
                                return;
                            };

                            *submitting.write() = true;
                            *message.write() = None;

                            spawn(async move {
                                let text = match file.read_string().await {
                                    Ok(s) => s,
                                    Err(err) => {
                                        *message.write() = Some(err.to_string());
                                        *submitting.write() = false;
                                        return;
                                    }
                                };

                                match import_projects_json(text).await {
                                    Ok(res) => {
                                        let mut msg = format!(
                                            "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                            res.total,
                                            res.upserted,
                                            res.skipped_invalid,
                                            res.failed_upsert
                                        );
                                        if !res.invalid_examples.is_empty() {
                                            msg.push_str("\ninvalid_examples: ");
                                            msg.push_str(&res.invalid_examples.join(", "));
                                        }
                                        if !res.error_examples.is_empty() {
                                            msg.push_str("\nerror_examples: ");
                                            msg.push_str(&res.error_examples.join(" | "));
                                        }
                                        *message.write() = Some(msg);
                                        refresh.with_mut(|v| *v += 1);
                                        *json_file.write() = None;
                                        *json_file_name.write() = String::new();
                                    }
                                    Err(err) => {
                                        *message.write() = Some(err.to_string());
                                    }
                                }

                                *submitting.write() = false;
                            });
                        },
                        "上传导入"
                    }
                }

                div { class: "flex items-center gap-3",
                    button {
                        class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 px-4 py-2 text-sm font-medium text-secondary-5 hover:bg-primary-3 hover:text-secondary-4 disabled:cursor-not-allowed disabled:opacity-50",
                        disabled: submitting(),
                        onclick: move |_| {
                            let repo_id_v = repo_id();
                            let name_v = name();
                            let slug_v = slug();
                            let description_v = description();

                            if repo_id_v.trim().is_empty() || name_v.trim().is_empty() {
                                *message.write() = Some("repo_id 和 name 不能为空".to_string());
                                return;
                            }

                            *submitting.write() = true;
                            *message.write() = None;

                            spawn(async move {
                                let item = ProjectImportItem {
                                    id: None,
                                    repo_id: repo_id_v,
                                    name: name_v,
                                    slug: slug_v,
                                    description: description_v,
                                    override_description: false,
                                    url: None,
                                    override_url: false,
                                    status: None,
                                    logo: None,
                                    twitter: None,
                                    comments: None,
                                };

                                match import_projects(vec![item]).await {
                                    Ok(res) => {
                                        let mut msg = format!(
                                            "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                            res.total,
                                            res.upserted,
                                            res.skipped_invalid,
                                            res.failed_upsert
                                        );
                                        if !res.invalid_examples.is_empty() {
                                            msg.push_str("\ninvalid_examples: ");
                                            msg.push_str(&res.invalid_examples.join(", "));
                                        }
                                        if !res.error_examples.is_empty() {
                                            msg.push_str("\nerror_examples: ");
                                            msg.push_str(&res.error_examples.join(" | "));
                                        }
                                        *message.write() = Some(msg);
                                        refresh.with_mut(|v| *v += 1);
                                    }
                                    Err(e) => {
                                        *message.write() = Some(e.to_string());
                                    }
                                }

                                *submitting.write() = false;
                            });
                        },
                        "添加/导入"
                    }

                    if submitting() {
                        span { class: "text-sm text-secondary-5", "提交中..." }
                    }
                }

                if let Some(msg) = message() {
                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                }
            }

            div { class: "space-y-2 border-t border-dashed border-primary-6 pt-2",
                div { class: "flex items-end justify-between",
                    h3 { class: "text-base font-semibold", "Projects 列表" }
                    match projects() {
                        Some(Ok(page)) => rsx! {
                            span { class: "text-sm text-secondary-5", "total: {page.meta.total}" }
                        },
                        _ => rsx! { span { class: "text-sm text-secondary-5", "" } },
                    }
                }

                match projects() {
                    Some(Ok(page)) => rsx! {
                        div { class: "max-h-[520px] space-y-2 overflow-auto",
                            for p in page.items {
                                div { key: "{p.id}", class: "flex items-center justify-between gap-3 rounded-md border border-primary-6 bg-primary-1 px-3 py-2",
                                    div { class: "min-w-0",
                                        div { class: "font-medium truncate", "{p.name}" }
                                        div { class: "text-xs text-secondary-5 truncate", "{p.repo_id}" }
                                    }
                                    div { class: "flex items-center gap-3 shrink-0",
                                        div { class: "text-xs text-secondary-5", "{p.slug}" }
                                        button {
                                            class: "text-xs text-red-600 hover:underline disabled:cursor-not-allowed disabled:opacity-50",
                                            disabled: submitting(),
                                            onclick: move |_| {
                                                let repo_id = p.repo_id.clone();
                                                *submitting.write() = true;
                                                *message.write() = None;

                                                spawn(async move {
                                                    match remove_project(repo_id.clone()).await {
                                                        Ok(()) => {
                                                            *message.write() = Some(format!("已删除 project: {repo_id}"));
                                                            refresh.with_mut(|v| *v += 1);
                                                        }
                                                        Err(e) => {
                                                            *message.write() = Some(e.to_string());
                                                        }
                                                    }
                                                    *submitting.write() = false;
                                                });
                                            },
                                            "删除"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => Err(e)?,
                    None => rsx! { div { class: "text-sm text-secondary-5", "Loading projects..." } },
                }
            }
        }
    }
}

