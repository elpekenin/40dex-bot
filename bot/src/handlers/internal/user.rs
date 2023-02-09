use crate::utils;
use teloxide::utils::markdown;

pub async fn generate_search_string(maxed: bool) -> String {
    let pokemons = match database::get_pokemons().await {
        Err(e) => return utils::format_error("There was an error reading pokemons data", e),
        Ok(pokemons) => pokemons,
    };

    let families = match database::get_families().await {
        Err(e) => return utils::format_error("There was an error reading families data`", e),
        Ok(families) => families,
    };

    let string = match maxed {
        true => utils::already_maxed_string(pokemons, families),
        false => utils::non_maxed_string(pokemons, families),
    };

    format!("`{}`", markdown::escape(&string))
}
