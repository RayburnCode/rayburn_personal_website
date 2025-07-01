use dioxus::prelude::*;
use dioxus_signals::Signal;
use serde::{Deserialize, Serialize};
use crate::views::BlogPostDetail;
use crate::Route;

// Define your blog post structure
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogPost {
    id: i32,
    title: String,
    slug: String,
    content: String,
    excerpt: String,
    published_at: String,
    updated_at: Option<String>,
    tags: Vec<String>,
    cover_image: Option<String>,
    author: String,
}

/// The Blog page component
#[component]
pub fn Blog() -> Element {
    let mut posts = use_signal::<Option<Vec<BlogPost>>>(|| None);
    let mut loading = use_signal(|| true);
    let  error = use_signal::<Option<String>>(|| None);

    // Fetch posts from Supabase when component mounts
    use_signal(move || async move {
        loading.set(true);
        
        // In a real app, you would replace this with actual Supabase API call
        // Example:
        // let client = supabase_rs::new(SUPABASE_URL, SUPABASE_KEY);
        // let response = client.from("posts").select("*").execute().await;
        
        // Simulate API call
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        
        // Mock data - replace with your actual Supabase response handling
        let mock_posts = vec![
            BlogPost {
                id: 1,
                slug: "combining-finance-and-technology".to_string(),
                excerpt: "Exploring the intersection of finance and technology...".to_string(),
                title: "Combining Finance and Technology".to_string(),
                author: "Dylan".to_string(),
                content: "How I bridge my financial expertise with my passion for coding...".to_string(),
                published_at: "2023-10-15".to_string(),
                updated_at: Some("2023-10-16".to_string()),
                tags: vec!["Finance".to_string(), "Tech".to_string()],
                cover_image: Some("/images/blog-finance-tech.jpg".to_string()),
            },
            BlogPost {
                id: 2,
                slug: "getting-started-with-3d-printing".to_string(),
                excerpt: "My journey into 3D printing and how it complements my technical skills...".to_string(),
                title: "Getting Started with 3D Printing".to_string(),
                author: "Dylan".to_string(),
                content: "My journey into 3D printing and how it complements my technical skills...".to_string(),
                published_at: "2023-09-22".to_string(),
                updated_at: Some("2023-09-23".to_string()),
                tags: vec!["3D Printing".to_string(), "CAD".to_string()],
                cover_image: Some("/images/blog-3d-printing.jpg".to_string()),
            },
            BlogPost {
                id: 3,
                slug: "building-financial-tools-with-rust".to_string(),
                excerpt: "Why I chose Rust for building financial applications and calculators...".to_string(),
                title: "Building Financial Tools with Rust".to_string(),
                author: "Dylan".to_string(),
                content: "Why I chose Rust for building financial applications and calculators...".to_string(),
                published_at: "2023-08-10".to_string(),
                updated_at: Some("2023-09-23".to_string()),

                tags: vec!["Rust".to_string(), "Finance".to_string(), "Programming".to_string()],
                cover_image: None,
            },
        ];
        
        posts.set(Some(mock_posts));
        loading.set(false);
    });

    rsx! {
        div { class: "max-w-4xl mx-auto p-6 text-gray-800 dark:text-gray-200",
            h1 { class: "text-3xl font-bold mb-8", "Blog" }
            p { class: "mb-8 text-lg",
                "Thoughts on finance, technology, 3D printing, and the intersection of these passions."
            }
            
            // Loading state
            if *loading.read() {
                div { class: "flex justify-center py-12",
                    div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500" }
                }
            }
            
            // Error state
            if let Some(err) = error.read().as_ref() {
                div { class: "p-4 mb-6 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded",
                    "Error loading posts: {err}"
                }
            }
            
            // Blog posts list
            if let Some(posts) = posts.read().as_ref() {
                div { class: "space-y-8",
                    for post in posts {
                        BlogPostCard { post: post.clone() }
                    }
                }
            } else if !*loading.read() {
                div { class: "text-center py-12 text-gray-500",
                    "No blog posts found."
                }
            }
        }
    }
}

/// Individual blog post card component
#[component]
fn BlogPostCard(post: BlogPost) -> Element {
    rsx! {
        article { 
            class: "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden hover:shadow-lg transition-shadow",
            
            // Cover image if available
            if let Some(image_url) = &post.cover_image {
                img { 
                    class: "w-full h-48 object-cover",
                    src: "{image_url}",
                    alt: "Cover image for {post.title}" 
                }
            }
            
            div { class: "p-6",
                // Post metadata
                div { class: "flex items-center text-sm text-gray-500 dark:text-gray-400 mb-2",
                    span { class: "mr-4", "{post.published_at}" }
                    div { class: "flex flex-wrap gap-2",
                        for tag in &post.tags {
                            span { 
                                class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-xs",
                                "{tag}"
                            }
                        }
                    }
                }
                
                // Title
                h2 { class: "text-2xl font-bold mb-3",
                    Link { 
                        to: Route::BlogPostDetail { slug: post.slug.clone() },
                        class: "hover:text-blue-600 dark:hover:text-blue-400",
                        "{post.title}"
                    }
                }
                
                // Excerpt
                p { class: "text-gray-600 dark:text-gray-400 mb-4",
                    "{post.content.chars().take(150).collect::<String>()}..."
                }
                
                // Read more link
                Link { 
                    to: Route::BlogPostDetail { slug: post.slug.clone() },
                    class: "text-blue-600 dark:text-blue-400 hover:underline font-medium",
                    "Read more →"
                }
            }
        }
    }
}

