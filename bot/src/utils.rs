use database::{Family, Pokemon};
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
        Err(_) => String::from("NA"),
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(string) => string,
                Err(_) => String::from("NA")
            }
        },
    }
}

fn get_poke_from_list(pokemons: &Vec<Pokemon>, dex: i32) -> &Pokemon {
    pokemons
        .iter()
        .find(
            |x| x.dex == dex
        )
        .unwrap()
}

fn block_str(block_start: &i32, last: &i32) -> String {
    if last != block_start {
        return format!("{}-{}, ", block_start, last);
    }

    format!("{}, ", last)
}

fn pokemon_vec_to_string(vector: Vec<&i32>) -> String {
    let mut output = String::new();
    let mut block_start = vector[0];
    let mut last = vector[0];

    for dex in vector {
        // Non-contiguous number, add previous block to string
        if *dex > last + 1 {
            output.push_str(&block_str(block_start, last));
            block_start = dex;
        }

        last = dex;
    }

    // Add last block if not there yet
    let last_block = &block_str(block_start, last);
    if !output.ends_with(last_block) {
        output.push_str(last_block);
    }

    // Remove last comma and space
    output.trim_end_matches(", ").to_string()
}

pub fn generate_already_maxed_string(pokemons: Vec<Pokemon>, families: Vec<Family>) -> String {
    let mut filtered: Vec<&i32> = families
                                    .iter()
                                    .filter( // Get only the families with any maxed mon, ie families where any `level40` counter is > 0
                                        |family| family.pokemons.iter().any(
                                            |dex| get_poke_from_list(&pokemons, *dex).level40 > 0
                                        )
                                    )
                                    .flat_map( // Flatten the `pokemons` Vec on each family into a single Vec
                                        |family| family.pokemons.iter()
                                    )
                                    .collect(); // Convert Iterator into Vec
    filtered.sort();

    let mut string = pokemon_vec_to_string(filtered);
    string.push_str(" & !n40 & shiny & lucky");

    string
}

pub fn generate_non_maxed_string(pokemons: Vec<Pokemon>, families: Vec<Family>) -> String {
    let mut filtered: Vec<&i32> = families
                                    .iter()
                                    .filter( // Get only the families with no maxed mons, ie families where all `level40` counters are 0
                                        |family| family.pokemons.iter().all(
                                            |dex| get_poke_from_list(&pokemons, *dex).level40 == 0
                                        )
                                    )
                                    .flat_map( // Flatten the `pokemons` Vec on each family into a single Vec
                                        |family| family.pokemons.iter()
                                    )
                                    .collect(); // Convert Iterator into Vec
    filtered.sort();

    pokemon_vec_to_string(filtered)
}
