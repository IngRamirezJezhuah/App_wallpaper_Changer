use std::sync::Mutex;
use crate::config::Config;

pub struct InnerState {
    pub config: Config,
}

pub struct AppState {
    pub inner: Mutex<InnerState>
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            inner: Mutex::new(InnerState { 
                config: Config::load(),
            }),
        }
    }
}