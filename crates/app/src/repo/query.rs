use std::collections::{BTreeSet, HashMap, HashSet};
use std::slice::from_ref;
use std::sync::Arc;

use domain::{Repo, RepoId, RepoWithTags, Tag};

use crate::app_error::AppResult;
use crate::common::{Page, Pagination};
use crate::repo::{
    GithubGateway, RepoRankMetric, RepoRankQuery, RepoRepo, RepoSearchCache, RepoTagFacet,
    RepoTagListItem, RepoTagRepo,
};

#[derive(Debug, Clone)]
pub struct RepoSearchResult {
    pub repos: Page<Repo>,
    pub tags: Page<RepoSearchTagItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSearchTagItem {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub repos_total: u64,
}

#[derive(Clone)]
pub struct RepoQueryHandler {
    repos: Arc<dyn RepoRepo>,
    repo_tags: Arc<dyn RepoTagRepo>,
    github: Arc<dyn GithubGateway>,
    cache: Option<Arc<dyn RepoSearchCache>>,
}

impl RepoQueryHandler {
    pub fn new(
        repos: Arc<dyn RepoRepo>,
        repo_tags: Arc<dyn RepoTagRepo>,
        github: Arc<dyn GithubGateway>,
    ) -> Self {
        Self {
            repos,
            repo_tags,
            github,
            cache: None,
        }
    }

    pub fn new_with_cache(
        repos: Arc<dyn RepoRepo>,
        repo_tags: Arc<dyn RepoTagRepo>,
        github: Arc<dyn GithubGateway>,
        cache: Arc<dyn RepoSearchCache>,
    ) -> Self {
        Self {
            repos,
            repo_tags,
            github,
            cache: Some(cache),
        }
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
        let tags: Vec<Tag> = pairs
            .into_iter()
            .filter_map(|(id, tag)| if id == *repo_id { Some(tag) } else { None })
            .collect();
        Ok(Some(RepoWithTags { repo, tags }))
    }

