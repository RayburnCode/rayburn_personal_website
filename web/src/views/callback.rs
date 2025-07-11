#[allow(unused_imports)]
use crate::api::auth::set_session;
#[allow(unused_imports)]
use crate::views::Protected;
use dioxus::prelude::*;
use crate::Route;
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[component]
pub fn Callback() -> Element {
    spawn(async move {
        client! {
            #[cfg(target_arch = "wasm32")]
            {
                let hash = window().unwrap().location().hash().unwrap();
                let params_parsed: HashMap<String, String> = serde_urlencoded::from_str(hash
                    .strip_prefix("#").unwrap()).unwrap();
                
                if let (Some(access_token), Some(refresh_token)) = (params_parsed.get("access_token"), 
                    params_parsed.get("refresh_token")) {
                    set_session(
                        access_token.to_owned(),
                        refresh_token.to_owned(),
                    ).await;
                    let nav = navigator();
                    nav.replace(Route::Protected {});
                }
            }
        }
    });
    rsx! {}
}