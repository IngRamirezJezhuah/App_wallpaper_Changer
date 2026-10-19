use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::fs;
/*
Notas de mantenimiento para mi yo del futuro
Este archivo es el que escanea la carpeta de wallpapers
aqui la busca y la escanea
*/

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub type_ext: String,
    pub is_animated: bool,
    pub is_placeholder: bool,
}

// variable que busca la carpeta de wallpaper
pub fn get_wallpaper_dir() -> PathBuf{
    let dir = dirs::picture_dir();
    let mut path = dir.unwrap_or_else(|| {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures")
    });
    path.push("wallpapers");

    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

//variable que escanea el contenido de la carpeta de wallpapers
pub fn scan_dir(dir: &Path, filter: &str) -> Vec<Wallpaper> {
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

