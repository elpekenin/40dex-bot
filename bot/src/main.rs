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
#[command(rename_rule = "snake_case", description = "These commands are supported")]
enum Command {
    #[command(description = "add 1 to a pokemon level40 counter")]
    Add(String),

    #[command(description = "search string to cleanup already 40d pokemon species")]
    AlreadyMaxed,

    #[command(description = "add 1 to a pokemon tradeable counter")]
    Catch(String),

    #[command(description = "substract 1 from a pokemon level40 counter")]
    Dec(String),

    #[command(description = "display this help message")]
    Help,

    #[command(description = "search string for non 40d pokemon species")]
    NonMaxed,

    #[command(description = "substract 1 from a pokemon tradeable counter")]
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
    let text = match cmd {
        Command::Help => Some(markdown::escape(&Command::descriptions().to_string())),
        Command::Version => Some(format!("🤖 I was built with commit: _{}_", utils::get_commit_hash())),
        _  => None
    };

    match text {
        Some(text) => { 
            bot.send_message(msg.chat.id, text).await?;
            return Ok(());
        },
        None => { }
    }

    // Permission guard clause
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

    // Restricted commands
    let text = match cmd {
        Command::Add(name) => handlers::update_level40(name, 1).await,
        Command::AlreadyMaxed => handlers::already_maxed_string().await,
        Command::Catch(name) => handlers::update_tradeable(name, 1).await,
        Command::Dec(name) => handlers::update_level40(name, -1).await,
        Command::NonMaxed => handlers::non_maxed_string().await,
        Command::Trade(name) => handlers::update_tradeable(name, -1).await,

        _ => "Unimplemented command".to_string()
    };

    bot.send_message(
        msg.chat.id,
        text,
    ).await?;

    Ok(())
}
