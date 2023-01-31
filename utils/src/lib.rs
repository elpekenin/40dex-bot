#![allow(dead_code)]
#![allow(unused_imports)]

use dotenvy::dotenv;
use tokio;
use sqlx::{
    Error, Pool, Postgres,
    postgres::PgPoolOptions,
};
use std;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connection_test() {
        dotenv().ok();

        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        
        let result = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await;

        assert!(!result.is_err(), "Couldn't connect to database");
    }

    #[tokio::test]
    async fn dex2name_test() {
        let pool = db_connect().await;

        let bulbasaur = dex2name(&pool, 1).await.unwrap();
        assert_eq!(bulbasaur, "bulbasaur");

        let invalid = dex2name(&pool, -1).await;
        assert_eq!(invalid.is_err(), true);
    }

    #[tokio::test]
    async fn name2dex_test() {
        let pool = db_connect().await;

        let bulbasaur = name2dex(&pool, "bulbasaur").await.unwrap();
        assert_eq!(bulbasaur, 1);

        let bulbasaur = name2dex(&pool, "BuLbasAur").await.unwrap();
        assert_eq!(bulbasaur, 1);

        let invalid = name2dex(&pool, "").await;
        assert_eq!(invalid.is_err(), true);
    }
}

pub async fn db_connect() -> Pool<Postgres> {
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

pub async fn dex2name(pool: &Pool<Postgres>, dex: i32) -> Result<String, Error> {
    let record = sqlx::query!(
        "
            SELECT name
            FROM pokemons
            WHERE dex = $1
        ",
        dex
    )
    .fetch_one(pool)
    .await?;

    Ok(record.name)
}

pub async fn name2dex(pool: &Pool<Postgres>, name: impl Into<String>) -> Result<i32, Error> {
    let name = name.into().to_lowercase();

    let record = sqlx::query!(
        "
            SELECT dex
            FROM pokemons
            WHERE name = $1
        ",
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(record.dex)
}