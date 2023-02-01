use database;
use dotenvy::dotenv;
use teloxide::{
    adaptors::DefaultParseMode,
    prelude::*,
    types::ParseMode,
    utils::command::BotCommands,
};

mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot..");

    dotenv().ok();

    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

    Command::repl(bot, answer).await;
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "These commands are supported:")]
enum Command {
    #[command(description = "add 1 to a pokemon's `level40` counter")]
    Add(String),

    #[command(description = "add 1 to a pokemon's `tradeable` counter")]
    Catch(String),

    #[command(description = "substract 1 from a pokemon's `level40` counter")]
    Dec(String),

    #[command(description = "display this help message")]
    Help,

    #[command(description = "substract 1 from a pokemon's `tradeable` counter")]
    Trade(String),

    #[command(description = "display information about container and commit")]
    Version,
}


async fn answer(bot: DefaultParseMode<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        _ => {
             bot.send_message(msg.chat.id, format!("Fallback text")).await?
        }
    };

    Ok(())
}
