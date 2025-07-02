// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::Route;
use crate::components::Footer;
use super::Navbar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: " flex-1 bg-CustomBackground font-display text-CustomAccent",
                div { class: "mx-auto px-8 py-6", Outlet::<Route> {} }
            }
            Footer {}
        }
    }
}