use dioxus::prelude::*;

#[component]
pub fn ProjectDetail() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-8 py-16",
            h1 { class: "text-4xl font-bold mb-4", "Project Detail" }
            p { "This page will show detailed information about a specific project." }
        }
    }
}
