use crate::conversation::{
    errors_str::QueryError,
    messages_str::MessageResponse,
    methods_aggregate::{get_method, ChatHistoryOptions, METHOD},
};

pub async fn get_chat_history(
    chat_id: &str,
    args: Option<ChatHistoryOptions>,
) -> Result<MessageResponse, QueryError> {
    let client = reqwest::Client::new();
    let slack_method = get_method(METHOD::ConversationHistory);
    if slack_method.action.eq("post") {
        return Err(QueryError::new("Slack action method, should be POST"));
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

    let mut slack_url = format!("https://slack.com/api/{}", slack_method.action);
    if let Some(query_args) = args {
        slack_url.push_str(&format!("?channel={}&", &chat_id));
        slack_url.push_str(&query_args.to_query_args());
    }

    let res = client.get(slack_url).headers(headers).send().await;
    let response;
    if let Ok(rexponse) = res {
        response = rexponse.json::<MessageResponse>().await;
    } else {
        println!("Ohh noo {:?}", res);
        return Err(QueryError::new(""));
    }

    let body_r = response;
    let body;
    if let Ok(bodyx) = body_r {
        // println!("casual body {:?}", bodyx);
        body = bodyx;
    } else {
        println!("Ohh body {:?}", body_r);
        return Err(QueryError::new(""));
    }

    Ok(body)
}

pub async fn get_chat_reply(
    history_options: ChatHistoryOptions,
) -> Result<MessageResponse, QueryError> {
    let client = reqwest::Client::new();

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

    let slack_url = format!(
        "https://slack.com/api/{}?{}",
        "conversations.history",
        history_options.to_query_one_args()
    );

    let res = client.get(slack_url).headers(headers).send().await;
    let response;
    if let Ok(rexponse) = res {
        response = rexponse.json::<MessageResponse>().await;
    } else {
        println!("Ohh noo {:?}", res);
        return Err(QueryError::new(""));
    }

    let body_r = response;
    let body;
    if let Ok(bodyx) = body_r {
        // println!("casual body {:?}", bodyx);
        body = bodyx;
    } else {
        println!("Ohh body {:?}", body_r);
        return Err(QueryError::new(""));
    }

    Ok(body)
}

#[cfg(test)]
mod test {
    use super::get_chat_history;

    #[tokio::test]
    async fn loads_the_history() {
        // Possitble response
        // { warning: "missing_charset", response_metadata: {"warnings": ["missing_charset"]} }
        // { error: "invalid_post_type" }
        // { error: "not_authed" }
        let res = get_chat_history("C07B1EWKYJX", None).await;
        let fu;
        if let Err(m) = res {
            println!("Error was cought");
            println!("{:?}", m);
            assert_eq!(m.to_string(), "".to_string());
            assert_eq!(false, true);
            return;
        } else {
            fu = res.unwrap();
        }

        println!("{:?}", fu);

        assert_eq!(fu.ok, true);
    }
}
