use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub enum METHOD {
    /// The chat history
    /// Fetches a conversation's history of messages and events.
    /// https://api.slack.com/methods/conversations.history
    ConversationHistory,
    /// Channels
    /// Lists all channels in a Slack team.
    /// https://api.slack.com/methods/conversations.list
    Channels,
    /// Chat replies
    /// Retrieve a thread of messages posted to a conversation
    /// https://api.slack.com/methods/conversations.replies
    Replies,
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
    let get = String::from("get");
    match m {
        METHOD::ConversationHistory => new_api_method(String::from("conversations.history"), get),
        METHOD::Channels => new_api_method(String::from("conversations.list"), get),
        METHOD::Replies => new_api_method(String::from("conversations.replies"), get),
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

    // Message id. Often ts
    message_id: Option<String>,
    // Channel id
    channel_id: Option<String>,
}

impl Default for ChatHistoryOptions {
    fn default() -> Self {
        let messages_since = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Some(n.as_secs() - 300),
            Err(_) => None,
        };

        Self {
            limit: 10,
            next_page: None,
            messages_since,
            message_id: None,
            channel_id: None,
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
    pub fn to_query_one_args(&self) -> String {
        let mut query_resp = format!("limit={}&inclusive=true", self.limit);
        if let Some(query) = &self.channel_id {
            query_resp.push_str(format!("&channel={}", query).as_str());
        }
        if let Some(query) = &self.message_id {
            query_resp.push_str(format!("&oldest={}", query).as_str());
        }

        query_resp
    }

    pub fn only_one(&mut self) {
        self.limit = 1;
    }

    pub fn set_message_thread(&mut self, channel_id: &str, message_id: &str) {
        self.channel_id = Some(channel_id.into());
        self.message_id = Some(message_id.into());
    }
}
