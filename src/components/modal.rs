use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub is_open: bool,
    pub onclose: EventHandler<()>,
    pub title: String,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.is_open {
        return rsx! { };
    }

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center animate-fade-in",

            div {
                class: "absolute inset-0 bg-black/50 backdrop-blur-sm",
                onclick: move |_| props.onclose.call(()),
            }

            div {
                class: "relative w-full max-w-lg bg-white dark:bg-gray-800 rounded-xl shadow-2xl p-6 z-10",

                div { class: "flex items-center justify-between mb-4",
                    h3 { class: "text-xl font-semibold text-gray-900 dark:text-gray-100", "{props.title}" }
                    button {
                        class: "text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 transition-colors",
                        onclick: move |_| props.onclose.call(()),
                        "✕"
                    }
                }

                div {
                    {props.children}
                }
            }
        }
    }
}
