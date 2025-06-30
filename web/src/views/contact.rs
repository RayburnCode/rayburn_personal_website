use dioxus::prelude::*;
use dioxus_signals::Signal;
use serde_json::json;

#[component]
pub fn Contact() -> Element {
    let mut name = Signal::new(String::new());
    let mut email = Signal::new(String::new());
    let mut message = Signal::new(String::new());
    let mut is_submitting = Signal::new(false);
    let mut submit_status = Signal::new(Option::<String>::None);

    rsx! {
        div { class: "max-w-2xl mx-auto p-6 text-gray-800 dark:text-gray-200",
            h1 { class: "text-3xl font-bold mb-6", "Get in Touch" }
            
            p { class: "mb-6",
                "Have questions about mortgages and finance? Want to discuss a coding project or 3D printing idea? I'd love to hear from you! Fill out the form below or connect with me directly through my GitHub."
            }

            // Contact Form
            form {
                class: "space-y-4 mb-8",
                onsubmit: move |event| {
                    event.prevent_default();
                    is_submitting.set(true);
                    
                    // In a real app, you would replace this with actual Supabase API call
                    async move {
                        // This would be your Supabase POST request
                        // let client = supabase_rs::new(SUPABASE_URL, SUPABASE_KEY);
                        // let response = client.from("contacts").insert(json!({
                        //     "name": name.read(),
                        //     "email": email.read(),
                        //     "message": message.read()
                        // })).execute().await;
                        
                        // Simulate API call delay
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        
                        is_submitting.set(false);
                        submit_status.set(Some("Thank you! Your message has been sent.".to_string()));
                        name.set(String::new());
                        email.set(String::new());
                        message.set(String::new());
                    }
                },

                // Name Field
                div {
                    label {
                        class: "block text-sm font-medium mb-1",
                        "Name"
                    }
                    input {
                        class: "w-full px-4 py-2 rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "text",
                        name: "name",
                        value: "{name}",
                        oninput: move |e| name.set(e.value().to_string()),
                        required: true
                    }
                }

                // Email Field
                div {
                    label {
                        class: "block text-sm font-medium mb-1",
                        "Email"
                    }
                    input {
                        class: "w-full px-4 py-2 rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "email",
                        name: "email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value().to_string()),
                        required: true
                    }
                }

                // Message Field
                div {
                    label {
                        class: "block text-sm font-medium mb-1",
                        "Message"
                    }
                    textarea {
                        class: "w-full px-4 py-2 rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 min-h-[120px]",
                        name: "message",
                        value: "{message}",
                        oninput: move |e| message.set(e.value().to_string()),
                        required: true
                    }
                }

                // Submit Button
                button {
                    class: "px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors disabled:opacity-50",
                    r#type: "submit",
                    disabled: *is_submitting.read(),
                    if *is_submitting.read() {
                        "Sending..."
                    } else {
                        "Send Message"
                    }
                }

                // Status Message
                if let Some(status) = submit_status.read().as_ref() {
                    div {
                        class: "mt-4 p-3 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded",
                        "{status}"
                    }
                }
            }

            // Additional Contact Info
            div { class: "space-y-2",
                h2 { class: "text-xl font-semibold", "Other Ways to Connect" }
                p {
                    "For professional inquiries related to mortgage and finance: "
                    a {
                        class: "text-blue-600 dark:text-blue-400 hover:underline",
                        href: "mailto:finance@example.com",
                        "finance@example.com"
                    }
                }
                p {
                    "For technical questions or project collaborations: "
                    a {
                        class: "text-blue-600 dark:text-blue-400 hover:underline",
                        href: "mailto:tech@example.com",
                        "tech@example.com"
                    }
                }
                p {
                    "Check out my coding projects on "
                    a {
                        class: "text-blue-600 dark:text-blue-400 hover:underline",
                        href: "https://github.com/yourusername",
                        target: "_blank",
                        "GitHub"
                    }
                }
            }
        }
    }
}