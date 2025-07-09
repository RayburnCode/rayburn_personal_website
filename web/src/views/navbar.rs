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
        nav { class: "sticky  top-0 z-50 w-full bg-white/80 backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4  sm:px-6 lg:px-8",
                div { class: "flex  h-16 items-center justify-between",


                    // Center navigation
                    div { class: "hidden md:flex items-center space-x-8",
                        Link {
                            to: Route::Home {},
                            class: active_class(
                                &Route::Home {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Home"
                        }
                        Link {
                            to: Route::About {},
                            class: active_class(
                                &Route::About {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "About"
                        }
                        Link {
                            to: Route::Projects {},
                            class: active_class(
                                &Route::Projects {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Projects"
                        }
                        Link {
                            to: Route::Blog {},
                            class: if matches!(current_route, Route::Blog { .. }) { "text-blue-600 font-medium border-b-2 border-blue-600 px-1 py-2 text-sm transition-colors" } else { "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors" },
                            "Blog"
                        }
                        Link {
                            to: Route::Resume {},
                            class: active_class(
                                &Route::Resume {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Resume"
                        }

                        Link {
                            to: Route::Login {},
                            class: active_class(
                                &Route::Login {},
                                &current_route,
                                "text-gray-600 hover:text-blue-600 px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Login"
                        }
                    }

                    // Right side (CTA/resume)

                    Link {
                        to: Route::Contact {},
                        class: if matches!(current_route, Route::Contact {}) { "ml-4 rounded-md bg-BurntSienna-Darker px-4 py-2 text-sm font-medium text-white shadow focus:outline-none transition-colors" } else { "ml-4 rounded-md bg-BurntSienna px-4 py-2 text-sm font-medium text-white shadow hover:bg-BurntSienna-Darker focus:outline-none transition-colors" },
                        "Contact Me"
                    }
                }
            }
        }
    }
    }




  