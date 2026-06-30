use dioxus::prelude::*;
use crate::components::hero::Hero;
use crate::components::social_links::SocialLinks;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "animate-fade-in",
            Hero {
                name: "Abolfazl Ghorbani",
                title: "Full-Stack Developer & Rust Enthusiast",
                subtitle: "I build scalable web applications with modern technologies. Passionate about clean code, performance, and user experience."
            }

            section { class: "py-20 px-8 bg-white dark:bg-gray-800",
                div { class: "max-w-4xl mx-auto text-center",
                    h2 { class: "text-3xl font-bold mb-6 text-gray-900 dark:text-gray-100", "About Me" }
                    p { class: "text-lg text-gray-600 dark:text-gray-400 mb-8",
                        "I'm a passionate developer with expertise in Rust, Dioxus, and modern web technologies. "
                        "I love building fast, reliable, and beautiful applications that solve real-world problems."
                    }
                    SocialLinks {}
                }
            }
        }
    }
}
