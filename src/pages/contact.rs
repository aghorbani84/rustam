use dioxus::prelude::*;
use crate::components::{button::{Button, ButtonVariant}, input::Input};

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut submitted = use_signal(|| false);

    let mut handle_submit = move || {
        log::info!("Contact form submitted: {} - {} - {}", name(), email(), message());
        submitted.set(true);
    };

    rsx! {
        div { class: "max-w-4xl mx-auto px-8 py-16 animate-fade-in",
            h1 { class: "text-4xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Get In Touch" }
            p { class: "text-lg text-gray-600 dark:text-gray-400 mb-12",
                "Have a project in mind or just want to say hello? Feel free to reach out!"
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-12",
                div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg",
                    if submitted() {
                        div { class: "text-center py-12",
                            div { class: "text-6xl mb-4", "✅" }
                            h2 { class: "text-2xl font-bold mb-2 text-gray-900 dark:text-gray-100", "Message Sent!" }
                            p { class: "text-gray-600 dark:text-gray-400 mb-6", "Thank you for reaching out. I'll get back to you soon!" }
                            Button {
                                variant: ButtonVariant::Secondary,
                                onclick: move |_| {
                                    submitted.set(false);
                                    name.set(String::new());
                                    email.set(String::new());
                                    message.set(String::new());
                                },
                                "Send Another Message"
                            }
                        }
                    } else {
                        h2 { class: "text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100", "Send Me a Message" }
                        form {
                            class: "space-y-6",
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                handle_submit();
                            },

                            Input {
                                label: "Your Name",
                                r#type: "text",
                                placeholder: "John Doe",
                                value: name(),
                                oninput: move |val| name.set(val),
                            }

                            Input {
                                label: "Your Email",
                                r#type: "email",
                                placeholder: "john@example.com",
                                value: email(),
                                oninput: move |val| email.set(val),
                            }

                            div { class: "space-y-1.5",
                                label {
                                    class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "Message"
                                }
                                textarea {
                                    class: "block w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors",
                                    rows: 5,
                                    placeholder: "Your message here...",
                                    value: "{message}",
                                    oninput: move |evt| message.set(evt.value()),
                                }
                            }

                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| handle_submit(),
                                "Send Message"
                            }
                        }
                    }
                }

                div { class: "space-y-6",
                    div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg",
                        h3 { class: "text-xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Contact Information" }
                        div { class: "space-y-4",
                            div { class: "flex items-center gap-3",
                                span { class: "text-2xl", "📧" }
                                div {
                                    p { class: "text-sm text-gray-600 dark:text-gray-400", "Email" }
                                    a { href: "mailto:abolfazlghorbani369@gmail.com", class: "text-blue-600 hover:text-blue-700 font-medium", "abolfazlghorbani369@gmail.com" }
                                }
                            }
                            div { class: "flex items-center gap-3",
                                span { class: "text-2xl", "📍" }
                                div {
                                    p { class: "text-sm text-gray-600 dark:text-gray-400", "Location" }
                                    p { class: "text-gray-900 dark:text-gray-100 font-medium", "Iran" }
                                }
                            }
                        }
                    }

                    div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg",
                        h3 { class: "text-xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Follow Me" }
                        p { class: "text-gray-600 dark:text-gray-400 mb-4", "Connect with me on social media" }
                        div { class: "flex gap-3",
                            a { href: "https://gitlab.com/abolfazlghorbani369", target: "_blank", class: "flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors text-center", "GitLab" }
                            a { href: "https://github.com/abolfazlghorbani369", target: "_blank", class: "flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors text-center", "GitHub" }
                        }
                    }
                }
            }
        }
    }
}
