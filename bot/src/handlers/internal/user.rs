use crate::utils;
use database::{Family, Pokemon};
use teloxide::utils::markdown;

pub fn get_commit_hash() -> String {
    let result = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output();

    match result {
        Err(_) => String::from("NA"),
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_) => String::from("NA"),
        },
    }
}

fn get_poke_from_list(pokemons: &Vec<Pokemon>, dex: i32) -> &Pokemon {
    pokemons.iter().find(|x| x.dex == dex).unwrap()
}

fn block_str(first: &i32, last: &i32) -> String {
    if first != last {
        return format!("{}-{}, ", first, last);
    }

    format!("{}, ", last)
}

fn pokemon_vec_to_string(vector: Vec<&i32>) -> String {
    let mut output = String::new();
    let mut first = vector[0];
    let mut last = vector[0];

    for dex in vector {
        // Non-contiguous number, add previous block to string
        if *dex > last + 1 {
            output.push_str(&block_str(first, last));
            first = dex;
        }

        last = dex;
    }

    // Add last block if not there yet
    let last_block = &block_str(first, last);
    if !output.ends_with(last_block) {
        output.push_str(last_block);
    }

    // Remove last comma and space
    output.trim_end_matches(", ").to_string()
}

pub fn already_maxed_string(pokemons: Vec<Pokemon>, families: Vec<Family>) -> String {
    let mut filtered: Vec<&i32> = families
        .iter()
        .filter(
            // Get only the families with any maxed mon, ie families where any `level40` counter is > 0
            |family| {
                family
                    .pokemons
                    .iter()
                    .any(|dex| get_poke_from_list(&pokemons, *dex).level40 > 0)
            },
        )
        .flat_map(
            // Flatten the `pokemons` Vec on each family into a single Vec
            |family| family.pokemons.iter(),
        )
        .collect(); // Convert Iterator into Vec
    filtered.sort();

    let mut string = pokemon_vec_to_string(filtered);
    string.push_str(" & !n40 & shiny & lucky");

    string
}

pub fn non_maxed_string(pokemons: Vec<Pokemon>, families: Vec<Family>) -> String {
    let mut filtered: Vec<&i32> = families
        .iter()
        .filter(
            // Get only the families with no maxed mons, ie families where all `level40` counters are 0
            |family| {
                family
                    .pokemons
                    .iter()
                    .all(|dex| get_poke_from_list(&pokemons, *dex).level40 == 0)
            },
        )
        .flat_map(
            // Flatten the `pokemons` Vec on each family into a single Vec
            |family| family.pokemons.iter(),
        )
        .collect(); // Convert Iterator into Vec
    filtered.sort();

    pokemon_vec_to_string(filtered)
}

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
        true => already_maxed_string(pokemons, families),
        false => non_maxed_string(pokemons, families),
    };

    format!("`{}`", markdown::escape(&string))
}
