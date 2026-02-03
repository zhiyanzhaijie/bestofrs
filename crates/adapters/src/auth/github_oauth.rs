use app::app_error::{AppError, AppResult};
use app::auth::{
    AccessToken, AuthorizationRedirect, OAuth2AuthorizationCodePkcePort, OAuth2ResourceOwnerPort,
    ResourceOwner,
};

use domain::UserId;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::redirect::Policy;

type GithubOAuthClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Debug, serde::Deserialize)]
struct GithubUserResponse {
    id: i64,
    login: String,
    avatar_url: Option<String>,
}

#[derive(Clone)]
pub struct GithubOAuthAdapter {
    client: GithubOAuthClient,
    http: reqwest::Client,
}

impl GithubOAuthAdapter {
    pub fn new(
        github_client_id: String,
        github_client_secret: String,
        github_redirect_url: String,
    ) -> AppResult<Self> {
        let client = BasicClient::new(ClientId::new(github_client_id))
            .set_client_secret(ClientSecret::new(github_client_secret))
            .set_auth_uri(
                AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                    .map_err(AppError::internal)?,
            )
            .set_token_uri(
                TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .map_err(AppError::internal)?,
            )
            .set_redirect_uri(RedirectUrl::new(github_redirect_url).map_err(AppError::internal)?);

        let http = reqwest::Client::builder()
            .user_agent("bestofrs")
            .redirect(Policy::none())
            .build()
            .map_err(AppError::internal)?;

        Ok(Self { client, http })
    }
}

#[async_trait::async_trait]
impl OAuth2AuthorizationCodePkcePort for GithubOAuthAdapter {
    async fn authz_req(&self) -> AppResult<AuthorizationRedirect> {
        let (pkce_challenge, code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok(AuthorizationRedirect {
            authorization_url: url.to_string(),
            state: csrf_token.secret().to_string(),
            code_verifier: code_verifier.secret().to_string(),
        })
    }

    async fn token_req(&self, code: String, code_verifier: String) -> AppResult<AccessToken> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(code_verifier))
            .request_async(&self.http)
            .await
            .map_err(AppError::upstream)?;

        Ok(AccessToken {
            access_token: token.access_token().secret().to_string(),
        })
    }
}

#[async_trait::async_trait]
impl OAuth2ResourceOwnerPort for GithubOAuthAdapter {
    async fn resource_owner_req(&self, token: AccessToken) -> AppResult<ResourceOwner> {
        let resp = self
            .http
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(AppError::upstream)?
            .error_for_status()
            .map_err(AppError::upstream)?;

        let u: GithubUserResponse = resp.json().await.map_err(AppError::upstream)?;

        Ok(ResourceOwner {
            user_id: UserId::new(u.id.to_string()),
            login: u.login,
            avatar_url: u.avatar_url,
        })
    }
}
