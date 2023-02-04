use teloxide::utils::markdown;

pub async fn level40_internal(name: impl Into<String>, amount: impl Into<i32>) -> String {
    match database::update_level40(name, amount).await {
        Ok(pokemon) => {
                format!(
                    "level40 counter for `{}` is now **{}**",
                    markdown::escape(&pokemon.name),
                    markdown::escape(&pokemon.level40.to_string())
                )
        },
        Err(_) => String::from("There was an error updating the counter")
    }
}