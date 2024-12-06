use std::thread;

use crate::conversation::channels_str::ConversationChannel;
use crate::conversation::errors_str::SlackChannelError;
use crate::conversation::messages_str::MessageNormal;
use crate::conversation::methods_aggregate::ChatHistoryOptions;
use crate::conversation::services::chat_history::get_chat_reply;
use crate::conversation::services::{
    channels_cache_fs::{create_cache, read_cache, ChannelStorage},
    chat_channels::get_conversation_channels,
    chat_history::get_chat_history,
};

#[derive(Debug, Clone)]
pub struct Channel {
    // Channel name with "-" instead of spaces
    pub name: String,
    // Identifier. Starts with "C"
    pub channel_id: String,
    // Should be included in further steps to fetch channel messages.
    pub should_skip: bool,
}

impl From<&ConversationChannel> for Channel {
    fn from(cc: &ConversationChannel) -> Self {
        return Channel::new(cc.name.clone(), cc.id.clone(), false);
    }
}
impl From<&ChannelStorage> for Channel {
    fn from(cs: &ChannelStorage) -> Self {
        return Channel::new(cs.name.clone(), cs.channel_id.clone(), cs.ignore);
    }
}

impl Channel {
    fn new(name: String, channel_id: String, should_skip: bool) -> Channel {
        return Channel {
            name,
            channel_id,
            should_skip,
        };
    }

    pub async fn load_slack_channels() -> Result<Vec<Channel>, SlackChannelError> {
        // Try load first the cache files.
        match read_cache() {
            Ok(cached) => {
                return Ok(cached.iter().map(|c| c.into()).collect());
            }
            Err(_) => (),
        };

        // Load the channels
        let channels = get_conversation_channels(None).await;
        if let Err(cha) = channels {
            return Err(SlackChannelError::new(&cha.to_string()));
        }

        let channel_details = channels.unwrap().channels;
        if let None = channel_details {
            return Ok(vec![]);
        }
        let channel_details = channel_details.unwrap();

        let lack_channs: Vec<Channel> = channel_details
            .iter()
            .filter(|c| c.is_elegible())
            .map(|f| f.into())
            .collect();

        // Store the cache
        let lack_channs_clone = lack_channs.clone();
        thread::spawn(move || {
            let to_cache_channels = lack_channs_clone
                .iter()
                .map(|c| ChannelStorage {
                    channel_id: c.channel_id.clone(),
                    name: c.name.clone(),
                    custom: false,
                    ignore: false,
                })
                .collect();
            if let Err(fail_cached) = create_cache(&to_cache_channels) {
                println!("Error Creating slack channels cache file.");
                println!("{fail_cached:?}");
            }
        });

        return Ok(lack_channs);
    }

    pub async fn load_channel_messages(&self) -> Result<Vec<Message>, SlackChannelError> {
        let history_options = ChatHistoryOptions::default();
        let chats = get_chat_history(&self.channel_id, Some(history_options)).await;
        if let Err(err) = chats {
            return Err(SlackChannelError::new(&err.to_string()));
        }

        let chat_details = chats.unwrap().messages;
        if let None = chat_details {
            return Ok(vec![]);
        }

        let messages: Vec<Message> = chat_details
            .unwrap()
            .iter()
            .filter(|f| f.is_elegible())
            .map(|f| f.into())
            .collect();
        return Ok(messages);
    }

    pub async fn load_replies(
        channel_id: &str,
        message_id: &str,
    ) -> Result<Option<Message>, SlackChannelError> {
        let mut history_options = ChatHistoryOptions::default();
        history_options.set_message_thread(channel_id, message_id);
        history_options.only_one();
        let chats = get_chat_reply(history_options).await;
        if let Err(err) = chats {
            return Err(SlackChannelError::new(&err.to_string()));
        }

        let chats = chats.unwrap();
        let chat_details = chats.messages;
        if let None = chat_details {
            println!("No new messages");
            return Ok(None); // ((vec![]);
        }

        println!(
            "Last message for channelId: {} message ts: {} is at {}.",
            channel_id,
            message_id,
            chats.oldest.unwrap_or("no_oldest".to_string()),
        );
        let messages: Option<Message> = chat_details
            .unwrap()
            .iter()
            .filter(|f| f.is_elegible())
            .map(|f| f.into())
            .next();

        return Ok(messages);
    }
}

#[derive(Debug, Clone)]
pub struct Reply {
    // pub message_count: usize,
    pub latest_reply: usize,
    // Required for further conversation.replies query
    pub latest_ts: String,
    // User_id list who have sent reply messages
    pub users: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    // Content text
    pub message: String,
    // Unique id of the channel, normally starts with C / D
    pub channel_id: Option<String>,
    // ts in seconds when the message arrived
    pub received_at: usize,
    // Required for further conversation.replies query
    pub received_ts: String,
    // Reply information
    pub reply: Option<Reply>,
    // user who sends the message
    pub sender: String,
}

impl From<&MessageNormal> for Message {
    fn from(mn: &MessageNormal) -> Self {
        let reply: Option<Reply> = match &mn.latest_reply {
            Some(latest_reply) => Some(Reply {
                latest_reply: Message::parse_ts(&latest_reply),
                latest_ts: latest_reply.clone(),
                users: mn.reply_users.clone().unwrap_or(Vec::new()),
            }),
            _ => None,
        };

        return Message::new(
            mn.text.clone(),
            mn.user.as_ref().unwrap_or(&String::from("")).into(),
            Message::parse_ts(&mn.ts),
            mn.ts.clone(),
            reply,
        );
    }
}

