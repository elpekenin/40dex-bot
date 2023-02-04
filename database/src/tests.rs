use super::*;

#[tokio::test]
async fn connect_test() {
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
    let pool = connect().await;

    let bulbasaur = dex2name(&pool, 1).await.unwrap();
    assert_eq!(bulbasaur, "bulbasaur");

    let invalid = dex2name(&pool, -1).await;
    assert_eq!(invalid.is_err(), true);
}

#[tokio::test]
async fn name2dex_test() {
    let pool = connect().await;

    let bulbasaur = name2dex(&pool, "bulbasaur").await.unwrap();
    assert_eq!(bulbasaur, 1);

    let bulbasaur = name2dex(&pool, "BuLbasAur").await.unwrap();
    assert_eq!(bulbasaur, 1);

    let invalid = name2dex(&pool, "").await;
    assert_eq!(invalid.is_err(), true);
}

#[tokio::test]
async fn update_level40_test() {
    let pool = connect().await;

    let start = get_by_name(&pool, "bulbasaur").await.unwrap();
    
    let _ = update_level40(&pool, "bulbasaur", 1).await;
    let plus_one = get_by_name(&pool, "bulbasaur").await.unwrap();
    assert_eq!(start.level40 + 1, plus_one.level40);

    let _ = update_level40(&pool, "bulbasaur", -1).await;
    let back = get_by_name(&pool, "bulbasaur").await.unwrap();
    assert_eq!(start.level40, back.level40);
}

#[tokio::test]
async fn update_tradeable_test() {
    let pool = connect().await;

    let start = get_by_name(&pool, "bulbasaur").await.unwrap();
    
    let _ = update_tradeable(&pool, "bulbasaur", 1).await;
    let plus_one = get_by_name(&pool, "bulbasaur").await.unwrap();
    assert_eq!(start.tradeable + 1, plus_one.tradeable);

    let _ = update_tradeable(&pool, "bulbasaur", -1).await;
    let back = get_by_name(&pool, "bulbasaur").await.unwrap();
    assert_eq!(start.tradeable, back.tradeable);
}