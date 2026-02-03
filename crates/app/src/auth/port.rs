use domain::{Role, UserId};

use crate::app_error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRedirect {
    pub authorization_url: String,
    pub state: String,
    pub code_verifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceOwner {
    pub user_id: UserId,
    pub login: String,
    pub avatar_url: Option<String>,
}

#[async_trait::async_trait]
pub trait OAuth2AuthorizationCodePkcePort: Send + Sync {
    async fn authz_req(&self) -> AppResult<AuthorizationRedirect>;
    async fn token_req(&self, code: String, code_verifier: String) -> AppResult<AccessToken>;
}

#[async_trait::async_trait]
pub trait OAuth2ResourceOwnerPort: Send + Sync {
    async fn resource_owner_req(&self, token: AccessToken) -> AppResult<ResourceOwner>;
}

pub trait RolePolicy: Send + Sync {
    fn role_for(&self, user_id: &UserId) -> Role;
}
