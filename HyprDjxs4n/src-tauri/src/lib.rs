use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
//importacion de los archivos para no tener un archivo enorme
mod scanner;
mod commands;
//mod config;

// Punto de entrada invocado desde main.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
                commands::get_wallpapers,
                commands::apply_wallpaper
            ])
        .run(tauri::generate_context!())
        .expect("Error ejecutando la aplicación de Tauri");
}