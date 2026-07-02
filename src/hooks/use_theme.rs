use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Theme {
    Light,
    Dark,
    System,
}

pub fn use_theme_provider() -> Signal<Theme> {
    let theme_signal = use_signal(|| Theme::System);
    use_context_provider(|| theme_signal)
}

pub fn use_apply_theme() {
    let theme = use_theme_provider();
    
    use_effect(move || {
        if let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(body) = document.body() {
            match theme() {
                Theme::Dark => {
                    body.set_class_name("dark");
                }
                Theme::Light => {
                    body.set_class_name("");
                }
                Theme::System => {
                    if let Some(media_query) = window.match_media("(prefers-color-scheme: dark)").ok().flatten() {
                        if media_query.matches() {
                            body.set_class_name("dark");
                        } else {
                            body.set_class_name("");
                        }
                    }
                }
            }
        }
    });
}
