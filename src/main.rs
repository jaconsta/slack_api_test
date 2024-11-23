mod conversation;

use std::{
    fmt::Error,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use conversation::entity::{
    channels_service::{Channel, Message},
    users_service,
};
use dotenv::dotenv;

use crate::conversation::entity::users::User;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Starting fetch data!");
    run().await.unwrap();
    println!("Close, bye!");
}

async fn run() -> Result<(), Error> {
    let slack_chann_res = Channel::load_slack_channels().await;
    if let Err(e) = slack_chann_res {
        println!("There was an error loading {:?}", e);
        return Ok(());
    }
    let slack_users = users_service::load_slack_users();
    let users_sould_notify = User::get_notifyable(&slack_users);

    // Load time
    let mut last_capture = SystemTime::now().duration_since(UNIX_EPOCH);
    // let mut timestamp: i64 = 0;

    let mut should_notify: bool;
    const NONE_REPLY: std::option::Option<Message> = None;
    let mut message_replies: Box<[Option<Message>; 30]> = Box::new([NONE_REPLY; 30]);

    loop {
        should_notify = false;
        // Load replies to messages
        for (i, msg_el) in message_replies.to_owned().iter().enumerate() {
            let message = match msg_el {
                Some(e) => e,
                _ => continue,
            };
            let msg_response = match Channel::load_replies(
                &message.channel_id.clone().unwrap(),
                &message.received_ts,
            )
            .await
            {
                Ok(c) => c,
                Err(e) => {
                    println!("Error loading replies {:?}", e);
                    return Ok(());
                }
            };

            let mut reply = match msg_response.first() {
                Some(r) => r.to_owned(),
                None => {
                    println!(
                        "Error get reply for channel_id {:?} ts {:?}",
                        message.channel_id, message.received_ts
                    );

                    return Ok(());
                }
            };

            reply.channel_id = message.channel_id.to_owned();

            // Should notifiy
            let msg_users = reply.users_list();
            if User::ids_intersect(&msg_users, &users_sould_notify) {
                should_notify = true;
            }

            // End by updating the index with the new reply data
            message_replies[i] = Some(reply.to_owned());
        }

        // Load new messages
        for s in slack_chann_res.as_ref().unwrap() {
            if s.should_skip {
                continue;
            }

            let msg_response = s.load_channel_messages().await;
            println!("Queried channel: {}", s.name);

            if let Err(e) = msg_response {
                println!("There was an error loading messages {:?}", e);
                continue;
            }

            for mut msg in msg_response.unwrap() {
                msg.set_channel_id(&s.channel_id);

                let msg_users = msg.users_list();
                if User::ids_intersect(&msg_users, &users_sould_notify) {
                    should_notify = true;
                }

                message_replies.rotate_right(1);
                message_replies[0] = Some(msg);
            }
        }

        Message::bubble_sort(&mut message_replies);

        if should_notify {
            println!("-----\n---\n----\nHey! check slack----\n---\n----\n");
        }
        thread::sleep(Duration::from_secs(300));
    }

    // Ok(())
}
