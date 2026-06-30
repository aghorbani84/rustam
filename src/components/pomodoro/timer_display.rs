use dioxus::prelude::*;
use crate::models::pomodoro::TimerMode;

#[derive(Props, Clone, PartialEq)]
pub struct TimerDisplayProps {
    pub time_left: u32, // seconds
    pub total_time: u32, // seconds
    pub mode: TimerMode,
    pub is_running: bool,
}

#[component]
pub fn TimerDisplay(props: TimerDisplayProps) -> Element {
    let minutes = props.time_left / 60;
    let seconds = props.time_left % 60;

    let progress = if props.total_time > 0 {
        (props.total_time - props.time_left) as f32 / props.total_time as f32
    } else {
        0.0
    };

    let circumference = 2.0 * std::f32::consts::PI * 45.0;
    let stroke_dashoffset = circumference * (1.0 - progress);

    let _mode_color = match props.mode {
        TimerMode::Work => "text-red-500",
        TimerMode::ShortBreak => "text-green-500",
        TimerMode::LongBreak => "text-blue-500",
    };

    let mode_bg = match props.mode {
        TimerMode::Work => "stroke-red-500",
        TimerMode::ShortBreak => "stroke-green-500",
        TimerMode::LongBreak => "stroke-blue-500",
    };

    let mode_label = match props.mode {
        TimerMode::Work => "Focus Time",
        TimerMode::ShortBreak => "Short Break",
        TimerMode::LongBreak => "Long Break",
    };

    rsx! {
        div { class: "flex flex-col items-center justify-center",
            // Mode label
            div { class: "mb-8 text-center",
                h2 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2", "{mode_label}" }
                p { class: "text-gray-600 dark:text-gray-400",
                    if props.is_running {
                        "Stay focused!"
                    } else {
                        "Ready to start?"
                    }
                }
            }

            // Circular timer
            div { class: "relative w-64 h-64",
                svg {
                    class: "transform -rotate-90 w-full h-full",
                    view_box: "0 0 100 100",

                    // Background circle
                    circle {
                        class: "stroke-gray-200 dark:stroke-gray-700",
                        cx: "50",
                        cy: "50",
                        r: "45",
                        fill: "none",
                        stroke_width: "8"
                    }

                    // Progress circle
                    circle {
                        class: "{mode_bg} transition-all duration-1000 ease-linear",
                        cx: "50",
                        cy: "50",
                        r: "45",
                        fill: "none",
                        stroke_width: "8",
                        stroke_linecap: "round",
                        stroke_dasharray: "{circumference}",
                        stroke_dashoffset: "{stroke_dashoffset}"
                    }
                }

                // Time display
                div { class: "absolute inset-0 flex items-center justify-center",
                    div { class: "text-center",
                        div { class: "text-6xl font-bold text-gray-900 dark:text-gray-100",
                            "{minutes:02}:{seconds:02}"
                        }
                    }
                }
            }
        }
    }
}
