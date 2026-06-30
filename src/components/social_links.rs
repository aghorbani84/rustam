use dioxus::prelude::*;

#[component]
pub fn SocialLinks() -> Element {
    rsx! {
        div { class: "flex gap-4 justify-center",
            a {
                href: "https://gitlab.com/abolfazlghorbani369",
                target: "_blank",
                class: "p-3 bg-gray-100 dark:bg-gray-800 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors",
                title: "GitLab",
                "🦊"
            }
            a {
                href: "https://github.com/abolfazlghorbani369",
                target: "_blank",
                class: "p-3 bg-gray-100 dark:bg-gray-800 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors",
                title: "GitHub",
                "🐙"
            }
            a {
                href: "mailto:abolfazlghorbani369@gmail.com",
                class: "p-3 bg-gray-100 dark:bg-gray-800 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors",
                title: "Email",
                "📧"
            }
        }
    }
}
