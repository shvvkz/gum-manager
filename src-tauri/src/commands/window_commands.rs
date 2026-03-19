use tauri::Window;

#[tauri::command]
pub fn set_overlay(window: Window, enabled: bool) {
    crate::window::overlay::set_overlay(window, enabled);
}

#[tauri::command]
pub fn start_drag(window: Window) {
    let _ = window.start_dragging();
}
