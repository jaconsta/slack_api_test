mod conversation;

use std::fmt::Error;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    let mut last_capture = last_run();

    let mut should_notify: bool;
    const NONE_REPLY: std::option::Option<Message> = None;
    let mut message_replies: Box<[Option<Message>; 30]> = Box::new([NONE_REPLY; 30]);

    loop {
        println!(
            "-----\n---\nRunning cycle!---\n  last_capture {}.\n---\n",
            last_capture
        );
        should_notify = false;

        // Load replies to messages
        for (i, msg_el) in message_replies.to_owned().iter().enumerate() {
            let message = match msg_el {
                Some(e) => e,
                _ => continue,
            };

            let reply_resp = match Channel::load_replies(
                &message.channel_id.clone().unwrap(),
                &message.received_ts,
            )
            .await
            {
                Ok(c) => c,
                Err(e) => {
                    println!("\x1b[93mError loading replies {:?}\x1b[0m", e);
                    continue;
                }
            };
            let mut reply = match reply_resp {
                Some(r) => r,
                _ => continue,
                // None => {
                //     println!(
                //         "Get reply got no messages for channel_id {:?} ts {:?}",
                //         message.channel_id, message.received_ts
                //     );

                //     continue;
                // }
            };

            reply.channel_id = message.channel_id.to_owned();

            // Should notifiy
            let msg_users = reply.users_list();
            let has_replies = match &reply.reply {
                Some(r) => r.latest_reply > last_capture,
                None => false,
            };
            println!(
                "msg_response: match found one, users {:?} replies {}. r.latest_reply {:?} >? last_capture {}",
                &msg_users, &has_replies, &reply.reply, &last_capture
            );
            if User::ids_intersect(&msg_users, &users_sould_notify) && has_replies {
                should_notify = true;
            }

            if has_replies {
                println!(
                    "Found new reply on channel_id {} with ts {}",
                    &reply.channel_id.clone().unwrap_or("".to_string()),
                    message.received_ts
                );
                println!("users list {:?}", reply.users_list());
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
            // println!("Queried channel: {}", s.name);

            if let Err(e) = msg_response {
                println!("\x1b[93mThere was an error loading messages {:?}\x1b[0m", e);
                continue;
            }

            for mut msg in msg_response.unwrap() {
                println!("Found new message with ts {}", msg.received_at);
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
            println!("\x1b[93m-----\n---\n----\nHey! check slack----\n---\n----\n\x1b[0m");
        }

        last_capture = last_run();
        thread::sleep(Duration::from_secs(300));
    }

    // Ok(())
}

fn last_run() -> usize {
    (match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }) as usize
}
