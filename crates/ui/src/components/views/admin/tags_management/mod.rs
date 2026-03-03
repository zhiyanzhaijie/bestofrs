use app::prelude::Pagination;
use dioxus::prelude::*;

use crate::types::search::{SearchRepoDto, SearchResultDto};
use crate::types::tags::{TagDto, TagListItemDto};
use crate::IO::repos::{
    bulk_update_repo_tag, create_tag, delete_tag, get_repo, list_tags_with_meta, replace_repo_tags,
    search_repos, update_tag,
};

fn empty_search_result(page: Pagination) -> SearchResultDto {
    SearchResultDto {
        repos: page.to_page(Vec::new(), 0),
        tags: page.to_page(Vec::new(), 0),
    }
}

fn parse_owner_name(repo_id: &str) -> Option<(String, String)> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((owner.to_string(), name.to_string()))
}

#[component]
pub fn TagsManagement() -> Element {
    let mut refresh = use_signal(|| 0u32);
    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let mut selected_tag = use_signal(|| Option::<TagListItemDto>::None);
    let mut selected_tag_description = use_signal(String::new);
    let mut create_label = use_signal(String::new);
    let mut create_value = use_signal(String::new);
    let mut tag_message = use_signal(|| Option::<String>::None);
    let mut tag_pending = use_signal(|| false);

    let page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };
    let mut repo_search_key = use_signal(String::new);
    let mut selected_repo_ids = use_signal(Vec::<String>::new);
    let mut bulk_message = use_signal(|| Option::<String>::None);
    let mut bulk_pending = use_signal(|| false);
    let mut repo_search = use_action({
        let page = page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_search_result(page));
            }
            search_repos(key, page).await
        }
    });

    let mut repo_editor_search_key = use_signal(String::new);
    let mut repo_editor_message = use_signal(|| Option::<String>::None);
    let mut repo_editor_pending = use_signal(|| false);
    let mut repo_editor_selected_repo = use_signal(|| Option::<SearchRepoDto>::None);
    let mut repo_editor_current_tags = use_signal(Vec::<TagDto>::new);
    let mut repo_editor_selected_tags = use_signal(Vec::<TagDto>::new);
    let mut repo_editor_search = use_action({
        let page = page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_search_result(page));
            }
            search_repos(key, page).await
        }
    });

    rsx! {
        div { class: "space-y-6",
            section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
                div { class: "space-y-1",
                    div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "TAGS / LIFECYCLE" }
                    h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "All Tags + Create/Delete" }
                }
                div { class: "grid grid-cols-1 gap-3 md:grid-cols-2",
                    input {
                        class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "new label (e.g. CATEGORY)",
                        value: create_label,
                        oninput: move |e| *create_label.write() = e.value(),
                    }
                    input {
                        class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "new value (e.g. cli)",
                        value: create_value,
                        oninput: move |e| *create_value.write() = e.value(),
                    }
                }
                button {
                    class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                    disabled: tag_pending(),
                    onclick: move |_| {
                        let label = create_label().trim().to_string();
                        let value = create_value().trim().to_string();
                        if label.is_empty() || value.is_empty() {
                            *tag_message.write() = Some("label/value 不能为空".to_string());
                            return;
                        }
                        *tag_pending.write() = true;
                        *tag_message.write() = None;
                        spawn(async move {
                            match create_tag(label, value).await {
                                Ok(()) => {
                                    *tag_message.write() = Some("创建成功".to_string());
                                    refresh.with_mut(|v| *v += 1);
                                }
                                Err(err) => *tag_message.write() = Some(err.to_string()),
                            }
                            *tag_pending.write() = false;
                        });
                    },
                    "创建 Tag"
                }
                match tags_page() {
                    Some(Ok(page)) => rsx! {
                        div { class: "max-h-[300px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                            for tag in page.items {
                                div { key: "{tag.label}:{tag.value}", class: "flex items-center justify-between gap-2 rounded-md border border-primary-6 bg-primary px-2 py-2",
                                    button {
                                        class: if selected_tag().as_ref().map(|x| (&x.label, &x.value))
                                            == Some((&tag.label, &tag.value)) {
                                            "rounded-md border border-secondary-2 bg-secondary-2 px-2 py-1 text-xs font-medium text-primary"
                                        } else {
                                            "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs text-secondary-5 hover:bg-primary-3"
                                        },
                                        onclick: {
                                            let tag_for_select = tag.clone();
                                            move |_| {
                                                selected_tag_description.set(tag_for_select.description.clone().unwrap_or_default());
                                                selected_tag.set(Some(tag_for_select.clone()));
                                            }
                                        },
                                        "{tag.label}:{tag.value}"
                                    }
                                    button {
                                        class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                        disabled: tag_pending(),
                                        onclick: {
                                            let tag_for_delete = tag.clone();
                                            move |_| {
                                                *tag_pending.write() = true;
                                                *tag_message.write() = None;
                                                let target = tag_for_delete.clone();
                                                spawn(async move {
                                                    match delete_tag(target.label.clone(), target.value.clone()).await {
                                                        Ok(()) => {
                                                            if selected_tag().as_ref().map(|x| (&x.label, &x.value))
                                                                == Some((&target.label, &target.value)) {
                                                                selected_tag.set(None);
                                                                selected_tag_description.set(String::new());
                                                            }
                                                            *tag_message.write() = Some("删除成功".to_string());
                                                            refresh.with_mut(|v| *v += 1);
                                                        }
                                                        Err(err) => *tag_message.write() = Some(err.to_string()),
                                                    }
                                                    *tag_pending.write() = false;
                                                });
                                            }
                                        },
                                        "删除"
                                    }
                                }
                            }
                        }
                        if let Some(tag) = selected_tag() {
                            div { class: "space-y-2 rounded-md border border-primary-6 bg-primary p-3",
                                div { class: "text-xs font-mono text-secondary-5", "更新描述: {tag.label}:{tag.value}" }
                                textarea {
                                    class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                    rows: "3",
                                    value: selected_tag_description,
                                    oninput: move |e| *selected_tag_description.write() = e.value(),
                                }
                                button {
                                    class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                                    disabled: tag_pending(),
                                    onclick: move |_| {
                                        let Some(target) = selected_tag() else {
                                            return;
                                        };
                                        *tag_pending.write() = true;
                                        *tag_message.write() = None;
                                        let description = selected_tag_description().trim().to_string();
                                        let description = if description.is_empty() { None } else { Some(description) };
                                        spawn(async move {
                                            match update_tag(target.label.clone(), target.value.clone(), description).await {
                                                Ok(()) => {
                                                    *tag_message.write() = Some("更新成功".to_string());
                                                    refresh.with_mut(|v| *v += 1);
                                                }
                                                Err(err) => *tag_message.write() = Some(err.to_string()),
                                            }
                                            *tag_pending.write() = false;
                                        });
                                    },
                                    "更新描述"
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! { div { class: "text-sm text-primary-error", "{err}" } },
                    None => rsx! { div { class: "text-sm text-secondary-5", "Loading tags..." } },
                }
                if let Some(msg) = tag_message() {
                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                }
            }

            section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
                div { class: "space-y-1",
                    div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "REPO / TAG EDITOR" }
                    h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "搜索 Repo 定向更新 Tag" }
                    p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                        "支持按 repo 维度，将多个 tag 添加到该 repo，或从该 repo 移除。"
                    }
                }
                div { class: "flex flex-col gap-2 md:flex-row",
                    input {
                        class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                        placeholder: "搜索 repo（owner/name）",
                        value: repo_editor_search_key,
                        oninput: move |e| *repo_editor_search_key.write() = e.value(),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                repo_editor_search.call(repo_editor_search_key());
                            }
                        },
                    }
                    button {
                        class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-3",
                        onclick: move |_| repo_editor_search.call(repo_editor_search_key()),
                        "搜索 Repo"
                    }
                }
                if let Some(Ok(result)) = repo_editor_search.value() {
                    {
                        let repos = result().repos.items.clone();
                        rsx! {
                            div { class: "max-h-[220px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                                for repo in repos {
                                    div { key: "{repo.id}", class: "flex items-center justify-between gap-2 rounded-md border border-primary-6 bg-primary px-2 py-2",
                                        div { class: "min-w-0",
                                            div { class: "truncate text-sm font-medium", "{repo.id}" }
                                            if let Some(full_name) = &repo.full_name {
                                                div { class: "truncate text-xs text-secondary-5", "{full_name}" }
                                            }
                                        }
                                        button {
                                            class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3 disabled:opacity-50",
                                            disabled: repo_editor_pending(),
                                            onclick: {
                                                let target_repo = repo.clone();
                                                move |_| {
                                                    *repo_editor_pending.write() = true;
                                                    *repo_editor_message.write() = None;
                                                    repo_editor_selected_repo.set(Some(target_repo.clone()));
                                                    let repo_for_load = target_repo.clone();
                                                    spawn(async move {
                                                        let Some((owner, name)) = parse_owner_name(&repo_for_load.id) else {
                                                            *repo_editor_message.write() = Some(format!("无效 repo id: {}", repo_for_load.id));
                                                            repo_editor_current_tags.set(Vec::new());
                                                            repo_editor_selected_tags.set(Vec::new());
                                                            *repo_editor_pending.write() = false;
                                                            return;
                                                        };

                                                        match get_repo(owner, name).await {
                                                            Ok(Some(repo_detail)) => {
                                                                repo_editor_current_tags.set(repo_detail.tags);
                                                                repo_editor_selected_tags.set(Vec::new());
                                                                *repo_editor_message.write() = Some(format!("已选中 repo: {}", repo_for_load.id));
                                                            }
                                                            Ok(None) => {
                                                                repo_editor_current_tags.set(Vec::new());
                                                                repo_editor_selected_tags.set(Vec::new());
                                                                *repo_editor_message.write() = Some("未找到该 repo".to_string());
                                                            }
                                                            Err(err) => {
                                                                repo_editor_current_tags.set(Vec::new());
                                                                repo_editor_selected_tags.set(Vec::new());
                                                                *repo_editor_message.write() = Some(err.to_string());
                                                            }
                                                        }
                                                        *repo_editor_pending.write() = false;
                                                    });
                                                }
                                            },
                                            "选择"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Some(Err(err)) = repo_editor_search.value() {
                    div { class: "text-sm text-primary-error", "{err}" }
                }

                if let Some(repo) = repo_editor_selected_repo() {
                    div { class: "space-y-3 rounded-md border border-primary-6 bg-primary-1 p-3",
                        div { class: "text-sm font-medium", "当前 Repo: {repo.id}" }

                        div { class: "space-y-2",
                            div { class: "text-xs font-semibold tracking-wide text-secondary-5", "当前已绑定 Tags" }
                            if repo_editor_current_tags().is_empty() {
                                div { class: "text-xs text-secondary-5", "（空）" }
                            } else {
                                div { class: "flex flex-wrap gap-2",
                                    for tag in repo_editor_current_tags() {
                                        span { key: "{tag.label}:{tag.value}", class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs",
                                            "{tag.label}:{tag.value}"
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "space-y-2 border-t border-dashed border-primary-6 pt-3",
                            div { class: "text-xs font-semibold tracking-wide text-secondary-5", "选择要更新到该 Repo 的 Tags" }
                            match tags_page() {
                                Some(Ok(page)) => rsx! {
                                    div { class: "max-h-[220px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary p-2",
                                        for tag in page.items {
                                            label { key: "repo-editor-{tag.label}:{tag.value}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: repo_editor_selected_tags().iter().any(|x| x.label == tag.label && x.value == tag.value),
                                                    onchange: {
                                                        let target_tag = TagDto {
                                                            label: tag.label.clone(),
                                                            value: tag.value.clone(),
                                                        };
                                                        move |_| {
                                                            let mut selected_tags = repo_editor_selected_tags();
                                                            if let Some(index) = selected_tags.iter().position(|x| x == &target_tag) {
                                                                selected_tags.remove(index);
                                                            } else {
                                                                selected_tags.push(target_tag.clone());
                                                            }
                                                            repo_editor_selected_tags.set(selected_tags);
                                                        }
                                                    },
                                                }
                                                span { class: "text-xs", "{tag.label}:{tag.value}" }
                                            }
                                        }
                                    }
                                },
                                Some(Err(err)) => rsx! { div { class: "text-sm text-primary-error", "{err}" } },
                                None => rsx! { div { class: "text-xs text-secondary-5", "Loading tags..." } },
                            }
                        }

                        div { class: "flex flex-wrap gap-2 border-t border-dashed border-primary-6 pt-3",
                            button {
                                class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                                disabled: repo_editor_pending(),
                                onclick: move |_| {
                                    let Some(selected_repo) = repo_editor_selected_repo() else {
                                        *repo_editor_message.write() = Some("请先选择 repo".to_string());
                                        return;
                                    };
                                    let picked_tags = repo_editor_selected_tags();
                                    if picked_tags.is_empty() {
                                        *repo_editor_message.write() = Some("请先选择 tag".to_string());
                                        return;
                                    }
                                    let Some((owner, name)) = parse_owner_name(&selected_repo.id) else {
                                        *repo_editor_message.write() = Some(format!("无效 repo id: {}", selected_repo.id));
                                        return;
                                    };

                                    let mut next_tags = repo_editor_current_tags();
                                    for tag in picked_tags {
                                        if !next_tags.contains(&tag) {
                                            next_tags.push(tag);
                                        }
                                    }

                                    *repo_editor_pending.write() = true;
                                    *repo_editor_message.write() = None;
                                    spawn(async move {
                                        match replace_repo_tags(owner, name, next_tags.clone()).await {
                                            Ok(()) => {
                                                repo_editor_current_tags.set(next_tags);
                                                *repo_editor_message.write() = Some("Add 完成".to_string());
                                            }
                                            Err(err) => *repo_editor_message.write() = Some(err.to_string()),
                                        }
                                        *repo_editor_pending.write() = false;
                                    });
                                },
                                "Add 到 Repo"
                            }
                            button {
                                class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                                disabled: repo_editor_pending(),
                                onclick: move |_| {
                                    let Some(selected_repo) = repo_editor_selected_repo() else {
                                        *repo_editor_message.write() = Some("请先选择 repo".to_string());
                                        return;
                                    };
                                    let picked_tags = repo_editor_selected_tags();
                                    if picked_tags.is_empty() {
                                        *repo_editor_message.write() = Some("请先选择 tag".to_string());
                                        return;
                                    }
                                    let Some((owner, name)) = parse_owner_name(&selected_repo.id) else {
                                        *repo_editor_message.write() = Some(format!("无效 repo id: {}", selected_repo.id));
                                        return;
                                    };

                                    let mut next_tags = repo_editor_current_tags();
                                    next_tags.retain(|tag| !picked_tags.contains(tag));

                                    *repo_editor_pending.write() = true;
                                    *repo_editor_message.write() = None;
                                    spawn(async move {
                                        match replace_repo_tags(owner, name, next_tags.clone()).await {
                                            Ok(()) => {
                                                repo_editor_current_tags.set(next_tags);
                                                *repo_editor_message.write() = Some("Remove 完成".to_string());
                                            }
                                            Err(err) => *repo_editor_message.write() = Some(err.to_string()),
                                        }
                                        *repo_editor_pending.write() = false;
                                    });
                                },
                                "从 Repo 移除"
                            }
                        }
                    }
                }

                if repo_editor_pending() {
                    div { class: "text-xs text-secondary-5", "处理中..." }
                }
                if let Some(msg) = repo_editor_message() {
                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                }
            }

            if let Some(tag) = selected_tag() {
                section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
                    div { class: "space-y-1",
                        div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "TAGS / BULK UPDATE" }
                        h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Selected: {tag.label}:{tag.value}" }
                        p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                            "搜索并多选 repo，然后对该 tag 批量 add/remove。"
                        }
                    }
                    div { class: "flex flex-col gap-2 md:flex-row",
                        input {
                            class: "w-full rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                            placeholder: "搜索 repo（owner/name）",
                            value: repo_search_key,
                            oninput: move |e| *repo_search_key.write() = e.value(),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    repo_search.call(repo_search_key());
                                }
                            },
                        }
                        button {
                            class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-3",
                            onclick: move |_| repo_search.call(repo_search_key()),
                            "搜索"
                        }
                    }
                    if let Some(Ok(result)) = repo_search.value() {
                        {
                            let repos_for_select_all = result().repos.items.clone();
                            let repos_for_list = result().repos.items.clone();
                            rsx! {
                                div { class: "space-y-2",
                                    div { class: "flex items-center gap-2",
                                        button {
                                            class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3",
                                            onclick: move |_| {
                                                let mut ids = selected_repo_ids();
                                                for repo in &repos_for_select_all {
                                                    if !ids.contains(&repo.id) {
                                                        ids.push(repo.id.clone());
                                                    }
                                                }
                                                selected_repo_ids.set(ids);
                                            },
                                            "全选当前结果"
                                        }
                                        button {
                                            class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3",
                                            onclick: move |_| selected_repo_ids.set(Vec::new()),
                                            "清空"
                                        }
                                        span { class: "text-xs text-secondary-5", "已选 {selected_repo_ids().len()} 项" }
                                    }
                                    div { class: "max-h-[260px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                                        for repo in repos_for_list {
                                            label { key: "{repo.id}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: selected_repo_ids().contains(&repo.id),
                                                    onchange: move |_| {
                                                        let mut ids = selected_repo_ids();
                                                        if let Some(idx) = ids.iter().position(|id| *id == repo.id) {
                                                            ids.remove(idx);
                                                        } else {
                                                            ids.push(repo.id.clone());
                                                        }
                                                        selected_repo_ids.set(ids);
                                                    },
                                                }
                                                div { class: "min-w-0",
                                                    div { class: "truncate text-sm font-medium", "{repo.id}" }
                                                    if let Some(full_name) = &repo.full_name {
                                                        div { class: "truncate text-xs text-secondary-5", "{full_name}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if let Some(Err(err)) = repo_search.value() {
                        div { class: "text-sm text-primary-error", "{err}" }
                    }
                    div { class: "flex flex-wrap gap-2 border-t border-dashed border-primary-6 pt-3",
                        button {
                            class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                            disabled: bulk_pending(),
                            onclick: move |_| {
                                let repo_ids = selected_repo_ids();
                                if repo_ids.is_empty() {
                                    *bulk_message.write() = Some("请先选择 repo".to_string());
                                    return;
                                }
                                *bulk_pending.write() = true;
                                *bulk_message.write() = None;
                                let Some(target) = selected_tag() else {
                                    *bulk_message.write() = Some("请先选中 tag".to_string());
                                    *bulk_pending.write() = false;
                                    return;
                                };
                                spawn(async move {
                                    match bulk_update_repo_tag(
                                        repo_ids,
                                        target.label.clone(),
                                        target.value.clone(),
                                        "add".to_string(),
                                    )
                                    .await
                                    {
                                        Ok(res) => *bulk_message.write() = Some(format!(
                                            "Add 完成: total={} updated={} skipped={}",
                                            res.total, res.updated, res.skipped
                                        )),
                                        Err(err) => *bulk_message.write() = Some(err.to_string()),
                                    }
                                    *bulk_pending.write() = false;
                                });
                            },
                            "批量 Add"
                        }
                        button {
                            class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                            disabled: bulk_pending(),
                            onclick: move |_| {
                                let repo_ids = selected_repo_ids();
                                if repo_ids.is_empty() {
                                    *bulk_message.write() = Some("请先选择 repo".to_string());
                                    return;
                                }
                                *bulk_pending.write() = true;
                                *bulk_message.write() = None;
                                let Some(target) = selected_tag() else {
                                    *bulk_message.write() = Some("请先选中 tag".to_string());
                                    *bulk_pending.write() = false;
                                    return;
                                };
                                spawn(async move {
                                    match bulk_update_repo_tag(
                                        repo_ids,
                                        target.label.clone(),
                                        target.value.clone(),
                                        "remove".to_string(),
                                    )
                                    .await
                                    {
                                        Ok(res) => *bulk_message.write() = Some(format!(
                                            "Remove 完成: total={} updated={} skipped={}",
                                            res.total, res.updated, res.skipped
                                        )),
                                        Err(err) => *bulk_message.write() = Some(err.to_string()),
                                    }
                                    *bulk_pending.write() = false;
                                });
                            },
                            "批量 Remove"
                        }
                    }
                    if bulk_pending() {
                        div { class: "text-xs text-secondary-5", "处理中..." }
                    }
                    if let Some(msg) = bulk_message() {
                        div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                    }
                }
            }
        }
    }
}
