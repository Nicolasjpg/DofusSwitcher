#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod actions;
mod win_api;
mod gui;

use std::thread;
use std::path::Path;
use eframe::egui;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use crate::state::{GLOBAL_STATE, KeyBinding};
use crate::actions::GameAction;

fn main() -> eframe::Result<()> {    
    {
        let mut state = GLOBAL_STATE.lock().unwrap();        
        let code_next = VK_OEM_102.0 as u32;
        let code_prev = VK_F2.0 as u32;

        state.bindings = vec![
            KeyBinding { 
                action: GameAction::NextWindow, 
                vk_code: code_next,                 
                name: win_api::get_key_name(code_next) 
            },
            KeyBinding { 
                action: GameAction::PrevWindow, 
                vk_code: code_prev, 
                name: win_api::get_key_name(code_prev) 
            },
        ];
    }

    
    win_api::scan_open_windows();

    thread::spawn(|| {
        actions::start_hotkey_listener();
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 600.0]) 
            .with_icon(load_icon())
            .with_transparent(true),        
        ..Default::default()
    };

    eframe::run_native(
        "Dofus Switcher",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(gui::DofusApp::default()))
        })
    )
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../app_icon.png"); 

    let image = image::load_from_memory(icon_bytes)
        .expect("Error al cargar la imagen embebida")
        .to_rgba8();

    let (width, height) = image.dimensions();
    
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}