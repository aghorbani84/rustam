use dioxus::prelude::*;
use crate::components::skill_badge::SkillBadge;

#[component]
pub fn Skills() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto px-8 py-16 animate-fade-in",
            h1 { class: "text-4xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Skills & Expertise" }
            p { class: "text-lg text-gray-600 dark:text-gray-400 mb-12",
                "A comprehensive overview of my technical skills and what I bring to the table."
            }

            // Technical Skills
            section { class: "mb-16",
                h2 { class: "text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100", "Technical Skills" }

                div { class: "mb-8",
                    h3 { class: "text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200", "Languages & Frameworks" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                        SkillBadge { name: "Rust", level: 85, icon: "🦀" }
                        SkillBadge { name: "Dioxus", level: 80, icon: "⚛️" }
                        SkillBadge { name: "JavaScript/TypeScript", level: 85, icon: "📜" }
                        SkillBadge { name: "React", level: 80, icon: "⚛️" }
                        SkillBadge { name: "Python", level: 75, icon: "🐍" }
                        SkillBadge { name: "HTML/CSS", level: 90, icon: "🎨" }
                    }
                }

                div { class: "mb-8",
                    h3 { class: "text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200", "Backend & Database" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                        SkillBadge { name: "PostgreSQL", level: 80, icon: "🐘" }
                        SkillBadge { name: "MongoDB", level: 75, icon: "🍃" }
                        SkillBadge { name: "Redis", level: 70, icon: "🔴" }
                        SkillBadge { name: "Docker", level: 80, icon: "🐳" }
                        SkillBadge { name: "Linux", level: 85, icon: "🐧" }
                        SkillBadge { name: "WebAssembly", level: 75, icon: "⚡" }
                    }
                }

                div { class: "mb-8",
                    h3 { class: "text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200", "Tools & Practices" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                        SkillBadge { name: "Git", level: 90, icon: "📝" }
                        SkillBadge { name: "CI/CD", level: 80, icon: "🔄" }
                        SkillBadge { name: "Testing", level: 85, icon: "🧪" }
                        SkillBadge { name: "Agile/Scrum", level: 80, icon: "🏃" }
                        SkillBadge { name: "REST APIs", level: 85, icon: "🔌" }
                        SkillBadge { name: "GraphQL", level: 70, icon: "📊" }
                    }
                }
            }

            // What I'm Learning
            section {
                h2 { class: "text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100", "Currently Learning" }
                div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg",
                    ul { class: "space-y-3 text-gray-700 dark:text-gray-300",
                        li { "🚀 Advanced Rust patterns and async programming" }
                        li { "⚡ WebAssembly optimization techniques" }
                        li { "🎨 Advanced CSS animations and micro-interactions" }
                        li { "🔧 DevOps practices and cloud deployment" }
                        li { "📱 Mobile-first responsive design patterns" }
                    }
                }
            }
        }
    }
}
