use crate::models::{ApplyResult, FilterKind, WallpaperItem };
use crate::scanner::{resolve_wallpaper_dir, scan_wallpapers};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_wallpapers(filter: FilterKind, state: State<'_, AppState>) -> Vec<WallpaperItem> {
    let custom_dir = state
        .inner
        .lock()
        .ok()
        .and_then(|s| s.config.wallpaper_dir.clone());

    let dir = resolve_wallpaper_dir(custom_dir.as_ref());
    scan_wallpapers(&dir, filter)
}

#[tauri::command]
pub fn apply_wallpaper(wallpaper: WallpaperItem) -> ApplyResult {
    ApplyResult {
        success: true,
        engine: "stub".into(),
        message: format!("Seleccionado ({:?}): {}", wallpaper.kind, wallpaper.name),
    }
}