use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MessageJoin {
    pub subtype: String,
    pub user: String,
    pub text: String,
    pub inviter: String,
    // field: type
    pub message_type: String,
    pub ts: String, // f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MessageNormal {
    pub user: Option<String>,
    // field: type
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    pub ts: String, // f64,
    pub text: String,

    // Random data
    pub client_msg_id: Option<String>,
    pub subtype: Option<String>,
    pub inviter: Option<String>,

    // Message parts
    pub blocks: Option<Vec<BlockInfo>>,
    // Was the message pinned.
    pub pinned_to: Option<Vec<String>>,
    pub pinned_info: Option<PinnedInfo>,
    // Emojis
    pub reactions: Option<Vec<Reactions>>,
    // pub team: String,

    // Thread activitiy
    // Ignored attachement
    // pub attachement: Option<AttachementInfo>,
    pub reply_count: Option<usize>,
    pub reply_users_count: Option<usize>,
    pub latest_reply: Option<String>,
    pub reply_users: Option<Vec<String>>,
    pub thread_ts: Option<String>,

    // Bot identifier
    pub bot_id: Option<String>,
}

impl MessageNormal {
    pub fn is_elegible(&self) -> bool {
        self.message_type == Some(String::from("message")) && self.user.is_some()
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
// struct AttachementInfo {
//     pub service_name: String,
//     pub text: String,
//     pub fallback: String,
//     pub thumb_url: String,
//     pub thumb_width: usize,
//     pub thumb_height: usize,
//     pub id: usize,
// }

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BlockInfo {
    pub block_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PinnedInfo {
    pub channel: String,
    pub pinned_by: String,
    pub pinned_ts: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Reactions {
    pub name: String,
    pub users: Vec<String>,
    pub count: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MessageResponse {
    pub ok: bool,
    pub messages: Option<Vec<MessageNormal>>,
    pub latest: Option<String>,
    // The oldest message included in the response
    pub oldest: Option<String>,

    pub has_more: Option<bool>,
    pub pin_count: Option<usize>,
    pub channel_actions_ts: Option<f64>,
    pub channel_actions_count: Option<usize>,
    pub warning: Option<String>,
    // pub response_metadata: Option<HashMap<String, Vec<String>>>,
    pub error: Option<String>, // Option<HashMap<String, Vec<String>>>,
}
