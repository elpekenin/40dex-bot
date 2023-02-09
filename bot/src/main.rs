mod commands;
mod handlers;
mod utils;

use commands::*;
use dotenvy::dotenv;
use teloxide::{prelude::*, types::ParseMode};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    dotenv().ok();

    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

    let commands = utils::command_list();

    let _ = bot.set_my_commands(commands).await;

    AdminCommand::repl(bot.clone(), handlers::admin::handle).await;
    // UserCommand::repl(bot.clone(), user_command_handler).await;
}
