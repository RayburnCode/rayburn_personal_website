#[cfg(not(target_arch = "wasm32"))]
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use sqlx::postgres::PgPoolOptions;
#[cfg(not(target_arch = "wasm32"))]
use sqlx::{Pool, Postgres};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;
#[cfg(not(target_arch = "wasm32"))]
use dotenv::dotenv;
#[cfg(not(target_arch = "wasm32"))]
use envy::from_env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub database_url: String,
}

#[cfg(not(target_arch = "wasm32"))]
static ONCE: std::sync::OnceLock<Pool<Postgres>> = std::sync::OnceLock::new();

// Load config from environment variables
#[cfg(not(target_arch = "wasm32"))]
pub fn load_config() -> Result<DatabaseConfig, envy::Error> {
    dotenv().ok(); // Load `.env` file (if it exists)
    from_env::<DatabaseConfig>()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn connection() -> Result<&'static Pool<Postgres>, sqlx::Error> {
    if let Some(pool) = ONCE.get() {
        return Ok(pool);
    }
    
    let config = load_config().expect("Failed to load database config");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    
    Ok(ONCE.get_or_init(|| pool))
}