use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Projects() -> Element {
    rsx! {
        p { "Welcome to the Projects page!" }
        p { "This is where you can find all the projects." }
    }
}
