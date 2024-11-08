use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelResponse {
    pub ok: bool,
    pub channels: Option<Vec<ConversationChannel>>,
    pub error: Option<String>,
    pub needed: Option<String>,
    pub provided: Option<String>,
    pub response_metadata: Option<PaginationMetadata>,
}

//  A channel-like conversations in a workspace
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConversationChannel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub is_group: bool,
    pub is_im: bool,
    pub is_mpim: bool,
    pub is_private: bool,
    pub created: usize,
    pub is_archived: bool,
    pub is_general: bool,
    pub unlinked: usize,
    pub name_normalized: String,
    pub is_shared: bool,
    pub is_org_shared: bool,
    pub is_pending_ext_shared: bool,
    pub pending_shared: Vec<String>,
    pub context_team_id: String,
    pub updated: usize,
    pub parent_conversation: Option<String>,
    pub creator: String,
    pub is_ext_shared: bool,
    pub shared_team_ids: Option<Vec<String>>,
    pub pending_connected_team_ids: Vec<String>,
    pub is_member: bool,
    pub topic: Topic,
    pub purpose: Topic,
    pub previous_names: Vec<String>,
    pub num_members: usize,
    // properties: Map<String, Map<String, usize | String | Boolean>>
}

impl ConversationChannel {
    pub fn is_elegible(&self) -> bool {
        return self.is_member == true && !self.is_archived;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Topic {
    pub value: String,
    pub creator: String,
    pub last_set: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PaginationMetadata {
    pub next_cursor: String,
}

#[cfg(test)]
mod test {
    use crate::conversation::channels_str::{ChannelResponse, ConversationChannel};

    #[test]
    fn sample_load() {
        let serialized = "
        {
          \"id\": \"C07BSNU3GG1\",
          \"name\": \"feature-navigation\",
          \"is_channel\": true,
          \"is_group\": false,
          \"is_im\": false,
          \"is_mpim\": false,
          \"is_private\": false,
          \"created\": 1720428655,
          \"is_archived\": false,
          \"is_general\": false,
          \"unlinked\": 0,
          \"name_normalized\": \"feature-navigation\",
          \"is_shared\": false,
          \"is_org_shared\": false,
          \"is_pending_ext_shared\": false,
          \"pending_shared\": [],
          \"context_team_id\": \"T0279E2GQPQ\",
          \"updated\": 1720428655907,
          \"parent_conversation\": null,
          \"creator\": \"U04853SN1AP\",
          \"is_ext_shared\": false,
          \"shared_team_ids\": [
            \"T0279E2GQPQ\"
          ],
          \"pending_connected_team_ids\": [],
          \"is_member\": false,
          \"topic\": {
            \"value\": \"\",
            \"creator\": \"\",
            \"last_set\": 0
          },
          \"purpose\": {
            \"value\": \"\",
            \"creator\": \"\",
            \"last_set\": 0
          },
          \"previous_names\": [],
          \"num_members\": 8
        }
        ";
        let channel: ConversationChannel = serde_json::from_str(&serialized).unwrap();

        assert_eq!(channel.id, "C07BSNU3GG1");
    }

    #[test]
    fn parse_missing_scope() {
        let error_response = "{
           \"ok\": false,
           \"error\":  \"missing_scope\",
           \"needed\": \"usergroups:read\",
           \"provided\": \"identify,channels:history,groups:history,im:history,mpim:history,channels:read,groups:read,im:read,calls:write,calls:read\"
        }";
        let channel: ChannelResponse = serde_json::from_str(&error_response).unwrap();

        assert_eq!(channel.channels, None);
        assert_eq!(channel.error, Some(String::from("missing_scope")));
    }
}
