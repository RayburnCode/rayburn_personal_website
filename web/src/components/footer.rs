use dioxus::prelude::*;

#[component]
pub fn Footer(children: Element) -> Element {
    rsx!(
        footer { class: "bg-white rounded-lg shadow-sm m-4 dark:bg-gray-800",
            div { class: "w-full mx-auto max-w-screen-xl p-4 flex items-center justify-center",
                span { class: "text-sm text-gray-500 text-center dark:text-gray-400",
                    "© 2025 "
                    a {
                        href: "https://dylanrayburn.com",
                        class: "hover:underline",
                        "Dylan Rayburn"
                    }
                    ". All Rights Reserved."
                }
            }
        }
    )
}