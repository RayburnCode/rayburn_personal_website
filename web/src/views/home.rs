use crate::components::{Echo, Hero};
use dioxus::prelude::*;
use gloo_timers::future::sleep;




#[component]
pub fn Home() -> Element {
    // State for typewriter effect
    let current_text = use_signal(|| String::from(""));
    let current_index = use_signal(|| 0);
    let full_text = "Hi, I'm [Your Name]\nBuilding digital experiences";
    let is_deleting = use_signal(|| false);
    
    // Animated background state
    let bg_pos = use_signal(|| (0, 0));
    
    {
        let current_text = current_text.clone();
        let current_index = current_index.clone();
        let is_deleting = is_deleting.clone();
        use_future(move || {
            let current_text = current_text.clone();
            let current_index = current_index.clone();
            let is_deleting = is_deleting.clone();
            async move {
                sleep(std::time::Duration::from_millis(100)).await;
    
                let idx = *current_index.read();
                let mut is_del = is_deleting.write();
                if *is_del {
                    if idx > 0 {
                        current_text.set(full_text[..idx-1].to_string());
                        current_index.set(idx - 1);
                    } else {
                        *is_del = false;
                    }
                } else {
                    if idx < full_text.len() {
                        current_text.set(full_text[..idx+1].to_string());
                        current_index.set(idx + 1);
                    } else {
                        sleep(std::time::Duration::from_secs(2)).await;
                        *is_del = true;
                    }
                }
            }
        });
    }

    // Animate background
    {
        let bg_pos = bg_pos.clone();
        use_future(move || {
            let bg_pos = bg_pos.clone();
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
                        "Developer · Designer · Creator"
                    }
                    // Action buttons
                    div { class: "flex flex-wrap justify-center gap-4",
                        p { "Add something here " }
                    }
                }
                // Social links (bottom of screen)
                div { class: "absolute bottom-8 flex gap-6",
                    a {
                        href: "https://github.com/you",
                        target: "_blank",
                        class: "text-white hover:text-indigo-200 transition-colors",
                        "GitHub"
                    }
                    a {
                        href: "https://linkedin.com/in/you",
                        target: "_blank",
                        class: "text-white hover:text-indigo-200 transition-colors",
                        "LinkedIn"
                    }
                    a {
                        href: "https://twitter.com/you",
                        target: "_blank",
                        class: "text-white hover:text-indigo-200 transition-colors",
                        "Twitter"
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