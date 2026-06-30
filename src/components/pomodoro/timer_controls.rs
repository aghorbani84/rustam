use dioxus::prelude::*;
use crate::models::pomodoro::TimerState;

#[derive(Props, Clone, PartialEq)]
pub struct TimerControlsProps {
    pub timer_state: TimerState,
    pub on_start: EventHandler<()>,
    pub on_pause: EventHandler<()>,
    pub on_resume: EventHandler<()>,
    pub on_reset: EventHandler<()>,
    pub on_skip: EventHandler<()>,
}

#[component]
pub fn TimerControls(props: TimerControlsProps) -> Element {
    rsx! {
        div { class: "flex gap-4 justify-center mt-8",
            match props.timer_state {
                TimerState::Idle => {
                    rsx! {
                        button {
                            class: "px-8 py-3 bg-green-600 text-white font-semibold rounded-lg hover:bg-green-700 transition-colors shadow-lg",
                            onclick: move |_| props.on_start.call(()),
                            "Start"
                        }
                    }
                }
                TimerState::Running => {
                    rsx! {
                        button {
                            class: "px-8 py-3 bg-yellow-600 text-white font-semibold rounded-lg hover:bg-yellow-700 transition-colors shadow-lg",
                            onclick: move |_| props.on_pause.call(()),
                            "Pause"
                        }
                        button {
                            class: "px-8 py-3 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-100 font-semibold rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors",
                            onclick: move |_| props.on_skip.call(()),
                            "Skip"
                        }
                    }
                }
                TimerState::Paused => {
                    rsx! {
                        button {
                            class: "px-8 py-3 bg-green-600 text-white font-semibold rounded-lg hover:bg-green-700 transition-colors shadow-lg",
                            onclick: move |_| props.on_resume.call(()),
                            "Resume"
                        }
                        button {
                            class: "px-8 py-3 bg-red-600 text-white font-semibold rounded-lg hover:bg-red-700 transition-colors shadow-lg",
                            onclick: move |_| props.on_reset.call(()),
                            "Reset"
                        }
                    }
                }
            }
        }
    }
}
