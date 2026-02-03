use std::collections::HashSet;

use app::auth::RolePolicy;
use domain::{Role, UserId};

#[derive(Clone)]
pub struct ConfigRolePolicy {
    admin_ids: HashSet<String>,
}

impl ConfigRolePolicy {
    pub fn new(admin_github_ids: Vec<i64>) -> Self {
        let admin_ids = admin_github_ids.into_iter().map(|id| id.to_string()).collect();
        Self { admin_ids }
    }
}

impl RolePolicy for ConfigRolePolicy {
    fn role_for(&self, user_id: &UserId) -> Role {
        if self.admin_ids.contains(user_id.as_str()) {
            Role::Admin
        } else {
            Role::Member
        }
    }
}
