use serde::{Deserialize,Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ConfigTransicion {
    pub r#type: String,
    pub duration: f64,
    pub fps: u32,
    pub steps: u32,
    pub pos: String,
    pub angle: u32,
    pub bezier: String,
    pub wave: String,
    pub resize: String,
    pub filter: String,
}
impl Default for ConfigTransicion {
    fn default() -> Self {
        Self {
            r#type: "grow".into(),
            duration: 1.5,
            fps: 60,
            steps: 90,
            pos: "center".into(),
            angle: 45,
            bezier: ".54, 0, 0.34, 0.99".into(),
            wave: "20, 20".into(),
            resize: "crop".into(),
            filter: "Lanczos3".into(),
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub wallpaper_dir: Option<PathBuf>,
    pub mpv_output: String,
    pub mpv_options: String,
    pub transition: ConfigTransicion,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            wallpaper_dir: None, 
            mpv_output:"*".into(), 
            mpv_options: "no-audio loop-file=inf".into(), 
            transition: ConfigTransicion::default(), 
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
    .unwrap_or_else(|| PathBuf::from("."))
    .join("hypaperdjxs4n")
    .join("config.json")
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|err| {
                eprintln!("config.json inválido ({err}), usando valores por defecto");
                Config::default()
            }),
            Err(_) => {
                let cfg = Config::default();
                let _ = cfg.save();
                cfg
            }
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self).expect("Config serializable");
        fs::write(path, json)
    }
}