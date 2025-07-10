use serde::{Deserialize, Serialize};
use dioxus::prelude::*;



// Define your blog post structure
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: String,
    pub published_at: String,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub tags: Vec<String>,
    pub cover_image: Option<String>,
    pub author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupabaseBlogPost {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: String,
    pub published_at: String,
    pub updated_at: Option<String>,
    pub tags: Option<serde_json::Value>, // JSON array from Supabase
    pub cover_image: Option<String>,
    pub author: String,
    pub is_published: Option<bool>,
    pub created_at: Option<String>,

}

impl From<SupabaseBlogPost> for BlogPost {
    fn from(supabase_post: SupabaseBlogPost) -> Self {
        let tags = if let Some(tags_json) = supabase_post.tags {
            if let Ok(tags_vec) = serde_json::from_value::<Vec<String>>(tags_json) {
                tags_vec
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        BlogPost {
            id: supabase_post.id,
            title: supabase_post.title,
            slug: supabase_post.slug,
            content: supabase_post.content,
            excerpt: supabase_post.excerpt,
            published_at: supabase_post.published_at,
            updated_at: supabase_post.updated_at,
            tags,
            cover_image: supabase_post.cover_image,
            author: supabase_post.author,
            created_at: supabase_post.created_at,
        }
    }
}



#[server(name = GetBlog)]
pub async fn get_blog() -> Result<Vec<BlogPost>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::api::auth::create_server_client;
        use tracing::info;

        info!("Fetching blog posts from Supabase...");
        
        let client = create_server_client();

        // Query blog posts with specific fields and ordering - remove any potential limits
        let resp = client
            .table("blog_posts")
            .select("id,title,content,author,created_at,slug, excerpt,tags,cover_image,published_at")
            .order("created_at.desc")
            .limit(1000) // Explicitly set a high limit to ensure we get all blog posts
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    info!("Raw Supabase response: {}", text);
                    info!("Response length: {} characters", text.len());

                    // Try to parse the JSON response into our BlogPost struct
                    match serde_json::from_str::<Vec<BlogPost>>(&text) {
                        Ok(blog_posts) => {
                            info!("Successfully parsed {} blog posts", blog_posts.len());
                            for (i, blog_post) in blog_posts.iter().enumerate() {
                                info!("Blog Post {}: '{}' (author: '{}')", i + 1, blog_post.title, blog_post.author);
                            }
                            Ok(blog_posts)
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

                            Err(ServerFnError::new(format!("Failed to parse blog posts JSON: {}", parse_error)))
                        }
                    }
                } else {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    let error_msg = format!("Failed to fetch blog posts. Status: {}, Response: {}", status, text);
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



#[server(name = GetBlogWithSlug)]
pub async fn get_blog_with_slug(slug: String) -> Result<BlogPost, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::api::auth::create_server_client;
        use tracing::info;

        info!("Fetching blog post with slug '{}' from Supabase...", slug);
        
        let client = create_server_client();

        // Query specific blog post by slug
        let resp = client
            .table("blog_posts")
            .select("id,title,content,author,created_at,slug,excerpt,tags,cover_image,published_at,updated_at")
            .eq("slug", &slug)
            .eq("is_published", "true") // Only fetch published posts
            .single() // Use single() to get one result instead of an array
            .execute()
            .await;
            
        match resp {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    info!("Raw Supabase response: {}", text);

                    // Parse the JSON response into our SupabaseBlogPost struct first
                    match serde_json::from_str::<SupabaseBlogPost>(&text) {
                        Ok(supabase_post) => {
                            info!("Successfully parsed blog post: '{}'", supabase_post.title);
                            let blog_post: BlogPost = supabase_post.into();
                            Ok(blog_post)
                        }
                        Err(parse_error) => {
                            info!("JSON parsing failed: {}", parse_error);
                            info!("Attempting to parse as serde_json::Value for debugging...");
                            
                            match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(value) => {
                                    info!("Raw JSON structure: {:#}", value);
                                }
                                Err(_) => info!("Response is not valid JSON at all")
                            }

                            Err(ServerFnError::new(format!("Failed to parse blog post JSON: {}", parse_error)))
                        }
                    }
                } else {
                    let text = response.text().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                    let error_msg = format!("Failed to fetch blog post. Status: {}, Response: {}", status, text);
                    info!("{}", error_msg);
                    if status.as_u16() == 404 {
                        Err(ServerFnError::new(format!("Blog post with slug '{}' not found", slug)))
                    } else {
                        Err(ServerFnError::new(error_msg))
                    }
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