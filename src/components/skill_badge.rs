use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SkillBadgeProps {
    pub name: String,
    pub level: u8, // 0-100
    pub icon: String,
}

#[component]
pub fn SkillBadge(props: SkillBadgeProps) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md hover:shadow-lg transition-shadow",
            div { class: "flex items-center gap-3 mb-2",
                span { class: "text-2xl", "{props.icon}" }
                span { class: "font-semibold text-gray-900 dark:text-gray-100", "{props.name}" }
            }
            div { class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2",
                div {
                    class: "bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all duration-500",
                    style: "width: {props.level}%"
                }
            }
            div { class: "text-right mt-1",
                span { class: "text-sm text-gray-600 dark:text-gray-400", "{props.level}%" }
            }
        }
    }
}
