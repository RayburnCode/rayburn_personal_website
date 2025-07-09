use dioxus::prelude::*;


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

#[server(name = GetProjects)]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::api::auth::create_server_client;
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