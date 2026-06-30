mod app;
mod components;
mod pages;
mod state;
mod hooks;
mod services;
mod utils;
mod models;


fn main() {
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    log::info!("🚀 Starting Dioxus App...");

    // Launch app
    dioxus::launch(app::App);
}
