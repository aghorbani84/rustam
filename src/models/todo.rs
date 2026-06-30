use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: u64,
    pub priority: Priority,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl Todo {
    pub fn new(title: String, priority: Priority) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            completed: false,
            created_at: js_sys::Date::now() as u64,
            priority,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TodoFilter {
    #[default]
    All,
    Active,
    Completed,
}
