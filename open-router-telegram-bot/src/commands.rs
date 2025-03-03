use crate::error::Error;
use crate::messages::bot_messages;
use crate::open_router::open_router;
use crate::telegram_bot::TgBot;

pub trait CommandTrait: for<'a> TryFrom<&'a str> {
    async fn execute(&self, bot: &mut TgBot, chat_id: i64) -> Result<(), Error>;
}

pub enum Command {
    Start,
    ListModels,
    Model,
    Frog(String),
    ChangeModel(String),
    Unknown,
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        tracing::debug!(?value, "value: ");
        match value {
            _ if value.starts_with("/startfrog") => Ok(Self::Start),
            _ if value.starts_with("/list_models") => Ok(Self::ListModels),
            _ if value.starts_with("/model") => Ok(Self::Model),
            _ if value.starts_with("/frog") => Ok(Self::Frog(value.replace("/frog", ""))),
            _ if value.starts_with("/change_model ") => {
                Ok(Self::ChangeModel(value.replace("/change_model ", "")))
            }
            _ => Ok(Self::Unknown),
        }
    }
}

impl CommandTrait for Command {
    async fn execute(&self, bot: &mut TgBot, chat_id: i64) -> Result<(), Error> {
        match self {
            Self::Start => {
                bot.http_client
                    .post(format!(
                        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                        bot.token,
                        chat_id,
                        bot_messages::INITIAL_MESSAGE
                    ))
                    .send()
                    .await?;
                Ok(())
            }
            Self::ListModels => {
                bot.http_client
                    .post(format!(
                        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                        bot.token,
                        chat_id,
                        bot_messages::MODEL_LIST
                    ))
                    .send()
                    .await?;
                Ok(())
            }
            Self::Model => {
                bot.http_client
                    .post(format!(
                        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                        bot.token,
                        chat_id,
                        format!("i'm currently using: {} ", bot.model.as_str())
                    ))
                    .send()
                    .await?;
                Ok(())
            }
            Self::Frog(query) => {
                tracing::debug!("answering query");
                let response = open_router(query, bot.model.as_str()).await;
                bot.http_client
                    .post(format!(
                        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                        bot.token,
                        chat_id,
                        response?.join("\n"),
                    ))
                    .send()
                    .await?;
                Ok(())
            }
            Self::ChangeModel(new_model) => {
                bot.change_model(new_model);
                bot.http_client
                    .post(format!(
                        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                        bot.token,
                        chat_id,
                        format!("changed model to: {} ", bot.model.as_str())
                    ))
                    .send()
                    .await?;
                Ok(())
            }
            Self::Unknown => {
                tracing::debug!("unknown command");
                Ok(())
            }
        }
    }
}
