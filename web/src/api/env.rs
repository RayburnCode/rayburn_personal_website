use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    pub supabase_project_id: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
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
            // For WASM, use compile-time environment variables set by build.rs
            EnvConfig {
                supabase_project_id: env!("APP_PUBLIC_ID").to_string(),
                supabase_url: env!("APP_PUBLIC_SUPABASE_URL").to_string(),
                supabase_anon_key: env!("APP_PUBLIC_SUPABASE_ANON_KEY").to_string(),
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref ENV_CONFIG: EnvConfig = EnvConfig::load();
    pub static ref APP_PUBLIC_ID: String = ENV_CONFIG.supabase_project_id.clone();
    pub static ref APP_PUBLIC_SUPABASE_URL: String = ENV_CONFIG.supabase_url.clone();
    pub static ref APP_PUBLIC_SUPABASE_ANON_KEY: String = ENV_CONFIG.supabase_anon_key.clone();
}

pub fn get_env_config() -> &'static EnvConfig {
    &ENV_CONFIG
}