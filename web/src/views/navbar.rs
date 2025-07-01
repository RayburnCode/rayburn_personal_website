use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_route;

#[component]
pub fn Navbar(children: Element) -> Element {
    let current_route = use_route::<Route>();

    // Helper function to determine active class
    fn active_class(route: &Route, current_route: &Route, class: &str) -> String {
        if route == current_route {
            format!("{} text-blue-600 font-medium border-b-2 border-blue-600", class)
        } else {
            class.to_string()
        }
    }

    rsx! {
        nav { 
            class: "sticky top-0 z-50 w-full bg-white/80 backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Left side (logo/name)
                    div { class: "flex items-center",
                        Link { 
                            to: Route::Home {},
                            class: "text-xl font-semibold text-gray-900 hover:text-blue-600",
                            "Your Name"
                        }
                    }

                    // Center navigation
                    div { class: "hidden md:flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: active_class(
                                &Route::Home {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors"
                            ),
                            "Home"
                        }
                        Link {
                            to: Route::About {},
                            class: active_class(
                                &Route::About {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors"
                            ),
                            "About"
                        }
                        Link {
                            to: Route::Projects {},
                            class: active_class(
                                &Route::Projects {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors"
                            ),
                            "Projects"
                        }
                        Link {
                            to: Route::BlogPostDetail { slug: "Blog Posts".to_string() },
                            class: if matches!(current_route, Route::BlogPostDetail { .. }) {
                                "text-blue-600 font-medium border-b-2 border-blue-600 px-1 py-2 text-sm transition-colors"
                            } else {
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors"
                            },
                            "Blog"
                        }
                    }

                    // Right side (CTA/resume)
                    div { class: "flex items-center space-x-4",
                        Link {
                            to: Route::Contact {},
                            class: active_class(
                                &Route::Contact {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors"
                            ),
                            "Contact"
                        }
                        Link {
                            to: Route::Resume {},
                            class: if matches!(current_route, Route::Resume {}) {
                                "ml-4 rounded-md bg-blue-700 px-4 py-2 text-sm font-medium text-white shadow focus:outline-none transition-colors"
                            } else {
                                "ml-4 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow hover:bg-blue-700 focus:outline-none transition-colors"
                            },
                            "Resume"
                        }
                    }
                }
            }
        }
    }
}