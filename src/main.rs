mod app;
mod components;
mod hooks;
mod models;
mod pages;
mod services;
mod state;
mod utils;

fn main() {
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    log::info!("🚀 Starting Dioxus App...");

    // Launch app
    dioxus::launch(app::App);
}
