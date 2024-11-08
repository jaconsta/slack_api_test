use std::time::{SystemTime, UNIX_EPOCH};

pub enum METHOD {
    /// The chat history
    /// Fetches a conversation's history of messages and events.
    /// https://api.slack.com/methods/conversations.history
    ConversationHistory,
    /// Channels
    /// Lists all channels in a Slack team.
    /// https://api.slack.com/methods/conversations.list
    Channels,
}

pub struct ApiMethod {
    pub action: String,
    pub method: String,
}

fn new_api_method(action: String, method: String) -> ApiMethod {
    ApiMethod { action, method }
}

// Get the API method to perform a certain operation.
pub fn get_method(m: METHOD) -> ApiMethod {
    let post = String::from("post");
    let get = String::from("get");
    match m {
        METHOD::ConversationHistory => new_api_method(String::from("conversations.history"), post),
        METHOD::Channels => new_api_method(String::from("conversations.list"), get),
    }
}

#[derive(Debug, Clone)]
pub struct ChatHistoryOptions {
    // Pagination limit (max: 100)
    limit: u32,
    // For pagination, In docs: Cursor
    next_page: Option<String>,
    // Messages after the given timestap
    messages_since: Option<u64>,
}

impl Default for ChatHistoryOptions {
    fn default() -> Self {
        let messages_since = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Some(n.as_secs() - 3000),
            Err(_) => None,
        };

        Self {
            limit: 10,
            next_page: None,
            messages_since,
        }
    }
}

impl ChatHistoryOptions {
    pub fn to_query_args(&self) -> String {
        let mut query_resp = format!("limit={}", self.limit.clone());
        if let Some(query) = &self.next_page {
            query_resp.push_str(format!("&cursor={}", query).as_str());
        }
        if let Some(query) = &self.messages_since {
            query_resp.push_str(format!("&oldest={}.000200", query).as_str());
        }

        query_resp
    }
}
