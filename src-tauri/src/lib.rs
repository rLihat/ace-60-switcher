mod profiles;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_profiles_cmd() -> Vec<profiles::ProfileEntry> {
    profiles::load_profiles()
}

#[tauri::command]
fn import_profile(path: String) -> Result<profiles::ProfileEntry, String> {
    let src = std::path::PathBuf::from(path);
    if !src.exists() {
        return Err("File not found".to_string());
    }
    let profiles_dir = profiles::get_profiles_dir();
    let file_name = src
        .file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy()
        .to_string();
    let dest = profiles_dir.join(&file_name);
    std::fs::copy(&src, &dest).map_err(|e| e.to_string())?;
    
    let content = std::fs::read_to_string(&dest).map_err(|e| e.to_string())?;
    let val: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let name = val
        .get("data")
        .and_then(|d| d.get("name"))
        .and_then(|n| n.as_str())
        .or_else(|| val.get("name").and_then(|n| n.as_str()))
        .unwrap_or("Unnamed")
        .to_string();
    let id = dest.file_stem().unwrap().to_string_lossy().to_string();
    
    Ok(profiles::ProfileEntry {
        id,
        name,
        path: dest,
        hotkey: None,
    })
}

#[tauri::command]
fn apply_profile(id: String) -> Result<String, String> {
    use hidapi::HidApi;
    
    let api = HidApi::new().map_err(|e| format!("Failed to init HID: {}", e))?;
    
    // Look for Ace 60 Pro (VID: 0x41E4, PID: 0x2113)
    let mut found = None;
    for device in api.device_list() {
        if device.vendor_id() == 0x41E4 && device.product_id() == 0x2113 {
            found = Some((device.vendor_id(), device.product_id(), device.path().to_string_lossy().to_string()));
            break;
        }
    }
    
    match found {
        Some((vid, pid, path)) => {
            // Try to open device
            match api.open(vid, pid) {
                Ok(_device) => {
                    Ok(format!("Found and opened Ace 60 Pro (VID:{:04X}, PID:{:04X}) at {}. HID protocol implementation in progress.", vid, pid, path))
                },
                Err(e) => Err(format!("Found device but failed to open: {}. Close MCHOSE M Hub and try again.", e))
            }
        },
        None => Err("Ace 60 Pro not found. Make sure it's connected via USB and MCHOSE M Hub is closed.".to_string())
    }
}

fn setup_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
    
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().cloned().unwrap())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            setup_tray(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, load_profiles_cmd, import_profile, apply_profile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
