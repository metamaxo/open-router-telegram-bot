use crate::commands;
use crate::error;
use crate::messages;
use commands::Command;
use commands::CommandTrait;
use error::Error;

#[derive(Default, Clone, Copy)]
pub enum Model {
    Weaver,
    Unslopnemo,
    Gemini,
    Deepseek,
    Claude,
    Llama,
    #[default]
    OpenAi,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl From<Model> for String {
    fn from(model: Model) -> Self {
        match model {
            Model::Unslopnemo => "thedrummer/unslopnemo-12b".to_string(),
            Model::Gemini => "google/gemini-2.0-flash-001".to_string(),
            Model::Deepseek => "deepseek/deepseek-r1-distill-llama-8b".to_string(),
            Model::Claude => "anthropic/claude-3.5-sonnet".to_string(),
            Model::Llama => "sao10k/13.1-70b-hanami-x1".to_string(),
            Model::OpenAi => "openai/gpt-4o".to_string(),
            Model::Weaver => "mancer/weaver".to_string(),
        }
    }
}

impl TryFrom<&str> for Model {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            _ if value.contains("weaver") => Self::Weaver,
            _ if value.contains("unslopnemo") => Self::Unslopnemo,
            _ if value.contains("gemini") => Self::Gemini,
            _ if value.contains("deepseek") => Self::Deepseek,
            _ if value.contains("claude") => Self::Claude,
            _ if value.contains("llama") => Self::Llama,
            _ if value.contains("openai") => Self::OpenAi,
            _ if value.contains("gpt") => Self::OpenAi,
            _ => return Err(()),
        })
    }
}

pub struct TgBot {
    pub http_client: reqwest::Client,
    pub token: String,
    pub model: Model,
    pub offset: i64,
}

impl TgBot {
    const POLLING_INTERVAL: u64 = 5000;

    pub fn new(token: &str) -> Self {
        TgBot {
            token: token.to_string(),
            http_client: reqwest::Client::new(),
            model: Model::default(),
            offset: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), reqwest::Error> {
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
            std::thread::sleep(std::time::Duration::from_millis(Self::POLLING_INTERVAL));
        }
    }

    pub fn change_model(&mut self, model: &str) {
        if let Ok(model) = Model::try_from(model) {
            self.model = model;
        }
    }

    pub async fn get_updates(&self) -> Result<messages::telegram::Response, Error> {
        tracing::debug!("getting updates");
        let url = format!(
            "https://api.telegram.org/bot{}/getUpdates?offset={}",
            self.token,
            self.offset + 1
        );
        Ok(self.http_client.get(&url).send().await?.json().await?)
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
