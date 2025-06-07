use crate::Route;
use dioxus::prelude::*;
 
use crate::components::button::{Button, ButtonScheme, ButtonSize};
// const LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub fn Navbar(children: Element) -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    rsx! {
        nav { id: "navbar", class: "w-full  text-white shadow-md",
            div { class: "px-8 py-2 mx-auto flex items-center justify-between",

                // // Left side: Logo
                // a { href: "https://dylanrayburn.com.com",
                //     img { class: "h-15", src: LOGO, alt: "Logo header image" }
                // }


                // Right side: Links and Button
                div { class: "flex gap-6 items-center text-sm text-gray-900",
                    Link {
                        to: Route::Home {},
                        class: "hover:text-blue-400 transition",
                        "Home"
                    }
                    Link {
                        to: Route::About {},
                        class: "hover:text-blue-400 transition",
                        "About"
                    }
                    Link {
                        to: Route::Blog { id: (1) },
                        class: "hover:text-blue-400 transition",
                        "Blog"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "hover:text-blue-400 transition",
                        "Contact"
                    }
                    Link {
                        to: Route::Projects {},
                        class: "hover:text-blue-400 transition",
                        "Projects"
                    }
                    Link {
                        to: Route::Resume {},
                        class: "hover:text-blue-400 transition",
                        "Resume"
                    }
                    Button {
                        button_scheme: ButtonScheme::Custom,
                        button_size: ButtonSize::Large,
                        on_click: handle_click,
                        "Learn More"
                    }
                }
            }
        }
    }
}