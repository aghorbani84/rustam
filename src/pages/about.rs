use dioxus::prelude::*;
use crate::components::social_links::SocialLinks;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-8 py-16 animate-fade-in",
            h1 { class: "text-4xl font-bold mb-8 text-gray-900 dark:text-gray-100", "About Me" }

            div { class: "prose prose-lg dark:prose-invert max-w-none",
                div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg mb-8",
                    h2 { class: "text-2xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Who Am I?" }
                    p { class: "text-gray-700 dark:text-gray-300 mb-4",
                        "Hello! I'm Abolfazl Ghorbani, a full-stack developer with a passion for building scalable and performant web applications. "
                        "My journey in software development has been driven by curiosity and a desire to create impactful solutions."
                    }
                    p { class: "text-gray-700 dark:text-gray-300",
                        "I specialize in Rust and modern web technologies, with a strong focus on clean architecture, "
                        "test-driven development, and continuous learning."
                    }
                }

                div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg mb-8",
                    h2 { class: "text-2xl font-bold mb-4 text-gray-900 dark:text-gray-100", "What I Do" }
                    ul { class: "space-y-3 text-gray-700 dark:text-gray-300",
                        li { "🚀 Building high-performance web applications with Rust and Dioxus" }
                        li { "🎨 Creating beautiful and responsive user interfaces" }
                        li { "⚡ Optimizing application performance and scalability" }
                        li { "🔧 Developing robust backend systems and APIs" }
                        li { "📚 Sharing knowledge through open-source contributions" }
                    }
                }

                div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg",
                    h2 { class: "text-2xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Let's Connect" }
                    p { class: "text-gray-700 dark:text-gray-300 mb-6",
                        "I'm always open to discussing new projects, creative ideas, or opportunities to be part of your vision."
                    }
                    SocialLinks {}
                }
            }
        }
    }
}
