use dioxus::prelude::*;
use crate::Route;



const HEADSHOT: Asset = asset!("/assets/Headshot_Rayburn.png");

#[component]
pub fn Home() -> Element {


    rsx! {
        div { class: "flex flex-col",
            // Hero Section
            div { class: "flex-1 flex flex-col justify-center items-center text-center py-6",
                // Name/Title
                div { class: "max-w-6xl mx-auto space-y-8",
                    h1 { class: "text-4xl sm:text-5xl lg:text-6xl font-light tracking-tight mb-4",
                        "Dylan Rayburn"
                    }
                    h2 { class: "text-xl sm:text-2xl font-normal mb-8",
                        "Software Developer & Mortgage Professional"
                    }
                }

                img {
                    class: "w-32 h-32 rounded-full object-cover mt-4 border-2 border-white shadow-sm",
                    src: HEADSHOT,
                    alt: "Dylan Rayburn",
                }
                // Divider
                div { class: "my-8 w-16 h-px bg-gray-300" }
                // Short Bio
                p { class: "max-w-2xl text-lg leading-relaxed mb-8",
                    "I build solutions at the intersection of technology and finance. 
                    With expertise in both software development and mortgage lending, 
                    I bridge technical and business domains to create impactful results."
                }
                // Add this below the bio section
                div { class: "mt-8 flex flex-wrap justify-center gap-2 max-w-lg mb-12",
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
                div { class: "flex flex-wrap justify-center gap-4",
                    Link {
                        to: Route::Projects {},
                        class: "px-6 py-3 bg-gray-900 border border-gray-300 text-white rounded-md font-medium hover:bg-gray-800 transition-colors shadow-sm",
                        "View My Work"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "px-6 py-3 border border-gray-300 text-CustomAccent bg-CustomNav rounded-md font-medium hover:bg-gray-100 transition-colors",
                        "Get In Touch"
                    }
                }
            }
        }
    }
}