use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::HWND;
use crate::actions::GameAction;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Clone, Debug)]
pub struct DofusWindow {
    pub hwnd: HWND,
    pub original_title: String, 
    pub name: String,           
    pub class: String,          
    pub icon: String,         
}

#[derive(Clone, Debug)]
pub struct KeyBinding {
    pub action: GameAction,
    pub vk_code: u32,
    pub name: String,
}

//Estructura de configuración del json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub character_icons: HashMap<String, String>, // map nombre del pj -> icono
}

pub struct AppState {
    pub windows: Vec<DofusWindow>,
    pub bindings: Vec<KeyBinding>,
    pub keys_need_update: bool,
    pub config: AppConfig,
}

impl AppState {
    //guardar en json
    pub fn save_config(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.config) {
            let _ = fs::write("dofus_config.json", json);
        }
    }

    //cargar del json
    pub fn load_config() -> AppConfig {
        if let Ok(content) = fs::read_to_string("dofus_config.json") {
            if let Ok(cfg) = serde_json::from_str(&content) {
                return cfg;
            }
        }
        
        AppConfig { character_icons: HashMap::new() }
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState {
        windows: Vec::new(),
        bindings: Vec::new(),
        keys_need_update: true,
        config: AppState::load_config(),
    }));
}