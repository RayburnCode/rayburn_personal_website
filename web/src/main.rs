// src/main.rs
use dioxus::prelude::*;
use views::{AppLayout, About, Blog, Contact, Home, Projects, Resume, BlogPostDetail, Protected, Callback, Login};

mod components;
mod views;
mod api;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        #[route("/blog")]
        Blog {},
        
        #[route("/blog/:slug")]
        BlogPostDetail { slug: String },

        #[route("/about")]
        About {},

        #[route("/contact")]
        Contact {},

        #[route("/projects")]
        Projects {},

        #[route("/resume")]
        Resume {},

        #[route("/protected")]
        Protected {},
        #[route("/login")]
        Login {},


        #[route("/callback")]
        Callback {},
}



