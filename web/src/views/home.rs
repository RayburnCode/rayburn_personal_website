use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { 
            class: "min-h-screen bg-gray-50 flex flex-col",
            
            // Hero Section
            div { 
                class: "flex-1 flex flex-col justify-center items-center text-center px-6 py-20 sm:py-24 lg:py-32",
                
                // Name/Title
                div { 
                    class: "max-w-3xl space-y-6",
                    h1 { 
                        class: "text-4xl sm:text-5xl lg:text-6xl font-light text-gray-900 tracking-tight",
                        "Dylan Rayburn"
                    }
                    h2 { 
                        class: "text-xl sm:text-2xl text-gray-600 font-normal",
                        "Software Developer & Mortgage Professional"
                    }
                }

                img { 
    class: "w-32 h-32 rounded-full object-cover mt-8 border-2 border-white shadow-sm",
    src: "/path/to/headshot.jpg",
    alt: "Dylan Rayburn"
}
                
                // Divider
                div { 
                    class: "my-8 w-16 h-px bg-gray-300"
                }
                
                // Short Bio
                p { 
                    class: "max-w-2xl text-lg text-gray-600 leading-relaxed",
                    "I build solutions at the intersection of technology and finance. 
                    With expertise in both software development and mortgage lending, 
                    I bridge technical and business domains to create impactful results."
                }
                
// Add this below the bio section
div { 
    class: "mt-8 flex flex-wrap justify-center gap-2 max-w-lg",
    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm", "Rust" }
    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm", "Web Development" }
    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm", "Mortgage Lending" }
    span { class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm", "Financial Technology" }
}

                // Primary Actions
                div { 
                    class: "mt-12 flex flex-wrap justify-center gap-4",
                    Link {
                        to: Route::Projects {},
                        class: "px-6 py-3 bg-gray-900 text-white rounded-md font-medium hover:bg-gray-800 transition-colors shadow-sm",
                        "View My Work"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "px-6 py-3 border border-gray-300 text-gray-700 rounded-md font-medium hover:bg-gray-100 transition-colors",
                        "Get In Touch"
                    }
                }
            }
            
            // Footer with social links
            div { 
                class: "py-8 border-t border-gray-200",
                div { 
                    class: "flex justify-center gap-6",
                    a {
                        href: "https://github.com/RayburnCode",
                        target: "_blank",
                        class: "text-gray-400 hover:text-gray-600 transition-colors",
                        "GitHub"
                    }
                    a {
                        href: "https://www.linkedin.com/in/dylan-rayburn-a6b93499/",
                        target: "_blank",
                        class: "text-gray-400 hover:text-gray-600 transition-colors",
                        "LinkedIn"
                    }
                }
            }
        }
    }
}