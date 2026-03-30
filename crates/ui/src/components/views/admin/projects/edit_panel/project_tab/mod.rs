pub(crate) mod skeleton;

use dioxus::prelude::*;
use domain::ProjectStatus;

use crate::components::icons::{PlusIcon, SaveIcon};

use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::components::ui::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use crate::components::ui::textarea::Textarea;
use crate::IO::projects::{import_projects, import_projects_json, update_projects};
use crate::IO::repos::list_tags_with_meta;
use crate::types::projects::{ProjectDto, ProjectImportItem};
use crate::types::tags::TagDto;

use super::super::context::{ProjectPanelMode, ProjectsContext};

#[derive(Clone, PartialEq)]
struct ProjectFormData {
    repo_id: String,
    name: String,
    slug: String,
    description: String,
    url: Option<String>,
    avatar_url: Option<String>,
    status: ProjectStatus,
    twitter: Option<String>,
    selected_tag_values: Vec<String>,
}

impl Default for ProjectFormData {
    fn default() -> Self {
        Self {
            repo_id: String::new(),
            name: String::new(),
            slug: String::new(),
            description: String::new(),
            url: None,
            avatar_url: None,
            status: ProjectStatus::Unknown,
            twitter: None,
            selected_tag_values: Vec::new(),
        }
    }
}

impl From<&ProjectDto> for ProjectFormData {
    fn from(project: &ProjectDto) -> Self {
        Self {
            repo_id: project.repo_id.clone(),
            name: project.name.clone(),
            slug: project.slug.clone(),
            description: project.description.clone(),
            url: project.url.clone(),
            avatar_url: project.avatar_url.clone(),
            status: project.status,
            twitter: project.twitter.clone(),
            selected_tag_values: Vec::new(),
        }
    }
}

