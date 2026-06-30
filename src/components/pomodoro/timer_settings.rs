use dioxus::prelude::*;
use crate::models::pomodoro::PomodoroSettings;

#[derive(Props, Clone, PartialEq)]
pub struct TimerSettingsProps {
    pub settings: PomodoroSettings,
    pub on_settings_change: EventHandler<PomodoroSettings>,
}

#[component]
pub fn TimerSettings(props: TimerSettingsProps) -> Element {
    let mut is_open = use_signal(|| false);

    // Clone settings for each closure
    let settings_for_work = props.settings.clone();
    let settings_for_short = props.settings.clone();
    let settings_for_long = props.settings.clone();
    let settings_for_sessions = props.settings.clone();

    rsx! {
        div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg",
            button {
                class: "w-full flex items-center justify-between text-left",
                onclick: move |_| is_open.toggle(),

                h3 { class: "text-lg font-bold text-gray-900 dark:text-gray-100", "Settings" }
                span { class: "text-2xl text-gray-500", if is_open() { "▲" } else { "▼" } }
            }

            if is_open() {
                div { class: "mt-6 space-y-4",
                    // Work duration
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Work Duration: {props.settings.work_duration} minutes"
                        }
                        input {
                            class: "w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer",
                            r#type: "range",
                            min: "5",
                            max: "60",
                            value: "{props.settings.work_duration}",
                            oninput: move |evt| {
                                let value = evt.value().parse::<u32>().unwrap_or(25);
                                let mut new_settings = settings_for_work.clone();
                                new_settings.work_duration = value;
                                props.on_settings_change.call(new_settings);
                            }
                        }
                    }

                    // Short break
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Short Break: {props.settings.short_break} minutes"
                        }
                        input {
                            class: "w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer",
                            r#type: "range",
                            min: "1",
                            max: "15",
                            value: "{props.settings.short_break}",
                            oninput: move |evt| {
                                let value = evt.value().parse::<u32>().unwrap_or(5);
                                let mut new_settings = settings_for_short.clone();
                                new_settings.short_break = value;
                                props.on_settings_change.call(new_settings);
                            }
                        }
                    }

                    // Long break
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Long Break: {props.settings.long_break} minutes"
                        }
                        input {
                            class: "w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer",
                            r#type: "range",
                            min: "5",
                            max: "30",
                            value: "{props.settings.long_break}",
                            oninput: move |evt| {
                                let value = evt.value().parse::<u32>().unwrap_or(15);
                                let mut new_settings = settings_for_long.clone();
                                new_settings.long_break = value;
                                props.on_settings_change.call(new_settings);
                            }
                        }
                    }

                    // Sessions before long break
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Sessions before Long Break: {props.settings.sessions_before_long}"
                        }
                        input {
                            class: "w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer",
                            r#type: "range",
                            min: "2",
                            max: "8",
                            value: "{props.settings.sessions_before_long}",
                            oninput: move |evt| {
                                let value = evt.value().parse::<u32>().unwrap_or(4);
                                let mut new_settings = settings_for_sessions.clone();
                                new_settings.sessions_before_long = value;
                                props.on_settings_change.call(new_settings);
                            }
                        }
                    }
                }
            }
        }
    }
}