    pub async fn list_with_tags(
        &self,
        page: Pagination,
        active_tag_values: Option<Vec<String>>,
        metric: Option<RepoRankMetric>,
    ) -> AppResult<Page<RepoWithTags>> {
        let mut dedup = BTreeSet::new();
        let mut normalized_values = Vec::new();
        for value in active_tag_values.unwrap_or_default() {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            if dedup.insert(value.to_string()) {
                normalized_values.push(value.to_string());
            }
        }

        let repos_page = if normalized_values.is_empty() {
            self.repos.list(page).await?
        } else {
            let tags = self
                .repo_tags
                .find_tags_by_values(&normalized_values)
                .await?;
            let mut tags_by_value = HashMap::new();
            for tag in tags {
                tags_by_value.insert(tag.value.as_str().to_string(), tag);
            }

            if normalized_values
                .iter()
                .any(|value| !tags_by_value.contains_key(value))
            {
                Page {
                    items: Vec::new(),
                    meta: page.meta(0),
                }
            } else {
                let mut matched_repo_ids: Option<HashSet<String>> = None;
                for value in &normalized_values {
                    let Some(tag) = tags_by_value.get(value) else {
                        return Ok(Page {
                            items: Vec::new(),
                            meta: page.meta(0),
                        });
                    };

                    let mut current_repo_ids = HashSet::new();
                    let mut offset = 0u32;
                    loop {
                        let ids_page = self
                            .repo_tags
                            .list_repo_ids_by_label(
                                tag.label.as_str(),
                                Some(tag.value.as_str()),
                                Pagination {
                                    limit: Some(Pagination::MAX_LIMIT),
                                    offset: Some(offset),
                                },
                            )
                            .await?;
                        for repo_id in ids_page.items {
                            current_repo_ids.insert(repo_id.as_str().to_string());
                        }
                        offset = offset.saturating_add(Pagination::MAX_LIMIT);
                        if offset as u64 >= ids_page.meta.total {
                            break;
                        }
                    }

                    matched_repo_ids = match matched_repo_ids {
                        Some(mut existing) => {
                            existing.retain(|repo_id| current_repo_ids.contains(repo_id));
                            Some(existing)
                        }
                        None => Some(current_repo_ids),
                    };
                }

                let matched_repo_ids = matched_repo_ids
                    .unwrap_or_default()
                    .into_iter()
                    .map(RepoId::new_unchecked)
                    .collect::<Vec<_>>();
                let mut repos = self.repos.list_by_ids(&matched_repo_ids).await?;
                let metric = metric.unwrap_or(RepoRankMetric::Star);
                repos.sort_by(|a, b| {
                    let order = match metric {
                        RepoRankMetric::Star => b.stars.cmp(&a.stars),
                        RepoRankMetric::Fork => b.forks.cmp(&a.forks),
                        RepoRankMetric::Issue => b.open_issues.cmp(&a.open_issues),
                        RepoRankMetric::Recent => b.created_at.cmp(&a.created_at),
                    };
                    order.then_with(|| a.id.as_str().cmp(b.id.as_str()))
                });
                let total = repos.len() as u64;
                let offset = page.offset() as usize;
                let limit = page.limit() as usize;
                let repos = repos.into_iter().skip(offset).take(limit).collect::<Vec<_>>();

                Page {
                    items: repos,
                    meta: page.meta(total),
                }
            }
        };
        let repo_ids: Vec<RepoId> = repos_page
            .items
            .iter()
            .map(|repo| repo.id.clone())
            .collect();
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

    pub async fn list_ranked_with_tags(
        &self,
        query: RepoRankQuery,
        page: Pagination,
    ) -> AppResult<Page<RepoWithTags>> {
        let repos_page = self.repos.list_ranked(query, page).await?;
        let repo_ids: Vec<RepoId> = repos_page
            .items
            .iter()
            .map(|repo| repo.id.clone())
            .collect();
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
        let repo_ids_page = if label.trim().is_empty() {
            self.repo_tags.list_repo_ids_without_tags(page).await?
        } else {
            self.repo_tags
                .list_repo_ids_by_label(label, value, page)
                .await?
        };
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

    pub async fn get_readme_by_owner_name(
        &self,
        owner: &str,
        name: &str,
    ) -> AppResult<Option<crate::repo::GithubReadme>> {
        let full_name = format!("{owner}/{name}");
        self.github.fetch_readme(&full_name).await
    }

    pub async fn list_tags(&self, page: Pagination) -> AppResult<Page<Tag>> {
        self.repo_tags.list_tags(page).await
    }

    pub async fn search_tags_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Tag>> {
        self.repo_tags.search_tags_by_key(key, page).await
    }

    pub async fn list_tags_with_meta(
        &self,
        page: Pagination,
        top_n: u32,
    ) -> AppResult<Page<RepoTagListItem>> {
        self.repo_tags.list_tags_with_meta(page, top_n).await
    }

    pub async fn list_tags_with_meta_by_values(
        &self,
        values: Vec<String>,
        top_n: u32,
    ) -> AppResult<Vec<RepoTagListItem>> {
        let mut normalized = Vec::new();
        let mut dedup = HashSet::new();
        for value in values {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            let value = value.to_string();
            if dedup.insert(value.clone()) {
                normalized.push(value);
            }
        }
        if normalized.is_empty() {
            return Ok(Vec::new());
        }

        let mut by_value = self
            .repo_tags
            .list_tags_with_meta_by_values(&normalized, top_n)
            .await?
            .into_iter()
            .map(|item| (item.value.clone(), item))
            .collect::<HashMap<_, _>>();

        Ok(normalized
            .into_iter()
            .filter_map(|value| by_value.remove(&value))
            .collect())
    }

    pub async fn get_tag_with_meta_by_value(
        &self,
        value: String,
        top_n: u32,
    ) -> AppResult<Option<RepoTagListItem>> {
        let mut items = self.list_tags_with_meta_by_values(vec![value], top_n).await?;
        Ok(items.pop())
    }

    pub async fn search_by_key(&self, key: &str, page: Pagination) -> AppResult<RepoSearchResult> {
        let key = key.trim();
        if let Some(cache) = &self.cache {
            if let Ok(Some(cached)) = cache.get(key, page).await {
                return Ok(cached);
            }
        }

        let result = if key.is_empty() {
            RepoSearchResult {
                repos: self.repos.list(page).await?,
                tags: self.enrich_search_tags(self.repo_tags.list_tags(page).await?).await?,
            }
        } else {
            RepoSearchResult {
                repos: self.repos.search_by_key(key, page).await?,
                tags: self
                    .enrich_search_tags(self.repo_tags.search_tags_by_key(key, page).await?)
                    .await?,
            }
        };

        if let Some(cache) = &self.cache {
            let _ = cache.set(key, page, &result).await;
        }

        Ok(result)
    }

    async fn enrich_search_tags(&self, page: Page<Tag>) -> AppResult<Page<RepoSearchTagItem>> {
        let totals = self.repo_tags.count_repos_by_tags(&page.items).await?;
        let items = page
            .items
            .into_iter()
            .map(|tag| {
                let label = tag.label.as_str().to_string();
                let value = tag.value.as_str().to_string();
                let repos_total = totals
                    .get(&(label.clone(), value.clone()))
                    .copied()
                    .unwrap_or(0);
                RepoSearchTagItem {
                    label,
                    value,
                    description: tag.description,
                    repos_total,
                }
            })
            .collect();
        Ok(Page {
            items,
            meta: page.meta,
        })
    }
    pub async fn list_tag_facets_by_active_values(
        &self,
        active_values: Vec<String>,
        limit: Option<u32>,
    ) -> AppResult<Vec<RepoTagFacet>> {
        let mut dedup = BTreeSet::new();
        let mut normalized = Vec::new();
        for value in active_values {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            if dedup.insert(value.to_string()) {
                normalized.push(value.to_string());
            }
        }
        self.repo_tags
            .list_tag_facets_by_active_values(&normalized, limit)
            .await
    }
}
