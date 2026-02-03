use std::sync::Arc;

use domain::AuthUser;

use crate::app_error::AppResult;
use crate::auth::{
    AuthorizationRedirect, OAuth2AuthorizationCodePkcePort, OAuth2ResourceOwnerPort, RolePolicy,
};

#[derive(Clone)]
pub struct AuthCommandHandler {
    oauth: Arc<dyn OAuth2AuthorizationCodePkcePort>,
    resource_owner: Arc<dyn OAuth2ResourceOwnerPort>,
    roles: Arc<dyn RolePolicy>,
}

impl AuthCommandHandler {
    pub fn new(
        oauth: Arc<dyn OAuth2AuthorizationCodePkcePort>,
        resource_owner: Arc<dyn OAuth2ResourceOwnerPort>,
        roles: Arc<dyn RolePolicy>,
    ) -> Self {
        Self {
            oauth,
            resource_owner,
            roles,
        }
    }

    pub async fn start_login(&self) -> AppResult<AuthorizationRedirect> {
        self.oauth.authz_req().await
    }

    pub async fn complete_login(&self, code: String, code_verifier: String) -> AppResult<AuthUser> {
        let token = self.oauth.token_req(code, code_verifier).await?;
        let owner = self.resource_owner.resource_owner_req(token).await?;

        let role = self.roles.role_for(&owner.user_id);

        Ok(AuthUser {
            user_id: owner.user_id,
            login: owner.login,
            avatar_url: owner.avatar_url,
            role,
        })
    }
}
