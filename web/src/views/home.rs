use dioxus::prelude::*;
use gloo_timers::future::sleep;
use crate::Route;


#[component]
pub fn Home() -> Element {
    // State for typewriter effect
    let current_text = "Hi, I'm [Your Name]";

    
    // Animated background state
    let bg_pos = use_signal(|| (0, 0));
    
    // Animate background
    {
        let  bg_pos = bg_pos.clone();
        use_future(move || {
            let mut bg_pos = bg_pos.clone();
            async move {
                let mut pos = 0;
                loop {
                    sleep(std::time::Duration::from_millis(50)).await;
                    pos = (pos + 1) % 360;
                    *bg_pos.write() = (pos, 0);
                }
            }
        });
    }

    rsx! {
        div { class: "relative h-screen w-full overflow-hidden",
            // Animated gradient background
            div {
                class: "absolute inset-0 z-0",
                style: format!(
                    "background: linear-gradient({}deg, #6366f1, #8b5cf6, #ec4899); background-size: 400% 400%; animation: gradient 15s ease infinite;",
                    bg_pos.read().0,
                ),
            }
            // Content container
            div { class: "relative z-10 h-full flex flex-col justify-center items-center text-white px-4",
                // Main content
                div { class: "max-w-3xl text-center",
                    // Typewriter text
                    h1 { class: "text-4xl md:text-6xl font-bold mb-6 min-h-[4rem] md:min-h-[6rem]",
                        "{current_text}"
                    }
                    // Subtitle
                    p { class: "text-xl md:text-2xl opacity-80 mb-12",
                        "Developer · Mortgage Pro · Problem Solver
"
                    }
                    // Action buttons
                    div { class: "flex flex-wrap justify-center gap-4",
                        Link {
                            to: Route::Projects {},
                            class: "px-8 py-3 bg-white text-indigo-600 rounded-full font-semibold hover:bg-opacity-90 transition-all transform hover:scale-105 shadow-lg",
                            "View My Work"
                        }
                        Link {
                            to: Route::About {},
                            class: "px-8 py-3 bg-transparent border-2 border-white text-white rounded-full font-semibold hover:bg-white hover:bg-opacity-10 transition-all transform hover:scale-105",
                            "About Me"
                        }
                    }
                }
                // Social links (bottom of screen)
                div { class: "absolute bottom-8 flex gap-6",
                    a {
                        href: "https://github.com/RayburnCode",
                        target: "_blank",
                        class: "text-white hover:text-indigo-200 transition-colors",
                        "GitHub"
                    }
                    a {
                        href: "https://www.linkedin.com/in/dylan-rayburn-a6b93499/",
                        target: "_blank",
                        class: "text-white hover:text-indigo-200 transition-colors",
                        "LinkedIn"
                    }
                }
                // Scroll indicator
                div { class: "absolute bottom-4 animate-bounce",
                    svg {
                        class: "w-6 h-6 text-white",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M19 14l-7 7m0 0l-7-7m7 7V3",
                        }
                    }
                }
            }
        }
    }
}