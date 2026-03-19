use tauri::AppHandle;

#[tauri::command]
pub fn get_gums(app: AppHandle) -> Result<Vec<crate::models::gum::Gum>, String> {
    crate::services::pack_service::get_gums(&app)
}
