use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio::sync::OnceCell;
use dotenv::dotenv;
use envy::from_env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub database_url: String,
}


static ONCE: OnceCell<Pool<Postgres>> = OnceCell::const_new();

// Load config from environment variables
pub fn load_config() -> Result<DatabaseConfig, envy::Error> {
    dotenv().ok(); // Load `.env` file (if it exists)
    from_env::<DatabaseConfig>()
}

pub async fn connection<'a>() -> &'a Pool<Postgres> {
    ONCE.get_or_init(|| async {
        let config = load_config().expect("Failed to load database config");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .expect("Failed to connect to database")
    })
    .await
}