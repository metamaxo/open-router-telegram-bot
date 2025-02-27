use crate::{error::Error, TgBot};

pub trait CommandTrait: for<'a> TryFrom<&'a str> {
    async fn execute(&self, bot: &mut TgBot, chat_id: i64, text: &str) -> Result<(), Error>;
}

pub enum Command {
    ListModels,
    Model,
    ChangeModel(String),
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "/list_models" => Ok(Self::ListModels),
            "/model" => Ok(Self::Model),
            _ if value.starts_with("/change_model ") => {
                Ok(Self::ChangeModel(value.replace("/change_model ", "")))
            }
            _ => Err(Error::InvalidCommand(value.to_string())),
        }
    }
}

impl CommandTrait for Command {
    async fn execute(&self, bot: &mut TgBot, chat_id: i64, text: &str) -> Result<(), Error> {
        match self {
            Self::ListModels => {
                let url = format!(
                    "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                    bot.token,
                    chat_id,
                    TgBot::MODEL_LIST,
                );
                tracing::debug!("sending initial message");
                bot.http_client.post(&url).send().await?;
                Ok(())
            }
            Self::Model => {
                let response = format!("i'm currently using: {} ", bot.model.as_str());
                let url = format!(
                    "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                    bot.token, chat_id, response,
                );
                tracing::debug!("sending initial message");
                bot.http_client.post(&url).send().await?;
                Ok(())
            }
            Self::ChangeModel(new_model) => {
                bot.change_model(new_model);
                let response = format!("changed model to: {} ", bot.model.as_str());
                let url = format!(
                    "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
                    bot.token, chat_id, response,
                );
                tracing::debug!("sending initial message");
                bot.http_client.post(&url).send().await?;
                Ok(())
            }
        }
    }
}
