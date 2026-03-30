use domain::{Project, Repo};

pub trait RepoProjectOverrideExt {
    fn with_project_overrides(self, project: &Project) -> Self;
}

impl RepoProjectOverrideExt for Repo {
    fn with_project_overrides(mut self, project: &Project) -> Self {
        let existing_homepage = self.homepage_url.clone();
        let existing_avatar = self.avatar_url.clone();

        self.homepage_url =
            clean_url(existing_homepage.as_deref()).or_else(|| clean_url(project.url.as_deref()));
        self.avatar_url = clean_url(project.avatar_url.as_deref())
            .or_else(|| clean_url(existing_avatar.as_deref()))
            .or_else(|| owner_avatar_fallback(self.id.as_str()));
        self
    }
}

fn clean_url(value: Option<&str>) -> Option<String> {
    let trimmed = value?.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

fn owner_avatar_fallback(repo_id: &str) -> Option<String> {
    let owner = repo_id.split('/').next()?.trim();
    if owner.is_empty() {
        return None;
    }
    Some(format!("https://github.com/{owner}.png"))
}
