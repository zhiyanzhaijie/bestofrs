use serde::{Deserialize, Serialize};

use super::{Role, UserId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: UserId,
    pub login: String,
    pub avatar_url: Option<String>,
    pub role: Role,
}
