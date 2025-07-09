use dioxus::prelude::*;
use crate::Route;
use crate::api::blog::{fetch_blog_posts, BlogPost};

/// The Blog page component
#[component]
pub fn Blog() -> Element {
    let mut posts = use_signal::<Vec<BlogPost>>(|| vec![]);
    let loading = use_signal(|| false); // Start as false to show content immediately
    let error = use_signal::<Option<String>>(|| None);

    // Set up mock data immediately
    use_effect(move || {
        let mock_posts = vec![
            BlogPost {
                id: 1,
                slug: "combining-finance-and-technology".to_string(),
                excerpt: "Exploring the intersection of finance and technology...".to_string(),
                title: "Combining Finance and Technology".to_string(),
                author: "Dylan Rayburn".to_string(),
                content: "In today's rapidly evolving digital landscape, the intersection of finance and technology has become more crucial than ever. As someone with a background in both fields, I've seen firsthand how technology can revolutionize traditional financial processes and create new opportunities for innovation.

From building automated trading systems to developing personal finance calculators, the possibilities are endless when you combine financial knowledge with programming skills. In this post, I'll share some insights from my journey bridging these two worlds and explore some exciting projects that demonstrate this powerful synergy.".to_string(),
                published_at: "2024-10-15".to_string(),
                updated_at: Some("2024-10-16".to_string()),
                tags: vec!["Finance".to_string(), "Technology".to_string(), "Programming".to_string()],
                cover_image: Some("/assets/blog-finance-tech.jpg".to_string()),
            },
            BlogPost {
                id: 2,
                slug: "getting-started-with-3d-printing".to_string(),
                excerpt: "My journey into 3D printing and how it complements my technical skills...".to_string(),
                title: "Getting Started with 3D Printing".to_string(),
                author: "Dylan Rayburn".to_string(),
                content: "3D printing has opened up a whole new world of creativity and problem-solving for me. What started as curiosity about this emerging technology has evolved into a valuable skill that complements my software development work.

In this post, I'll walk you through my 3D printing journey, from choosing my first printer to designing and printing custom solutions for everyday problems. Whether you're a fellow developer looking to expand your maker skills or someone curious about getting into 3D printing, this guide will give you a solid foundation to start your own journey.".to_string(),
                published_at: "2024-09-22".to_string(),
                updated_at: Some("2024-09-23".to_string()),
                tags: vec!["3D Printing".to_string(), "CAD".to_string(), "Maker".to_string()],
                cover_image: Some("/assets/blog-3d-printing.jpg".to_string()),
            },
            BlogPost {
                id: 3,
                slug: "building-financial-tools-with-rust".to_string(),
                excerpt: "Why I chose Rust for building financial applications and calculators...".to_string(),
                title: "Building Financial Tools with Rust".to_string(),
                author: "Dylan Rayburn".to_string(),
                content: "When it comes to financial applications, precision, performance, and safety are paramount. This is why I chose Rust as my primary language for building financial tools and calculators.

Rust's memory safety guarantees, zero-cost abstractions, and excellent performance characteristics make it an ideal choice for financial software. In this post, I'll share my experience building various financial tools with Rust, including compound interest calculators, portfolio analyzers, and risk assessment tools.".to_string(),
                published_at: "2024-08-10".to_string(),
                updated_at: Some("2024-08-12".to_string()),
                tags: vec!["Rust".to_string(), "Finance".to_string(), "Programming".to_string(), "Web Development".to_string()],
                cover_image: None,
            },
        ];

        // Set mock data immediately
        posts.set(mock_posts);
        
        // Try to fetch from Supabase in a separate future
        spawn(async move {
            match fetch_blog_posts().await {
                Ok(fetched_posts) => {
                    if !fetched_posts.is_empty() {
                        gloo::console::log!("Successfully fetched", fetched_posts.len(), "blog posts from Supabase");
                        posts.set(fetched_posts);
                    } else {
                        gloo::console::log!("No posts found in Supabase, keeping mock data");
                    }
                }
                Err(e) => {
                    gloo::console::warn!("Failed to fetch from Supabase, using mock data:", e.to_string());
                    // Keep the mock data we already set
                }
            }
        });
    });

    rsx! {
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            div { class: "max-w-6xl mx-auto p-6",
                // Hero section
                div { class: "text-center py-12 mb-12",
                    h1 { class: "text-5xl font-bold text-gray-900 dark:text-white mb-6",
                        "Blog"
                    }
                    p { class: "text-xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto leading-relaxed",
                        "Exploring the intersection of finance, technology, and innovation. Sharing insights from my journey as a developer, 3D printing enthusiast, and financial professional."
                    }
                }
                // Loading state
                if *loading.read() {
                    div { class: "flex justify-center py-20",
                        div { class: "flex flex-col items-center",
                            div { class: "animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-indigo-600 dark:border-indigo-400" }
                            p { class: "mt-4 text-gray-600 dark:text-gray-400",
                                "Loading blog posts..."
                            }
                        }
                    }
                }
                // Error state
                if let Some(err) = error.read().as_ref() {
                    div { class: "p-6 mb-8 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-800 dark:text-red-200 rounded-lg",
                        div { class: "flex items-center",
                            svg {
                                class: "w-5 h-5 mr-2",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                    clip_rule: "evenodd",
                                }
                            }
                            strong { "Error loading posts: " }
                            "{err}"
                        }
                    }
                }
                // Blog posts list
                if posts.read().is_empty() && !*loading.read() {
                    div { class: "text-center py-20",
                        div { class: "mb-4",
                            svg {
                                class: "mx-auto h-16 w-16 text-gray-400 dark:text-gray-600",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                                }
                            }
                        }
                        h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-2",
                            "No blog posts found"
                        }
                        p { class: "text-gray-500 dark:text-gray-400",
                            "Check back soon for new content!"
                        }
                    }
                } else {
                    div { class: "grid gap-8 md:grid-cols-2 lg:grid-cols-3",
                        for post in posts.read().iter() {
                            BlogPostCard { post: post.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// Individual blog post card component
#[component]
fn BlogPostCard(post: BlogPost) -> Element {
    rsx! {
        article { class: "bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 overflow-hidden group",
            // Cover image if available
            if let Some(image_url) = &post.cover_image {
                div { class: "relative overflow-hidden h-48",
                    img {
                        class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                        src: "{image_url}",
                        alt: "Cover image for {post.title}",
                    }
                    div { class: "absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-10 transition-opacity duration-300" }
                }
            } else {
                div { class: "h-48 bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center",
                    svg {
                        class: "w-16 h-16 text-white opacity-80",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z" }
                    }
                }
            }
            div { class: "p-6",
                // Post metadata
                div { class: "flex items-center justify-between mb-4",
                    div { class: "flex items-center text-sm text-gray-500 dark:text-gray-400",
                        svg {
                            class: "w-4 h-4 mr-1",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            path {
                                fill_rule: "evenodd",
                                d: "M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z",
                                clip_rule: "evenodd",
                            }
                        }
                    }
                    div { class: "text-sm text-gray-500 dark:text-gray-400", "By {post.author}" }
                }
                // Title
                h2 { class: "text-xl font-bold text-gray-900 dark:text-white mb-3 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors duration-200",
                    Link {
                        to: Route::BlogPostDetail {
                            slug: post.slug.clone(),
                        },
                        class: "hover:text-indigo-600 dark:hover:text-indigo-400",
                        "{post.title}"
                    }
                }
                // Excerpt
                p { class: "text-gray-600 dark:text-gray-300 mb-4 line-clamp-3",
                    "{post.content.chars().take(120).collect::<String>()}..."
                }
                // Tags
                if !post.tags.is_empty() {
                    div { class: "flex flex-wrap gap-2 mb-4",
                        for tag in &post.tags {
                            span { class: "px-2 py-1 text-xs font-medium bg-indigo-100 dark:bg-indigo-900 text-indigo-800 dark:text-indigo-200 rounded-full",
                                "{tag}"
                            }
                        }
                    }
                }
                // Read more link
                div { class: "flex items-center justify-between",
                    Link {
                        to: Route::BlogPostDetail {
                            slug: post.slug.clone(),
                        },
                        class: "inline-flex items-center text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300 font-medium group/link",
                        span { "Read more" }
                        svg {
                            class: "w-4 h-4 ml-1 group-hover/link:translate-x-1 transition-transform duration-200",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            path {
                                fill_rule: "evenodd",
                                d: "M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z",
                                clip_rule: "evenodd",
                            }
                        }
                    }
                    div { class: "text-sm text-gray-500 dark:text-gray-400",
                        "{post.content.chars().count() / 250} min read"
                    }
                }
            }
        }
    }
}

