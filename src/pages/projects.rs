use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Project {
    name: &'static str,
    description: &'static str,
    tech: &'static str,
    emoji: &'static str,
    github: &'static str,
    demo: &'static str,
    github_label: &'static str,
    internal: bool,
    tag: &'static str,
    tag_class: &'static str,
    tech_class: &'static str,
}

// external = pinned standalone repos (demo opens in a NEW tab)
// internal = apps built INTO this site (demo opens in the SAME tab,
//            relative URL so it works both locally and on GitHub Pages)
const PROJECTS: &[Project] = &[
    // ─────────────── EXTERNAL / PINNED ───────────────
    Project {
        name: "Rustam",
        description: "High-performance portfolio website built with Dioxus and compiled to WebAssembly for blazing speed.",
        tech: "Rust · Dioxus · WASM",
        emoji: "🦀",
        github: "https://github.com/aghorbani84/rustam",
        demo: "https://aghorbani84.github.io/rustam/",
        github_label: "GitHub",
        internal: false,
        tag: "⭐ Pinned",
        tag_class: "bg-gray-500/10 text-gray-300 border-gray-400/30",
        tech_class: "bg-orange-500/10 text-orange-400 border-orange-500/30",
    },
    Project {
        name: "Anews",
        description: "Fast, native news reader application engineered in C++ for performance and low resource usage.",
        tech: "C++",
        emoji: "📰",
        github: "https://github.com/aghorbani84/Anews",
        demo: "https://aghorbani84.github.io/Anews/",
        github_label: "GitHub",
        internal: false,
        tag: "⭐ Pinned",
        tag_class: "bg-gray-500/10 text-gray-300 border-gray-400/30",
        tech_class: "bg-pink-500/10 text-pink-400 border-pink-500/30",
    },
    Project {
        name: "Chapar",
        description: "Local-first, secure desktop API client built with Tauri — test and manage endpoints offline.",
        tech: "Rust · Tauri",
        emoji: "🔌",
        github: "https://github.com/aghorbani84/Chapar",
        demo: "https://aghorbani84.github.io/Chapar/",
        github_label: "GitHub",
        internal: false,
        tag: "⭐ Pinned",
        tag_class: "bg-gray-500/10 text-gray-300 border-gray-400/30",
        tech_class: "bg-amber-500/10 text-amber-400 border-amber-500/30",
    },
    Project {
        name: "APromptEngenier",
        description: "Elite prompt-engineering tool that transforms raw ideas into structured, high-quality AI prompts.",
        tech: "HTML · JS",
        emoji: "🤖",
        github: "https://github.com/aghorbani84/APromptEngenier",
        demo: "https://aghorbani84.github.io/APromptEngenier/",
        github_label: "GitHub",
        internal: false,
        tag: "⭐ Pinned",
        tag_class: "bg-gray-500/10 text-gray-300 border-gray-400/30",
        tech_class: "bg-emerald-500/10 text-emerald-400 border-emerald-500/30",
    },
    Project {
        name: "AreadmeForge",
        description: "GitHub profile README generator with live preview, themeable accents and one-click export.",
        tech: "HTML · JS",
        emoji: "📄",
        github: "https://github.com/aghorbani84/AreadmeForge",
        demo: "https://aghorbani84.github.io/AreadmeForge/",
        github_label: "GitHub",
        internal: false,
        tag: "⭐ Pinned",
        tag_class: "bg-gray-500/10 text-gray-300 border-gray-400/30",
        tech_class: "bg-sky-500/10 text-sky-400 border-sky-500/30",
    },
    Project {
        name: "AtelegramPostCreator",
        description: "Create and format professional, ready-to-publish Telegram posts in seconds with live preview.",
        tech: "HTML · JS",
        emoji: "✈️",
        github: "https://github.com/aghorbani84/AtelegramPostCreator",
        demo: "https://aghorbani84.github.io/AtelegramPostCreator/",
        github_label: "GitHub",
        internal: false,
        tag: "⭐ Pinned",
        tag_class: "bg-gray-500/10 text-gray-300 border-gray-400/30",
        tech_class: "bg-blue-500/10 text-blue-400 border-blue-500/30",
    },
    // ─────────────── BUILT-IN APPS ───────────────
    Project {
        name: "Todo App",
        description: "Built-in task manager with priorities, filters and search — fully persisted in LocalStorage.",
        tech: "Rust · Dioxus",
        emoji: "✅",
        github: "https://github.com/aghorbani84/rustam/tree/main/src/pages/todo.rs",
        demo: "todo",
        github_label: "Source",
        internal: true,
        tag: "⚡ Built-in",
        tag_class: "bg-emerald-500/10 text-emerald-400 border-emerald-500/30",
        tech_class: "bg-cyan-500/10 text-cyan-400 border-cyan-500/30",
    },
    Project {
        name: "Pomodoro Timer",
        description: "Built-in focus timer with circular progress, customizable sessions and statistics tracking.",
        tech: "Rust · Dioxus",
        emoji: "⏱️",
        github: "https://github.com/aghorbani84/rustam/tree/main/src/pages/pomodoro.rs",
        demo: "pomodoro",
        github_label: "Source",
        internal: true,
        tag: "⚡ Built-in",
        tag_class: "bg-emerald-500/10 text-emerald-400 border-emerald-500/30",
        tech_class: "bg-rose-500/10 text-rose-400 border-rose-500/30",
    },
    Project {
        name: "Markdown Editor",
        description: "Built-in Markdown editor with real-time preview, split view and one-click HTML export.",
        tech: "Rust · Dioxus",
        emoji: "📝",
        github: "https://github.com/aghorbani84/rustam/tree/main/src/pages/markdown.rs",
        demo: "markdown",
        github_label: "Source",
        internal: true,
        tag: "⚡ Built-in",
        tag_class: "bg-emerald-500/10 text-emerald-400 border-emerald-500/30",
        tech_class: "bg-violet-500/10 text-violet-400 border-violet-500/30",
    },
];

