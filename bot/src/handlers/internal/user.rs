use crate::utils;
use database::MergedFamily;
use teloxide::utils::markdown;

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

pub fn already_maxed_string(families: Vec<MergedFamily>) -> String {
    let mut filtered: Vec<&i32> = families
        .iter()
        .filter(|f| f.pokemons.iter().any(|p| p.level40 > 0))
        .flat_map(|f| f.pokemons.iter().map(|p| &p.dex))
        .collect(); // Convert Iterator into Vec
    filtered.sort();

    let mut string = pokemon_vec_to_string(filtered);
    string.push_str(" & !n40 & shiny & lucky");

    string
}

pub fn non_maxed_string(families: Vec<MergedFamily>) -> String {
    let mut filtered: Vec<&i32> = families
        .iter()
        .filter(|f| f.pokemons.iter().all(|p| p.level40 == 0))
        .flat_map(|f| f.pokemons.iter().map(|p| &p.dex))
        .collect(); // Convert Iterator into Vec
    filtered.sort();

    pokemon_vec_to_string(filtered)
}

pub async fn generate_search_string(maxed: bool) -> String {
    let families = match database::get_merged().await {
        Err(e) => return utils::format_error("There was an error reading database`", e),
        Ok(families) => families,
    };

    let string = match maxed {
        true => already_maxed_string(families),
        false => non_maxed_string(families),
    };

    format!("`{}`", markdown::escape(&string))
}

pub async fn stats() -> String {
    let families = match database::get_merged().await {
        Err(e) => return utils::format_error("There was an error reading database`", e),
        Ok(families) => families,
    };

    let maxed: Vec<&MergedFamily> = families
        .iter()
        .filter(|f| f.pokemons.iter().any(|p| p.level40 > 0))
        .collect();

    let n_families = families.len();
    let maxed_families = maxed.len();
    let maxed_pokes = families.iter().fold(0, |acc, x| {
        acc + x.pokemons.iter().fold(0, |acc, x| acc + x.level40)
    });

    format!(
        "Level40: {}\nFamilies: {}/{}",
        maxed_pokes, maxed_families, n_families
    )
}

pub fn version() -> String {
    format!(
        "🌐: _{}_\n⏰:` {}`",
        option_env!("GIT_HASH").unwrap_or("NA"),
        option_env!("DATE").unwrap_or("NA")
    )
}
