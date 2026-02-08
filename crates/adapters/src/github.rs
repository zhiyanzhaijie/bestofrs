use app::app_error::{AppError, AppResult};
use app::repo::{GithubGateway, GithubReadme, GithubRepoInfo};
use base64::Engine;

#[derive(Clone)]
pub struct GithubClient {
    client: reqwest::Client,
    token: Option<String>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent("bestofrs")
            .build()
            .map_err(AppError::internal)?;
        let token = token
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        Ok(Self { client, token })
    }

    fn authorized_get(&self, url: String) -> reqwest::RequestBuilder {
        let mut req = self.client.get(url);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("token {token}"));
        }
        req
    }

    fn decode_readme_content(content: &str, encoding: &str) -> AppResult<String> {
        if !encoding.eq_ignore_ascii_case("base64") {
            return Err(AppError::upstream(format!(
                "Unsupported README encoding: {encoding}"
            )));
        }

        let cleaned = content.replace(['\n', '\r'], "");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(cleaned)
            .map_err(AppError::upstream)?;

        String::from_utf8(decoded).map_err(AppError::upstream)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubRepoResponse {
    id: i64,
    full_name: String,
    stargazers_count: i64,
    forks_count: i64,
    open_issues_count: i64,
    subscribers_count: i64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubReadmeResponse {
    content: String,
    encoding: String,
    html_url: Option<String>,
    download_url: Option<String>,
}

#[async_trait::async_trait]
impl GithubGateway for GithubClient {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo> {
        let url = format!("https://api.github.com/repos/{full_name}");

        let resp = self
            .authorized_get(url)
            .send()
            .await
            .map_err(AppError::upstream)?
            .error_for_status()
            .map_err(AppError::upstream)?;
        let repo: GithubRepoResponse = resp.json().await.map_err(AppError::upstream)?;

        Ok(GithubRepoInfo {
            id: repo.id,
            full_name: repo.full_name,
            stargazers_count: repo.stargazers_count,
            forks_count: repo.forks_count,
            open_issues_count: repo.open_issues_count,
            subscribers_count: repo.subscribers_count,
        })
    }

    async fn fetch_readme(&self, full_name: &str) -> AppResult<Option<GithubReadme>> {
        let url = format!("https://api.github.com/repos/{full_name}/readme");

        let resp = self
            .authorized_get(url)
            .send()
            .await
            .map_err(AppError::upstream)?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let body: GithubReadmeResponse = resp
            .error_for_status()
            .map_err(AppError::upstream)?
            .json()
            .await
            .map_err(AppError::upstream)?;

        let content = Self::decode_readme_content(&body.content, &body.encoding)?;

        Ok(Some(GithubReadme {
            content,
            html_url: body.html_url,
            download_url: body.download_url,
        }))
    }
}
