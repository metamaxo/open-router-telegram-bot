use crate::messages;
use crate::Error;
use messages::bot_messages;
use serde_json::json;
use std::env;

///uses the open_router api to generate a response
pub async fn open_router(message: &String, model: &str) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    let response = get_response(
        json!({
            "model": model,
            "messages": [
            {
                "role": "user",
                "content": format!("{}{}", bot_messages::PROMPT, message)
            }
            ]
        }),
        env::var("OPEN_ROUTER_KEY").expect("Open-Router key must be set!"),
    )
    .await;

    tracing::debug!("got response");
    match response {
        Ok(content) => {
            for update in content.choices {
                let r = update.message.content;
                result.push(r);
            }
            Ok(result)
        }
        Err(e) => {
            tracing::error!(?e, "Failed to get response from OpenAI");
            Err(Error::Generic(
                "I'm sorry, something went wrong".to_string(),
            ))
        }
    }
}
pub async fn get_response(
    body: serde_json::Value,
    token: String,
) -> Result<messages::openrouter::Response, reqwest::Error> {
    tracing::debug!("sending request to openrouter");
    let client = reqwest::Client::new();
    const URL: &str = "https://openrouter.ai/api/v1/chat/completions";
    client
        .post(URL)
        .header("Authorization", token)
        .json(&body)
        .send()
        .await?
        .json()
        .await
}
