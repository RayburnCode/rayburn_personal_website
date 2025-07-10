use dioxus::prelude::*;
const PHOTO: Asset = asset!("/assets/about_page_min.JPG");


#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto",
            div { class: "flex flex-col md:flex-row gap-8 mb-8",
                // Profile photo container (left side)
                div { class: "flex-shrink-0 mx-auto md:mx-0",
                    img {
                        class: "w-32 h-32 md:w-48 md:h-48 rounded-full object-cover shadow-lg",
                        src: PHOTO, // Update this path
                        alt: "Profile photo of Dylan",
                    }
                }
                // Text content (right side)
                div { class: "flex-1",
                    h1 { class: "text-3xl sm:text-4xl font-bold mb-6", "Who is Dylan?" }
                    p { class: "mb-6",
                        "Hello there! Welcome to my personal website! I'm Dylan, a finance professional with a passion for technology and making."
                    }
                    p { class: "mb-6",
                        "I fully believe in the famous quote from James Clear's Atomic Habits: "
                        span { class: "italic",
                            "\"If you can get 1% better each day for one year, you'll end up 37 times better by the time you're done.\""
                        }
                    }
                }
            }

            // Rest of your content (will wrap around the image on desktop)

            p { class: "mb-6",
                "With over 6 years of experience in the "
                span { class: "font-semibold", "Mortgage and Finance industry" }
                ", I've developed deep expertise in financial systems and client relationships. This professional foundation complements my growing skills in "
                span { class: "font-semibold ", "software development" }
                " and passion for "
                span { class: "font-semibold ", "3D printing and CAD design" }
                "."
            }
            p { class: "mb-6",
                "When I'm not analyzing financial data or working with clients, you'll find me:"
                ul { class: "list-disc pl-8 mt-2 space-y-2",
                    li {
                        "Writing code to automate financial processes or build web applications (like this one!)"
                    }
                    li { "Designing and printing 3D objects, constantly learning new CAD techniques" }
                    li {
                        "Exploring the intersection of finance and technology through personal projects"
                    }
                }
            }
            p { class: "mb-6",
                "This website showcases my journey at the crossroads of these interests - from financial tools I've developed to 3D printing projects I'm particularly proud of. Each project represents my commitment to continuous learning and applying technical solutions to real-world challenges."
            }
            p { class: "mb-6",
                "Through blog posts and project showcases, I aim to share knowledge that might inspire others to bridge their professional expertise with personal passions."
            }
            p { class: "mb-6",
                "Feel free to explore and don't hesitate to reach out if you have questions about mortgage/finance topics, coding projects, or 3D printing. I'm always excited to connect with fellow finance professionals, developers, and makers!"
            }
            p {
                "Thanks for stopping by, and I look forward to sharing this journey of continuous improvement with you!"
            }
        }
    }
}