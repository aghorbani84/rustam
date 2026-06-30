use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectCardProps {
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub tech_stack: Vec<String>,
    pub github_url: String,
    pub demo_url: String,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    rsx! {
        div { class: "group bg-white dark:bg-gray-800 rounded-xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2",
            // Image
            div { class: "relative h-48 overflow-hidden",
                img {
                    src: "{props.image_url}",
                    alt: "{props.title}",
                    class: "w-full h-full object-cover group-hover:scale-110 transition-transform duration-300"
                }
                div { class: "absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" }
            }

            // Content
            div { class: "p-6",
                h3 { class: "text-xl font-bold mb-2 text-gray-900 dark:text-gray-100", "{props.title}" }
                p { class: "text-gray-600 dark:text-gray-400 mb-4 line-clamp-3", "{props.description}" }

                // Tech stack
                div { class: "flex flex-wrap gap-2 mb-4",
                    for tech in props.tech_stack.iter() {
                        span {
                            class: "px-3 py-1 text-xs font-semibold bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full",
                            "{tech}"
                        }
                    }
                }

                // Links
                div { class: "flex gap-3",
                    a {
                        href: "{props.github_url}",
                        target: "_blank",
                        class: "flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors text-center",
                        "GitHub"
                    }
                    a {
                        href: "{props.demo_url}",
                        target: "_blank",
                        class: "flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors text-center",
                        "Live Demo"
                    }
                }
            }
        }
    }
}
