use dioxus::prelude::*;
use crate::components::pomodoro::{
    timer_display::TimerDisplay,
    timer_controls::TimerControls,
    timer_settings::TimerSettings,
    stats_display::StatsDisplay,
};
use crate::hooks::use_local_storage::{use_local_storage, save_to_local_storage};
use crate::hooks::use_interval::use_interval;
use crate::models::pomodoro::{
    PomodoroSettings, PomodoroStats, TimerMode, TimerState,
};
use std::time::Duration;

#[component]
pub fn Pomodoro() -> Element {
    let mut settings = use_local_storage::<PomodoroSettings>("pomodoro_settings", PomodoroSettings::default());
    let mut stats = use_local_storage::<PomodoroStats>("pomodoro_stats", PomodoroStats::default());

    let mut timer_state = use_signal(|| TimerState::Idle);
    let mut mode = use_signal(|| TimerMode::Work);
    let mut time_left = use_signal(|| settings().work_duration * 60);
    let mut current_session = use_signal(|| 1u32);

    // Get total time for current mode
    let total_time = match mode() {
        TimerMode::Work => settings().work_duration * 60,
        TimerMode::ShortBreak => settings().short_break * 60,
        TimerMode::LongBreak => settings().long_break * 60,
    };

    // Handle timer completion - با mut
    let mut handle_timer_complete = move || {
        let mut current_stats = stats();
        match mode() {
            TimerMode::Work => {
                current_stats.total_sessions += 1;
                current_stats.total_work_minutes += settings().work_duration;
                current_stats.current_streak += 1;
                if current_stats.current_streak > current_stats.longest_streak {
                    current_stats.longest_streak = current_stats.current_streak;
                }
            }
            TimerMode::ShortBreak | TimerMode::LongBreak => {
                current_stats.total_break_minutes += match mode() {
                    TimerMode::ShortBreak => settings().short_break,
                    TimerMode::LongBreak => settings().long_break,
                    _ => 0,
                };
            }
        }
        stats.set(current_stats);

        match mode() {
            TimerMode::Work => {
                if current_session() >= settings().sessions_before_long {
                    mode.set(TimerMode::LongBreak);
                    time_left.set(settings().long_break * 60);
                    current_session.set(1);
                } else {
                    mode.set(TimerMode::ShortBreak);
                    time_left.set(settings().short_break * 60);
                    current_session.set(current_session() + 1);
                }
            }
            TimerMode::ShortBreak | TimerMode::LongBreak => {
                mode.set(TimerMode::Work);
                time_left.set(settings().work_duration * 60);
            }
        }

        timer_state.set(TimerState::Idle);
        log::info!("Timer completed! Switching to next mode.");
    };

    // Timer interval
    use_interval(
        move || {
            if timer_state() == TimerState::Running {
                let current_time = time_left();
                if current_time > 0 {
                    time_left.set(current_time - 1);
                } else {
                    handle_timer_complete();
                }
            }
        },
        Duration::from_secs(1),
    );

    // Save settings and stats to localStorage
    use_effect(move || {
        save_to_local_storage("pomodoro_settings", &settings());
    });

    use_effect(move || {
        save_to_local_storage("pomodoro_stats", &stats());
    });

    // Control handlers
    let handle_start = move |_| {
        timer_state.set(TimerState::Running);
    };

    let handle_pause = move |_| {
        timer_state.set(TimerState::Paused);
    };

    let handle_resume = move |_| {
        timer_state.set(TimerState::Running);
    };

    let handle_reset = move |_| {
        timer_state.set(TimerState::Idle);
        time_left.set(total_time);
    };

    let handle_skip = move |_| {
        handle_timer_complete();
    };

    let handle_settings_change = move |new_settings: PomodoroSettings| {
        settings.set(new_settings.clone());
        if timer_state() == TimerState::Idle {
            time_left.set(match mode() {
                TimerMode::Work => new_settings.work_duration * 60,
                TimerMode::ShortBreak => new_settings.short_break * 60,
                TimerMode::LongBreak => new_settings.long_break * 60,
            });
        }
    };

    rsx! {
        div { class: "max-w-6xl mx-auto px-8 py-16 animate-fade-in",
            h1 { class: "text-4xl font-bold mb-4 text-gray-900 dark:text-gray-100", "Pomodoro Timer" }
            p { class: "text-lg text-gray-600 dark:text-gray-400 mb-12",
                "Stay focused and productive with the Pomodoro Technique."
            }

            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                div { class: "lg:col-span-2 space-y-8",
                    div { class: "bg-white dark:bg-gray-800 p-8 rounded-xl shadow-lg",
                        TimerDisplay {
                            time_left: time_left(),
                            total_time: total_time,
                            mode: mode(),
                            is_running: timer_state() == TimerState::Running,
                        }

                        TimerControls {
                            timer_state: timer_state(),
                            on_start: handle_start,
                            on_pause: handle_pause,
                            on_resume: handle_resume,
                            on_reset: handle_reset,
                            on_skip: handle_skip,
                        }
                    }

                    TimerSettings {
                        settings: settings(),
                        on_settings_change: handle_settings_change,
                    }
                }

                div { class: "lg:col-span-1",
                    StatsDisplay {
                        stats: stats(),
                        current_session: current_session(),
                        sessions_before_long: settings().sessions_before_long,
                    }

                    div { class: "bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg mt-6",
                        h3 { class: "text-lg font-bold text-gray-900 dark:text-gray-100 mb-4", "💡 Tips" }
                        ul { class: "space-y-2 text-sm text-gray-700 dark:text-gray-300",
                            li { "🎯 Focus on one task at a time" }
                            li { "📵 Remove distractions during work sessions" }
                            li { "🚶 Take real breaks - step away from screen" }
                            li { "💧 Stay hydrated during breaks" }
                            li { "📝 Track what you accomplish each session" }
                        }
                    }
                }
            }
        }
    }
}
