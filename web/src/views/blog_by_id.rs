use dioxus::prelude::*;
use crate::api::{get_blog_with_slug, BlogPost};
use crate::Route;

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
        
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                match get_blog_with_slug(slug).await {
                    Ok(blog_post) => {
                        post.set(Some(blog_post));
                    }
                    Err(err) => {
                        error.set(Some(format!("Failed to load blog post: {}", err)));
                    }
                }
                
                loading.set(false);
            });
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For non-WASM builds, just set an error message
            error.set(Some("Blog detail view not available in non-WASM builds".to_string()));
            loading.set(false);
        }
    });

    rsx! {
        div { class: "max-w-6xl mx-auto py-8",
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
                        class: "inline-flex items-center text-CustomHover dark:text-blue-400 hover:underline mb-8",
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
                        h1 { class: "text-3xl md:text-4xl font-bold text-CustomAccent dark:text-white mb-4",
                            "{post.title}"
                        }
                        // Meta
                        div { class: "flex items-center text-sm text-gray-300 dark:text-gray-400",
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
                            div { class: "flex space-x-4" }
                            // Back to top
                            button {
                                onclick: move |_| {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            window.scroll_to_with_x_and_y(0.0, 0.0);
                                        }
                                    }
                                },
                                class: "text-CustomHover cursor-pointer dark:text-blue-400 hover:underline flex items-center",
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

