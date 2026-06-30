use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "space-y-6 animate-fade-in",
            h2 { class: "text-2xl font-bold", "Dashboard Overview" }

            div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                div { class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                    h3 { class: "text-gray-500 dark:text-gray-400 text-sm font-medium", "Total Users" }
                    p { class: "mt-2 text-3xl font-bold", "12,450" }
                }
                div { class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                    h3 { class: "text-gray-500 dark:text-gray-400 text-sm font-medium", "Revenue" }
                    p { class: "mt-2 text-3xl font-bold", "$84,200" }
                }
                div { class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700",
                    h3 { class: "text-gray-500 dark:text-gray-400 text-sm font-medium", "Active Sessions" }
                    p { class: "mt-2 text-3xl font-bold", "1,204" }
                }
            }
        }
    }
}
