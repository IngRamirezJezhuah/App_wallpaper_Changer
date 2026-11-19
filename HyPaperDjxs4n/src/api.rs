use crate::models::{ApplyResult, FilterKind, WallpaperItem};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = convertFileSrc)]
    fn convert_file_src_js(file_path: &str) -> String;
}

#[derive(Serialize)]
struct GetWallpapersArgs {
    filter: FilterKind,
}

#[derive(Serialize)]
struct ApplyWallpaperArgs {
    wallpaper: WallpaperItem,
}

pub fn asset_url(path: &str) -> String {
    convert_file_src_js(path)
}

pub async fn fetch_wallpapers(filter: FilterKind) -> Vec<WallpaperItem> {
    let Ok(args) = serde_wasm_bindgen::to_value(&GetWallpapersArgs { filter }) else {
        return Vec::new();
    };
    let res = invoke("get_wallpapers", args).await;
    serde_wasm_bindgen::from_value(res).unwrap_or_default()
}

pub async fn apply_wallpaper(wallpaper: WallpaperItem) -> Option<ApplyResult> {
    let args = serde_wasm_bindgen::to_value(&ApplyWallpaperArgs { wallpaper }).ok()?;
    let res = invoke("apply_wallpaper", args).await;
    serde_wasm_bindgen::from_value(res).ok()
}