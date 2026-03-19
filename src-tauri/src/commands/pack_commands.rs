use tauri::AppHandle;

#[tauri::command]
pub fn get_pack(app: AppHandle) -> Result<Vec<String>, String> {
    crate::services::pack_service::get_pack(&app)
}

#[tauri::command]
pub fn set_gum_in_pack(app: AppHandle, index: usize, gum_id: String) -> Result<(), String> {
    crate::services::pack_service::set_gum_in_pack(&app, index, gum_id)
}
