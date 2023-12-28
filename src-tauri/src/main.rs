#![allow(dead_code, unused_variables)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;

mod config;
mod error;
mod utils;
mod tauri_impl;

fn main() -> Result<(), error::Error> {
    let config = Config::load()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tauri_impl::commands::get_servers,
            tauri_impl::commands::get_server,
            tauri_impl::commands::start_server,
            tauri_impl::commands::stop_server,
            tauri_impl::commands::rename_server,
            tauri_impl::commands::is_server_running,
            tauri_impl::commands::delete_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri_impl application");

    Ok(())
}
