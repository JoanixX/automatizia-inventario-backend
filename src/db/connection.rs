use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use anyhow::Result;

pub async fn create_pool() -> Result<Pool<Postgres>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
