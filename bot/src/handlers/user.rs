use crate::{commands::UserCommand, utils};
use teloxide::{
    adaptors::DefaultParseMode, prelude::Requester, requests::ResponseResult, types::Message, Bot,
};

pub async fn handle(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    cmd: UserCommand,
) -> ResponseResult<()> {
    let text = match cmd {
        UserCommand::Help => utils::help_message(),
        UserCommand::Version => {
            format!("🤖 I was built with commit: _{}_", utils::get_commit_hash())
        }
        UserCommand::AlreadyMaxed => super::internal::user::generate_search_string(true).await,
        UserCommand::NonMaxed => super::internal::user::generate_search_string(false).await,

        _ => "Unimplemented command".to_string(),
    };

    let _ = bot.send_message(msg.chat.id, text).await;

    Ok(())
}
