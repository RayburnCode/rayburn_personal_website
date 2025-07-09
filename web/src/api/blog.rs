use serde::{Deserialize, Serialize};

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
        }
    }
}

/// Fetch all published blog posts from Supabase
pub async fn fetch_blog_posts() -> Result<Vec<BlogPost>, Box<dyn std::error::Error>> {
    #[cfg(target_arch = "wasm32")]
    {
        use crate::api::env::{APP_PUBLIC_SUPABASE_URL, APP_PUBLIC_SUPABASE_ANON_KEY};
        use gloo::net::http::Request;
        
        let url = format!(
            "{}/rest/v1/blog_posts?is_published=eq.true&order=published_at.desc",
            *APP_PUBLIC_SUPABASE_URL
        );
        
        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", *APP_PUBLIC_SUPABASE_ANON_KEY))
            .header("apikey", &*APP_PUBLIC_SUPABASE_ANON_KEY)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        
        if !response.ok() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to fetch blog posts: {} (status: {})", error_text, response.status()).into());
        }
        
        let supabase_posts: Vec<SupabaseBlogPost> = response.json().await?;
        let blog_posts: Vec<BlogPost> = supabase_posts.into_iter().map(|p| p.into()).collect();
        
        Ok(blog_posts)
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For server-side, return empty for now
        Ok(vec![])
    }
}

/// Fetch a single blog post by slug from Supabase
#[allow(dead_code)]
pub async fn fetch_blog_post_by_slug(_slug: &str) -> Result<Option<BlogPost>, Box<dyn std::error::Error>> {
    #[cfg(target_arch = "wasm32")]
    {
        use crate::api::env::{APP_PUBLIC_SUPABASE_URL, APP_PUBLIC_SUPABASE_ANON_KEY};
        use gloo::net::http::Request;
        
        let url = format!(
            "{}/rest/v1/blog_posts?slug=eq.{}&is_published=eq.true&limit=1",
            *APP_PUBLIC_SUPABASE_URL,
            _slug
        );
        
        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", *APP_PUBLIC_SUPABASE_ANON_KEY))
            .header("apikey", &*APP_PUBLIC_SUPABASE_ANON_KEY)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        
        if !response.ok() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Failed to fetch blog post: {} (status: {})", error_text, response.status()).into());
        }
        
        let mut supabase_posts: Vec<SupabaseBlogPost> = response.json().await?;
        
        if let Some(supabase_post) = supabase_posts.pop() {
            Ok(Some(supabase_post.into()))
        } else {
            Ok(None)
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For server-side, return None for now
        Ok(None)
    }
}
