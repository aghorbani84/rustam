use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PomodoroSettings {
    pub work_duration: u32,      // minutes
    pub short_break: u32,        // minutes
    pub long_break: u32,         // minutes
    pub sessions_before_long: u32,
}

impl Default for PomodoroSettings {
    fn default() -> Self {
        Self {
            work_duration: 25,
            short_break: 5,
            long_break: 15,
            sessions_before_long: 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PomodoroStats {
    pub total_sessions: u32,
    pub total_work_minutes: u32,
    pub total_break_minutes: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimerMode {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
}
