use database;
use teloxide::prelude::*;


pub fn check_permission(msg: Message) -> Option<bool> {
    let allowed = std::env::var("USERNAME").ok()?;

    let username = msg.from()?.username.as_deref()?;

    Some(allowed == username)
}

// get_commit
// get_container_date