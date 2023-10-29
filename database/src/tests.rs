use super::*;

#[tokio::test]
async fn connect_test() {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    let result = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await;

    assert!(result.is_ok(), "Couldn't connect to database");
}

#[tokio::test]
async fn dex2name_test() {
    let bulbasaur = dex2name(1).await.unwrap();
    assert_eq!(bulbasaur, "bulbasaur");

    let invalid = dex2name(-1).await;
    assert!(invalid.is_err());
}

#[tokio::test]
async fn name2dex_test() {
    let bulbasaur = name2dex("bulbasaur").await.unwrap();
    assert_eq!(bulbasaur, 1);

    let bulbasaur = name2dex("BuLbasAur").await.unwrap();
    assert_eq!(bulbasaur, 1);

    let invalid = name2dex("").await;
    assert!(invalid.is_err());
}

#[tokio::test]
async fn update_level40_test() {
    let start = get_by_name("bulbasaur").await.unwrap();
    
    let _ = update_level40("bulbasaur", 1).await;
    let plus_one = get_by_name("bulbasaur").await.unwrap();
    assert_eq!(start.level40 + 1, plus_one.level40);

    let _ = update_level40("bulbasaur", -1).await;
    let back = get_by_name("bulbasaur").await.unwrap();
    assert_eq!(start.level40, back.level40);
}

#[tokio::test]
async fn update_tradeable_test() {
    let start = get_by_name("bulbasaur").await.unwrap();
    
    let _ = update_tradeable("bulbasaur", 1).await;
    let plus_one = get_by_name("bulbasaur").await.unwrap();
    assert_eq!(start.tradeable + 1, plus_one.tradeable);

    let _ = update_tradeable("bulbasaur", -1).await;
    let back = get_by_name("bulbasaur").await.unwrap();
    assert_eq!(start.tradeable, back.tradeable);
}

#[tokio::test]
async fn get_families_test() {
    let valid = get_families().await;
    assert!(valid.is_ok());
}

#[tokio::test]
async fn get_merged_test() {
    let valid = get_merged().await;
    assert!(valid.is_ok());
}
