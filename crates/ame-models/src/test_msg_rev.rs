#![cfg(test)]

use sqlx::postgres::PgPoolOptions;

use crate::prelude::{fetch_latest_msg_rev, fetch_latest_tiu};

#[tokio::test]
async fn test_connection() {
    let _ = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://localhost/Services")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_fetch_latest_msg_rev() {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://localhost/Services")
        .await
        .unwrap();

    let res = fetch_latest_msg_rev(&pool, 5).await.unwrap();

    println!("{:#?}", res);
}

#[tokio::test]
async fn test_fetch_latest_tiu() {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://localhost/Services")
        .await
        .unwrap();

    let res = fetch_latest_tiu(&pool, 5, 0).await.unwrap();

    println!("{:#?}", res);
}
