use crate::{commands::AdminCommand, utils};
use teloxide::{
    adaptors::DefaultParseMode, prelude::Requester, requests::ResponseResult, types::Message, Bot,
};

pub async fn handle(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    cmd: AdminCommand,
) -> ResponseResult<()> {
    // Permission guard clause
    match utils::check_permission(msg.clone()) {
        Some(x) => {
            if !x {
                let _ = bot
                    .send_message(msg.chat.id, "You are not allowed to do that")
                    .await;

                log::warn!(
                    "{} tried using an admin command",
                    msg.from().unwrap().username.as_deref().unwrap()
                );

                return Ok(());
            }
        }
        None => {
            log::error!("Could't check permission...");
            return Ok(());
        }
    }

    let text = match cmd {
        AdminCommand::Add(name) => super::internal::admin::update_table(name, 1, "level40").await,
        AdminCommand::Dec(name) => super::internal::admin::update_table(name, -1, "level40").await,

        AdminCommand::Catch(name) => {
            super::internal::admin::update_table(name, 1, "tradeable").await
        }
        AdminCommand::Trade(name) => {
            super::internal::admin::update_table(name, -1, "tradeable").await
        }

        _ => "Unimplemented command".to_string(),
    };

    bot.send_message(msg.chat.id, text).await;

    Ok(())
}
