use core::str;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
//use std::process::Command;
use std::fs;
use std::path::Path;

#[derive(Default)]
struct Wallpaper {
    path: String,
    name: String,
    type_ext: String, 
}

#[tauri::command]
async fn get_wallpapers() -> Vec<Wallpaper> {
    let path = Path::new("./public/wallpapers/");
    
    if !path.exists() || !path.is_dir() {
        return vec![];
    }

    let mut wallpapers = Vec::new();
    for entry in fs::read_dir(path).expect("No se pudo leer el directorio") {
        let entry = entry.expect("Error al leeer entrada del directorio");
        let path = entry.path();
        if path.is_file() {
            let _metadata = fs::metadata(&path).expect("No se pudo obtener metadatos del archivo");
            let file_type_ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

            wallpapers.push(Wallpaper {
                path: path.to_string_lossy().to_string(),
                name: entry.file_name().to_string_lossy().to_string(),
                type_ext : file_type_ext.to_string(),
            });
        }
    }
    wallpapers
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
    