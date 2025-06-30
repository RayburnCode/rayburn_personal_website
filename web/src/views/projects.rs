use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto p-6 text-gray-800 dark:text-gray-200",
            h1 { class: "text-3xl font-bold mb-8", "My Projects" }
            
            // Introduction
            p { class: "mb-8 text-lg",
                "Here's a collection of my work spanning web development, financial tools, and 3D printing projects. Each represents my passion for combining technology with practical solutions."
            }
            
            // Web Development Projects
            section { class: "mb-12",
                h2 { class: "text-2xl font-bold mb-6 pb-2 border-b border-gray-300 dark:border-gray-600", 
                    "Web Development"
                }
                
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // Project Card 1
                    div { class: "border border-gray-200 dark:border-gray-700 rounded-lg p-6 hover:shadow-lg transition-shadow",
                        h3 { class: "text-xl font-semibold mb-2", "Mortgage Calculator" }
                        p { class: "text-gray-600 dark:text-gray-400 mb-4", 
                            "A responsive mortgage calculator built with Rust, Dioxus, and Tailwind CSS that helps users estimate monthly payments."
                        }
                        div { class: "flex flex-wrap gap-2 mb-4",
                            span { class: "px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm", "Rust" }
                            span { class: "px-2 py-1 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 rounded text-sm", "Dioxus" }
                            span { class: "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded text-sm", "Tailwind CSS" }
                        }
                        a { 
                            href: "https://github.com/yourusername/mortgage-calculator",
                            target: "_blank",
                            class: "text-blue-600 dark:text-blue-400 hover:underline flex items-center",
                            svg { 
                                class: "w-5 h-5 mr-1",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path { 
                                    d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" 
                                }
                            }
                            "View on GitHub"
                        }
                    }
                    
                    // Project Card 2
                    div { class: "border border-gray-200 dark:border-gray-700 rounded-lg p-6 hover:shadow-lg transition-shadow",
                        h3 { class: "text-xl font-semibold mb-2", "Loan Pipeline Tracker" }
                        p { class: "text-gray-600 dark:text-gray-400 mb-4", 
                            "A web application for mortgage professionals to track loan applications through different stages of the pipeline."
                        }
                        div { class: "flex flex-wrap gap-2 mb-4",
                            span { class: "px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm", "React" }
                            span { class: "px-2 py-1 bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded text-sm", "JavaScript" }
                            span { class: "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded text-sm", "Firebase" }
                        }
                        a { 
                            href: "https://github.com/yourusername/loan-pipeline",
                            target: "_blank",
                            class: "text-blue-600 dark:text-blue-400 hover:underline flex items-center",
                            svg { 
                                class: "w-5 h-5 mr-1",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path { 
                                    d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" 
                                }
                            }
                            "View on GitHub"
                        }
                    }
                }
            }
            
            // 3D Printing Projects
            section { class: "mb-12",
                h2 { class: "text-2xl font-bold mb-6 pb-2 border-b border-gray-300 dark:border-gray-600", 
                    "3D Printing & CAD Designs"
                }
                
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    // 3D Project 1
                    div { class: "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden hover:shadow-lg transition-shadow",
                        img { 
                            class: "w-full h-48 object-cover",
                            src: "/images/3d-print-1.jpg", 
                            alt: "Custom mortgage calculator stand" 
                        }
                        div { class: "p-6",
                            h3 { class: "text-xl font-semibold mb-2", "Mortgage Calculator Stand" }
                            p { class: "text-gray-600 dark:text-gray-400 mb-4", 
                                "Custom-designed stand for financial calculators with cable management and ergonomic angle."
                            }
                            div { class: "flex flex-wrap gap-2 mb-4",
                                span { class: "px-2 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded text-sm", "Fusion 360" }
                                span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm", "PLA" }
                            }
                            a { 
                                href: "https://www.thingiverse.com/yourdesign",
                                target: "_blank",
                                class: "text-blue-600 dark:text-blue-400 hover:underline",
                                "View on Thingiverse"
                            }
                        }
                    }
                    
                    // 3D Project 2
                    div { class: "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden hover:shadow-lg transition-shadow",
                        img { 
                            class: "w-full h-48 object-cover",
                            src: "/images/3d-print-2.jpg", 
                            alt: "Loan document organizer" 
                        }
                        div { class: "p-6",
                            h3 { class: "text-xl font-semibold mb-2", "Loan Document Organizer" }
                            p { class: "text-gray-600 dark:text-gray-400 mb-4", 
                                "Modular organizer for mortgage documents with labeled compartments for different loan stages."
                            }
                            div { class: "flex flex-wrap gap-2 mb-4",
                                span { class: "px-2 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded text-sm", "Tinkercad" }
                                span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm", "PETG" }
                            }
                            a { 
                                href: "https://www.printables.com/yourmodel",
                                target: "_blank",
                                class: "text-blue-600 dark:text-blue-400 hover:underline",
                                "View on Printables"
                            }
                        }
                    }
                    
                    // 3D Project 3
                    div { class: "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden hover:shadow-lg transition-shadow",
                        img { 
                            class: "w-full h-48 object-cover",
                            src: "/images/3d-print-3.jpg", 
                            alt: "Custom keyboard wrist rest" 
                        }
                        div { class: "p-6",
                            h3 { class: "text-xl font-semibold mb-2", "Ergonomic Keyboard Wrist Rest" }
                            p { class: "text-gray-600 dark:text-gray-400 mb-4", 
                                "Custom-designed wrist rest with adjustable tilt and embedded storage for small items."
                            }
                            div { class: "flex flex-wrap gap-2 mb-4",
                                span { class: "px-2 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded text-sm", "SolidWorks" }
                                span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm", "TPU" }
                            }
                            a { 
                                href: "#",
                                class: "text-blue-600 dark:text-blue-400 hover:underline",
                                "Download STL"
                            }
                        }
                    }
                }
            }
            
            // Open Source Contributions
            section {
                h2 { class: "text-2xl font-bold mb-6 pb-2 border-b border-gray-300 dark:border-gray-600", 
                    "Open Source Contributions"
                }
                
                ul { class: "space-y-4",
                    li { class: "border-b border-gray-200 dark:border-gray-700 pb-4",
                        a { 
                            href: "https://github.com/dioxuslabs/dioxus/pull/123",
                            target: "_blank",
                            class: "text-blue-600 dark:text-blue-400 hover:underline font-medium",
                            "Dioxus UI Framework: Added form validation component"
                        }
                        p { class: "text-gray-600 dark:text-gray-400", 
                            "Contributed a reusable form validation component to the Dioxus ecosystem."
                        }
                    }
                    li { class: "border-b border-gray-200 dark:border-gray-700 pb-4",
                        a { 
                            href: "https://github.com/tailwindlabs/tailwindcss/discussions/7890",
                            target: "_blank",
                            class: "text-blue-600 dark:text-blue-400 hover:underline font-medium",
                            "Tailwind CSS: Proposed financial component examples"
                        }
                        p { class: "text-gray-600 dark:text-gray-400", 
                            "Suggested and contributed examples of financial components for the Tailwind CSS docs."
                        }
                    }
                }
            }
        }
    }
}