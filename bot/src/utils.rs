use teloxide::prelude::*;

pub fn check_permission(msg: Message) -> Option<bool> {
    let allowed = std::env::var("USERNAME").ok()?;
    let username = msg.from()?.username.as_deref()?;
    Some(allowed == username)
}

pub fn get_commit_hash() -> String {
    let result = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output();

    match result {
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(string) => string,
                Err(_) => String::from("NA")
            }
        },
        Err(_) => String::from("NA")
    }
}
// get_container_date