impl ProjectFormData {
    fn from_mode(mode: &ProjectPanelMode) -> Self {
        match mode {
            ProjectPanelMode::Add => Self::default(),
            ProjectPanelMode::Edit(project) => project.into(),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.repo_id.trim().is_empty() || self.name.trim().is_empty() || self.slug.trim().is_empty() {
            return Err("必填字段不能为空（repo_id/name/slug）".to_string());
        }
        Ok(())
    }

    fn to_import_item(&self, is_add_mode: bool) -> ProjectImportItem {
        ProjectImportItem {
            id: None,
            repo_id: self.repo_id.trim().to_string(),
            name: self.name.trim().to_string(),
            slug: self.slug.trim().to_string(),
            description: self.description.clone(),
            url: self.url.clone(),
            avatar_url: self.avatar_url.clone(),
            status: self.status,
            twitter: self.twitter.clone(),
            tags: if is_add_mode && !self.selected_tag_values.is_empty() {
                Some(self.selected_tag_values.clone())
            } else {
                None
            },
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct ProjectFormUi {
    tag_query: String,
    submit_pending: bool,
    json_import_pending: bool,
    panel_message: Option<String>,
    json_import_message: Option<String>,
    json_file_name: String,
    json_file_text: String,
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct ProjectTabProps {
    pub mode: ProjectPanelMode,
    pub busy: Signal<bool>,
}

#[component]
pub(super) fn ProjectTab(props: ProjectTabProps) -> Element {
    let mut refresh = use_context::<ProjectsContext>().refresh;
    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let mut data = use_signal(|| ProjectFormData::from_mode(&props.mode));
    let mut ui = use_signal(ProjectFormUi::default);
    let mut mode_snapshot = use_signal(|| Option::<ProjectPanelMode>::None);
    if mode_snapshot() != Some(props.mode.clone()) {
        mode_snapshot.set(Some(props.mode.clone()));
        data.set(ProjectFormData::from_mode(&props.mode));
        ui.set(ProjectFormUi::default());
    }

    let mut busy = props.busy;
    let next_busy = {
        let ui_state = ui();
        ui_state.submit_pending || ui_state.json_import_pending
    };
    if busy() != next_busy {
        busy.set(next_busy);
    }

    let all_tags = match tags_page() {
        Some(Ok(page)) => page
            .items
            .into_iter()
            .map(|tag| TagDto {
                label: tag.label,
                value: tag.value,
                description: tag.description,
                repos_total: Some(tag.repos_total),
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    let ui_state = ui();
    let data_state = data();
    let tag_filter_key = ui_state.tag_query.trim().to_lowercase();
    let filtered_form_tags = all_tags
        .iter()
        .filter(|tag| {
            if tag_filter_key.is_empty() {
                return true;
            }
            let text = format!(
                "{}:{}",
                tag.label.to_lowercase(),
                tag.value.to_lowercase()
            );
            text.contains(&tag_filter_key)
        })
        .cloned()
        .collect::<Vec<_>>();
    let selected_tag_count = data_state.selected_tag_values.len();
    let submit_pending = ui_state.submit_pending;
    let panel_message = ui_state.panel_message.clone();
    let json_import_pending = ui_state.json_import_pending;
    let json_import_message = ui_state.json_import_message.clone();
    let json_file_name = ui_state.json_file_name.clone();
    let is_add_mode = matches!(&props.mode, ProjectPanelMode::Add);

    rsx! {
        if let ProjectPanelMode::Edit(ref project) = props.mode {
            div { class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs text-secondary-5",
                "editing: {project.repo_id}"
            }
        }
        div { class: "space-y-2",
            Input {
                placeholder: "repo_id (owner/name) *",
                value: data_state.repo_id.clone(),
                disabled: matches!(&props.mode, ProjectPanelMode::Edit(_)),
                oninput: move |e: FormEvent| data.with_mut(|d| d.repo_id = e.value()),
            }
            Input {
                placeholder: "name *",
                value: data_state.name.clone(),
                oninput: move |e: FormEvent| data.with_mut(|d| d.name = e.value()),
            }
            Input {
                placeholder: "slug *",
                value: data_state.slug.clone(),
                oninput: move |e: FormEvent| data.with_mut(|d| d.slug = e.value()),
            }
            Textarea {
                placeholder: "description",
                value: data_state.description.clone(),
                oninput: move |e: FormEvent| data.with_mut(|d| d.description = e.value()),
            }
            Input {
                placeholder: "url",
                value: data_state.url.clone().unwrap_or_default(),
                oninput: move |e: FormEvent| data.with_mut(|d| {
                    d.url = if e.value().trim().is_empty() {
                        None
                    } else {
                        Some(e.value())
                    }
                }),
            }
            Input {
                placeholder: "avatar_url",
                value: data_state.avatar_url.clone().unwrap_or_default(),
                oninput: move |e: FormEvent| data.with_mut(|d| {
                    d.avatar_url = if e.value().trim().is_empty() {
                        None
                    } else {
                        Some(e.value())
                    }
                }),
            }
            Select::<ProjectStatus> {
                class: "select w-full",
                value: Some(data_state.status),
                placeholder: "status",
                on_value_change: move |next: Option<ProjectStatus>| {
                    if let Some(next) = next {
                        data.with_mut(|d| d.status = next);
                    }
                },
                SelectTrigger {
                    class: "select-trigger w-full px-3 py-2 text-sm",
                    aria_label: "project status",
                    SelectValue {}
                }
                SelectList { aria_label: "project status options",
                    SelectGroup {
                        SelectGroupLabel { "status" }
                        SelectOption::<ProjectStatus> {
                            index: 0usize,
                            value: ProjectStatus::Active,
                            text_value: Some("Active".to_string()),
                            "Active"
                            SelectItemIndicator {}
                        }
                        SelectOption::<ProjectStatus> {
                            index: 1usize,
                            value: ProjectStatus::Disabled,
                            text_value: Some("Disabled".to_string()),
                            "Disabled"
                            SelectItemIndicator {}
                        }
                        SelectOption::<ProjectStatus> {
                            index: 2usize,
                            value: ProjectStatus::Unknown,
                            text_value: Some("Unknown".to_string()),
                            "Unknown"
                            SelectItemIndicator {}
                        }
                    }
                }
            }
            Input {
                placeholder: "twitter",
                value: data_state.twitter.clone().unwrap_or_default(),
                oninput: move |e: FormEvent| data.with_mut(|d| {
                    d.twitter = if e.value().trim().is_empty() {
                        None
                    } else {
                        Some(e.value())
                    }
                }),
            }
            if is_add_mode {
                div { class: "space-y-2 rounded-md border border-primary-6 bg-primary p-2",
                    Input {
                        placeholder: "搜索 tags(label/value)",
                        value: ui_state.tag_query.clone(),
                        oninput: move |e: FormEvent| ui.with_mut(|u| u.tag_query = e.value()),
                    }
                    div { class: "flex items-center justify-between gap-2",
                        div { class: "text-xs text-secondary-5", "已选 {selected_tag_count} 项" }
                        Button {
                            class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3",
                            onclick: move |_| data.with_mut(|d| d.selected_tag_values.clear()),
                            "清空已选"
                        }
                    }
                    div { class: "max-h-[180px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                        for tag in filtered_form_tags.clone() {
                            label { key: "project-form-tag-{tag.label}:{tag.value}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                                Input {
                                    r#type: "checkbox",
                                    checked: data_state.selected_tag_values.contains(&tag.value),
                                    onchange: {
                                        let value = tag.value.clone();
                                        move |_| {
                                            data.with_mut(|d| {
                                                if let Some(index) = d.selected_tag_values.iter().position(|x| x == &value) {
                                                    d.selected_tag_values.remove(index);
                                                } else {
                                                    d.selected_tag_values.push(value.clone());
                                                }
                                            });
                                        }
                                    },
                                }
                                span { class: "text-xs", "{tag.label}:{tag.value}" }
                            }
                        }
                    }
                }
            }
        }

        section { class: "space-y-2 border border-primary-6 bg-primary p-3",
            div { class: "text-xs font-mono text-secondary-5", "SINGLE ADD / EDIT" }
            Button {
                class: "w-full rounded-md border border-secondary-2 bg-secondary-2 px-3 py-2 text-sm font-medium text-primary hover:opacity-90 disabled:opacity-50",
                disabled: submit_pending || json_import_pending,
                onclick: move |_| {
                    let is_add = matches!(&props.mode, ProjectPanelMode::Add);
                    let mut form = data();
                    if let ProjectPanelMode::Edit(project) = &props.mode {
                        form.repo_id = project.repo_id.clone();
                    }
                    if let Err(err) = form.validate() {
                        ui.with_mut(|u| u.panel_message = Some(err));
                        return;
                    }

                    let item = form.to_import_item(is_add);
                    ui.with_mut(|u| {
                        u.submit_pending = true;
                        u.panel_message = None;
                    });
                    spawn(async move {
                        let result = if is_add {
                            import_projects(vec![item]).await
                        } else {
                            update_projects(vec![item]).await
                        };
                        match result {
                            Ok(res) => {
                                ui.with_mut(|u| {
                                    u.panel_message = Some(format!(
                                        "完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                        res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                    ));
                                });
                                refresh.with_mut(|v| *v += 1);
                            }
                            Err(err) => {
                                ui.with_mut(|u| u.panel_message = Some(err.to_string()));
                            }
                        }
                        ui.with_mut(|u| u.submit_pending = false);
                    });
                },
                if is_add_mode {
                    span { class: "inline-flex items-center gap-1",
                        PlusIcon { width: 16, height: 16 }
                        "Add"
                    }
                } else {
                    span { class: "inline-flex items-center gap-1",
                        SaveIcon { width: 16, height: 16 }
                        "Save"
                    }
                }
            }
            if submit_pending {
                div { class: "text-xs text-secondary-5", "处理中..." }
            }
            if let Some(msg) = panel_message {
                div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
            }
        }

        if is_add_mode {
            section { class: "space-y-2 border border-primary-6 bg-primary p-3",
                div { class: "text-xs font-mono text-secondary-5", "JSON IMPORT" }
                p { class: "text-xs text-secondary-5", "支持上传 JSON 文件并批量导入 project。" }
                Input {
                    r#type: "file",
                    accept: ".json,application/json",
                    disabled: json_import_pending || submit_pending,
                    onchange: move |e: FormEvent| {
                        ui.with_mut(|u| u.json_import_message = None);
                        let files = e.files();
                        let Some(file_data) = files.first().cloned() else {
                            ui.with_mut(|u| u.json_import_message = Some("请选择 JSON 文件".to_string()));
                            return;
                        };
                        let file_name = file_data.name();
                        ui.with_mut(|u| u.json_file_name = file_name.clone());
                        spawn(async move {
                            match file_data.read_string().await {
                                Ok(text) => {
                                    ui.with_mut(|u| {
                                        u.json_file_text = text;
                                        u.json_import_message = Some(format!("已加载文件：{file_name}"));
                                    });
                                }
                                Err(err) => {
                                    ui.with_mut(|u| {
                                        u.json_file_text = String::new();
                                        u.json_import_message = Some(format!("读取文件失败: {err}"));
                                    });
                                }
                            }
                        });
                    },
                }
                if !json_file_name.is_empty() {
                    div { class: "text-xs text-secondary-5", "当前文件：{json_file_name}" }
                }
                Button {
                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                    disabled: json_import_pending || submit_pending,
                    onclick: move |_| {
                        let content = ui().json_file_text.clone();
                        if content.trim().is_empty() {
                            ui.with_mut(|u| u.json_import_message = Some("请先选择并加载 JSON 文件".to_string()));
                            return;
                        }
                        ui.with_mut(|u| {
                            u.json_import_pending = true;
                            u.json_import_message = None;
                        });
                        spawn(async move {
                            match import_projects_json(content).await {
                                Ok(res) => {
                                    ui.with_mut(|u| {
                                        u.json_import_message = Some(format!(
                                            "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                            res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                        ));
                                    });
                                    refresh.with_mut(|v| *v += 1);
                                }
                                Err(err) => {
                                    ui.with_mut(|u| u.json_import_message = Some(err.to_string()));
                                }
                            }
                            ui.with_mut(|u| u.json_import_pending = false);
                        });
                    },
                    "导入 JSON"
                }
                if json_import_pending {
                    div { class: "text-xs text-secondary-5", "导入处理中..." }
                }
                if let Some(msg) = json_import_message {
                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                }
            }
        }
    }
}
