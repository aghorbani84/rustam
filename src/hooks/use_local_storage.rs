use dioxus::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use web_sys::window;

pub fn use_local_storage<T: Serialize + DeserializeOwned + Clone + PartialEq + 'static>(
    key: &str,
    default: T,
) -> Signal<T> {
    let storage_key = key.to_string();
    
    use_signal(move || {
        if let Some(window) = window()
            && let Ok(Some(storage)) = window.local_storage()
            && let Ok(Some(value)) = storage.get_item(&storage_key)
            && let Ok(parsed) = serde_json::from_str::<T>(&value) {
            parsed
        } else {
            default.clone()
        }
    })
}

pub fn save_to_local_storage<T: Serialize>(key: &str, value: &T) {
    if let Some(window) = window()
        && let Ok(Some(storage)) = window.local_storage()
        && let Ok(json) = serde_json::to_string(value) {
        let _ = storage.set_item(key, &json);
    }
}
