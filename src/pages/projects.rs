use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Project {
    name: &'static str,
    description: &'static str,
    tech: &'static str,
    github: &'static str,
    demo: &'static str,
    emoji: &'static str,
    badge: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        name: "Rustam",
        description: "High-performance portfolio website built with Dioxus and compiled to WebAssembly for blazing speed.",
        tech: "Rust · Dioxus · WASM",
        github: "https://github.com/aghorbani84/rustam",
        demo: "https://aghorbani84.github.io/rustam/",
        emoji: "🦀",
        badge: "bg-orange-500/10 text-orange-400 border-orange-500/30",
    },
    Project {
        name: "Anews",
        description: "Fast, native news reader application engineered in C++ for performance and low resource usage.",
        tech: "C++",
        github: "https://github.com/aghorbani84/Anews",
        demo: "https://aghorbani84.github.io/Anews/",
        emoji: "📰",
        badge: "bg-pink-500/10 text-pink-400 border-pink-500/30",
    },
    Project {
        name: "Chapar",
        description: "Local-first, secure desktop API client built with Tauri — test and manage endpoints offline.",
        tech: "Rust · Tauri",
        github: "https://github.com/aghorbani84/Chapar",
        demo: "https://aghorbani84.github.io/Chapar/",
        emoji: "🔌",
        badge: "bg-amber-500/10 text-amber-400 border-amber-500/30",
    },
    Project {
        name: "APromptEngenier",
        description: "Elite prompt-engineering tool that transforms raw ideas into structured, high-quality AI prompts.",
        tech: "HTML · JS",
        github: "https://github.com/aghorbani84/APromptEngenier",
        demo: "https://aghorbani84.github.io/APromptEngenier/",
        emoji: "🤖",
        badge: "bg-emerald-500/10 text-emerald-400 border-emerald-500/30",
    },
    Project {
        name: "AreadmeForge",
        description: "GitHub profile README generator with live preview, themeable accents and one-click export.",
        tech: "HTML · JS",
        github: "https://github.com/aghorbani84/AreadmeForge",
        demo: "https://aghorbani84.github.io/AreadmeForge/",
        emoji: "📄",
        badge: "bg-sky-500/10 text-sky-400 border-sky-500/30",
    },
    Project {
        name: "AtelegramPostCreator",
        description: "Create and format professional, ready-to-publish Telegram posts in seconds with live preview.",
        tech: "HTML · JS",
        github: "https://github.com/aghorbani84/AtelegramPostCreator",
        demo: "https://aghorbani84.github.io/AtelegramPostCreator/",
        emoji: "✈️",
        badge: "bg-blue-500/10 text-blue-400 border-blue-500/30",
    },
];

#[derive(Props, Clone, PartialEq)]
struct CardProps {
    project: Project,
}

#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        div {
            class: "group relative flex flex-col bg-white dark:bg-gray-800/40 border border-gray-200 dark:border-gray-700 rounded-2xl p-6 transition-all duration-300 hover:border-blue-500 dark:hover:border-blue-400 hover:shadow-xl hover:-translate-y-1",
            div { class: "flex items-start justify-between",
                span { class: "text-4xl", "{project.emoji}" }
                a {
                    href: "{project.github}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "text-xs font-mono text-gray-400 hover:text-blue-400 transition-colors",
                    "source ↗"
                }
            }
            h3 { class: "mt-4 text-xl font-bold text-gray-900 dark:text-white", "{project.name}" }
            p { class: "mt-2 flex-1 text-sm text-gray-600 dark:text-gray-400 leading-relaxed", "{project.description}" }
            div { class: "mt-4",
                span {
                    class: "inline-block text-xs font-mono px-3 py-1 rounded-full border {project.badge}",
                    "{project.tech}"
                }
            }
            div { class: "mt-5 flex items-center gap-3",
                a {
                    href: "{project.demo}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "flex-1 text-center bg-gradient-to-r from-blue-600 to-purple-600 text-white text-sm font-semibold py-2 rounded-lg hover:opacity-90 transition-opacity",
                    "Live Demo ↗"
                }
                a {
                    href: "{project.github}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "flex-1 text-center border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 text-sm font-semibold py-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors",
                    "GitHub"
                }
            }
        }
    }
}

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { class: "min-h-screen py-20 px-6",
            div { class: "max-w-6xl mx-auto",
                div { class: "text-center mb-16",
                    span { class: "text-sm font-mono text-blue-500 tracking-widest uppercase", "// portfolio" }
                    h1 {
                        class: "mt-3 text-4xl md:text-5xl font-extrabold bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent",
                        "Featured Projects"
                    }
                    p {
                        class: "mt-4 text-gray-600 dark:text-gray-400 max-w-2xl mx-auto",
                        "A collection of tools and applications I have built — from high-performance Rust systems to polished web experiences."
                    }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    for project in PROJECTS {
                        ProjectCard { project: project.clone() }
                    }
                }
            }
        }
    }
}
