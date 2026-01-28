use std::slice::from_ref;
use std::sync::Arc;

use domain::{Repo, RepoId, Tag, TagLabel, TagValue};

use crate::app_error::AppResult;
use crate::repo::{RepoRepo, RepoTagRepo};

#[derive(Clone)]
pub struct RepoCommandHandler {
    repos: Arc<dyn RepoRepo>,
    repo_tags: Arc<dyn RepoTagRepo>,
}

#[derive(Debug, Clone)]
pub struct TagInput {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ReplaceRepoTagsCommand {
    pub repo_id: String,
    pub tags: Vec<TagInput>,
}

impl RepoCommandHandler {
    pub fn new(repos: Arc<dyn RepoRepo>, repo_tags: Arc<dyn RepoTagRepo>) -> Self {
        Self { repos, repo_tags }
    }
    pub async fn upsert(&self, repo: &Repo) -> AppResult<()> {
        self.upsert_many(from_ref(repo)).await
    }

    pub async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()> {
        if repos.is_empty() {
            return Ok(());
        }
        self.repos.upsert_many(repos).await?;
        let repo_ids = repos.iter().map(|repo| repo.id.clone()).collect::<Vec<_>>();
        self.repo_tags
            .ensure_default_tag_for_repos(&repo_ids)
            .await?;
        Ok(())
    }

    pub async fn replace_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()> {
        self.repo_tags.replace_repo_tags(repo_id, tags).await
    }

    pub async fn replace_tags_by_repo_id(
        &self,
        cmd: ReplaceRepoTagsCommand,
    ) -> AppResult<()> {
        let repo_id = RepoId::parse(&cmd.repo_id)?;
        let tags = cmd
            .tags
            .into_iter()
            .map(|tag| Tag {
                label: TagLabel::new(tag.label),
                value: TagValue::new(tag.value),
            })
            .collect::<Vec<_>>();
        self.replace_tags(&repo_id, &tags).await
    }
}
