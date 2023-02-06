use crate::utils;
use teloxide::utils::markdown;

/// Public interface
///
/// These are the functions being called when receiving a command to generate the String to be sent as answer
pub async fn update_level40(name: impl Into<String>, amount: impl Into<i32>) -> String {
    update_table(name, amount, "level40").await
}

pub async fn update_tradeable(name: impl Into<String>, amount: impl Into<i32>) -> String {
    update_table(name, amount, "tradeable").await
}

pub async fn already_maxed_string() -> String {
    generate_search_string(true).await
}

pub async fn non_maxed_string() -> String {
    generate_search_string(false).await
}


/// Helper functions
///
/// These are not part of the public API, mostly exist to reduce code duplication
fn format_error(message: impl Into<String>, error: impl ToString) -> String {
    let message = message.into();

    format!("❌ — {}\n\n`{}`", message, markdown::escape(&error.to_string()))
}

async fn update_table(name: impl Into<String>, amount: impl Into<i32>, table: impl Into<&str>) -> String {
    let table = table.into();

    let pokemon = match table {
        "level40" => database::update_level40(name, amount).await,
        "tradeable" => database::update_tradeable(name, amount).await,

        x => return format!("How did {x} end up as an input for `update_table`?")
    };

    match pokemon {
        Err(e) => format_error("There was an error updating the counter", e),
        Ok(pokemon) => {
            let counter = match table {
                "level40" => pokemon.level40,
                "tradeable" => pokemon.tradeable,
                _ => unreachable!()
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

async fn generate_search_string(maxed: bool) -> String {
    let pokemons = match database::get_pokemons().await {
        Err(e) => return format_error("There was an error reading pokemons data", e),
        Ok(pokemons) => pokemons,
    };

    let families = match database::get_families().await {
        Err(e) => return format_error("There was an error reading families data`", e),
        Ok(families) => families,
    };

    let string = match maxed {
        true => utils::generate_already_maxed_string(pokemons, families),
        false => utils::generate_non_maxed_string(pokemons, families)
    };

    format!("`{}`", markdown::escape(&string))
}