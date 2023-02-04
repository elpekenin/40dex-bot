use dotenvy::dotenv;
use teloxide::{
    adaptors::DefaultParseMode,
    prelude::*,
    types::ParseMode,
    utils::command::BotCommands,
};

mod utils;
mod handlers;

#[derive(BotCommands, Clone, Debug)]
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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    dotenv().ok();

    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

    Command::repl(bot, answer).await;
}

async fn answer(bot: DefaultParseMode<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
    // Commands that can be used by anyone
    match cmd {
        Command::Help => {
            let _ = bot.send_message(msg.chat.id, Command::descriptions().to_string()).await;
            return Ok(());
        },
        Command::Version => { return unimplemented!() },
        _  => { }
    }

    // Guard clause
    match utils::check_permission(msg.clone()) {
        Some(x) => {
            if !x {
                // Unauthorized user - Quit
                return Ok(());
            }
        },
        None => {
            log::error!("Could't check permission...");
            return Ok(());
        }
    }

    // Commands that require permission
    match cmd {
        Command::Add(name) => {
            bot.send_message(
                msg.chat.id,
                handlers::level40_internal(name, 1).await
            ).await?;
        },

        Command::Dec(name) => {
            bot.send_message(
                msg.chat.id,
                handlers::level40_internal(name, -1).await
            ).await?;
        },

        // Fallback for un-implemented commands
        x => {
            log::warn!("Un-handled command: {:?}", x);
        }
    }

    Ok(())
}
