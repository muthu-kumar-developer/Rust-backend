use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn create_pool() -> PgPool {
    // Load DATABASE_URL from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    // Create PostgreSQL connection pool
    PgPoolOptions::new()
        .max_connections(10)     // number of DB connections to keep in pool
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
