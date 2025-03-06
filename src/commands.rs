use crate::error::Error;
use crate::messages::bot_messages;
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
        let message = match self {
            Self::Start => bot_messages::INITIAL_MESSAGE.to_string(),
            Self::ListModels => bot_messages::MODEL_LIST.to_string(),
            Self::Model => format!("i'm currently using: {}", bot.model()),
            Self::Frog(query) => {
                tracing::debug!("answering query");
                bot.call_open_router(query).await?.join("\n")
            }
            Self::ChangeModel(new_model) => {
                bot.change_model(new_model);
                format!("changed model to: {}", bot.model())
            }
            Self::Unknown => {
                tracing::debug!("unknown command");
                return Ok(());
            }
        };

        bot.send_message(chat_id, &message).await
    }
}
