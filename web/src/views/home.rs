use dioxus::prelude::*;
use crate::Route;

use crate::{test_supabase_connection};
use crate::api::projects::get_projects;

const HEADSHOT: Asset = asset!("/assets/Headshot_Rayburn.png");

#[component]
pub fn Home() -> Element {
    let mut connection_status = use_signal(|| String::new());
    let mut is_testing = use_signal(|| false);
    let mut projects_data = use_signal(|| String::new());
    let mut is_fetching_projects = use_signal(|| false);
    
    let test_connection = move |_: Event<MouseData>| {
        spawn(async move {
            is_testing.set(true);
            connection_status.set("Testing connection...".to_string());
            
            match test_supabase_connection().await {
                Ok(result) => connection_status.set(result),
                Err(e) => connection_status.set(format!("❌ Error: {}", e)),
            }
            
            is_testing.set(false);
        });
    };

    let fetch_projects = move |_: Event<MouseData>| {
        spawn(async move {
            is_fetching_projects.set(true);
            projects_data.set("Fetching projects...".to_string());
            
            match get_projects().await {
                Ok(result) => {
                    // Pretty format the JSON response
                    projects_data.set(format!("✅ Projects fetched successfully!\n\n{:?}", result));
                },
                Err(e) => projects_data.set(format!("❌ Error fetching projects: {}", e)),
            }
            
            is_fetching_projects.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen  flex flex-col",
            // Hero Section
            div { class: "flex-1 flex flex-col justify-center items-center text-center px-6 py-10 sm:py-24 lg:py-20",
                // Name/Title
                div { class: "max-w-3xl space-y-6",
                    h1 { class: "text-4xl sm:text-5xl lg:text-6xl font-light  tracking-tight",
                        "Dylan Rayburn"
                    }
                    h2 { class: "text-xl sm:text-2xl  font-normal",
                        "Software Developer & Mortgage Professional"
                    }
                }

                img {
                    class: "w-32 h-32 rounded-full object-cover mt-8 border-2 border-white shadow-sm",
                    src: HEADSHOT,
                    alt: "Dylan Rayburn",
                }
                // Divider
                div { class: "my-6 w-16 h-px bg-gray-300" }
                // Short Bio
                p { class: "max-w-2xl text-lg  leading-relaxed",
                    "I build solutions at the intersection of technology and finance. 
                    With expertise in both software development and mortgage lending, 
                    I bridge technical and business domains to create impactful results."
                }
                // Add this below the bio section
                div { class: "mt-8 flex flex-wrap justify-center gap-2 max-w-lg",
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Rust"
                    }
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Web Development"
                    }
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Mortgage Lending"
                    }
                    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                        "Financial Technology"
                    }
                }

                // Primary Actions
                div { class: "mt-12 flex flex-wrap justify-center gap-4",
                    Link {
                        to: Route::Projects {},
                        class: "px-6 py-3 bg-gray-900 text-white rounded-md font-medium hover:bg-gray-800 transition-colors shadow-sm",
                        "View My Work"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "px-6 py-3 border border-gray-300 text-CustomAccent bg-CustomNav rounded-md font-medium hover:bg-gray-100 transition-colors",
                        "Get In Touch"
                    }
                }
                // Supabase Connection Test (Development feature)
                div { class: "mt-8 p-4 bg-gray-50 rounded-lg border border-gray-200 max-w-lg mx-auto",
                    h3 { class: "text-sm font-medium text-gray-700 mb-3", "Database Connection Test" }
                    // Connection test button
                    div { class: "flex gap-2 mb-3",
                        button {
                            onclick: test_connection,
                            disabled: is_testing(),
                            class: if is_testing() { "px-4 py-2 bg-gray-400 text-white rounded-md text-sm cursor-not-allowed" } else { "px-4 py-2 bg-blue-600 text-white rounded-md text-sm hover:bg-blue-700 transition-colors" },
                            if is_testing() {
                                "Testing..."
                            } else {
                                "Test Connection"
                            }
                        }
                        button {
                            onclick: fetch_projects,
                            disabled: is_fetching_projects(),
                            class: if is_fetching_projects() { "px-4 py-2 bg-gray-400 text-white rounded-md text-sm cursor-not-allowed" } else { "px-4 py-2 bg-green-600 text-white rounded-md text-sm hover:bg-green-700 transition-colors" },
                            if is_fetching_projects() {
                                "Fetching..."
                            } else {
                                "Get Projects"
                            }
                        }
                    }
                    // Connection test results
                    if !connection_status().is_empty() {
                        div { class: "mb-3",
                            h4 { class: "text-xs font-medium text-gray-600 mb-1", "Connection Test:" }
                            div { class: "p-3 bg-white rounded border text-sm font-mono text-xs break-words whitespace-pre-wrap",
                                "{connection_status()}"
                            }
                        }
                    }
                    // Projects data results
                    if !projects_data().is_empty() {
                        div {
                            h4 { class: "text-xs font-medium text-gray-600 mb-1", "Projects Data:" }
                            div { class: "p-3 bg-white rounded border text-sm font-mono text-xs break-words whitespace-pre-wrap max-h-64 overflow-y-auto",
                                "{projects_data()}"
                            }
                        }
                    }
                }
            }
        }
    }
}