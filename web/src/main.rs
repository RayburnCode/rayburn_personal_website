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
        use tracing::info;
        
        info!("Testing Supabase connection by reading projects...");
        
        let client = create_server_client();
        
        // First, get a count of all projects
        let count_resp = client
            .table("projects")
            .select("*")
            .execute()
            .await;
            
        match count_resp {
            Ok(response) => {
                let status = response.status();
                let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                
                info!("Supabase response status: {}", status);
                info!("Supabase response body: {}", text);
                
                if status.is_success() {
                    // Try to parse as JSON to count items
                    let count_info = match serde_json::from_str::<serde_json::Value>(&text) {
                        Ok(json) => {
                            if let Some(array) = json.as_array() {
                                format!("Found {} projects in database", array.len())
                            } else {
                                format!("Response is not an array: {}", text)
                            }
                        }
                        Err(_) => format!("Could not parse JSON: {}", text)
                    };
                    
                    Ok(format!("✅ Supabase connection successful!\n\nStatus: {}\n\n{}\n\nRaw data:\n{}", status, count_info, text))
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

