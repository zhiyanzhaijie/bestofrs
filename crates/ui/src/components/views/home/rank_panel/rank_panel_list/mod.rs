use dioxus::prelude::*;

use crate::IO::repos::list_repos_with_query;
use app::prelude::Pagination;
use app::repo::RepoListQuery;

use super::repo_row::HomeRankRepoRow;
use super::{map_rank_repo, RankType, TimeRange};

pub(super) mod skeleton;

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeRankPanelListProps {
    active_tab: Signal<RankType>,
    time_range: Signal<TimeRange>,
}

#[component]
pub(super) fn HomeRankPanelList(props: HomeRankPanelListProps) -> Element {
    let repos = use_server_future(move || {
        list_repos_with_query(RepoListQuery {
            page: Pagination {
                limit: Some(7),
                offset: Some(0),
            },
            metric: Some((props.active_tab)()),
            range: Some(if (props.active_tab)() == RankType::Recent {
                TimeRange::All
            } else {
                (props.time_range)()
            }),
            tags: None,
        })
    })?;
    match repos() {
        Some(Ok(page)) => {
            let current_list = page
                .items
                .into_iter()
                .map(map_rank_repo)
                .collect::<Vec<_>>();
            rsx! {
                div { class: "flex flex-col gap-2.5",
                    for (idx, repo) in current_list.into_iter().enumerate() {
                        HomeRankRepoRow {
                            idx,
                            repo,
                            active_tab: (props.active_tab)(),
                        }
                    }
                }
            }
        }
        Some(Err(e)) => rsx! {
            div { class: "bg-primary border border-primary-6 shadow-2xl rounded-[2rem] overflow-hidden p-6 text-sm text-primary-error relative z-10",
                "{e}"
            }
        },
        None => rsx! {},
    }
}
