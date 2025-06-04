use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        p { "Welcome to the Contacts page!" }
        p { "This is where you can contact me." }

        h1 { "Get in Touch" }
        form {
            label { "Name" }
            input { r#type: "text", name: "name" }
            label { "Email" }
            input { r#type: "email", name: "email" }
            label { "Message" }
            textarea { name: "message" }
            button { r#type: "submit", "Send" }
        }
        a { href: "https://github.com/you", "GitHub" }
    }
    }

