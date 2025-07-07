use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    pub app_public_id: String,
    pub app_public_supabase_url: String,
    pub app_public_supabase_anon_key: String,
}

impl EnvConfig {
    pub fn load() -> Self {
        dotenv::dotenv().ok(); // Load `.env` file
        
        envy::from_env::<EnvConfig>()
            .expect("Failed to parse environment variables")
    }
}

lazy_static::lazy_static! {
    pub static ref APP_PUBLIC_ID: String = EnvConfig::load().app_public_id;
    pub static ref APP_PUBLIC_SUPABASE_URL: String = EnvConfig::load().app_public_supabase_url;
    pub static ref APP_PUBLIC_SUPABASE_ANON_KEY: String = EnvConfig::load().app_public_supabase_anon_key;
}