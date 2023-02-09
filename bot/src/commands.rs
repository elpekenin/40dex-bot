use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "snake_case",
    description = "Commands available for everyone"
)]
pub enum UserCommand {
    #[command(description = "search string to cleanup already 40d pokemon species")]
    AlreadyMaxed,

    #[command(description = "display this help message")]
    Help,

    #[command(description = "search string for non 40d pokemon species")]
    NonMaxed,

    #[command(description = "display information about current version")]
    Version,
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "snake_case",
    description = "Commands that can only be used by admins"
)]
pub enum AdminCommand {
    #[command(description = "add 1 to a pokemon level40 counter")]
    Add(String),

    #[command(description = "add 1 to a pokemon tradeable counter")]
    Catch(String),

    #[command(description = "substract 1 from a pokemon level40 counter")]
    Dec(String),

    #[command(description = "substract 1 from a pokemon tradeable counter")]
    Trade(String),
}
