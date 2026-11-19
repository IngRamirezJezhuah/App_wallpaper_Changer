use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    Image,
    Gif,
    Video,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum FilterKind {
    All,
    Img,
    Gif,
    Vid,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct WallpaperItem {
    pub path: String,
    pub name: String,
    pub ext: String,
    pub kind: MediaKind,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplyResult {
    pub success: bool,
    pub engine: String,
    pub message: String,
}
