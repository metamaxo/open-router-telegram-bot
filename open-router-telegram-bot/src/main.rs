use crate::telegram_bot::TgBot;
mod commands;
mod error;
mod messages;
mod open_router;
mod telegram_bot;
mod utils;
use dotenvy::dotenv;
use error::Error;
use std::collections::VecDeque;
use std::env;

///bot starts here
///make sure bot key and open-router key are set in .env
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let id_list = VecDeque::new();
    let _guard = utils::init_logger();
    let mut bot = TgBot::new(
        env::var("TG_BOT_KEY")
            .expect("bot key must be set!")
            .as_str(),
    );
    bot.run(id_list).await
}
