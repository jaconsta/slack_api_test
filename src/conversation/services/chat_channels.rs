use crate::conversation::{
    channels_str::ChannelResponse,
    errors_str::QueryError,
    methods_aggregate::{get_method, METHOD},
};

pub async fn get_conversation_channels(
    paginate: Option<&str>,
) -> Result<ChannelResponse, QueryError> {
    let client = reqwest::Client::new();

    let slack_method = get_method(METHOD::Channels);
    if !slack_method.method.eq("get") {
        return Err(QueryError::new("Slack action method, should be GET"));
    }

    let mut headers = reqwest::header::HeaderMap::new();
    let auth_token = std::env::var("SLACK_TOKEN").unwrap();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json; charset=utf-8".parse().unwrap(),
    );

    let mut url: String = format!(
        "https://slack.com/api/{}?exclude_archived=true",
        slack_method.action
    );
    if let Some(paginate_cursor) = paginate {
        url = format!(
            "{}&cursor={}",
            String::from(url),
            String::from(paginate_cursor)
        )
        .as_str()
        .to_owned();
    }

    let res = client.get(url).headers(headers).send().await;

    if let Err(x) = res {
        println!("{:?}", x);
        return Err(QueryError::new("Query convert res to response"));
    }

    let response = res.unwrap();
    let res_json = response.json::<ChannelResponse>().await;
    if let Err(x) = res_json {
        println!("{:?}", x);
        return Err(QueryError::new("Query convert response to json"));
    }

    let body = res_json.unwrap();
    Ok(body)
}

#[cfg(test)]
mod test {
    use crate::conversation::services::chat_channels::get_conversation_channels;

    #[tokio::test]
    async fn loads_the_chat_lists() {
        let res = get_conversation_channels(None).await;
        let fu;
        if let Err(m) = res {
            println!("Error was cought");
            println!("{:?}", m);
            assert_eq!(false, true);
            return;
        }

        fu = res.unwrap();
        println!("{:?}", &fu.channels);
        assert_eq!(fu.ok, true);
    }

    #[tokio::test]
    async fn processes_the_channel_lists() {
        let res = get_conversation_channels(None).await;
        let fu;
        if let Err(m) = res {
            println!("Error was cought");
            println!("{:?}", m);
            assert_eq!(false, true);
            return;
        }

        fu = res.unwrap();
        assert_eq!(fu.ok, true);

        if let Some(slack_channels) = fu.channels {
            println!("Found {} channels", slack_channels.len());
            for slack_channel in &slack_channels {
                println!("{} - {}", &slack_channel.id, &slack_channel.name);
            }
        }

        assert!(fu.response_metadata.is_some());
        let more_lists: &str = &fu.response_metadata.unwrap().next_cursor;

        let res = get_conversation_channels(Some(more_lists)).await;
        let fu;
        if let Err(m) = res {
            println!("Error was cought");
            println!("{:?}", m);
            assert_eq!(false, true);
            return;
        }

        fu = res.unwrap();
        assert_eq!(fu.ok, true);

        if let Some(slack_channels) = fu.channels {
            println!("Found {} channels", slack_channels.len());
            for slack_channel in &slack_channels {
                println!("{} - {}", &slack_channel.id, &slack_channel.name);
            }
        }
        assert!(fu.response_metadata.is_some_and(|x| x.next_cursor.eq("")));
    }
}
