use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{
    Error, Pool, Postgres,
    postgres::PgPoolOptions,
};

#[derive(Debug, Serialize, Clone)]
pub struct Pokemon {
    pub dex: i32,
    pub name: String,
    pub level40: i32,
    pub tradeable: i32,
}

#[derive(Debug, Serialize)]
pub struct Family {
    pub id: i32,
    pub regions: Vec<i32>,
    pub pokemons: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct MergedFamily {
    pub id: i32,
    pub regions: Vec<i32>,
    pub pokemons: Vec<Pokemon>,
}

#[cfg(test)]
mod tests;

async fn connect() -> Pool<Postgres> {
    // Load config variables from `.env` file
    // Only needed on development
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap()
}

pub async fn get_by_dex(dex: impl Into<i32>) -> Result<Pokemon, Error> {
    let dex = dex.into();

    let pool = connect().await;
    let record = sqlx::query_as!(
        Pokemon,
        "
            SELECT *
            FROM pokemons
            WHERE dex = $1
        ",
        dex
    )
    .fetch_one(&pool)
    .await?;

    Ok(record)
}

pub async fn get_by_name(name: impl Into<String>) -> Result<Pokemon, Error> {
    let name = name.into().to_lowercase();

    let pool = connect().await;
    let record = sqlx::query_as!(
        Pokemon,
        "
            SELECT *
            FROM pokemons
            WHERE name = $1
        ",
        name
    )
    .fetch_one(&pool)
    .await?;

    Ok(record)
}

pub async fn dex2name(dex: impl Into<i32>) -> Result<String, Error> {
    let pokemon = get_by_dex(dex).await?;
    Ok(pokemon.name)
}

pub async fn name2dex(name: impl Into<String>) -> Result<i32, Error> {
    let pokemon = get_by_name(name).await?;
    Ok(pokemon.dex)
}

pub async fn update_level40(name: impl Into<String>, amount: impl Into<i32>) -> Result<Pokemon, Error> {
    let amount = amount.into();
    let name = name.into().to_lowercase();

    let pool = connect().await;
    let _ = sqlx::query!(
        "
            UPDATE pokemons
            SET level40 = level40 + $1
            WHERE name = $2
        ",
        amount,
        name
    )
    .fetch_one(&pool)
    .await;

    get_by_name(name).await
}

pub async fn update_tradeable(name: impl Into<String>, amount: impl Into<i32>) -> Result<Pokemon, Error> {
    let amount = amount.into();
    let name = name.into().to_lowercase();

    let pool = connect().await;
    let _ = sqlx::query!(
        "
            UPDATE pokemons
            SET tradeable = tradeable + $1
            WHERE name = $2
        ",
        amount,
        name
    )
    .fetch_one(&pool)
    .await;

    get_by_name(name).await
}

pub async fn get_families() -> Result<Vec<Family>, Error> {
    let pool = connect().await;
    sqlx::query_as!(
        Family,
        "
            SELECT *
            FROM families
            ORDER BY regions[1], id
        "
    )
    .fetch_all(&pool)
    .await
}

pub async fn get_pokemons() -> Result<Vec<Pokemon>, Error> {
    let pool = connect().await;
    sqlx::query_as!(
        Pokemon,
        "
            SELECT *
            FROM pokemons
            ORDER BY dex
        "
    )
    .fetch_all(&pool)
    .await
}

pub async fn get_merged() -> Result<Vec<MergedFamily>, Error> {
    let families = get_families().await?;
    let pokemons = get_pokemons().await?;

    let mut merged: Vec<MergedFamily> = families
            .iter()
            .map(|f| {
                let pokemons: Vec<Pokemon> = pokemons.clone().into_iter().filter(|p| f.pokemons.contains(&p.dex)).collect();

                MergedFamily {
                    id: f.id,
                    regions: f.regions.clone(),
                    pokemons
                }
            })
            .collect();

    // sort families by their first region
    merged.sort_by(|x, y| x.regions.first().cmp(&y.regions.first()));

    Ok(merged)
}