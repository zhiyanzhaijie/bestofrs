use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::components::common::CommonBreadcrumb;
use crate::components::icons::{ArrowLeftIcon, ScrollTextIcon, TagsIcon};
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem, SidebarProvider, SidebarRail, SidebarTrigger, SidebarVariant,
};
use crate::components::AdminStateHint;
use crate::root::layouts::{UserContext, UserState};
use crate::root::Route;

#[component]
pub fn AdminLayout() -> Element {
    let navigator = use_navigator();
    let navigator_for_projects = navigator.clone();
    let navigator_for_tags = navigator.clone();
    let navigator_for_job = navigator.clone();
    let current_route = use_route::<Route>();
    let mut redirected = use_signal(|| false);
    let user_state = use_context::<UserContext>();
    let (is_admin, user_info) = match &*user_state.read() {
        UserState::User(user) if user.role == "Admin" => (true, Some(user.clone())),
        _ => (false, None),
    };
    let is_projects = matches!(current_route, Route::AdminProjectsView {});
    let is_tags = matches!(current_route, Route::AdminTagsView {});
    let is_job = matches!(current_route, Route::AdminJobView {});

    use_effect(move || {
        if redirected() {
            return;
        }

        let unauthorized = match &*user_state.read() {
            UserState::User(me) => me.role != "Admin",
            UserState::Loading => false,
            UserState::Guest | UserState::Error(_) => true,
        };

        if unauthorized {
            redirected.set(true);
            navigator.replace(Route::HomeView {});
        }
    });

    rsx! {
        if is_admin {
            div { class: "min-h-[calc(100svh-4rem)]",
                SidebarProvider {
                    Sidebar {
                        variant: SidebarVariant::Sidebar,
                        collapsible: SidebarCollapsible::Icon,
                        SidebarHeader {
                            SidebarMenu {
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        size: SidebarMenuButtonSize::Lg,
                                        as: move |attributes: Vec<Attribute>| rsx! {
                                            div {
                                                ..attributes,
                                                if let Some(user) = &user_info {
                                                    Avatar {
                                                        size: AvatarImageSize::Small,
                                                        AvatarImage {
                                                            src: user.avatar_url.clone().unwrap_or_default(),
                                                            alt: user.login.clone(),
                                                        }
                                                        AvatarFallback {
                                                            "{user.login.chars().next().unwrap_or('?')}"
                                                        }
                                                    }
                                                    div { class: "grid flex-1 text-left text-sm leading-tight",
                                                        span { class: "truncate font-semibold", "{user.login}" }
                                                        span { class: "truncate text-xs", "Admin" }
                                                    }
                                                }
                                            }
                                        },
                                    }
                                }
                            }
                        }
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Management" }
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: is_projects,
                                            tooltip: rsx!("Project 管理"),
                                            as: move |attributes: Vec<Attribute>| rsx! {
                                                button {
                                                    onclick: move |_| {
                                                        let _ = navigator_for_projects.push(Route::AdminProjectsView {});
                                                    },
                                                    ..attributes,
                                                    ScrollTextIcon {}
                                                    span { "Project 管理" }
                                                }
                                            },
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: is_tags,
                                            tooltip: rsx!("Tags 管理"),
                                            as: move |attributes: Vec<Attribute>| rsx! {
                                                button {
                                                    onclick: move |_| {
                                                        let _ = navigator_for_tags.push(Route::AdminTagsView {});
                                                    },
                                                    ..attributes,
                                                    TagsIcon {}
                                                    span { "Tags 管理" }
                                                }
                                            },
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: is_job,
                                            tooltip: rsx!("Job 管理"),
                                            as: move |attributes: Vec<Attribute>| rsx! {
                                                button {
                                                    onclick: move |_| {
                                                        let _ = navigator_for_job.push(Route::AdminJobView {});
                                                    },
                                                    ..attributes,
                                                    ScrollTextIcon {}
                                                    span { "Job 管理" }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                        SidebarFooter {
                            SidebarMenu {
                                Link {
                                    to: Route::HomeView {},
                                    class: "sidebar-menu-item block",
                                    div {
                                        class: "sidebar-menu-button flex items-center gap-2",
                                        "data-size": "lg",
                                        div { class: "flex aspect-square h-8 w-8 items-center justify-center rounded-lg bg-primary-6 text-primary-1",
                                            ArrowLeftIcon { width: 16 }
                                        }
                                        div { class: "grid flex-1 text-left text-sm leading-tight",
                                            span { class: "truncate font-semibold", "返回" }
                                            span { class: "truncate text-xs", "返回首页" }
                                        }
                                    }
                                }
                            }
                        }
                        SidebarRail {}
                    }
                    SidebarInset {
                        header { class: "flex h-14 shrink-0 items-center gap-3 border-b border-primary-6 bg-primary-1 px-4",
                            SidebarTrigger {}
                            Separator { height: "1rem", horizontal: false }
                            div { class: "min-w-0",
                                div { class: "font-mono text-[11px] tracking-widest text-secondary-5", "ADMIN PANEL" }
                                h1 { class: "truncate text-sm font-semibold text-secondary-3 md:text-base",
                                    if is_projects {
                                        "Project Management"
                                    } else if is_tags {
                                        "Tags Management"
                                    } else if is_job {
                                        "Job Management"
                                    } else {
                                        "Admin Management"
                                    }
                                }
                            }
                        }
                        CommonBreadcrumb {  }
                        div { class: "min-h-0 flex-1 overflow-y-auto overflow-x-hidden p-4 md:p-6",
                            SuspenseBoundary {
                                fallback: move |_: SuspenseContext| {
                                    rsx! { AdminStateHint { message: "Loading...".to_string() } }
                                },
                                Outlet::<Route> {}
                            }
                        }
                    }
                }
            }
        } else if matches!(*user_state.read(), UserState::Loading) {
            AdminStateHint { message: "Loading...".to_string() }
        } else {
            AdminStateHint { message: "Redirecting...".to_string() }
        }
    }
}
