use dioxus::prelude::*;
use crate::components::todo::{
    todo_item::TodoItem,
    todo_form::TodoForm,
    todo_filters::TodoFilters,
};
use crate::hooks::use_local_storage::{use_local_storage, save_to_local_storage};
use crate::models::todo::{Todo, TodoFilter, Priority};

#[component]
pub fn TodoPage() -> Element {
    let mut todos = use_local_storage::<Vec<Todo>>("todos", Vec::new());
    let mut filter = use_signal(|| TodoFilter::All);
    let mut search_query = use_signal(String::new);

    // Save to localStorage whenever todos change
    use_effect(move || {
        save_to_local_storage("todos", &todos());
    });

    // Add new todo
    let handle_add = move |(title, priority): (String, Priority)| {
        let new_todo = Todo::new(title, priority);
        let mut current = todos();
        current.push(new_todo);
        todos.set(current);
    };

    // Toggle todo completion
    let handle_toggle = move |id: String| {
        let mut current = todos();
        if let Some(todo) = current.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
            todos.set(current);
        }
    };

    // Delete todo
    let handle_delete = move |id: String| {
        let mut current = todos();
        current.retain(|t| t.id != id);
        todos.set(current);
    };

    // Edit todo
    let handle_edit = move |(id, new_title): (String, String)| {
        let mut current = todos();
        if let Some(todo) = current.iter_mut().find(|t| t.id == id) {
            todo.title = new_title;
            todos.set(current);
        }
    };

    // Filter and search todos
    let filtered_todos = todos()
        .into_iter()
        .filter(|todo| {
            let matches_filter = match filter() {
                TodoFilter::All => true,
                TodoFilter::Active => !todo.completed,
                TodoFilter::Completed => todo.completed,
            };

            let matches_search = search_query().is_empty()
                || todo.title.to_lowercase().contains(&search_query().to_lowercase());

            matches_filter && matches_search
        })
        .collect::<Vec<_>>();

    let active_count = todos().iter().filter(|t| !t.completed).count();
    let total_count = todos().len();

    rsx! {
        div { class: "max-w-4xl mx-auto px-8 py-16 animate-fade-in",
            h1 { class: "text-4xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Todo App" }
            p { class: "text-lg text-gray-600 dark:text-gray-400 mb-8",
                "A professional task management app built with Dioxus and Rust."
            }

            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                div { class: "lg:col-span-1",
                    TodoForm { on_submit: handle_add }
                }

                div { class: "lg:col-span-2 space-y-6",
                    TodoFilters {
                        current_filter: filter(),
                        on_filter_change: move |f| filter.set(f),
                        search_query: search_query(),
                        on_search_change: move |q| search_query.set(q),
                        total_count: total_count,
                        active_count: active_count,
                    }

                    div { class: "space-y-3",
                        if filtered_todos.is_empty() {
                            div { class: "text-center py-12 bg-white dark:bg-gray-800 rounded-xl shadow-lg",
                                div { class: "text-6xl mb-4", "📝" }
                                p { class: "text-gray-500 dark:text-gray-400 text-lg",
                                    if search_query().is_empty() {
                                        "No tasks yet. Add your first task!"
                                    } else {
                                        "No tasks match your search."
                                    }
                                }
                            }
                        } else {
                            for todo in filtered_todos {
                                TodoItem {
                                    todo: todo.clone(),
                                    on_toggle: handle_toggle,
                                    on_delete: handle_delete,
                                    on_edit: handle_edit,
                                }
                            }
                        }
                    }

                    if total_count > 0 {
                        div { class: "bg-white dark:bg-gray-800 p-4 rounded-xl shadow-lg",
                            div { class: "flex justify-between text-sm text-gray-600 dark:text-gray-400",
                                span { "Total: {total_count}" }
                                span { "Active: {active_count}" }
                                span { "Completed: {total_count - active_count}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
