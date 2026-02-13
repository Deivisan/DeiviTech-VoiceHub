use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

/// Inject text into currently active window using ydotool
#[tauri::command]
async fn inject_text(text: String) -> Result<String, String> {
    use std::process::Command;

    // Execute ydotool to type text
    let output = Command::new("ydotool")
        .arg("type")
        .arg("--")
        .arg(&text)
        .output()
        .map_err(|e| format!("Failed to execute ydotool: {}", e))?;

    if output.status.success() {
        Ok(format!("Successfully typed {} characters", text.len()))
    } else {
        Err(format!(
            "ydotool failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Check if ydotool is available
#[tauri::command]
async fn check_ydotool() -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("which")
        .arg("ydotool")
        .output()
        .map_err(|e| format!("Failed to check ydotool: {}", e))?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(format!("ydotool found at: {}", path))
    } else {
        Err("ydotool not found. Please install: sudo pacman -S ydotool".to_string())
    }
}

/// Toggle recording via global hotkey - emits event to frontend
#[tauri::command]
async fn toggle_recording(app: tauri::AppHandle) -> Result<String, String> {
    // Emit event to frontend to toggle recording state
    app.emit("toggle-recording", ())
        .map_err(|e| format!("Failed to emit toggle event: {}", e))?;
    
    Ok("Toggle recording event sent".to_string())
}

/// Setup system tray with icon and menu
fn setup_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let _tray = TrayIconBuilder::with_id("main-tray")
        .tooltip("DeiviTech VoiceHub")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Setup system tray
            setup_tray(&app.handle())?;

            // Register global hotkey: Super+H
            use tauri_plugin_global_shortcut::ShortcutState;
            
            app.handle()
                .plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(|app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                // Emit event to frontend to toggle recording
                                let _ = app.emit("toggle-recording", ());
                            }
                        })
                        .build(),
                )?;

            // Temporarily commented out - hotkey conflict with system
            // app.global_shortcut().register("Super+H")?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![inject_text, check_ydotool, toggle_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
