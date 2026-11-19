use crate::models::{FilterKind, MediaKind, WallpaperItem};
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_wallpaper_dir(custom: Option<&PathBuf>) -> PathBuf {
    let path = match custom {
        Some(dir) if !dir.as_os_str().is_empty() => dir.clone(),
        _ => dirs::picture_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures"))
            .join("wallpapers"),
    };

    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }

    path
}

fn classify_extension(ext_lower: &str) -> Option<MediaKind> {
    match ext_lower {
        "png" | "jpg" | "jpeg" | "webp" | "avif" | "bmp" => Some(MediaKind::Image),
        "gif" => Some(MediaKind::Gif),
        "mp4" | "webm" | "mkv" | "mov" => Some(MediaKind::Video),
        _ => None,
    }
}

pub fn scan_wallpapers(dir: &Path, filter: FilterKind) -> Vec<WallpaperItem> {
    let mut items = Vec::new();

    let Ok(entries) = fs::read_dir(dir) else {
        return items;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        let ext_lower = ext.to_lowercase();

        let Some(kind) = classify_extension(&ext_lower) else {
            continue;
        };

        let include = match filter {
            FilterKind::All => true,
            FilterKind::Img => kind == MediaKind::Image,
            FilterKind::Gif => kind == MediaKind::Gif,
            FilterKind::Vid => kind == MediaKind::Video,
        };

        if include {
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            items.push(WallpaperItem {
                path: path.to_string_lossy().to_string(),
                name,
                ext: ext_lower,
                kind,
            });
        }
    }

    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    items
}