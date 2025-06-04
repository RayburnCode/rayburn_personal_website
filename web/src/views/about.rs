use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        p { "Welcome to the About page!" }
        p { "This is where you can learn about me." }
    }
    }

