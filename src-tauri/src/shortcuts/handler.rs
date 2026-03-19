use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, Shortcut};

pub fn handle_shortcut(app: &AppHandle, shortcut: &Shortcut) {
    let mapping = [
        (Code::F5, 0),
        (Code::F6, 1),
        (Code::F7, 2),
        (Code::F8, 3),
        (Code::F9, 4),
    ];

    for (code, index) in mapping {
        if shortcut == &Shortcut::new(None, code) {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("use-gum", index);
            }
        }
    }
}
