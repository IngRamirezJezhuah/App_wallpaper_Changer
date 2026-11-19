// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod models;
mod config;
mod state;
mod scanner;
mod commands;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_wallpapers,
            commands::apply_wallpaper,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
