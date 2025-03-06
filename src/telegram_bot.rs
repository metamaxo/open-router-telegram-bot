use crate::commands;
use crate::error;
use crate::messages;
use crate::model::Model;
use commands::Command;
use commands::CommandTrait;
use error::Error;

#[derive(Default)]
pub struct Config {
    pub tg_bot_key: String,
    pub open_router_key: String,
    pub polling_interval: u64,
}

pub struct TgBot {
    pub http_client: reqwest::Client,
    model: Model,
    offset: i64,
    cfg: Config,
}

impl Default for TgBot {
    fn default() -> Self {
        TgBot {
            http_client: reqwest::Client::new(),
            model: Model::default(),
            cfg: Config::default(),
            offset: 0,
        }
    }
}

impl TgBot {
    /// NOTE: added `Default` implementation to `Config` struct so we can use `..Default::default()`
    /// notation to fill in the missing fields in a predictable way.
    /// This is nice because now if we ever change the `Config` struct, we don't have to change the
    /// `new` function, we can just change the `Default` implementation.
    pub fn new(cfg: Config) -> Self {
        TgBot {
            cfg,
            ..Default::default()
        }
    }

    /// Explicit **read only** methods for these fields
    pub fn model(&self) -> Model {
        self.model
    }
    pub fn open_router_key(&self) -> &str {
        &self.cfg.open_router_key
    }
    pub fn tg_bot_key(&self) -> &str {
        &self.cfg.tg_bot_key
    }

    /// Send a message to a chat
    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<(), Error> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.tg_bot_key(),
        );
        let body = serde_json::json!({
            "chat_id": chat_id,
            "text": text,
        });
        self.http_client.post(&url).json(&body).send().await?;
        Ok(())
    }

    /// Get all updates from the telegram bot
    pub async fn get_updates(&self) -> Result<messages::telegram::Response, Error> {
        tracing::debug!("getting updates");
        let url = format!(
            "https://api.telegram.org/bot{}/getUpdates?offset={}",
            self.tg_bot_key(),
            self.offset + 1
        );
        Ok(self.http_client.get(&url).send().await?.json().await?)
    }

    /// Change model
    pub fn change_model(&mut self, model: &str) {
        if let Ok(model) = Model::try_from(model) {
            self.model = model;
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.get_updates().await {
                Ok(response) => {
                    for update in response.result {
                        self.offset = update.update_id;
                        match update.message {
                            None => {}
                            _ => match self.handle_update(&update.message.unwrap()).await {
                                Ok(_) => {}
                                Err(e) => {
                                    tracing::error!(?e, "Failed to handle update");
                                }
                            },
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(?e, "Failed to get updates");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(self.cfg.polling_interval));
        }
    }

    pub async fn handle_update(
        &mut self,
        update: &messages::telegram::Message,
    ) -> Result<(), Error> {
        tracing::debug!(?update, "handling update");
        match update.text {
            None => {
                tracing::debug!("no text in update.message");
                Ok(())
            }
            _ => {
                let text = update.text.as_ref().expect("must be text");
                tracing::debug!(?text, "handling update: ");
                Command::try_from(text.as_ref())
                    .expect("unknown command")
                    .execute(self, update.chat.get_id())
                    .await
            }
        }
    }
}
