use std::process::Child;
use std::sync::Mutex;

use crate::config::Config;

// Este decide que motor ejecuta al usarlo
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Engine {
    None,
    Swww, // imagen o gif via swww-daemon
    Mpvpaper, // video via mpvpaper
}

pub struct Inner {
    pub config: Config,
    pub engine: Engine,
    pub current_wallpaper: Option<String>, // ruta curda dle utlimo wallpaper puesto
    pub mpv_child: Option<Child>, // procesos de mpvpaper vivo
    pub daemon_child: Option<Child>, // swww-daemon que lanza por defecto hypr
}

pub struct AppState {
    pub inner: Mutex<Inner>,
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            inner: Mutex::new(Inner {
                config: Config::load(),
                engine: Engine::None,
                current_wallpaper: None,
                mpv_child: None,
                daemon_child: None,
            }),
        }
    }
}