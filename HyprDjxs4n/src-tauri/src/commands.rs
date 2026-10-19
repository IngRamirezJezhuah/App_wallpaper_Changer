use serde::Serialize;
use crate::scanner::{Wallpaper, get_wallpaper_dir, scan_dir};
use std::{fmt::format, path::{self, PathBuf}};
/*
Notas de mantenimiento para mi yo del futuro
Este archivo es el que se encarga de seleccionar el wallpaper
y el de aplicarlo es lo que lo escoje valgame
*/
#[derive(Serialize)]
pub struct ApplyResult {
    pub success: bool,
    pub engine : String,
    pub  message: String,
}
#[tauri::command]
pub fn get_wallpapers(filter: String) -> Vec<Wallpaper> {
    let dir: PathBuf = get_wallpaper_dir();
    let mut wallpapers = scan_dir(&dir, &filter);
    let count = wallpapers.len();
    if count < 5 {
        for i in count..5{
            wallpapers.push(Wallpaper{
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

#[tauri::command]
pub fn apply_wallpaper(wallpaper: Wallpaper) -> ApplyResult {
    if wallpaper.is_placeholder {
        return ApplyResult { 
            success: false, 
            engine: "none".into(), 
            message: "Placeholder, no hay anda que aplicar".into(),
        };
    }
    ApplyResult {
        success: true,
        engine: "none".into(),
        message: format!("(stub) Ruta recibida: {}", wallpaper.path),
    }
}

