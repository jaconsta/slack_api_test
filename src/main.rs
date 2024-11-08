mod conversation;

use std::fmt::Error;

use conversation::entity::channels_service::Channel;
use dotenv::dotenv;

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
    for s in slack_chann_res.unwrap() {
        if s.should_skip {
            continue;
        }

        let msags = s.load_channel_messages().await;
        println!("Queried channel: {}", s.name);

        if let Err(e) = msags {
            println!("There was an error loading messages {:?}", e);
            continue;
        }

        for m in msags.unwrap() {
            println!("Users in message {:?}", m.users_list());
            println!("message text {:?}", m.message);
            println!("message text {:?}", m.sender);
            if m.reply.is_some() {
                println!("message text {:?}", m.reply);
            }
        }
    }

    Ok(())
}
