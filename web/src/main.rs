// src/main.rs
use dioxus::prelude::*;
 use tracing::info;
use views::{AppLayout, About, Blog, Contact, Home, Projects, Resume, BlogPostDetail, Protected, Callback, Login};

mod components;
mod views;
mod api;

use api::auth::AuthorizedClient;

// Project data structure to match our database schema
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub technologies: Vec<String>,
    pub image_url: Option<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    pub featured: bool,
    pub created_at: String,
}


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

#[server(name = GetProjects)]
async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use api::auth::create_server_client;
        use tracing::info;
        
        info!("Fetching projects from Supabase...");
        
        let client = create_server_client();
        
        // Query projects with specific fields and ordering - remove any potential limits
        let resp = client
            .table("projects")
            .select("id,title,description,category,technologies,github_url,demo_url,featured,created_at,image_url")
            .order("created_at.desc")
            .limit(1000) // Explicitly set a high limit to ensure we get all projects
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    info!("Raw Supabase response: {}", text);
                    info!("Response length: {} characters", text.len());
                    
                    // Try to parse the JSON response into our Project struct
                    match serde_json::from_str::<Vec<Project>>(&text) {
                        Ok(projects) => {
                            info!("Successfully parsed {} projects", projects.len());
                            for (i, project) in projects.iter().enumerate() {
                                info!("Project {}: '{}' (category: '{}')", i + 1, project.title, project.category);
                            }
                            Ok(projects)
                        }
                        Err(parse_error) => {
                            info!("JSON parsing failed: {}", parse_error);
                            info!("Attempting to parse as serde_json::Value for debugging...");
                            
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(value) => {
                                    info!("Raw JSON structure: {:#}", value);
                                    if let Some(array) = value.as_array() {
                                        info!("Found {} items in JSON array", array.len());
                                    }
                                }
                                Err(_) => info!("Response is not valid JSON at all")
                            }
                            
                            Err(ServerFnError::new(format!("Failed to parse projects JSON: {}", parse_error)))
                        }
                    }
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