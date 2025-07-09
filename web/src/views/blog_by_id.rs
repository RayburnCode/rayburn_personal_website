use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use crate::Route;
use crate::views::Blog;


// 1. Define your blog post structure
#[derive(Clone, Debug, Serialize, Deserialize)]
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

// 2. Blog Post Detail Component
#[component]
pub fn BlogPostDetail(slug: String) -> Element {
    let post = use_signal::<Option<BlogPost>>(|| None);
    let loading = use_signal(|| true);
    let error = use_signal::<Option<String>>(|| None);

    // Fetch post from Supabase when component mounts
    use_effect(move || {
        let mut post = post.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        let slug = slug.clone();
        wasm_bindgen_futures::spawn_local(async move {
            loading.set(true);
            error.set(None);

            // Simulate API call - replace with actual Supabase fetch
            // Example real implementation:
            /*
            let client = supabase_rs::new(SUPABASE_URL, SUPABASE_KEY);
            let response = client.from("posts")
                .select("*")
                .eq("slug", slug)
                .single()
                .execute()
                .await;

            match response {
                Ok(post) => {
                    post.set(Some(post));
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to load post: {}", e)));
                    loading.set(false);
                }
            }
            */

            // Mock data for demonstration

            let mock_post = BlogPost {
                id: 1,
                slug: slug.clone(),
                title: "Combining Finance and Technology".to_string(),
                content: markdown_content(),
                excerpt: "How I bridge my financial expertise with my passion for coding...".to_string(),
                published_at: "2023-10-15".to_string(),
                updated_at: Some("2023-10-20".to_string()),
                tags: vec!["Finance".to_string(), "Tech".to_string(), "Career".to_string()],
                cover_image: Some("/images/blog-finance-tech.jpg".to_string()),
                author: "Dylan".to_string(),
            };

            post.set(Some(mock_post));
            loading.set(false);
        });
    });

    rsx! {
        div { class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
            // Loading state
            if *loading.read() {
                div { class: "flex justify-center py-12",
                    div {
                        class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500",
                        aria_label: "Loading...",
                    }
                }
            }
            // Error state
            if let Some(err) = error.read().as_ref() {
                div {
                    class: "bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-6",
                    role: "alert",
                    p { class: "font-bold", "Error" }
                    p { "{err}" }
                }
            }
            // Blog post content
            if let Some(post) = post.read().as_ref() {
                article {
                    // Back button
                    Link {
                        to: Route::Blog {},
                        class: "inline-flex items-center text-blue-600 dark:text-blue-400 hover:underline mb-8",
                        svg {
                            class: "w-5 h-5 mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M10 19l-7-7m0 0l7-7m-7 7h18",
                            }
                        }
                        "Back to Blog"
                    }
                    // Header
                    header { class: "mb-8",
                        // Tags
                        div { class: "flex flex-wrap gap-2 mb-4",
                            for tag in &post.tags {
                                span { class: "px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full text-sm font-medium",
                                    "{tag}"
                                }
                            }
                        }
                        // Title
                        h1 { class: "text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4",
                            "{post.title}"
                        }
                        // Meta
                        div { class: "flex items-center text-sm text-gray-500 dark:text-gray-400",
                            // Author
                            span { class: "flex items-center mr-6",
                                svg {
                                    class: "w-4 h-4 mr-1",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
                                    }
                                }
                                "{post.author}"
                            }
                            // Date
                            span { class: "flex items-center",
                                svg {
                                    class: "w-4 h-4 mr-1",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
                                    }
                                }
                                "{post.published_at}"
                                if let Some(updated) = &post.updated_at {
                                    span { class: "ml-2 italic", "(updated {updated})" }
                                }
                            }
                        }
                    }
                    // Cover Image
                    if let Some(image_url) = &post.cover_image {
                        div { class: "mb-8 rounded-lg overflow-hidden",
                            img {
                                class: "w-full h-auto object-cover",
                                src: "{image_url}",
                                alt: "Cover image for {post.title}",
                            }
                        }
                    }
                    // Content
                    div { class: "prose dark:prose-invert prose-lg max-w-none",
                        // In a real app, you would use a markdown renderer here
                        // For this example, we'll just render paragraphs
                        for paragraph in post.content.split("\n\n") {
                            p { class: "mb-6", "{paragraph}" }
                        }
                    }
                    // Footer
                    footer { class: "mt-12 pt-8 border-t border-gray-200 dark:border-gray-700",
                        div { class: "flex justify-between items-center",
                            // Share buttons
                            div { class: "flex space-x-4",
                                p { class: "text-sm text-gray-500 dark:text-gray-400 mr-2",
                                    "Share:"
                                }
                                a {
                                    href: "#",
                                    class: "text-gray-500 hover:text-gray-700 dark:hover:text-gray-300",
                                    "Twitter"
                                }
                                a {
                                    href: "#",
                                    class: "text-gray-500 hover:text-gray-700 dark:hover:text-gray-300",
                                    "LinkedIn"
                                }
                            }
                            // Back to top
                            button {
                                onclick: move |_| {
                                    if let Some(window) = web_sys::window() {
                                        window.scroll_to_with_x_and_y(0.0, 0.0);
                                    }
                                },
                                class: "text-blue-600 dark:text-blue-400 hover:underline flex items-center",
                                svg {
                                    class: "w-4 h-4 mr-1",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M5 10l7-7m0 0l7 7m-7-7v18",
                                    }
                                }
                                "Back to top"
                            }
                        }
                    }
                }
            }
        }
    }
}

// Helper function for mock markdown content
fn markdown_content() -> String {
    r#"
## Bridging Two Worlds

For the past six years, I've worked in the mortgage and finance industry while simultaneously cultivating my passion for technology. This unique combination has given me a perspective that few possess.

### Financial Expertise Meets Technical Skills

My experience in credit analysis has taught me to:
- Analyze complex financial statements
- Assess risk factors
- Understand regulatory requirements

Meanwhile, my coding skills allow me to:
- Automate repetitive tasks
- Build custom financial tools
- Visualize data more effectively

### Practical Applications

Here are some examples of how I've combined these skills:

1. **Automated Spreadsheet Analysis**  
   Created Python scripts that parse Excel financials and flag potential issues.

2. **Custom Mortgage Calculators**  
   Built web-based calculators that help clients understand different scenarios.

3. **Document Processing Pipeline**  
   Developed a system that extracts key data from loan documents using OCR.

### The Future of FinTech

I believe the intersection of finance and technology will only grow more important. By understanding both domains, I'm positioned to:
- Help traditional financial institutions modernize
- Contribute to innovative FinTech solutions
- Bridge the communication gap between technical and financial teams

### Getting Started

If you're a finance professional looking to add technical skills (or vice versa), I recommend:
- Starting with basic Python or JavaScript
- Learning how APIs work in financial systems
- Exploring automation opportunities in your current workflow

"#.to_string()
}

// 3. Update your Route enum to include the blog post detail route
