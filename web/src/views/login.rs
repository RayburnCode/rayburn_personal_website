use dioxus::prelude::*;

use crate::{
    components::{auth::Auth},
    views::routes::GuardContext,
};
use crate::api::auth::signin_with_google;
use crate::components::button::Button;

#[component]
pub fn Login() -> Element {
    let google_login = move |_| {
        spawn(async move {
            client! {
                signin_with_google().await;
            }
        });
    };

    rsx! {
        div { class: "max-w-lg mx-auto px-2 h-dvh place-content-center",
            h1 { class: "text-3xl", "SupaDioxus" }
            Auth {
                on_success: move |user| {
                    GuardContext::redirect_next_or_home();
                },
            }
            div { class: "mx-auto place-content-center mt-3",
                Button { text: "Sign In with Google", on_click: google_login }
            }
        }
    }
    }
