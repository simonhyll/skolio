// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    app_lib::run();
}
