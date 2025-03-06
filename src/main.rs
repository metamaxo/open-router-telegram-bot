use crate::telegram_bot::TgBot;
use clap::Parser;

mod commands;
mod constants;
mod error;
mod messages;
mod model;
mod open_router;
mod telegram_bot;
mod utils;

use dotenvy::dotenv;
use error::Error;
use std::env;

#[derive(clap::Parser)]
#[command(
    name = "OpenRouter Telegram Bot",
    author = "Metamaxo",
    version = "1.0.0",
    about = "A Telegram bot that uses OpenRouter's API to interact with various AI models",
    long_about = "This bot allows users to interact with various AI models through Telegram, \
                  powered by OpenRouter's API. It supports multiple AI models and provides \
                  a simple interface for chatting with AI directly in Telegram."
)]
struct Args {
    /// Polling interval in milliseconds
    /// NOTE: This was hardcoded before, now it's configurable but the default value is 5000
    /// which was the previously hardcoded value.
    #[clap(
        long,
        default_value = "5000",
        help = "Set the bot's polling interval in milliseconds"
    )]
    polling_interval: u64,
}

/// One single implementation to create a telegram_bot::Config from the command line arguments
/// This is to make sure that we only have to change the implementation in one place if the
/// telegram_bot::Config struct changes or if we want to add more command line arguments
impl From<&Args> for telegram_bot::Config {
    fn from(args: &Args) -> Self {
        telegram_bot::Config {
            polling_interval: args.polling_interval,
            tg_bot_key: env::var("TG_BOT_KEY").expect("bot key must be set!"),
            open_router_key: env::var("OPEN_ROUTER_KEY").expect("open-router key must be set!"),
        }
    }
}

impl Args {
    /// Core function to run the program, based on the provided command line arguments
    pub async fn run(&self) -> Result<(), Error> {
        // Create a config object from the command line arguments
        let cfg: telegram_bot::Config = self.into();

        // Create a new Telegram bot instance with the config
        let mut bot = TgBot::new(cfg);

        // Run the bot
        bot.run().await
    }
}

///bot starts here
///make sure bot key and open-router key are set in .env
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize logger
    let _guard = utils::init_logger();

    // Load environment variables from .env file
    dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();

    // Run the program
    args.run().await
}
