use tauri::{AppHandle, Manager};

pub fn get_db_path(app: &AppHandle) -> std::path::PathBuf {
    let mut path = app.path().app_data_dir().unwrap();
    std::fs::create_dir_all(&path).ok();
    path.push("db.json");
    path
}
