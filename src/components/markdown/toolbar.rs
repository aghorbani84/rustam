use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToolbarProps {
    pub on_insert: EventHandler<String>,
    pub on_export: EventHandler<()>,
}

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-1 p-3 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
            button {
                class: "px-3 py-1.5 text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600",
                onclick: move |_| props.on_insert.call("**bold**".to_string()),
                "B"
            }
            button {
                class: "px-3 py-1.5 text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600",
                onclick: move |_| props.on_insert.call("*italic*".to_string()),
                "I"
            }
            button {
                class: "px-3 py-1.5 text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600",
                onclick: move |_| props.on_insert.call("# Heading\n".to_string()),
                "H1"
            }
            button {
                class: "px-3 py-1.5 text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600",
                onclick: move |_| props.on_insert.call("- List item\n".to_string()),
                "List"
            }
            button {
                class: "px-3 py-1.5 text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600",
                onclick: move |_| props.on_insert.call("```\ncode\n```\n".to_string()),
                "Code"
            }
            button {
                class: "px-3 py-1.5 text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600",
                onclick: move |_| props.on_insert.call("[link](url)\n".to_string()),
                "Link"
            }
        }
    }
}
