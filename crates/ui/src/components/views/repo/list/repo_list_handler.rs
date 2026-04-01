use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};

use super::{normalize_page_size, repo_list_route_from_ctx, FilterType, RepoListContext, SortType};

#[component]
pub(super) fn RepoListHandler() -> Element {
    let ctx = use_context::<RepoListContext>();
    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col md:flex-row items-center justify-between gap-4",
            div { class: "text-xs font-mono tracking-wide text-secondary-5",
                "List "
                span { class: "font-semibold text-grid-accent",
                    "{(ctx.summary)().from}-{(ctx.summary)().to}"
                }
                " of "
                span { class: "font-semibold text-grid-accent", "{(ctx.summary)().total}" }
            }
            div { class: "grid w-full grid-cols-3 items-center gap-2 md:flex md:w-auto md:gap-4",
                Select::<FilterType> {
                    class: "select w-full min-w-0",
                    value: Some((ctx.filter_type)()),
                    placeholder: t!("view_repo_list_handler_filter_placeholder"),
                    on_value_change: move |next: Option<FilterType>| {
                        if let Some(next_filter) = next {
                            let next_sort = (ctx.sort_type)();
                            let (metric_q, range_q) =
                                super::query_params_from_filter_sort(next_filter, next_sort);
                            navigator.push(crate::root::Route::RepoListView {
                                tags: super::active_tags_to_query(&(ctx.active_tags)()),
                                metric: metric_q,
                                range: range_q,
                                page: Some(1),
                                size: Some((ctx.page_size)()),
                            });
                        }
                    },
                    SelectTrigger {
                        class: "select-trigger w-full min-w-0 px-2 py-2 text-[10px] tracking-[0.08em] md:min-w-[9rem] md:px-4 md:py-3 md:text-xs md:tracking-[0.14em]",
                        aria_label: t!("view_repo_list_handler_filter_aria_label"),
                        SelectValue {}
                    }
                    SelectList { aria_label: t!("view_repo_list_handler_filter_options_aria_label"),
                        SelectGroup {
                            SelectGroupLabel { {t!("view_repo_list_handler_filter_group_label")} }
                            SelectOption::<FilterType> {
                                index: 0usize,
                                value: FilterType::Total,
                                text_value: Some(filter_label_i18n(FilterType::Total)),
                                {filter_label_i18n(FilterType::Total)}
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 1usize,
                                value: FilterType::Daily,
                                text_value: Some(filter_label_i18n(FilterType::Daily)),
                                {filter_label_i18n(FilterType::Daily)}
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 2usize,
                                value: FilterType::Weekly,
                                text_value: Some(filter_label_i18n(FilterType::Weekly)),
                                {filter_label_i18n(FilterType::Weekly)}
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 3usize,
                                value: FilterType::Monthly,
                                text_value: Some(filter_label_i18n(FilterType::Monthly)),
                                {filter_label_i18n(FilterType::Monthly)}
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
                Select::<u32> {
                    class: "select w-full min-w-0",
                    value: Some((ctx.page_size)()),
                    placeholder: t!("view_repo_list_handler_page_size_placeholder"),
                    on_value_change: move |v: Option<u32>| {
                        if let Some(v) = v {
                            let next_size = normalize_page_size(v);
                            navigator.replace(repo_list_route_from_ctx(ctx, 1, next_size));
                        }
                    },
                    SelectTrigger {
                        class: "select-trigger w-full min-w-0 px-2 py-2 text-[10px] tracking-[0.08em] md:min-w-[7rem] md:px-4 md:py-3 md:text-xs md:tracking-[0.14em]",
                        aria_label: t!("view_repo_list_handler_page_size_aria_label"),
                        SelectValue {}
                    }
                    SelectList { aria_label: t!("view_repo_list_handler_page_size_options_aria_label"),
                        SelectGroup {
                            SelectGroupLabel {
                                {t!("view_repo_list_handler_page_size_group_label")}
                            }
                            SelectOption::<u32> {
                                index: 0usize,
                                value: 20u32,
                                text_value: Some("20".to_string()),
                                "20"
                                SelectItemIndicator {}
                            }
                            SelectOption::<u32> {
                                index: 1usize,
                                value: 50u32,
                                text_value: Some("50".to_string()),
                                "50"
                                SelectItemIndicator {}
                            }
                            SelectOption::<u32> {
                                index: 2usize,
                                value: 100u32,
                                text_value: Some("100".to_string()),
                                "100"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
                Select::<SortType> {
                    class: "select w-full min-w-0",
                    value: Some((ctx.sort_type)()),
                    placeholder: t!("view_repo_list_handler_sort_placeholder"),
                    on_value_change: move |next: Option<SortType>| {
                        if let Some(next_sort) = next {
                            let next_filter = if next_sort == SortType::AddTime {
                                FilterType::Total
                            } else {
                                (ctx.filter_type)()
                            };
                            let (metric_q, range_q) =
                                super::query_params_from_filter_sort(next_filter, next_sort);
                            navigator.push(crate::root::Route::RepoListView {
                                tags: super::active_tags_to_query(&(ctx.active_tags)()),
                                metric: metric_q,
                                range: range_q,
                                page: Some(1),
                                size: Some((ctx.page_size)()),
                            });
                        }
                    },
                    SelectTrigger {
                        class: "select-trigger w-full min-w-0 px-2 py-2 text-[10px] tracking-[0.08em] md:min-w-[10rem] md:px-4 md:py-3 md:text-xs md:tracking-[0.14em]",
                        aria_label: t!("view_repo_list_handler_sort_aria_label"),
                        SelectValue {}
                    }
                    SelectList { aria_label: t!("view_repo_list_handler_sort_options_aria_label"),
                        SelectGroup {
                            SelectGroupLabel { {t!("view_repo_list_handler_sort_group_label")} }
                            SelectOption::<SortType> {
                                index: 0usize,
                                value: SortType::Star,
                                text_value: Some(sort_label_i18n(SortType::Star)),
                                {sort_label_i18n(SortType::Star)}
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 1usize,
                                value: SortType::Fork,
                                text_value: Some(sort_label_i18n(SortType::Fork)),
                                {sort_label_i18n(SortType::Fork)}
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 2usize,
                                value: SortType::Issue,
                                text_value: Some(sort_label_i18n(SortType::Issue)),
                                {sort_label_i18n(SortType::Issue)}
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 3usize,
                                value: SortType::AddTime,
                                text_value: Some(sort_label_i18n(SortType::AddTime)),
                                {sort_label_i18n(SortType::AddTime)}
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn filter_label_i18n(filter: FilterType) -> String {
    match filter {
        FilterType::Total => t!("view_repo_list_handler_filter_total").to_string(),
        FilterType::Daily => t!("view_repo_list_handler_filter_daily").to_string(),
        FilterType::Weekly => t!("view_repo_list_handler_filter_weekly").to_string(),
        FilterType::Monthly => t!("view_repo_list_handler_filter_monthly").to_string(),
    }
}

fn sort_label_i18n(sort: SortType) -> String {
    match sort {
        SortType::Star => "Star".to_string(),
        SortType::Fork => "Fork".to_string(),
        SortType::Issue => "Issue".to_string(),
        SortType::AddTime => t!("view_repo_list_handler_sort_create_time").to_string(),
    }
}
