#[derive(Debug, Clone)]
pub struct User {
    slack_user_id: String,
    #[allow(dead_code)]
    name: String,
    pub should_follow: bool,
}

impl User {
    pub fn new(slack_user_id: &str, name: &str, should_follow: bool) -> User {
        User {
            slack_user_id: slack_user_id.into(),
            name: name.into(),
            should_follow,
        }
    }

    pub fn get_notifyable(users: &Vec<User>) -> Vec<&str> {
        users
            .iter()
            .filter(|u| u.should_follow)
            .map(|u| u.slack_user_id.as_str())
            .collect()
    }

    pub fn ids_intersect(msg_users: &Vec<String>, notify_to: &Vec<&str>) -> bool {
        let mut user_intersection = msg_users.iter().filter(|u| notify_to.contains(&u.as_str()));

        user_intersection.next().is_some()
    }
}
