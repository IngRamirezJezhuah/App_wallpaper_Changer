use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub type_ext: String,
    pub is_animated: bool,
    pub is_placeholder: bool,
}

fn get_wallpaper_dir() -> PathBuf {
    /*let mut path = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("wallpapers");
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }*/// Detecta ~/Pictures o ~/Imágenes automáticamente según el idioma del sistema
    let mut path = dirs::picture_dir().unwrap_or_else(|| {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures")
    });
    
    // Subcarpeta del widget
    path.push("wallpapers");

    // Si la carpeta no existe, la crea automáticamente en el sistema del usuario
    if !path.exists() {
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

pub fn get_wallpapers(filter: String) -> Vec<Wallpaper> {
    let dir = get_wallpaper_dir();
    let mut wallpapers = scan_dir(&dir, &filter);

    let count = wallpapers.len();
    if count < 5 {
        for i in count..5 {
            wallpapers.push(Wallpaper {
                path: String::new(),
                name: format!("Empty {}", i + 1),
                type_ext: "none".to_string(),
                is_animated: false,
                is_placeholder: true,
            });
        }
    }
    wallpapers
}

pub async fn _apply_wallpaper(wallpaper: Wallpaper) {
    if wallpaper.is_placeholder {
        return;
    }
    println!("Aplicando wallpaper: {}", wallpaper.path);
}

// Punto de entrada invocado desde main.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error ejecutando la aplicación de Tauri");
}