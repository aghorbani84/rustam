use dioxus::prelude::*;
use crate::models::pomodoro::PomodoroStats;

#[derive(Props, Clone, PartialEq)]
pub struct StatsDisplayProps {
    pub stats: PomodoroStats,
    pub current_session: u32,
    pub sessions_before_long: u32,
}

#[component]
pub fn StatsDisplay(props: StatsDisplayProps) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg",
            h3 { class: "text-lg font-bold text-gray-900 dark:text-gray-100 mb-4", "Statistics" }

            div { class: "grid grid-cols-2 gap-4",
                div { class: "text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg",
                    div { class: "text-3xl font-bold text-blue-600 dark:text-blue-400", "{props.stats.total_sessions}" }
                    div { class: "text-sm text-gray-600 dark:text-gray-400 mt-1", "Total Sessions" }
                }

                div { class: "text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg",
                    div { class: "text-3xl font-bold text-green-600 dark:text-green-400", "{props.stats.total_work_minutes}" }
                    div { class: "text-sm text-gray-600 dark:text-gray-400 mt-1", "Work Minutes" }
                }

                div { class: "text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg",
                    div { class: "text-3xl font-bold text-purple-600 dark:text-purple-400", "{props.stats.current_streak}" }
                    div { class: "text-sm text-gray-600 dark:text-gray-400 mt-1", "Current Streak" }
                }

                div { class: "text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg",
                    div { class: "text-3xl font-bold text-orange-600 dark:text-orange-400", "{props.stats.longest_streak}" }
                    div { class: "text-sm text-gray-600 dark:text-gray-400 mt-1", "Longest Streak" }
                }
            }

            // Progress to long break
            div { class: "mt-6",
                div { class: "flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-2",
                    span { "Session {props.current_session} of {props.sessions_before_long}" }
                    span { "Until Long Break" }
                }
                div { class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2",
                    div {
                        class: "bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all",
                        style: "width: {(props.current_session as f32 / props.sessions_before_long as f32 * 100.0) as i32}%"
                    }
                }
            }
        }
    }
}
