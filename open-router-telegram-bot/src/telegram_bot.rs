use crate::commands;
use crate::error;
use crate::messages;
use commands::Command;
use commands::CommandTrait;
use error::Error;
use std::collections::VecDeque;

pub struct TgBot {
    pub http_client: reqwest::Client,
    pub token: String,
    pub model: String,
}

impl TgBot {
    const POLLING_INTERVAL: u64 = 5000;

    pub fn new(token: &str) -> Self {
        TgBot {
            token: token.to_string(),
            http_client: reqwest::Client::new(),
            model: String::from("openai/gpt-4o"),
        }
    }

    pub async fn run(&mut self, mut id_list: VecDeque<i64>) -> Result<(), reqwest::Error> {
        loop {
            let length = id_list.len();
            tracing::debug!(?length, "id list length");
            if id_list.len() > 100 {
                tracing::debug!(?id_list, "popping list");
                id_list.pop_front();
            } else {
                match self.get_updates().await {
                    Ok(response) => {
                        for update in response.result {
                            if id_list.contains(&update.update_id) {
                                continue;
                            } else {
                                id_list.push_back(update.update_id);
                                match self.handle_update(&update).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        tracing::error!(?e, "Failed to handle update");
                                    }
                                }
                            }
                        }
                    }
                    Err(_e) => continue,
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(Self::POLLING_INTERVAL));
        }
    }

    pub fn change_model(&mut self, model: &String) {
        match model.to_lowercase().as_str() {
            "unslopnemo" => self.model = String::from("thedrummer/unslopnemo-12b"),
            "gemini" => self.model = String::from("google/gemini-2.0-flash-001"),
            "deepseek" => self.model = String::from("deepseek/deepseek-r1-distill-llama-8b"),
            "claude" => self.model = String::from("anthropic/claude-3.5-sonnet"),
            "llama" => self.model = String::from("sao10k/13.1-70b-hanami-x1"),
            "open-ai" => self.model = String::from("openai/gpt-4o"),
            "weaver" => self.model = String::from("mancer/weaver"),
            _ => (),
        }
    }

    pub async fn get_updates(&self) -> Result<messages::telegram::Response, Error> {
        tracing::debug!("getting updates");
        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        Ok(self.http_client.get(&url).send().await?.json().await?)
    }

    pub async fn handle_update(
        &mut self,
        update: &messages::telegram::Update,
    ) -> Result<(), Error> {
        match update.message.text {
            None => Ok(()),
            _ => {
                let text = update.message.text.as_ref().expect("must be text");
                tracing::debug!(?text, "handling update: ");
                Command::try_from(text.as_ref())
                    .expect("unknown command")
                    .execute(self, update.message.chat.get_id())
                    .await
            }
        }
    }
}
