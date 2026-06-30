use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    pub label: String,
    pub r#type: String,
    pub placeholder: String,

    #[props(default = false)]
    pub error: bool,

    pub error_message: Option<String>,

    pub value: String,
    pub oninput: EventHandler<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let border_color = if props.error {
        "border-red-500 focus:ring-red-500"
    } else {
        "border-gray-300 dark:border-gray-600 focus:ring-brand-500"
    };

    rsx! {
        div { class: "space-y-1.5",
            label {
                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                "{props.label}"
            }

            input {
                class: "block w-full px-4 py-2.5 rounded-lg border {border_color} bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-2 focus:border-transparent transition-colors",
                r#type: "{props.r#type}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                oninput: move |evt| {
                    props.oninput.call(evt.value());
                },
                "aria-invalid": props.error,
            }

            if props.error {
                if let Some(msg) = &props.error_message {
                    p {
                        class: "text-sm text-red-600 dark:text-red-400 animate-fade-in",
                        "{msg}"
                    }
                }
            }
        }
    }
}
