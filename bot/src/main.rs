mod commands;
mod handlers;
mod utils;

use dotenvy::dotenv;
use teloxide::{
    prelude::*,
    types::{BotCommandScope, ParseMode},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

    let _ = bot
        .set_my_commands(commands::descriptions())
        .scope(BotCommandScope::AllPrivateChats)
        .await;

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<commands::AdminCommand>()
                .endpoint(handlers::admin::handle),
        )
        .branch(
            dptree::entry()
                .filter_command::<commands::UserCommand>()
                .endpoint(handlers::user::handle),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error ocurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
