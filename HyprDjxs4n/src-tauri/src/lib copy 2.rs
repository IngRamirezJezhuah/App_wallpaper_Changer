// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::{fs, path::{self, Path, PathBuf}};

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tauri::webview::cookie::time::ext;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub type_ext: String,
    pub is_animated: bool,
    pub is_placeholder: bool,
}

fn get_wallpaper_dir() -> PathBuf {
    let mut  path = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("wallpapers");
    if !path.exist() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

fn scan_dir(dir: &Path, filter: &str) -> Vec<Wallpaper> {
    let mut list = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    let is_anim = matches!(ext_lower.as_str(), "mp4" | "gif" | "webm");
                    let is_stat = matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp");

                    let include = match filter {
                        "VID" => is_anim,
                        "PIC" => is_stat,
                        _ => is_anim || is_stat,
                    };

                    if include {
                        list.push(Wallpaper {
                            // Convierte la ruta nativa a URL de protocolo seguro asset://
                            path: format!("asset://{}", path.to_string_lossy()),
                            name: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                            type_ext: ext_lower,
                            is_animated: is_anim,
                            is_placeholder: false,
                        });
                    }
                }
            }
        }
    }
    list
}

#[tauri::command]
pub fn get_wallpaper(filter: String) -> Vec<Wallpaper> {
    let dir = get_wallpaper_dir();
    let mut wallpapers = scan_dir(&dir, &filter);

    let count = wallpapers.len();
    if count < 5 {
        for i in count..5 {
            wallpapers.push(Wallpaper {
                path: String::new(),
                name: format!("Empty {}", i +1),
                type_ext: "none".to_string(),
                is_animated: false,
                is_placeholder: true,
            });
        }
    }
    wallpapers
}

#[tauri::command]
pub async fn apply_wallpaper( wallpaper: Wallpaper ) {
    if wallpaper.is_placeholder { return; }
    println!("Aplicando: {}", wallpaper.path);
}

/*
use serde::{Deserialize, Serialize};
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
*/