use dioxus::prelude::*;
use crate::components::{button::{Button, ButtonVariant}, input::Input};

#[component]
pub fn Register() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());

    // Business logic جدا از UI
    let handle_register = move || {
        log::info!("Attempting registration for: {}", email());
        // اینجا می‌توانیم API call یا validation اضافه کنیم
    };

    rsx! {
        div { class: "flex items-center justify-center min-h-[80vh] animate-fade-in",
            div { class: "w-full max-w-md p-8 bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700",
                h2 { class: "text-2xl font-bold text-center mb-6 text-gray-900 dark:text-gray-100", "Create Account" }

                form {
                    class: "space-y-6",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        handle_register();
                    },

                    Input {
                        label: "Email",
                        r#type: "email",
                        placeholder: "you@example.com",
                        value: email(),
                        oninput: move |val| email.set(val),
                    }

                    Input {
                        label: "Password",
                        r#type: "password",
                        placeholder: "••••••••",
                        value: password(),
                        oninput: move |val| password.set(val),
                    }

                    Input {
                        label: "Confirm Password",
                        r#type: "password",
                        placeholder: "••••••••",
                        value: confirm_password(),
                        oninput: move |val| confirm_password.set(val),
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| handle_register(),
                        "Create Account"
                    }
                }
            }
        }
    }
}
