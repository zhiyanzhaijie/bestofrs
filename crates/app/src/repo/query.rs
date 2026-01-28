use std::collections::HashMap;
use std::slice::from_ref;
use std::sync::Arc;

use domain::{Repo, RepoId, RepoWithTags, Tag};

use crate::app_error::AppResult;
use crate::common::{Page, Pagination};
use crate::repo::{RepoRepo, RepoTagRepo};

#[derive(Clone)]
pub struct RepoQueryHandler {
    repos: Arc<dyn RepoRepo>,
    repo_tags: Arc<dyn RepoTagRepo>,
}

impl RepoQueryHandler {
    pub fn new(repos: Arc<dyn RepoRepo>, repo_tags: Arc<dyn RepoTagRepo>) -> Self {
        Self { repos, repo_tags }
    }

    pub async fn get(&self, repo_id: &RepoId) -> AppResult<Option<Repo>> {
        self.repos.get(repo_id).await
    }

    pub async fn list(&self, page: Pagination) -> AppResult<Page<Repo>> {
        self.repos.list(page).await
    }
    pub async fn get_with_tags(&self, repo_id: &RepoId) -> AppResult<Option<RepoWithTags>> {
        let repo = self.repos.get(repo_id).await?;
        let repo = match repo {
            Some(repo) => repo,
            None => return Ok(None),
        };
        let pairs = self.repo_tags.list_by_repo_ids(from_ref(repo_id)).await?;
        let tags = pairs
            .into_iter()
            .filter_map(|(id, tag)| if id == *repo_id { Some(tag) } else { None })
            .collect();
        Ok(Some(RepoWithTags { repo, tags }))
    }

    pub async fn list_with_tags(&self, page: Pagination) -> AppResult<Page<RepoWithTags>> {
        let repos_page = self.repos.list(page).await?;
        let repo_ids: Vec<RepoId> = repos_page.items.iter().map(|repo| repo.id.clone()).collect();
        let pairs = self.repo_tags.list_by_repo_ids(&repo_ids).await?;
        let mut tags_by_repo: HashMap<RepoId, Vec<Tag>> = HashMap::new();
        for (repo_id, tag) in pairs {
            tags_by_repo.entry(repo_id).or_default().push(tag);
        }
        Ok(repos_page.map(|repo| {
            let tags = match tags_by_repo.remove(&repo.id) {
                Some(tags) => tags,
                None => Vec::new(),
            };
            RepoWithTags { repo, tags }
        }))
    }

    pub async fn list_by_label_with_tags(
        &self,
        label: &str,
        value: Option<&str>,
        page: Pagination,
    ) -> AppResult<Page<RepoWithTags>> {
        let repo_ids_page = self
            .repo_tags
            .list_repo_ids_by_label(label, value, page)
            .await?;
        let mut repos = Vec::with_capacity(repo_ids_page.items.len());
        for repo_id in &repo_ids_page.items {
            if let Some(repo) = self.repos.get(repo_id).await? {
                repos.push(repo);
            }
        }
        let repo_ids: Vec<RepoId> = repos.iter().map(|repo| repo.id.clone()).collect();
        let pairs = self.repo_tags.list_by_repo_ids(&repo_ids).await?;
        let mut tags_by_repo: HashMap<RepoId, Vec<Tag>> = HashMap::new();
        for (repo_id, tag) in pairs {
            tags_by_repo.entry(repo_id).or_default().push(tag);
        }
        let items = repos
            .into_iter()
            .map(|repo| {
                let tags = match tags_by_repo.remove(&repo.id) {
                    Some(tags) => tags,
                    None => Vec::new(),
                };
                RepoWithTags { repo, tags }
            })
            .collect();
        Ok(Page {
            items,
            meta: repo_ids_page.meta,
        })
    }

    pub async fn get_by_owner_name(&self, owner: &str, name: &str) -> AppResult<Option<Repo>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.get(&repo_id).await
    }

    pub async fn get_by_owner_name_with_tags(
        &self,
        owner: &str,
        name: &str,
    ) -> AppResult<Option<RepoWithTags>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.get_with_tags(&repo_id).await
    }
}
