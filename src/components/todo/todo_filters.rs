use dioxus::prelude::*;
use crate::models::todo::TodoFilter;

#[derive(Props, Clone, PartialEq)]
pub struct TodoFiltersProps {
    pub current_filter: TodoFilter,
    pub on_filter_change: EventHandler<TodoFilter>,
    pub search_query: String,
    pub on_search_change: EventHandler<String>,
    pub total_count: usize,
    pub active_count: usize,
}

#[component]
pub fn TodoFilters(props: TodoFiltersProps) -> Element {
    let filter_button_class = |filter: &TodoFilter| {
        if &props.current_filter == filter {
            "px-4 py-2 bg-blue-600 text-white rounded-lg font-medium"
        } else {
            "px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600"
        }
    };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg space-y-4",
            // Search
            div {
                label {
                    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                    "Search Tasks"
                }
                input {
                    class: "w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500",
                    r#type: "text",
                    placeholder: "Search by title...",
                    value: "{props.search_query}",
                    oninput: move |evt| props.on_search_change.call(evt.value()),
                }
            }

            // Filters
            div { class: "flex gap-2 flex-wrap",
                button {
                    class: filter_button_class(&TodoFilter::All),
                    onclick: move |_| props.on_filter_change.call(TodoFilter::All),
                    "All ({props.total_count})"
                }
                button {
                    class: filter_button_class(&TodoFilter::Active),
                    onclick: move |_| props.on_filter_change.call(TodoFilter::Active),
                    "Active ({props.active_count})"
                }
                button {
                    class: filter_button_class(&TodoFilter::Completed),
                    onclick: move |_| props.on_filter_change.call(TodoFilter::Completed),
                    "Completed ({props.total_count - props.active_count})"
                }
            }
        }
    }
}
