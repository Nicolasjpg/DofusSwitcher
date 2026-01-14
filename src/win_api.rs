use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use crate::state::{GLOBAL_STATE, DofusWindow};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

struct ScannedWindow {
    hwnd: HWND,
    title: String,
}


pub fn scan_open_windows() {
    unsafe {
        let mut found_windows: Vec<ScannedWindow> = Vec::new();
        
        let _ = EnumWindows(Some(enum_window_proc), LPARAM(&mut found_windows as *mut _ as isize));

        let mut state = GLOBAL_STATE.lock().unwrap();

        //limpiar ventanas cerradas
        state.windows.retain(|existing| {
            found_windows.iter().any(|new| new.hwnd == existing.hwnd)
        });

        //agregar bnuevas ventanas
        for scanned in found_windows {
            
            //sincronizar si no esta ena lista
            if !state.windows.iter().any(|w| w.hwnd == scanned.hwnd) {
                                
                let parts: Vec<&str> = scanned.title.split(" - ").collect();
                
                let (name, class) = if parts.len() >= 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (scanned.title.clone(), "Dofus".to_string())
                };
                //Se recupera el icono por nombre del pj desde la config
                let saved_icon = state.config.character_icons
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| "default".to_string());

                let new_window = DofusWindow {
                    hwnd: scanned.hwnd,
                    original_title: scanned.title,
                    name: name,
                    class: class,
                    icon: saved_icon,
                };

                state.windows.push(new_window);
            }
        }
    }
}

pub fn get_foreground_window() -> isize {
    unsafe { GetForegroundWindow().0 as isize }
}

pub fn switch_to_window(hwnd_val: isize) {
    unsafe {
        let hwnd = HWND(hwnd_val as _);
        let _ = SetForegroundWindow(hwnd);
    }
}

pub fn focus_window(hwnd: HWND) {
    unsafe {
        if IsIconic(hwnd).as_bool() {
            ShowWindow(hwnd, SW_RESTORE);
        }
        SetForegroundWindow(hwnd);
    }
}

pub fn detect_keypress() -> Option<u32> {
    unsafe {
        for key in 1..255u32 {
            // Ignorar clicks de mouse
            if key >= 1 && key <= 6 { continue; } 
            if (GetAsyncKeyState(key as i32) as u16 & 0x8000) != 0 {
                return Some(key);
            }
        }
    }
    None
}

pub fn get_key_name(vk_code: u32) -> String {
    unsafe {
        let scan_code = MapVirtualKeyW(vk_code, MAPVK_VK_TO_VSC);
        let param = scan_code << 16; 
        let mut buffer = [0u16; 128];
        let len = GetKeyNameTextW(param as i32, &mut buffer);
        if len > 0 { String::from_utf16_lossy(&buffer[..len as usize]) } 
        else { format!("VK_{}", vk_code) }
    }
}

// Callback interno
unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    if !IsWindowVisible(hwnd).as_bool() { return BOOL(1); }

    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));
    if process_id == std::process::id() { return BOOL(1); }

    let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id);
    if let Ok(handle) = process_handle {
        let mut buffer = [0u16; 1024];
        let mut size = 1024u32;
        let result = QueryFullProcessImageNameW(handle, PROCESS_NAME_FORMAT(0), PWSTR(buffer.as_mut_ptr()), &mut size);
        let _ = CloseHandle(handle);

        if result.is_ok() {
            let full_path = String::from_utf16_lossy(&buffer[..size as usize]);
            let exe_name = full_path.split('\\').last().unwrap_or("Unknown").to_string();

            if exe_name.to_lowercase().contains("dofus") {
                let mut title_buf = [0u16; 512];
                let len = GetWindowTextW(hwnd, &mut title_buf);
                if len > 0 {
                    let title = String::from_utf16_lossy(&title_buf[..len as usize]);
                    
                    let list = &mut *(lparam.0 as *mut Vec<ScannedWindow>);
                    list.push(ScannedWindow { hwnd, title });
                }
            }
        }
    }
    BOOL(1)
}