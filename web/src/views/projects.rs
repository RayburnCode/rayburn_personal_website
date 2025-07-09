use dioxus::prelude::*;
use crate::api::projects::{get_projects, Project};





#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(|| Vec::<Project>::new());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| Option::<String>::None);

    // Load projects when component mounts
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match get_projects().await {
                Ok(fetched_projects) => {
                    projects.set(fetched_projects);
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to load projects: {}", e)));
                    loading.set(false);
                }
            }
        });
    });

    // Separate projects by category
    let web_projects = projects().into_iter().filter(|p| p.category == "web").collect::<Vec<_>>();
    let printing_projects = projects().into_iter().filter(|p| p.category == "3d-printing").collect::<Vec<_>>();

    rsx! {
        div { class: "max-w-6xl mx-auto p-6",
            h1 { class: "text-3xl font-bold mb-8", "My Projects" }
            // Introduction
            p { class: "mb-8 text-lg",
                "Here's a collection of my work spanning web development, financial tools, and 3D printing projects. Each represents my passion for combining technology with practical solutions."
            }

            // Loading state
            if loading() {
                div { class: "flex justify-center items-center py-12",
                    div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600" }
                    span { class: "ml-4 text-lg", "Loading projects..." }
                }
            }

            // Error state
            if let Some(error_msg) = error() {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-8",
                    span { class: "font-bold", "Error: " }
                    span { "{error_msg}" }
                }
            }

            // Web Development Projects
            if !loading() && !web_projects.is_empty() {
                section { class: "mb-12",
                    h2 { class: "text-2xl font-bold mb-6 pb-2 border-b border-gray-300 dark:border-gray-600",
                        "Web Development"
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        for project in web_projects.iter() {
                            ProjectCard { project: project.clone() }
                        }
                    }
                }
            }

            // 3D Printing Projects
            if !loading() && !printing_projects.is_empty() {
                section { class: "mb-12",
                    h2 { class: "text-2xl font-bold mb-6 pb-2 border-b border-gray-300 dark:border-gray-600",
                        "3D Printing & CAD Designs"
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for project in printing_projects.iter() {
                            ProjectCard3D { project: project.clone() }
                        }
                    }
                }
            }

            // Fallback content if no projects loaded but no error
            if !loading() && projects().is_empty() && error().is_none() {
                div { class: "text-center py-12",
                    h3 { class: "text-xl font-semibold mb-4", "No projects found" }
                    p { class: "text-gray-600",
                        "Projects will appear here once they're added to the database."
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        div { class: "border border-gray-200 dark:border-gray-700 rounded-lg p-6 hover:shadow-lg transition-shadow",
            h3 { class: "text-xl font-semibold mb-2", "{project.title}" }
            p { class: "text-gray-600 dark:text-gray-400 mb-4", "{project.description}" }
            div { class: "flex flex-wrap gap-2 mb-4",
                for tech in project.technologies.iter() {
                    span { class: "px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm",
                        "{tech}"
                    }
                }
            }
            if let Some(github_url) = &project.github_url {
                a {
                    href: "{github_url}",
                    target: "_blank",
                    class: "text-blue-600 dark:text-blue-400 hover:underline flex items-center",
                    svg {
                        class: "w-5 h-5 mr-1",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" }
                    }
                    "View on GitHub"
                }
            }
            if let Some(demo_url) = &project.demo_url {
                a {
                    href: "{demo_url}",
                    target: "_blank",
                    class: "text-green-600 dark:text-green-400 hover:underline ml-4",
                    "Live Demo"
                }
            }
        }
    }
}

#[component]
fn ProjectCard3D(project: Project) -> Element {
    
    rsx! {
        div { class: "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden hover:shadow-lg transition-shadow",
            img {
                class: "w-full h-48 object-cover",
                src: "{project.image_url.as_deref().unwrap_or(\"/placeholder.png\")}",
                alt: "{project.title}",
            }
            div { class: "p-6",
                h3 { class: "text-xl font-semibold mb-2", "{project.title}" }
                p { class: "text-gray-600 dark:text-gray-400 mb-4", "{project.description}" }
                div { class: "flex flex-wrap gap-2 mb-4",
                    for tech in project.technologies.iter() {
                        span { class: "px-2 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded text-sm",
                            "{tech}"
                        }
                    }
                }
                if let Some(github_url) = &project.github_url {
                    a {
                        href: "{github_url}",
                        target: "_blank",
                        class: "text-blue-600 dark:text-blue-400 hover:underline",
                        "View Design Files"
                    }
                }
                if let Some(demo_url) = &project.demo_url {
                    a {
                        href: "{demo_url}",
                        target: "_blank",
                        class: "text-blue-600 dark:text-blue-400 hover:underline ml-4",
                        "Download STL"
                    }
                }
            }
        }
    }
}