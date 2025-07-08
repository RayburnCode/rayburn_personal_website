use std::future::Future;

use dioxus::{
    prelude::{
        server_fn::{
            client::{browser::BrowserClient, Client},
            request::browser::BrowserRequest,
            response::browser::BrowserResponse,
        },
        ServerFnError,
    },
    signals::{GlobalSignal, Readable, Signal},
};
use gloo::storage::Storage;
use serde_json::Value;
use serde_wasm_bindgen::to_value;
use supabase_js_rs::*;
use tracing::debug;
use wasm_bindgen::{closure::Closure, JsValue};

use crate::api::env;
use crate::api::env::EnvConfig;


pub static CLIENT: GlobalSignal<SupabaseClient> = Signal::global(create_client);

// auth funcs https://github.com/supabase/auth-js/blob/59ec9affa01c780fb18f668291fa7167a65c391d/src/GoTrueClient.ts#L1152
// https://github.com/supabase/auth-js/blob/59ec9affa01c780fb18f668291fa7167a65c391d/src/lib/types.ts#L325
// https://supabase.github.io/auth-js/v2/

// Custom web client to attach supabase bearer information onto requests
pub struct AuthorizedClient;
// https://github.com/leptos-rs/leptos/blob/97fd8ff6c46f742ce809398aa05161567b90a16b/examples/server_fns_axum/src/app.rs#L808-L852
impl<CustErr> Client<CustErr> for AuthorizedClient {
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, ServerFnError<CustErr>>> + Send {
        let res = gloo::storage::LocalStorage::get::<Session>(format!(
            "sb-{}-auth-token",
            env::APP_PUBLIC_ID.as_str()
        ));
        if let Ok(session) = res {
            let headers = req.headers();
            headers.append("Authorization", &format!("Bearer {}", session.access_token));
        }
        BrowserClient::send(req)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub is_anonymous: Option<bool>,
    pub role: Option<String>,
    pub aud: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

// get_user types

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthError {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetUserData {
    user: Option<User>,
} 

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetUserResponse {
    data: GetUserData,
    error: Option<AuthError>,
}

pub async fn get_user() -> Result<User, AuthError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.get_user(None).await {
        let res = serde_wasm_bindgen::from_value::<GetUserResponse>(res).unwrap();
        if let Some(error) = res.error {
            return Err(error);
        }

        return Ok(res.data.user.unwrap());
    }
    panic!("todo error")
}

pub async fn on_state_change() {
    let auth = CLIENT.read().auth();
    auth.on_auth_state_change(&Closure::new(move |event: JsValue, session: JsValue| {}));
}

// signin_with_password types
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthResponseData {
    user: Option<User>,
    session: Option<Session>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthResponseError {
    // Todo
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthResponsePassword {
    data: AuthResponseData,
    error: Option<AuthResponseError>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserWithSession {
    pub user: User,
    session: Session,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub redirect_to: String,
}

pub async fn signin_with_password(
    credentials: Credentials,
) -> Result<UserWithSession, AuthResponseError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.sign_in_with_password(credentials).await {
        let res = serde_wasm_bindgen::from_value::<AuthResponsePassword>(res).unwrap();

        if let Some(error) = res.error {
            return Err(error);
        }

        return Ok(UserWithSession {
            user: res.data.user.unwrap(),
            session: res.data.session.unwrap(),
        });
    }
    panic!("todo error")
}

pub async fn signin_with_google() {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth
        .sign_in_with_oauth(SignInWithOAuthCredentials {
            provider: "google".to_string(),
            options: to_value(&Options {
                redirect_to: "http://localhost:8081/callback".to_string(),
            })
            .unwrap(),
        })
        .await
    {
        debug!("{res:?}");
        return;
    }
    panic!("todo error");
}

pub async fn set_session(access_token: String, refresh_token: String) {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth
        .set_session(CurrentSession {
            access_token,
            refresh_token,
        })
        .await
    {
        debug!("{res:?}");
        return;
    }
    panic!("todo error");
}

pub async fn signout() -> Result<(), AuthResponseError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.sign_out().await {
        let res = serde_wasm_bindgen::from_value::<Value>(res).unwrap();
        debug!("{res:?}");
        return Ok(());
    }
    Err(AuthResponseError {})
}

fn create_client() -> SupabaseClient {
    let config = EnvConfig::load(); // Load config once
    
    supabase_js_rs::create_client(
        config.app_public_supabase_url.as_str(),
        config.app_public_supabase_anon_key.as_str(),
    )
}

// Server-side Supabase client (for native/server builds)
#[cfg(not(target_arch = "wasm32"))]
pub struct ServerSupabaseClient {
    pub client: postgrest::Postgrest,
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
pub fn create_server_client() -> ServerSupabaseClient {
    use crate::api::env::EnvConfig;
    let config = EnvConfig::load();
    ServerSupabaseClient::new(
        &config.app_public_supabase_url,
        &config.app_public_supabase_anon_key,
    )
}