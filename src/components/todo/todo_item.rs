use dioxus::prelude::*;
use crate::models::todo::{Todo, Priority};

#[derive(Props, Clone, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
    pub on_toggle: EventHandler<String>,
    pub on_delete: EventHandler<String>,
    pub on_edit: EventHandler<(String, String)>,
}

#[component]
pub fn TodoItem(props: TodoItemProps) -> Element {
    let mut is_editing = use_signal(|| false);
    let mut edit_title = use_signal(|| props.todo.title.clone());

    let priority_color = match props.todo.priority {
        Priority::Low => "border-l-4 border-l-green-500",
        Priority::Medium => "border-l-4 border-l-yellow-500",
        Priority::High => "border-l-4 border-l-red-500",
    };

    let priority_badge = match props.todo.priority {
        Priority::Low => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
        Priority::Medium => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
        Priority::High => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
    };

    // Clone جداگانه برای هر closure
    let id_for_keydown_enter = props.todo.id.clone();
    let title_for_keydown_escape = props.todo.title.clone();
    let id_for_save = props.todo.id.clone();
    let title_for_cancel = props.todo.title.clone();
    let id_for_toggle = props.todo.id.clone();
    let title_for_edit_btn = props.todo.title.clone();
    let id_for_delete = props.todo.id.clone();

    rsx! {
        div {
            class: "group bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-all {priority_color}",

            if is_editing() {
                div { class: "p-4",
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500",
                        value: "{edit_title}",
                        oninput: move |evt| edit_title.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                props.on_edit.call((id_for_keydown_enter.clone(), edit_title()));
                                is_editing.set(false);
                            } else if evt.key() == Key::Escape {
                                edit_title.set(title_for_keydown_escape.clone());
                                is_editing.set(false);
                            }
                        },
                        autofocus: true,
                    }
                    div { class: "flex gap-2 mt-2",
                        button {
                            class: "px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700",
                            onclick: move |_| {
                                props.on_edit.call((id_for_save.clone(), edit_title()));
                                is_editing.set(false);
                            },
                            "Save"
                        }
                        button {
                            class: "px-3 py-1 text-sm bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600",
                            onclick: move |_| {
                                edit_title.set(title_for_cancel.clone());
                                is_editing.set(false);
                            },
                            "Cancel"
                        }
                    }
                }
            } else {
                div { class: "flex items-center gap-3 p-4",
                    input {
                        class: "w-5 h-5 text-blue-600 rounded focus:ring-2 focus:ring-blue-500 cursor-pointer",
                        r#type: "checkbox",
                        checked: props.todo.completed,
                        onchange: move |_| props.on_toggle.call(id_for_toggle.clone()),
                    }

                    div { class: "flex-1",
                        p {
                            class: if props.todo.completed {
                                "text-gray-500 dark:text-gray-400 line-through"
                            } else {
                                "text-gray-900 dark:text-gray-100"
                            },
                            "{props.todo.title}"
                        }
                        span {
                            class: "inline-block mt-1 px-2 py-0.5 text-xs font-medium rounded {priority_badge}",
                            match props.todo.priority {
                                Priority::Low => "Low",
                                Priority::Medium => "Medium",
                                Priority::High => "High",
                            }
                        }
                    }

                    div { class: "flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity",
                        button {
                            class: "p-2 text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400 transition-colors",
                            onclick: move |_| {
                                edit_title.set(title_for_edit_btn.clone());
                                is_editing.set(true);
                            },
                            title: "Edit",
                            "✏️"
                        }
                        button {
                            class: "p-2 text-gray-500 hover:text-red-600 dark:text-gray-400 dark:hover:text-red-400 transition-colors",
                            onclick: move |_| props.on_delete.call(id_for_delete.clone()),
                            title: "Delete",
                            "🗑️"
                        }
                    }
                }
            }
        }
    }
}
