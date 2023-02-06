use dotenvy::dotenv;
use teloxide::{
    adaptors::DefaultParseMode,
    prelude::*,
    types::ParseMode,
    utils::{
        markdown,
        command::BotCommands
    },
};

mod handlers;
mod utils;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "snake_case", description = "These commands are supported:")]
enum Command {
    #[command(description = "add 1 to a pokemon's level40 counter")]
    Add(String),

    #[command(description = "search string to cleanup already 40'd pokemon species")]
    AlreadyMaxed,

    #[command(description = "add 1 to a pokemon's tradeable counter")]
    Catch(String),

    #[command(description = "substract 1 from a pokemon's level40 counter")]
    Dec(String),

    #[command(description = "display this help message")]
    Help,

    #[command(description = "search string for non 40'd pokemon species")]
    NonMaxed,

    #[command(description = "substract 1 from a pokemon's `tradeable` counter")]
    Trade(String),

    #[command(description = "display information about current version")]
    Version,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    dotenv().ok();

    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);
    let _ = bot.set_my_commands(Command::bot_commands()).await;

    Command::repl(bot, answer).await;
}

async fn answer(bot: DefaultParseMode<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
    // Commands that can be used by anyone
    match cmd {
        Command::Help => {
            let _ = bot.send_message(msg.chat.id, markdown::escape(&Command::descriptions().to_string())).await;
            return Ok(());
        },
        Command::Version => {
            let _ = bot.send_message(
                msg.chat.id,
                format!(
                    "🤖 I was built with commit: _{}_",
                    utils::get_commit_hash()
                )
            ).await;
            return Ok(());
        },
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
            let _ = bot.send_message(
                msg.chat.id,
                handlers::level40_internal(name, 1).await
            ).await?;
        },

        Command::Dec(name) => {
            let _ = bot.send_message(
                msg.chat.id,
                handlers::level40_internal(name, -1).await
            ).await?;
        },

        Command::Catch(name) => {
            let _ = bot.send_message(
                msg.chat.id,
                handlers::tradeable_internal(name, 1).await
            ).await?;
        },

        Command::Trade(name) => {
            let _ = bot.send_message(
                msg.chat.id,
                handlers::tradeable_internal(name, -1).await
            ).await?;
        },

        Command::AlreadyMaxed => {
            let _ = bot.send_message(
                msg.chat.id,
                handlers::maxed_internal().await
            ).await?;
        },

        Command::NonMaxed => {
            let _ = bot.send_message(
                msg.chat.id,
                handlers::non_maxed_internal().await
            ).await?;
        }

        // Fallback for un-implemented commands
        x => {
            log::warn!("Un-handled command: {:?}", x);
            let _ = bot.send_message(
                msg.chat.id,
                "Unimplemented"
            ).await?;
        }
    }

    Ok(())
}