#[component]
fn ProjectCard(project: Project) -> Element {
    // internal apps open in the SAME tab (relative URL), external in a NEW tab
    let demo_target = if project.internal { "_self" } else { "_blank" };

    rsx! {
        div {
            class: "group relative flex flex-col bg-white dark:bg-gray-800/40 border border-gray-200 dark:border-gray-700 rounded-2xl p-6 transition-all duration-300 hover:border-blue-500 dark:hover:border-blue-400 hover:shadow-xl hover:-translate-y-1",
            div { class: "flex items-start justify-between",
                span { class: "text-4xl", "{project.emoji}" }
                span {
                    class: "text-[10px] font-mono px-2 py-1 rounded-full border {project.tag_class}",
                    "{project.tag}"
                }
            }
            h3 { class: "mt-4 text-xl font-bold text-gray-900 dark:text-white", "{project.name}" }
            p { class: "mt-2 flex-1 text-sm text-gray-600 dark:text-gray-400 leading-relaxed", "{project.description}" }
            div { class: "mt-4",
                span {
                    class: "inline-block text-xs font-mono px-3 py-1 rounded-full border {project.tech_class}",
                    "{project.tech}"
                }
            }
            div { class: "mt-5 flex items-center gap-3",
                a {
                    href: "{project.demo}",
                    target: "{demo_target}",
                    rel: "noopener noreferrer",
                    class: "flex-1 text-center bg-gradient-to-r from-blue-600 to-purple-600 text-white text-sm font-semibold py-2 rounded-lg hover:opacity-90 transition-opacity",
                    "Live Demo ↗"
                }
                a {
                    href: "{project.github}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "flex-1 text-center border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 text-sm font-semibold py-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors",
                    "{project.github_label}"
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
                        "Standalone tools I ship — plus the interactive apps built right into this site."
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
