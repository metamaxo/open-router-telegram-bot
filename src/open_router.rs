use crate::constants::OPEN_ROUTER_COMPLETIONS_URL;
use crate::messages;
use crate::messages::openrouter::{Message, Request};
use crate::telegram_bot::TgBot;
use crate::Error;
use messages::bot_messages;

// NOTE: Simplified the open router calling logic and added it to the TgBot struct.
// This makes it so that fields like `model` are always consistent with the bot instance.
// Also makes it so that the http client used is that of the bot instance, so we don't
// have to create a new client every time we want to make a request.
//
// NOTE: Also removed use of `json` macro in favor of constructing the JSON object as a struct,
// which is then serialized to JSON. This makes it easier to make changes later.
impl TgBot {
    pub async fn call_open_router(&self, message: &str) -> Result<Vec<String>, Error> {
        let request = Request {
            model: self.model(),
            messages: vec![Message {
                role: "user".to_string(),
                content: format!("{}{}", bot_messages::PROMPT, message),
            }],
        };

        let req = self
            .http_client
            .post(OPEN_ROUTER_COMPLETIONS_URL)
            .header("Authorization", format!("Bearer {}", self.open_router_key()))
            .header("Content-Type", "application/json")
            .json(&request)
            .build()?;

        println!("{:?}", req);
        let response = self.http_client.execute(req).await?.json::<serde_json::Value>().await?;

        println!("{:?}", response);

        let response = serde_json::from_value::<messages::openrouter::Response>(response)?;

        let mut result = Vec::new();
        for update in response.choices {
            let r = update.message.content;
            result.push(r);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::telegram_bot::Config;
    use dotenvy::dotenv;
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_call_open_router() -> Result<(), Error> {
        // Load .env file if present
        dotenv().ok();
        
        // Get API key from environment or skip test
        let api_key = match env::var("OPEN_ROUTER_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("Skipping test: OPEN_ROUTER_KEY not set");
                return Ok(());
            }
        };

        let cfg = Config {
            open_router_key: api_key,
            ..Default::default()
        };
        
        let bot = TgBot::new(cfg);
        let message = "hello";
        let response = bot.call_open_router(message).await?;
        
        // Basic validation of response
        assert!(!response.is_empty());
        Ok(())
    }
}
