use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewProps {
    pub html: String,
}

#[component]
pub fn Preview(props: PreviewProps) -> Element {
    rsx! {
        div {
            class: "p-8 prose prose-lg dark:prose-invert max-w-none h-full overflow-auto bg-white dark:bg-gray-900",
            dangerous_inner_html: "{props.html}"
        }
    }
}
