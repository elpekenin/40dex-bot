use crate::utils;
use teloxide::utils::markdown;

pub async fn update_pokemon(
    name: impl Into<String>,
    amount: impl Into<i32>,
    table: impl Into<&str>,
) -> String {
    let table = table.into();

    let pokemon = match table {
        "level40" => database::update_level40(name, amount).await,
        "tradeable" => database::update_level40(name, amount).await,

        x => return format!("How did {x} end up as an input for `update_pokemon`?"),
    };

    match pokemon {
        Err(e) => utils::format_error("There was an error updating the counter", e),
        Ok(pokemon) => {
            let counter = match table {
                "level40" => pokemon.level40,
                "tradeable" => pokemon.tradeable,
                _ => unreachable!(),
            };

            format!(
                "✅ — _{}_ counter for *{}* is now *_{}_*",
                table,
                markdown::escape(&pokemon.name),
                markdown::escape(&counter.to_string())
            )
        }
    }
}
