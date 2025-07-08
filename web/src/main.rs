// src/main.rs
use dioxus::prelude::*;
use tracing::info;

use views::{AppLayout, About, Blog, Contact, Home, Projects, Resume, BlogPostDetail, Protected, Callback, Login};

mod components;
mod views;
mod api;

use api::auth::AuthorizedClient;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



fn main() {
    dioxus::logger::initialize_default();

    server_only!({
        dotenv::dotenv().ok();
        info!("loaded env variables");
    });
    
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        #[route("/blog")]
        Blog {},
        
        #[route("/blog/:slug")]
        BlogPostDetail { slug: String },

        #[route("/about")]
        About {},

        #[route("/contact")]
        Contact {},

        #[route("/projects")]
        Projects {},

        #[route("/resume")]
        Resume {},

        #[route("/protected")]
        Protected {},
        #[route("/login")]
        Login {},


        #[route("/callback")]
        Callback {},
}



#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(()) 
}

#[server(name = GetServerData, client=AuthorizedClient)]
async fn get_server_data() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For server-side, create a new client instance
        use api::auth::create_server_client;
        let client = create_server_client();

        let resp = client 
            .table("created_tasks")
            .select("*")
            .execute()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let text = resp.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
        info!("Server response: {:#?}", text);
        Ok(format!("Server data: {}", text))
    }
    #[cfg(target_arch = "wasm32")]
    {
        // This shouldn't be called on WASM since it's a server function
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}

#[server(name = TestSupabaseConnection)]
async fn test_supabase_connection() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use api::auth::create_server_client;
        
        info!("Testing Supabase connection by reading projects...");
        
        let client = create_server_client();
        
        // Query the projects table (which has public read access)
        let resp = client
            .table("projects")
            .select("*")
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                
                info!("Supabase response status: {}", status);
                info!("Supabase response body: {}", text);
                
                if status.is_success() {
                    Ok(format!("✅ Supabase connection successful!\n\nStatus: {}\n\nProjects data:\n{}", status, text))
                } else {
                    Ok(format!("⚠️ Supabase responded but with error status: {}\n\nResponse: {}", status, text))
                }
            }
            Err(e) => {
                let error_msg = format!("❌ Failed to connect to Supabase: {}", e);
                info!("{}", error_msg);
                Ok(error_msg)
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}

#[server(name = GetProjects)]
async fn get_projects() -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use api::auth::create_server_client;
        
        info!("Fetching projects from Supabase...");
        
        let client = create_server_client();
        
        // Query projects with specific fields and ordering
        let resp = client
            .table("projects")
            .select("id,title,description,category,technologies,github_url,demo_url,featured,created_at")
            .order("created_at.desc")
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    info!("Successfully fetched projects: {}", text);
                    Ok(text)
                } else {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    let error_msg = format!("Failed to fetch projects. Status: {}, Response: {}", status, text);
                    info!("{}", error_msg);
                    Err(ServerFnError::new(error_msg))
                }
            }
            Err(e) => {
                let error_msg = format!("Request failed: {}", e);
                info!("{}", error_msg);
                Err(ServerFnError::new(error_msg))
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(ServerFnError::new("Server function called on client side".to_string()))
    }
}

mod compat;

#[cfg(target_arch = "wasm32")]
use compat::wasm as platform;

#[cfg(not(target_arch = "wasm32"))]
use compat::native as platform;

async fn make_request() {
    let response = platform::fetch("https://api.example.com/data").await;
    // Handle response
}