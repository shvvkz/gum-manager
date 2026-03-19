use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut};

#[tauri::command]
pub fn register_shortcut(app: AppHandle) -> Result<(), String> {
    let shortcuts = [
        Shortcut::new(None, Code::F5),
        Shortcut::new(None, Code::F6),
        Shortcut::new(None, Code::F7),
        Shortcut::new(None, Code::F8),
        Shortcut::new(None, Code::F9),
    ];

    for shortcut in &shortcuts {
        app.global_shortcut()
            .register(*shortcut)
            .map_err(|e| format!("Failed to register shortcut {:?}: {}", shortcut, e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn unregister_shortcut(app: AppHandle) -> Result<(), String> {
    let shortcuts = [
        Shortcut::new(None, Code::F5),
        Shortcut::new(None, Code::F6),
        Shortcut::new(None, Code::F7),
        Shortcut::new(None, Code::F8),
        Shortcut::new(None, Code::F9),
    ];

    for shortcut in &shortcuts {
        app.global_shortcut()
            .unregister(*shortcut)
            .map_err(|e| format!("Failed to unregister shortcut {:?}: {}", shortcut, e))?;
    }

    Ok(())
}
