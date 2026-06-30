use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimelineItem {
    pub year: String,
    pub title: String,
    pub company: String,
    pub description: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    pub items: Vec<TimelineItem>,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    rsx! {
        div { class: "space-y-8",
            for item in props.items.iter() {
                div { class: "flex gap-4",
                    // Year
                    div { class: "flex-shrink-0 w-20",
                        span { class: "text-sm font-bold text-blue-600 dark:text-blue-400", "{item.year}" }
                    }

                    // Line and dot
                    div { class: "flex flex-col items-center",
                        div { class: "w-3 h-3 bg-blue-600 rounded-full" }
                        div { class: "w-0.5 flex-1 bg-gray-300 dark:bg-gray-600" }
                    }

                    // Content
                    div { class: "flex-1 pb-8",
                        h3 { class: "text-lg font-bold text-gray-900 dark:text-gray-100", "{item.title}" }
                        p { class: "text-sm text-gray-600 dark:text-gray-400 mb-2", "{item.company}" }
                        p { class: "text-gray-700 dark:text-gray-300", "{item.description}" }
                    }
                }
            }
        }
    }
}
