#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde_json;
#[cfg(target_arch = "wasm32")]
use gloo::dialogs::alert;

// Update the import path to match the actual location of signin_with_password and User.
// For example, if auth.rs is in src/components and api/auth.rs is in src/api, use super::super::api::auth
use crate::components::{
    button::Button,
    input::{PasswordInput, TextInput},
};
// Update this path as needed:
// Update the path below if your api module is in a different location
use crate::api::auth::{signin_with_password, User, Credentials};

#[component]
pub fn Auth(on_success: EventHandler<User>) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut remember_me = use_signal(|| false);

    let login = move |_| {
        spawn(async move {
            client! {
              let res = signin_with_password(Credentials { email: email.to_string(), password: password.to_string() }).await;

              if let Ok(_message) = res {
                email.set("".into());
                password.set("".into());
                // Create a dummy user for now since auth is not fully implemented
                let dummy_user = User {
                    id: "temp".to_string(),
                    aud: "".to_string(),
                    role: "".to_string(),
                    email: email.read().clone(),
                    email_confirmed_at: None,
                    phone: None,
                    confirmation_sent_at: None,
                    confirmed_at: None,
                    last_sign_in_at: None,
                    app_metadata: serde_json::Value::Null,
                    user_metadata: serde_json::Value::Null,
                    identities: vec![],
                    created_at: "".to_string(),
                    updated_at: "".to_string(),
                };
                on_success.call(dummy_user);
              } else {
                #[cfg(target_arch = "wasm32")]
                alert("failed to login user");
              }
            }
        });
    };

    rsx! {
        form {
            class: "space-y-6",
            onsubmit: move |event| {
                event.prevent_default();
                spawn(async move {
                    client! {
                        let res = signin_with_password(Credentials { email : email.to_string(),
                        password : password.to_string() }). await; if let Ok(_message) = res {
                        email.set("".into()); password.set("".into()); let dummy_user = User { id
                        : "temp".to_string(), aud : "".to_string(), role : "".to_string(), email
                        : email.read().clone(), email_confirmed_at : None, phone : None,
                        confirmation_sent_at : None, confirmed_at : None, last_sign_in_at : None,
                        app_metadata : serde_json::Value::Null, user_metadata :
                        serde_json::Value::Null, identities : vec![], created_at : ""
                        .to_string(), updated_at : "".to_string(), }; on_success
                        .call(dummy_user); } else { #[cfg(target_arch = "wasm32")]
                        alert("failed to login user"); }
                    }
                });
            },
            div {
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    r#for: "email",
                    "Email address"
                }
                TextInput {
                    i_value: email,
                    i_placeholder: Some("Enter your email".to_string()),
                    class: Some(
                        "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out"
                            .to_string(),
                    ),
                    on_input: move |event: FormEvent| {
                        email.set(event.value());
                    },
                }
            }
            div {
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    r#for: "password",
                    "Password"
                }
                PasswordInput {
                    i_value: password,
                    i_placeholder: Some("Enter your password".to_string()),
                    class: Some(
                        "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition duration-150 ease-in-out"
                            .to_string(),
                    ),
                    on_input: move |event: FormEvent| {
                        password.set(event.value());
                    },
                }
            }
            div { class: "flex items-center justify-between",
                div { class: "flex items-center",
                    input {
                        id: "remember-me",
                        name: "remember-me",
                        r#type: "checkbox",
                        class: "h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                        checked: remember_me(),
                        onchange: move |event| {
                            remember_me.set(event.checked());
                        },
                    }
                    label {
                        r#for: "remember-me",
                        class: "ml-2 block text-sm text-gray-900",
                        "Remember me"
                    }
                }
                div { class: "text-sm",
                    a {
                        href: "#",
                        class: "font-medium text-indigo-600 hover:text-indigo-500 transition duration-150 ease-in-out",
                        "Forgot your password?"
                    }
                }
            }
            div {
                Button {
                    text: "Sign in",
                    button_type: Some(crate::components::button::ButtonType::Submit),
                    class: Some(
                        "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out"
                            .to_string(),
                    ),
                    on_click: login,
                }
            }
        }
    }
}