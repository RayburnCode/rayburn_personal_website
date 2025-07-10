use dioxus::prelude::*;
use serde_json::json;
use crate::api::contact::{NewContactSubmission, submit_contact_form};

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);
    let mut submit_status = use_signal(|| Option::<String>::None);
    let mut status_type = use_signal(|| String::from("info")); // "success", "error", "info"

    rsx! {
        div { class: "max-w-6xl mx-auto dark:text-gray-200",
            h1 { class: "text-3xl sm:text-4xl font-bold mb-8", "Get in Touch" }
            p { class: "mb-8 text-lg",
                "Have questions about mortgages and finance? Want to discuss a coding project or 3D printing idea? I'd love to hear from you! Fill out the form below or connect with me directly through my GitHub."
            }

            // Contact Form
            form { 
                class: "space-y-6 mb-12",

                // Name Field
                div {
                    label { class: "block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300",
                        "Name *"
                    }
                    input {
                        class: "w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200",
                        r#type: "text",
                        name: "name",
                        placeholder: "Your full name",
                        value: name.read().clone(),
                        oninput: move |e| name.set(e.value()),
                        required: true,
                    }
                }

                // Email Field
                div {
                    label { class: "block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300",
                        "Email *"
                    }
                    input {
                        class: "w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200",
                        r#type: "email",
                        name: "email",
                        placeholder: "your.email@example.com",
                        value: email.read().clone(),
                        oninput: move |e| email.set(e.value()),
                        required: true,
                    }
                }

                // Message Field
                div {
                    label { class: "block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300",
                        "Message *"
                    }
                    textarea {
                        class: "w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 min-h-[120px] resize-y",
                        name: "message",
                        placeholder: "Tell us about your project, question, or how we can help you...",
                        value: message.read().clone(),
                        oninput: move |e| message.set(e.value()),
                        required: true,
                    }
                }

                // Submit Button
                button {
                    class: "px-6 py-3 bg-CustomAccent text-CustomBackground rounded-lg hover:bg-CustomHover cursor-pointer transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 min-w-[140px]",
                    r#type: "button",
                    disabled: *is_submitting.read(),
                    onclick: move |_| {
                        let name_val = name.read().clone();
                        let email_val = email.read().clone();
                        let message_val = message.read().clone();
                        
                        // Basic validation
                        if name_val.trim().is_empty() || email_val.trim().is_empty() || message_val.trim().is_empty() {
                            status_type.set("error".to_string());
                            submit_status.set(Some("❌ Please fill in all required fields.".to_string()));
                            return;
                        }
                        
                        // Basic email validation
                        if !email_val.contains('@') || !email_val.contains('.') {
                            status_type.set("error".to_string());
                            submit_status.set(Some("❌ Please enter a valid email address.".to_string()));
                            return;
                        }
                        
                        // Trigger the same logic as form submission
                        is_submitting.set(true);
                        submit_status.set(None);
                        
                        spawn(async move {
                            #[cfg(target_arch = "wasm32")]
                            let user_agent = {
                                web_sys::window().and_then(|w| w.navigator().user_agent().ok())
                            };
                            #[cfg(not(target_arch = "wasm32"))]
                            let user_agent = None;
                            
                            let submission = NewContactSubmission {
                                name: name_val,
                                email: email_val,
                                message: message_val,
                                ip_address: None,
                                user_agent,
                                subject: Some("Website Contact Form".to_string()),
                                metadata: Some(json!({
                                    "source": "website",
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                })),
                            };
                            
                            match submit_contact_form(submission).await {
                                Ok(_) => {
                                    status_type.set("success".to_string());
                                    submit_status.set(Some("🎉 Success! Your message has been sent successfully. We'll get back to you soon!".to_string()));
                                    name.set(String::new());
                                    email.set(String::new());
                                    message.set(String::new());
                                    
                                    // Auto-dismiss success message after 5 seconds
                                    let mut submit_status_clone = submit_status.clone();
                                    spawn(async move {
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            use gloo_timers::future::TimeoutFuture;
                                            TimeoutFuture::new(5000).await;
                                            submit_status_clone.set(None);
                                        }
                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                                            submit_status_clone.set(None);
                                        }
                                    });
                                }
                                Err(error) => {
                                    status_type.set("error".to_string());
                                    let error_msg = if error.contains("HTTP 400") {
                                        "❌ Please check your input and try again."
                                    } else if error.contains("HTTP 401") || error.contains("HTTP 403") {
                                        "🔐 Authentication error. Please try again later."
                                    } else if error.contains("HTTP 500") {
                                        "🔧 Server error. Please try again in a few moments."
                                    } else if error.contains("Failed to fetch") || error.contains("network") {
                                        "🌐 Network error. Please check your connection and try again."
                                    } else {
                                        "⚠️ An unexpected error occurred. Please try again."
                                    };
                                    submit_status.set(Some(format!("{} Error details: {}", error_msg, error)));
                                }
                            }
                            
                            is_submitting.set(false);
                        });
                    },
                    if *is_submitting.read() {
                        div { class: "animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent" }
                        "Sending..."
                    } else {
                        div { class: "flex items-center justify-center gap-2", "Send Message" }
                    }
                }

                // Status Message
                if let Some(status) = submit_status.read().as_ref() {
                    div {
                        class: match status_type.read().as_str() {
                            "success" => {
                                "mt-4 p-4 bg-green-50 dark:bg-green-900/30 text-green-800 dark:text-green-200 rounded-lg border border-green-200 dark:border-green-700 shadow-sm animate-pulse"
                            }
                            "error" => {
                                "mt-4 p-4 bg-red-50 dark:bg-red-900/30 text-red-800 dark:text-red-200 rounded-lg border border-red-200 dark:border-red-700 shadow-sm"
                            }
                            _ => {
                                "mt-4 p-4 bg-blue-50 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200 rounded-lg border border-blue-200 dark:border-blue-700 shadow-sm"
                            }
                        },
                        div { class: "flex items-start gap-3",
                            div { class: "flex-1",
                                div { class: "font-medium text-sm mb-1",
                                    match status_type.read().as_str() {
                                        "success" => "Message Sent Successfully!",
                                        "error" => "Submission Failed",
                                        _ => "Information",
                                    }
                                }
                                div { class: "text-sm leading-relaxed", "{status}" }
                            }
                            if status_type.read().as_str() == "success" {
                                button {
                                    class: "text-green-600 dark:text-green-400 hover:text-green-800 dark:hover:text-green-200 transition-colors",
                                    onclick: move |_| submit_status.set(None),
                                    "✕"
                                }
                            }
                        }
                    }
                }
            }

            // Additional Contact Info
            div { class: "space-y-2",
                h2 { class: "text-xl font-semibold", "Other Ways to Connect" }
                p {
                    "For professional inquiries related to mortgage and finance: "
                    a {
                        class: "text-blue-200 dark:text-blue-400 hover:underline",
                        href: "mailto:dylan@rayburnlp.com",
                        "dylan@rayburnlp.com"
                    }
                }
                p {
                    "For technical questions or project collaborations: "
                    a {
                        class: "text-blue-200 dark:text-blue-400 hover:underline",
                        href: "mailto:tech@dylanrayburn.com",
                        "tech@dylanrayburn.com"
                    }
                }
                p {
                    "Check out my coding projects on "
                    a {
                        class: "text-blue-200 dark:text-blue-400 hover:underline",
                        href: "https://github.com/RayburnCode",
                        target: "_blank",
                        "GitHub"
                    }
                }
            }
        }
    }
}