use crate::state::GLOBAL_STATE;
use crate::win_api;
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::HWND;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameAction {
    NextWindow,
    PrevWindow,    
}

//Acciones
pub fn execute(action: GameAction) {
    let state = GLOBAL_STATE.lock().unwrap();
    let count = state.windows.len();
    if count == 0 { return; }

    //Obtener indice
    let current_hwnd = HWND(win_api::get_foreground_window());
    let current_index = state.windows.iter().position(|w| w.hwnd == current_hwnd);
     
    let idx = current_index.unwrap_or(count - 1);

    match action {
        GameAction::NextWindow => {
            let next = (idx + 1) % count;
            win_api::focus_window(state.windows[next].hwnd);
        },
        GameAction::PrevWindow => {
            let prev = (idx + count - 1) % count;
            win_api::focus_window(state.windows[prev].hwnd);
        },
        //nuevas...por si acaso
    }
}

//Detección de teclas
pub fn start_hotkey_listener() {
    unsafe {
        let mut registered_ids = Vec::new();

        loop {
            //Recargar teclas si cambiaron en la GUI
            let mut need_rebind = false;
            {
                let mut state = GLOBAL_STATE.lock().unwrap();
                if state.keys_need_update {
                    need_rebind = true;
                    state.keys_need_update = false;
                }
            }

            if need_rebind {
                for id in &registered_ids { UnregisterHotKey(HWND(0), *id); }
                registered_ids.clear();

                let state = GLOBAL_STATE.lock().unwrap();
                for (index, binding) in state.bindings.iter().enumerate() {
                    let id = index as i32 + 1;
                    if RegisterHotKey(HWND(0), id, MOD_NOREPEAT, binding.vk_code).is_ok() {
                        registered_ids.push(id);
                    }
                }
            }

            let mut msg = MSG::default();
            if PeekMessageW(&mut msg, HWND(0), 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_HOTKEY {
                    let id = msg.wParam.0 as usize;
                    let action_opt = {
                        let state = GLOBAL_STATE.lock().unwrap();
                        if id > 0 && id <= state.bindings.len() {
                            Some(state.bindings[id - 1].action)
                        } else { None }
                    };

                    if let Some(action) = action_opt {
                        execute(action);
                    }
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}