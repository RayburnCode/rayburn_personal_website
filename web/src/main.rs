
use dioxus::prelude::*;

use views::{About, Blog, Contact, Home, Navbar, Projects, Resume};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },

        #[route("/about")]
        About {},

        #[route("/contact")]
        Contact {},

        #[route("/projects")]
        Projects {},

        #[route("/resume")]
        Resume {},

}

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
