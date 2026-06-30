use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroProps {
    pub name: String,
    pub title: String,
    pub subtitle: String,
}

#[component]
pub fn Hero(props: HeroProps) -> Element {
    rsx! {
        section { class: "min-h-[90vh] flex items-center justify-center relative overflow-hidden",
            // Background gradient
            div { class: "absolute inset-0 bg-gradient-to-br from-blue-50 via-white to-purple-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900" }

            // Animated circles
            div { class: "absolute top-20 left-10 w-72 h-72 bg-blue-300 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob" }
            div { class: "absolute top-40 right-10 w-72 h-72 bg-purple-300 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-2000" }
            div { class: "absolute bottom-20 left-1/2 w-72 h-72 bg-pink-300 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-4000" }

            // Content
            div { class: "relative z-10 text-center px-8 max-w-4xl mx-auto animate-fade-in",
                h1 { class: "text-5xl md:text-7xl font-bold mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent",
                    "{props.name}"
                }
                h2 { class: "text-2xl md:text-3xl text-gray-700 dark:text-gray-300 mb-6 font-semibold",
                    "{props.title}"
                }
                p { class: "text-lg md:text-xl text-gray-600 dark:text-gray-400 mb-8 max-w-2xl mx-auto",
                    "{props.subtitle}"
                }

                div { class: "flex gap-4 justify-center flex-wrap",
                    a {
                        href: "#projects",
                        class: "px-8 py-3 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 transition-all transform hover:scale-105 shadow-lg",
                        "View My Work"
                    }
                    a {
                        href: "#contact",
                        class: "px-8 py-3 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 rounded-lg font-semibold hover:bg-gray-100 dark:hover:bg-gray-700 transition-all transform hover:scale-105 shadow-lg border border-gray-200 dark:border-gray-700",
                        "Contact Me"
                    }
                }
            }
        }
    }
}
