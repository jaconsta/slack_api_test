use crate::conversation::services::users_cache_fs;

use super::users::User;

pub fn load_slack_users() -> Vec<User> {
    let slack_users = users_cache_fs::read_cache();

    slack_users.unwrap_or(Vec::new())
}
