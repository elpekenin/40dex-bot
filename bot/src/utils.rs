use teloxide::{prelude::*, utils::markdown};

pub fn check_permission(msg: &Message) -> Option<bool> {
    let allowed = std::env::var("USERNAME").ok()?;
    let username = msg.from()?.username.as_deref()?;
    Some(allowed == username)
}

pub fn format_error(message: impl Into<String>, error: &impl ToString) -> String {
    let message = message.into();

    format!(
        "❌ — {}\n\n`{}`",
        message,
        markdown::escape(&error.to_string())
    )
}
