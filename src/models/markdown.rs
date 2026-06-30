use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MarkdownDocument {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[allow(dead_code)]
impl MarkdownDocument {
    pub fn new(title: String) -> Self {
        let now = js_sys::Date::now() as u64;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn default_content() -> String {
        "# Welcome to Markdown Editor ✍️\n\nThis is a **professional** markdown editor built with *Rust* and `Dioxus`.\n\n## Features\n\n- 📝 Real-time preview\n- 💾 Auto-save to LocalStorage\n- 📤 Export to HTML or Markdown\n- 🎨 Syntax highlighting\n- 🌓 Dark mode support\n\n## Code Example\n\n```rust\nfn main() {\n    println!(\"Hello, Dioxus!\");\n}\n```\n\n## Links & Images\n\nVisit [Dioxus Labs](https://dioxuslabs.com) for more info.\n\n> \"The best way to predict the future is to invent it.\" - Alan Kay\n\n---\n\n### Task List\n\n- [x] Build the editor\n- [x] Add preview\n- [ ] Add more features\n\nEnjoy writing! 🚀\n".to_string()
    }
}
