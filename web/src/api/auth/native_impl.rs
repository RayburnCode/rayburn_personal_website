//! Native (non-WASM) authentication implementation

use dioxus::prelude::ServerFnError;
use crate::api::env;

use super::{User, Credentials};

pub async fn get_user() -> Option<User> {
    // For native builds, return None
    None
}

pub async fn signin_with_password(_credentials: Credentials) -> Result<String, ServerFnError> {
    Ok("Authentication not available in native builds".to_string())
}

pub async fn signin_with_google() -> Result<String, ServerFnError> {
    Ok("Google sign-in not available in native builds".to_string())
}

pub async fn signout() -> Result<String, ServerFnError> {
    Ok("Sign-out not available in native builds".to_string())
}

pub async fn set_session(_access_token: String, _refresh_token: String) -> Result<String, ServerFnError> {
    Ok("Session management not available in native builds".to_string())
}

// Server client for making API calls
pub struct ServerSupabaseClient {
    pub client: postgrest::Postgrest,
}

impl ServerSupabaseClient {
    pub fn new(url: &str, api_key: &str) -> Self {
        let client = postgrest::Postgrest::new(&format!("{}/rest/v1", url))
            .insert_header("apikey", api_key)
            .insert_header("Authorization", &format!("Bearer {}", api_key));
        
        Self { client }
    }

    pub fn table(&self, table_name: &str) -> postgrest::Builder {
        self.client.from(table_name)
    }
}

pub fn create_server_client() -> ServerSupabaseClient {
    let config = env::get_env_config();
    ServerSupabaseClient::new(
        &config.supabase_url,
        &config.supabase_anon_key,
    )
}
