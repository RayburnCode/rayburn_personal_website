use crate::api::auth::signout;
use crate::components::button::Button;
use crate::views::routes::{protected, GuardContext};
use dioxus::prelude::*;
use crate::Route;



#[component]
pub fn Protected() -> Element {
    protected(Route::Login {}, Route::Protected {});

    let logout = move |_| {
        spawn(async move {
            client! {
              let _ = signout().await;
                GuardContext::redirect_next_or_home();
            }
        });
    };

    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Protected SupaDioxus" }
            Button { text: "Signout", on_click: logout }
        }
    }
} 