use crate::utils;
use teloxide::utils::markdown;

pub async fn level40_internal(name: impl Into<String>, amount: impl Into<i32>) -> String {
    match database::update_level40(name, amount).await {
        Err(_) => String::from("❌ — There was an error updating the counter"),
        Ok(pokemon) => {
                format!(
                    "✅ — _level40_ counter for *{}* is now *_{}_*",
                    markdown::escape(&pokemon.name),
                    markdown::escape(&pokemon.level40.to_string())
                )
        },
    }
}

pub async fn tradeable_internal(name: impl Into<String>, amount: impl Into<i32>) -> String {
    match database::update_tradeable(name, amount).await {
        Err(_) => String::from("❌ — There was an error updating the counter"),
        Ok(pokemon) => {
                format!(
                    "✅ — _tradeable_ counter for *{}* is now *_{}_*",
                    markdown::escape(&pokemon.name),
                    markdown::escape(&pokemon.tradeable.to_string())
                )
        },
    }
}

pub async fn maxed_internal() -> String {
    let pokemons = match database::get_pokemons().await {
        Err(_) => return String::from("❌ — There was an error reading pokemons data"),
        Ok(pokemons) => pokemons,
    };

    let families = match database::get_families().await {
        Err(_) => return String::from("❌ — There was an error reading families data"),
        Ok(families) => families,
    };

    format!("`{} & !n40 & shiny & lucky`", markdown::escape(&utils::generate_already_maxed_string(pokemons, families)))
}

pub async fn non_maxed_internal() -> String {
    let pokemons = match database::get_pokemons().await {
        Err(_) => return String::from("❌ — There was an error reading pokemons data"),
        Ok(pokemons) => pokemons,
    };

    let families = match database::get_families().await {
        Err(_) => return String::from("❌ — There was an error reading families data"),
        Ok(families) => families,
    };

    format!("`{}`", markdown::escape(&utils::generate_non_maxed_string(pokemons, families)))
}