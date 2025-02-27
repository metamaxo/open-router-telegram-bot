use serde_json::json;

mod error;
mod messages;
mod utils;
use dotenvy::dotenv;
use error::Error;
use std::env;

pub struct TgBot {
    http_client: reqwest::Client,
    token: String,
    model: String,
}

impl TgBot {
    const POLLING_INTERVAL: u64 = 5000;
    const INITIAL_MESSAGE: &str = "Hello, i'm FrogAI. I'm here to answer all your questions. Just type /frog and ask a question :)";

    pub fn new(token: &str) -> Self {
        TgBot {
            token: token.to_string(),
            http_client: reqwest::Client::new(),
            model: String::from("openai/gpt-4o"),
        }
    }
    ///checks message content for commands. if the bot finds a command it will send the appropriate
    ///response, if there is no command, the bot doesnt do anything
    pub async fn send_message(&self, chat_id: &str, text: &str) -> Result<(), Error> {
        if text.contains("/help") {
            let url = format!(
                "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                self.token,
                chat_id,
                TgBot::INITIAL_MESSAGE
            );
            tracing::debug!("sending initial message");
            self.http_client.post(&url).send().await?;
            Ok(())
        } else if text.contains("/frog") {
            let question = &text[5..].to_string();
            let realquestion = question.clone();
            tracing::debug!(?question, "Question");
            let response = open_router(realquestion, &self.model).await;

            // TODO: Let op, response is een vector van strings, niet een enkele string
            let url = format!(
                "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                self.token,
                chat_id,
                response?.join("\n")
            );
            tracing::debug!("sending response");
            self.http_client.post(&url).send().await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn get_updates(&self) -> Result<messages::telegram::Response, Error> {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        Ok(self.http_client.get(&url).send().await?.json().await?)
    }

    ///makes a list of chat id's so the bot doesnt respond to the same chats gets. when theres a new
    ///update it gets handled.
    pub async fn run(&self) -> Result<(), reqwest::Error> {
        let mut id_list = Vec::new();
        loop {
            match self.get_updates().await {
                Ok(response) => {
                    tracing::debug!(?response, "Got updates");
                    for update in response.result {
                        tracing::debug!(id = ?update.update_id, ?update, ?id_list, "Handling update");
                        if id_list.contains(&update.update_id) {
                            continue;
                        } else {
                            id_list.push(update.update_id);
                            match self.handle_update(&update).await {
                                Ok(_) => {}
                                Err(e) => {
                                    tracing::error!(?e, "Failed to handle update");
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(?e, "Failed to get updates");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(Self::POLLING_INTERVAL));
        }
    }

    ///extracts chat_id and message content. passes them to the send_message function.
    pub async fn handle_update(&self, update: &messages::telegram::Update) -> Result<(), Error> {
        tracing::debug!("HANDLE UPDATE");
        let chat_id_num = update.message.chat.get_id().to_string();
        let chat_id = chat_id_num.as_str();
        let text = update.message.text.as_ref().expect("must be text");
        tracing::debug!(?chat_id, ?text, "Handling update");
        self.send_message(chat_id, text).await
    }
}

///bot starts here
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let _guard = utils::init_logger();
    let bot = TgBot::new(
        env::var("TG_BOT_KEY")
            .expect("Telegram bot key must be set!")
            .as_str(),
    );
    bot.run().await
}
///uses the open_router api to generate a response
pub async fn get_response(
    body: serde_json::Value,
    token: String,
) -> Result<messages::openrouter::Response, reqwest::Error> {
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

// TODO: Let op, wat wil je hier bereiken?
// Je eerdere versie returnt de eerste update en negeert de andere updates.
// Nu returnt hij een vector van strings, waarvan je de eerste kunt pakken voor hetzelfde
// resultaat.

///promt instructions can be modified as desired.
async fn open_router(message: String, model: &String) -> Result<Vec<String>, Error> {
    let content = format!(
        "{}{}",
        env::var("PROMPT").expect("Prompt must be set"),
        message
    );
    let body = json!({
        "model": model,
        "messages": [
        {
            "role": "user",
            "content": content
        }
        ]
    });

    let mut result = Vec::new();

    let response = get_response(
        body,
        env::var("OPEN_ROUTER_KEY").expect("Open-Router key must be set!"),
    )
    .await;
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
