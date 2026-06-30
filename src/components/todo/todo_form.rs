use dioxus::prelude::*;
use crate::models::todo::Priority;

#[derive(Props, Clone, PartialEq)]
pub struct TodoFormProps {
    pub on_submit: EventHandler<(String, Priority)>,
}

#[component]
pub fn TodoForm(props: TodoFormProps) -> Element {
    let mut title = use_signal(String::new);
    let mut priority = use_signal(|| Priority::Medium);

    let mut handle_submit = move |_| {
        if !title().trim().is_empty() {
            props.on_submit.call((title(), priority()));
            title.set(String::new());
            priority.set(Priority::Medium);
        }
    };

    rsx! {
        form {
            class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg",
            onsubmit: move |evt| {
                evt.prevent_default();
                handle_submit(());
            },

            h2 { class: "text-xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Add New Task" }

            div { class: "space-y-4",
                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                        "Task Title"
                    }
                    input {
                        class: "w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "text",
                        placeholder: "What needs to be done?",
                        value: "{title}",
                        oninput: move |evt| title.set(evt.value()),
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                        "Priority"
                    }
                    select {
                        class: "w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500",
                        value: match priority() {
                            Priority::Low => "low",
                            Priority::Medium => "medium",
                            Priority::High => "high",
                        },
                        onchange: move |evt| {
                            let value = evt.value();
                            priority.set(match value.as_str() {
                                "low" => Priority::Low,
                                "high" => Priority::High,
                                _ => Priority::Medium,
                            });
                        },

                        option { value: "low", "Low" }
                        option { value: "medium", "Medium" }
                        option { value: "high", "High" }
                    }
                }

                button {
                    class: "w-full px-4 py-2 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-colors shadow-md",
                    r#type: "submit",
                    "Add Task"
                }
            }
        }
    }
}
