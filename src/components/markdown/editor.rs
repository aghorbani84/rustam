use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditorProps {
    pub content: String,
    pub on_change: EventHandler<String>,
}

#[component]
pub fn Editor(props: EditorProps) -> Element {
    let line_count = props.content.lines().count().max(1);

    rsx! {
        div { class: "flex h-full bg-white dark:bg-gray-900",
            // Line numbers
            div { class: "py-4 px-3 bg-gray-50 dark:bg-gray-800 text-right text-sm text-gray-400 dark:text-gray-500 select-none border-r border-gray-200 dark:border-gray-700 font-mono",
                for i in 1..=line_count {
                    div { "{i}" }
                }
            }

            // Textarea
            textarea {
                class: "flex-1 p-4 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 font-mono text-sm resize-none focus:outline-none",
                value: "{props.content}",
                placeholder: "Start writing markdown...",
                oninput: move |evt| props.on_change.call(evt.value()),
                spellcheck: "false",
            }
        }
    }
}
