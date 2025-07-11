//! Authentication module with platform-specific implementations

pub mod auth;


// Shared types
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub aud: String,
    pub role: String,
    pub email_confirmed_at: Option<String>,
    pub phone: Option<String>,
    pub confirmed_at: Option<String>,
    pub confirmation_sent_at: Option<String>,
    pub last_sign_in_at: Option<String>,
    pub app_metadata: serde_json::Value,
    pub user_metadata: serde_json::Value,
    pub identities: Vec<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

// Platform-specific implementations
#[cfg(target_arch = "wasm32")]
mod wasm_impl;
#[cfg(target_arch = "wasm32")]
pub use wasm_impl::*;

#[cfg(not(target_arch = "wasm32"))]
mod native_impl;
#[cfg(not(target_arch = "wasm32"))]
pub use native_impl::*;
