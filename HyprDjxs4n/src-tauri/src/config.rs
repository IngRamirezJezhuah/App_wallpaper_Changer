use std::fs;
use std::path::{self, PathBuf};
use serde::{Deserialize, Serialize};
/*
Notas de mantenimiento para mi yo del futuro
Este archivo es el que se encarga de seleccionar 
el efecto transicion del wallpaper y el de aplicarlo 
para que se vea chulo
*/
#[derive(Clone,Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub wallpaper_dir: Option<PathBuf>,
    pub mpv_output: String,
    pub mpv_options: String,
    pub transition: TransitionConfig,
}

#[derive(Clone,Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct TransitionConfig {
    // nota r#type es de rust, type es la palabra reservada de rust, el prefijo r# la hace usable
    pub r#type: String, // esta chingadera es la que me va a permitir usar las transisciones de sww
    pub duration: f64, //maneja los segundos
    pub fps: u32,
    pub step: u32, // cuanto sea los pasos a hacer el 255 es que es de ya
    pub pos: String, // la posicino que quiero que aparesca el efecto
    pub angle: u32, // lso grados de la onda que va a hacer
    pub bezier: String, // la chingada curva que va a hacer el fade
    pub wave: String, // lo que es el ancho y el alto de la onda que hara 
    pub resize: String, // esto es el tipo de forma que agarre
    pub filter: String, // el filtro que va a usar segun de que sea
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            r#type: "grow".into(),
            duration: 1.5,
            fps: 60,
            step: 90, 
            pos: "center".into(),
            angle: 45,
            bezier: ".54, 0, 0.34, 0.99".into(), 
            wave: "20, 20".into(), 
            resize: "crop".into(), 
            filter: "Lanczos3".into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            wallpaper_dir: None, 
            mpv_output: "*".into(), 
            mpv_options: "no-audio loop-file=1".into(), 
            transition:  TransitionConfig::default(),
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
    .unwrap_or_else(|| PathBuf::from("."))
    .join("hyprdjxs4n")
    .join("config.json")
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string(config_path()){
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e|{
                eprint!("config.json ilegible ({e}), usabdo defaults");
                Config::default()
            }),
            Err(_) => {
                let config = Config::default();
                let _ = config.save();
                config
            },
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path= config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, serde_json::to_string_pretty(self).expect("Config siempre serializable"))
    }
}