use dioxus::prelude::*;
use crate::components::markdown::{
    toolbar::Toolbar,
    editor::Editor,
    preview::Preview,
};
use crate::hooks::use_markdown::parse_markdown;

#[component]
pub fn MarkdownEditor() -> Element {
    let mut content = use_signal(|| "# Welcome to Markdown Editor\n\nStart typing...".to_string());
    let mut title = use_signal(|| "My Document".to_string());
    let mut view_mode = use_signal(|| ViewMode::Split);

    #[derive(Clone, PartialEq)]
    enum ViewMode {
        Editor,
        Preview,
        Split,
    }

    let html_content = parse_markdown(&content());

    let word_count = content()
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .count();
    let char_count = content().len();
    let line_count = content().lines().count();

    let handle_insert = move |text: String| {
        let current = content();
        let new_content = format!("{}{}", current, text);
        content.set(new_content);
    };

    rsx! {
        div { class: "max-w-full mx-auto px-8 py-8 animate-fade-in",
            div { class: "flex items-center justify-between mb-6 flex-wrap gap-4",
                div { class: "flex-1",
                    h1 { class: "text-3xl font-bold text-gray-900 dark:text-gray-100", "Markdown Editor" }
                    p { class: "text-gray-600 dark:text-gray-400 mt-1",
                        "Write markdown and see the preview in real-time."
                    }
                }

                div { class: "flex gap-2 bg-white dark:bg-gray-800 p-1 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                    button {
                        class: if view_mode() == ViewMode::Editor {
                            "px-4 py-1.5 bg-blue-600 text-white rounded text-sm font-medium"
                        } else {
                            "px-4 py-1.5 text-gray-700 dark:text-gray-300 rounded text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700"
                        },
                        onclick: move |_| view_mode.set(ViewMode::Editor),
                        "Editor"
                    }
                    button {
                        class: if view_mode() == ViewMode::Split {
                            "px-4 py-1.5 bg-blue-600 text-white rounded text-sm font-medium"
                        } else {
                            "px-4 py-1.5 text-gray-700 dark:text-gray-300 rounded text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700"
                        },
                        onclick: move |_| view_mode.set(ViewMode::Split),
                        "Split"
                    }
                    button {
                        class: if view_mode() == ViewMode::Preview {
                            "px-4 py-1.5 bg-blue-600 text-white rounded text-sm font-medium"
                        } else {
                            "px-4 py-1.5 text-gray-700 dark:text-gray-300 rounded text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700"
                        },
                        onclick: move |_| view_mode.set(ViewMode::Preview),
                        "Preview"
                    }
                }
            }

            div { class: "mb-4",
                input {
                    class: "w-full px-4 py-3 text-2xl font-bold bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 border border-gray-200 dark:border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                    value: "{title}",
                    placeholder: "Document Title",
                    oninput: move |evt| title.set(evt.value()),
                }
            }

            div { class: "bg-white dark:bg-gray-800 rounded-xl shadow-lg overflow-hidden border border-gray-200 dark:border-gray-700",
                Toolbar {
                    on_insert: handle_insert,
                    on_export: move |_| {},
                }

                div { class: "grid gap-0",
                    style: match view_mode() {
                        ViewMode::Editor => "grid-template-columns: 1fr",
                        ViewMode::Preview => "grid-template-columns: 1fr",
                        ViewMode::Split => "grid-template-columns: 1fr 1fr",
                    },

                    if view_mode() == ViewMode::Editor || view_mode() == ViewMode::Split {
                        div { class: "border-r border-gray-200 dark:border-gray-700",
                            div { class: "h-[600px] overflow-auto",
                                Editor {
                                    content: content(),
                                    on_change: move |val| content.set(val),
                                }
                            }
                        }
                    }

                    if view_mode() == ViewMode::Preview || view_mode() == ViewMode::Split {
                        div { class: "h-[600px] overflow-auto",
                            Preview { html: html_content }
                        }
                    }
                }
            }

            div { class: "mt-4 flex gap-6 text-sm text-gray-600 dark:text-gray-400 bg-white dark:bg-gray-800 p-4 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                div {
                    span { class: "font-semibold text-gray-900 dark:text-gray-100", "{word_count}" }
                    " words"
                }
                div {
                    span { class: "font-semibold text-gray-900 dark:text-gray-100", "{char_count}" }
                    " characters"
                }
                div {
                    span { class: "font-semibold text-gray-900 dark:text-gray-100", "{line_count}" }
                    " lines"
                }
            }
        }
    }
}