impl Message {
    fn new(
        message: String,
        sender: String,
        received_at: usize,
        received_ts: String,
        reply: Option<Reply>,
    ) -> Self {
        Message {
            message,
            received_at,
            received_ts,
            reply,
            sender,
            channel_id: None,
        }
    }

    // Gets the ts value in seconds
    pub fn parse_ts(ts: &str) -> usize {
        let in_seconds = ts
            .split(".")
            .take(1)
            .next()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        return in_seconds;
    }

    fn find_users_in_text(&self) -> Vec<String> {
        let mut found_users: Vec<String> = Vec::new();

        // 12 = 11(id)>

        for (start_i, _char) in self.message.match_indices("<@") {
            let end_i = start_i + 12;
            if end_i >= self.message.len() {
                continue;
            }
            if self.message.as_bytes()[end_i].eq(&">".as_bytes()[0]) {
                continue;
            }
            let char_start = start_i + 2;
            let user_id = &self.message[char_start..=end_i];
            found_users.push(user_id.into());
        }

        return found_users;
    }

    pub fn users_list(&self) -> Vec<String> {
        let mut users = self.find_users_in_text();

        users.push(self.sender.clone());
        if let Some(rreply) = &self.reply {
            for u in rreply.users.iter() {
                users.push(u.clone());
            }
        }

        users.sort();
        users.dedup();

        users
    }

    pub fn set_channel_id(&mut self, channel_id: &str) {
        self.channel_id = Some(channel_id.into());
    }
    pub fn bubble_sort<const M: usize>(messages: &mut Box<[Option<Message>; M]>) {
        for i in 0..messages.len() {
            for j in i..messages.len() - i - 1 {
                // Expect no elements to the left
                let msg_0 = match &messages[j] {
                    Some(msg) => msg.clone(),
                    None => continue,
                };
                let msg_1 = match &messages[j + 1] {
                    Some(msg) => msg.clone(),
                    None => continue,
                };
                // Get the reply times
                let reply_0 = match &msg_0.reply {
                    Some(r) => r.latest_reply,
                    None => 0,
                };
                let reply_1 = match &msg_1.reply {
                    Some(r) => r.latest_reply,
                    None => 0,
                };

                if msg_0.received_at < msg_1.received_at || reply_0 < reply_1 {
                    messages.swap(j + 1, j);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bubblesorting() {
        let unsorted_3 = [Some(Message {
            channel_id: None,
            message: "into".into(),
            received_at: 10000,
            received_ts: "10000.000".into(),
            reply: None,
            sender: "U001".into(),
        })];
        let mut messages = Box::new(unsorted_3);
        Message::bubble_sort(&mut messages);
        assert_eq!(messages[0].clone().unwrap().received_ts, "10000.000");
    }

    #[test]
    fn bubblesorting2() {
        let unsorted_3 = [
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 1_000,
                received_ts: "1000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 10_000,
                received_ts: "10000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
        ];
        let mut messages = Box::new(unsorted_3);
        Message::bubble_sort(&mut messages);
        assert_eq!(messages[0].clone().unwrap().received_ts, "10000.000");

        println!("bubblesorting 2 done");
    }

    #[test]
    fn bubblesorting3() {
        let unsorted_3 = [
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 100_000,
                received_ts: "100000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 1_000,
                received_ts: "1000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 10_000,
                received_ts: "10000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
        ];
        let mut messages = Box::new(unsorted_3);
        Message::bubble_sort(&mut messages);
        assert_eq!(messages[0].clone().unwrap().received_ts, "10000.000");

        println!("bubblesorting 3 done");
    }

    #[test]
    fn bubblesorting2_and_none() {
        let unsorted_3 = [
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 100_000,
                received_ts: "100000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 1_000,
                received_ts: "1000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            None,
        ];
        let mut messages = Box::new(unsorted_3);
        Message::bubble_sort(&mut messages);
        assert_eq!(messages[0].clone().unwrap().received_ts, "100000.000");

        println!("bubblesorting 2 and none . 3 done");
    }

    #[test]
    fn bubblesorting_none_and_1() {
        let unsorted_3 = [
            None,
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 1_000,
                received_ts: "1000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            None,
        ];
        let mut messages = Box::new(unsorted_3);
        Message::bubble_sort(&mut messages);
        assert_eq!(messages[0].is_none(), true);
        assert_eq!(messages[1].clone().unwrap().received_ts, "1000.000");

        println!("bubblesorting none and 1 . 2 done");
    }

    #[test]
    fn bubblesorting1_1reply() {
        let unsorted_3 = [
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 1_000,
                received_ts: "1000.000".into(),
                reply: None,
                sender: "U001".into(),
            }),
            Some(Message {
                channel_id: None,
                message: "into".into(),
                received_at: 100,
                received_ts: "100.000".into(),
                reply: Some(Reply {
                    latest_reply: 1_000_000,
                    latest_ts: "1000000".into(),
                    users: Vec::new(),
                }),
                sender: "U001".into(),
            }),
        ];
        let mut messages = Box::new(unsorted_3);
        Message::bubble_sort(&mut messages);
        assert_eq!(messages[0].clone().unwrap().received_ts, "100.000");

        println!("bubblesorting1_1reply done");
    }
}
