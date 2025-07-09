use dioxus::prelude::*;

use crate::{
    components::{auth::Auth},
    views::routes::GuardContext,
};
use crate::api::auth::signin_with_google;
use crate::components::button::Button;

#[component]
pub fn Login() -> Element {
    let mut is_loading = use_signal(|| false);

    let google_login = move |_| {
        is_loading.set(true);
        spawn(async move {
            client! {
                signin_with_google().await;
                // Google OAuth will handle the redirect, so we don't need error handling here
                // The loading state will be reset when the page redirects
            }
        });
    };

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-50 to-gray-100 py-12 px-4 sm:px-6 lg:px-8",
            div { class: "max-w-md w-full space-y-8",
                div { class: "text-center",
                    img {
                        class: "mx-auto h-16 w-16 rounded-full shadow-lg",
                        src: "/assets/Headshot_Rayburn.PNG",
                        alt: "Dylan Rayburn",
                    }
                    h2 { class: "mt-6 text-center text-3xl font-extrabold text-gray-900",
                        "Welcome Back"
                    }
                    p { class: "mt-2 text-center text-sm text-gray-600",
                        "Sign in to access Dylan Rayburn's Portfolio"
                    }
                }
                div { class: "mt-8 bg-white py-8 px-6 shadow-xl rounded-lg border border-gray-200",
                    Auth {
                        on_success: move |_user| {
                            GuardContext::redirect_next_or_home();
                        },
                    }
                    div { class: "mt-6",
                        div { class: "relative",
                            div { class: "absolute inset-0 flex items-center",
                                div { class: "w-full border-t border-gray-300" }
                            }
                            div { class: "relative flex justify-center text-sm",
                                span { class: "px-2 bg-white text-gray-500 font-medium",
                                    "Or continue with"
                                }
                            }
                        }
                    }
                    div { class: "mt-6",
                        Button {
                            text: if is_loading() { "Signing in..." } else { "Continue with Google" },
                            on_click: google_login,
                            disabled: is_loading(),
                            class: Some(
                                "w-full flex justify-center items-center py-3 px-4 border border-gray-300 rounded-md shadow-sm bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out disabled:opacity-50 disabled:cursor-not-allowed"
                                    .to_string(),
                            ),
                        }
                    }
                    div { class: "mt-6 text-center",
                        p { class: "text-xs text-gray-500",
                            "By signing in, you agree to our terms of service and privacy policy."
                        }
                    }
                }
            }
        }
    }
}
