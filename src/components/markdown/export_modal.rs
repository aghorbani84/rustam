use crate::components::modal::Modal;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Props, Clone, PartialEq)]
pub struct ExportModalProps {
    pub is_open: bool,
    pub on_close: EventHandler<()>,
    pub title: String,
    pub markdown_content: String,
    pub html_content: String,
}

#[component]
pub fn ExportModal(props: ExportModalProps) -> Element {
    let mut copied = use_signal(|| false);
    let mut export_type = use_signal(|| ExportType::Html);

    #[derive(Clone, PartialEq)]
    enum ExportType {
        Html,
        Markdown,
    }

    let current_content = match export_type() {
        ExportType::Html => props.html_content.clone(),
        ExportType::Markdown => props.markdown_content.clone(),
    };

    // Clone جداگانه برای هر closure
    let content_for_copy = current_content.clone();
    let content_for_download = current_content.clone();
    let content_for_preview = current_content.clone();

    let handle_copy = move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(navigator) = window.navigator().dyn_ref::<web_sys::Navigator>() {
                if let Some(clipboard) = navigator.clipboard().dyn_ref::<web_sys::Clipboard>() {
                    let _ = clipboard.write_text(&content_for_copy);
                }
            }
        }
        copied.set(true);

        let mut copied_signal = copied.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            copied_signal.set(false);
        });
    };

    let handle_download = move |_| {
        let content = content_for_download.clone();
        let filename = match export_type() {
            ExportType::Html => format!("{}.html", props.title.replace(' ', "_")),
            ExportType::Markdown => format!("{}.md", props.title.replace(' ', "_")),
        };

        download_file(&filename, &content);
    };

    rsx! {
        Modal {
            is_open: props.is_open,
            onclose: move |_| props.on_close.call(()),
            title: "Export Document",

            div { class: "space-y-4",
                div { class: "flex gap-2",
                    button {
                        class: if export_type() == ExportType::Html {
                            "flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg font-medium"
                        } else {
                            "flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600"
                        },
                        onclick: move |_| export_type.set(ExportType::Html),
                        "HTML"
                    }
                    button {
                        class: if export_type() == ExportType::Markdown {
                            "flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg font-medium"
                        } else {
                            "flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600"
                        },
                        onclick: move |_| export_type.set(ExportType::Markdown),
                        "Markdown"
                    }
                }

                div { class: "bg-gray-50 dark:bg-gray-800 p-4 rounded-lg max-h-64 overflow-auto",
                    pre { class: "text-xs text-gray-700 dark:text-gray-300 whitespace-pre-wrap font-mono",
                        "{content_for_preview}"
                    }
                }

                div { class: "flex gap-3",
                    button {
                        class: "flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg font-medium hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors",
                        onclick: handle_copy,
                        if copied() { "✅ Copied!" } else { "📋 Copy to Clipboard" }
                    }
                    button {
                        class: "flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors",
                        onclick: handle_download,
                        "💾 Download File"
                    }
                }
            }
        }
    }
}

fn download_file(filename: &str, content: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let parts = js_sys::Array::new();
            parts.push(&js_sys::JsString::from(content).into());

            let mut props = web_sys::BlobPropertyBag::new();
            props.type_("text/plain");

            if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &props) {
                if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                    if let Ok(link) = document.create_element("a") {
                        if let Ok(link) = link.dyn_into::<web_sys::HtmlElement>() {
                            let _ = link.set_attribute("href", &url);
                            let _ = link.set_attribute("download", filename);
                            link.click();
                            let _ = web_sys::Url::revoke_object_url(&url);
                        }
                    }
                }
            }
        }
    }
}
