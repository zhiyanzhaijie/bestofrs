use dioxus::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};

#[derive(Props, Clone, PartialEq)]
pub struct RepoAvatarProps {
    pub repo_id: String,
    pub avatar_urls: Vec<String>,
    #[props(default = AvatarImageSize::Large)]
    pub size: AvatarImageSize,
    #[props(default = "border border-primary-6 bg-primary".to_string())]
    pub class: String,
    #[props(default = "flex h-16 w-16 items-center justify-center border border-primary-6 bg-primary-2 font-bold text-secondary-4".to_string())]
    pub fallback_class: String,
}

#[component]
pub fn RepoAvatar(props: RepoAvatarProps) -> Element {
    let avatar_candidates = props.avatar_urls.iter().fold(Vec::new(), |mut acc, url| {
        let url = url.trim().to_string();
        if !url.is_empty() && !acc.contains(&url) {
            acc.push(url);
        }
        acc
    });
    let avatar_candidates_for_error = avatar_candidates.clone();
    let mut avatar_index = use_signal(|| 0usize);
    let mut remount_flip = use_signal(|| false);
    let fallback_owner = props
        .repo_id
        .split_once('/')
        .map(|(owner, _)| owner)
        .unwrap_or(props.repo_id.as_str());
    let fallback_char = fallback_owner.chars().next().unwrap_or('?');
    use_effect(use_reactive(
        (&props.repo_id, &props.avatar_urls),
        move |_| {
            avatar_index.set(0);
        },
    ));
    let current_index = avatar_index();

    if let Some(src) = avatar_candidates.get(current_index).cloned() {
        rsx! {
            if remount_flip() {
                section { class: "contents", key: "{props.repo_id}:{current_index}:{src}:1",
                    Avatar {
                        class: "{props.class}",
                        size: props.size,
                        on_error: move |_| {
                            let next = avatar_index() + 1;
                            if next < avatar_candidates_for_error.len() {
                                avatar_index.set(next);
                                remount_flip.set(!remount_flip());
                            } else {
                                avatar_index.set(usize::MAX);
                            }
                        },
                        AvatarImage {
                            src: src.clone(),
                            alt: "{props.repo_id} avatar",
                        }
                        AvatarFallback {
                            class: "{props.fallback_class}",
                            "{fallback_char}"
                        }
                    }
                }
            } else {
                div { class: "contents", key: "{props.repo_id}:{current_index}:{src}:0",
                    Avatar {
                        class: "{props.class}",
                        size: props.size,
                        on_error: move |_| {
                            let next = avatar_index() + 1;
                            if next < avatar_candidates_for_error.len() {
                                avatar_index.set(next);
                                remount_flip.set(!remount_flip());
                            } else {
                                avatar_index.set(usize::MAX);
                            }
                        },
                        AvatarImage {
                            src: src,
                            alt: "{props.repo_id} avatar",
                        }
                        AvatarFallback {
                            class: "{props.fallback_class}",
                            "{fallback_char}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div { class: "{props.fallback_class}",
                "{fallback_char}"
            }
        }
    }
}
