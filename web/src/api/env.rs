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
        #[cfg(not(target_arch = "wasm32"))]
        {
            dotenv::dotenv().ok(); // Load `.env` file
            
            envy::from_env::<EnvConfig>()
                .expect("Failed to parse environment variables")
        }
        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, we can't use environment variables the same way
            // You might want to pass these as build-time constants or use a different approach
            EnvConfig {
                app_public_id: "your_app_id".to_string(),
                app_public_supabase_url: "your_supabase_url".to_string(),
                app_public_supabase_anon_key: "your_anon_key".to_string(),
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref APP_PUBLIC_ID: String = EnvConfig::load().app_public_id;
    pub static ref APP_PUBLIC_SUPABASE_URL: String = EnvConfig::load().app_public_supabase_url;
    pub static ref APP_PUBLIC_SUPABASE_ANON_KEY: String = EnvConfig::load().app_public_supabase_anon_key;
}