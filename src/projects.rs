use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tech: &'static str,
    pub github_url: &'static str,
    pub demo_url: &'static str,
}

pub static PROJECTS: &[Project] = &[
    Project {
        name: "Rustam",
        description: "High-performance portfolio built with Dioxus and Rust (WASM).",
        tech: "Rust / Dioxus",
        github_url: "https://github.com/aghorbani84/rustam",
        demo_url: "https://aghorbani84.github.io/rustam/",
    },
    Project {
        name: "Chapar",
        description: "Local-first, highly secure Desktop API Client built with Tauri and Svelte.",
        tech: "Rust / Tauri / Svelte",
        github_url: "https://github.com/aghorbani84/Chapar",
        demo_url: "https://aghorbani84.github.io/Chapar/",
    },
    Project {
        name: "APromptEngenier",
        description: "Elite prompt engineering tool to transform raw ideas into structured AI prompts.",
        tech: "HTML / JS",
        github_url: "https://github.com/aghorbani84/APromptEngenier",
        demo_url: "https://aghorbani84.github.io/APromptEngenier/",
    },
    Project {
        name: "AreadmeForge",
        description: "Elite GitHub Profile README Generator with live preview and themeable accents.",
        tech: "HTML / JS",
        github_url: "https://github.com/aghorbani84/AreadmeForge",
        demo_url: "https://aghorbani84.github.io/AreadmeForge/",
    },
    Project {
        name: "AtelegramPostCreator",
        description: "Tool to create and format professional Telegram posts instantly.",
        tech: "HTML / JS",
        github_url: "https://github.com/aghorbani84/AtelegramPostCreator",
        demo_url: "https://aghorbani84.github.io/AtelegramPostCreator/",
    },
];

// A Dioxus component to render these projects
pub fn ProjectsList(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-6 p-4",
            PROJECTS.iter().map(|project| {
                rsx! {
                    div {
                        class: "bg-gray-800 p-6 rounded-lg shadow-lg border border-gray-700 hover:border-blue-500 transition-all",
                        h3 { class: "text-xl font-bold text-white", "{project.name}" }
                        p { class: "text-gray-400 mt-2 text-sm h-10", "{project.description}" }
                        div {
                            class: "mt-4 flex justify-between items-center",
                            span { class: "text-xs font-mono text-blue-400 bg-gray-900 px-2 py-1 rounded", "{project.tech}" }
                            div {
                                class: "flex gap-3",
                                a {
                                    href: "{project.github_url}",
                                    target: "_blank",
                                    class: "text-gray-300 hover:text-white text-sm font-semibold",
                                    "GitHub"
                                }
                                a {
                                    href: "{project.demo_url}",
                                    target: "_blank",
                                    class: "text-blue-400 hover:text-blue-300 text-sm font-semibold",
                                    "Live Demo ↗"
                                }
                            }
                        }
                    }
                }
            })
        }
    })
}
