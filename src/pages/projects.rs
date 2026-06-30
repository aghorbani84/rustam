use dioxus::prelude::*;
use crate::components::project_card::ProjectCard;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-8 py-16 animate-fade-in",
            h1 { class: "text-4xl font-bold mb-4 text-gray-900 dark:text-gray-100", "My Projects" }
            p { class: "text-lg text-gray-600 dark:text-gray-400 mb-12",
                "Here are some of the projects I've built. Each one represents a unique challenge and learning experience."
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                ProjectCard {
                    title: "Todo App",
                    description: "A professional task management application with priority levels, filtering, search, and persistent storage. Built with Dioxus and LocalStorage.",
                    image_url: "https://images.unsplash.com/photo-1484480974693-6ca0a78fb36b?w=800&h=600&fit=crop",
                    tech_stack: vec!["Rust".to_string(), "Dioxus".to_string(), "LocalStorage".to_string(), "Tailwind".to_string()],
                    github_url: "https://gitlab.com/abolfazlghorbani369",
                    demo_url: "/todo"
                }

                ProjectCard {
                    title: "Pomodoro Timer",
                    description: "A beautiful Pomodoro timer with circular progress animation, customizable settings, statistics tracking, and session management.",
                    image_url: "https://images.unsplash.com/photo-1501139083538-0139583c060f?w=800&h=600&fit=crop",
                    tech_stack: vec!["Rust".to_string(), "Dioxus".to_string(), "WebAssembly".to_string(), "SVG".to_string()],
                    github_url: "https://gitlab.com/abolfazlghorbani369",
                    demo_url: "/pomodoro"
                }

                ProjectCard {
                    title: "Markdown Editor",
                    description: "A real-time markdown editor with split view, syntax highlighting, auto-save, and export to HTML/Markdown. Built with pulldown-cmark.",
                    image_url: "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=800&h=600&fit=crop",
                    tech_stack: vec!["Rust".to_string(), "Dioxus".to_string(), "pulldown-cmark".to_string(), "WASM".to_string()],
                    github_url: "https://gitlab.com/abolfazlghorbani369",
                    demo_url: "/markdown"
                }

                ProjectCard {
                    title: "Portfolio Website",
                    description: "This very website! A modern, responsive portfolio built with Dioxus, featuring dark mode, smooth animations, and professional design.",
                    image_url: "https://images.unsplash.com/photo-1467232004584-a241de8bcf5d?w=800&h=600&fit=crop",
                    tech_stack: vec!["Rust".to_string(), "Dioxus".to_string(), "Tailwind CSS".to_string(), "WASM".to_string()],
                    github_url: "https://gitlab.com/abolfazlghorbani369",
                    demo_url: "/"
                }
            }
        }
    }
}
