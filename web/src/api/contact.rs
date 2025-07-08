use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactSubmission {
    pub id: Option<String>, // Always use String for cross-platform compatibility
    pub name: String,
    pub email: String,
    pub message: String,
    pub created_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct NewContactSubmission {
    pub name: String,
    pub email: String,
    pub message: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub subject: Option<String>,
    pub metadata: Option<Value>,
}

impl From<ContactSubmission> for NewContactSubmission {
    fn from(submission: ContactSubmission) -> Self {
        Self {
            name: submission.name,
            email: submission.email,
            message: submission.message,
            ip_address: submission.ip_address,
            user_agent: submission.user_agent,
            subject: submission.subject,
            metadata: submission.metadata,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn submit_contact_form(submission: NewContactSubmission) -> Result<ContactSubmission, String> {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};
    use crate::api::env::{APP_PUBLIC_SUPABASE_URL, APP_PUBLIC_SUPABASE_ANON_KEY};

    let url = format!("{}/rest/v1/contact_submissions", *APP_PUBLIC_SUPABASE_URL);
    
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|e| format!("Failed to create headers: {:?}", e))?;
    headers.set("Content-Type", "application/json").map_err(|e| format!("Failed to set content type: {:?}", e))?;
    headers.set("apikey", &APP_PUBLIC_SUPABASE_ANON_KEY).map_err(|e| format!("Failed to set API key: {:?}", e))?;
    headers.set("Authorization", &format!("Bearer {}", *APP_PUBLIC_SUPABASE_ANON_KEY)).map_err(|e| format!("Failed to set authorization: {:?}", e))?;
    headers.set("Prefer", "return=representation").map_err(|e| format!("Failed to set prefer header: {:?}", e))?;
    
    opts.headers(&headers);

    let body = serde_json::to_string(&submission).map_err(|e| format!("Failed to serialize submission: {}", e))?;
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));

    let request = Request::new_with_str_and_init(&url, &opts).map_err(|e| format!("Failed to create request: {:?}", e))?;

    let window = web_sys::window().ok_or("No window object")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|e| format!("Fetch failed: {:?}", e))?;
    let resp: Response = resp_value.dyn_into().map_err(|_| "Response is not a Response object")?;

    if !resp.ok() {
        let status = resp.status();
        let text = JsFuture::from(resp.text().map_err(|e| format!("Failed to get response text: {:?}", e))?)
            .await
            .map_err(|e| format!("Failed to read response text: {:?}", e))?;
        let text_str = text.as_string().unwrap_or_else(|| "Unknown error".to_string());
        return Err(format!("HTTP {}: {}", status, text_str));
    }

    let json_value = JsFuture::from(resp.json().map_err(|e| format!("Failed to parse JSON: {:?}", e))?)
        .await
        .map_err(|e| format!("Failed to read JSON: {:?}", e))?;

    let json_str = js_sys::JSON::stringify(&json_value).map_err(|e| format!("Failed to stringify JSON: {:?}", e))?;
    let json_str = json_str.as_string().ok_or("Failed to convert JSON to string")?;
    
    let submissions: Vec<ContactSubmission> = serde_json::from_str(&json_str).map_err(|e| format!("Failed to deserialize response: {}", e))?;
    
    submissions.into_iter().next().ok_or("No submission returned".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn submit_contact_form(submission: NewContactSubmission) -> Result<ContactSubmission, String> {
    use crate::api::database::connection;
    use sqlx::{query, Row};

    let pool = connection().await.map_err(|e| format!("Database connection failed: {}", e))?;

    let result = query(
        r#"
        INSERT INTO contact_submissions (name, email, message, ip_address, user_agent, subject, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id::text, name, email, message, created_at, ip_address::text, user_agent, subject, status, metadata
        "#
    )
    .bind(&submission.name)
    .bind(&submission.email)
    .bind(&submission.message)
    .bind(submission.ip_address.as_deref())
    .bind(submission.user_agent.as_deref())
    .bind(submission.subject.as_deref())
    .bind(&submission.metadata)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to insert contact submission: {}", e))?;

    Ok(ContactSubmission {
        id: result.get("id"),
        name: result.get("name"),
        email: result.get("email"),
        message: result.get("message"),
        created_at: result.get("created_at"),
        ip_address: result.get("ip_address"),
        user_agent: result.get("user_agent"),
        subject: result.get("subject"),
        status: result.get("status"),
        metadata: result.get("metadata"),
    })
}
