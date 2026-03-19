use reqwest::Client;
use serde::Deserialize;
use tauri::AppHandle;

use crate::services::file::get_app_file_path;

#[derive(Deserialize)]
struct GumData {
    version: String,
}

pub async fn get_remote_gum() -> String {
    let client = Client::new();

    let response = match client
        .get("https://raw.githubusercontent.com/shvvkz/gum-manager/refs/heads/main/gums.json")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return "unknown".to_string(),
    };

    match response.text().await {
        Ok(t) => t,
        Err(_) => "unknown".to_string(),
    }
}

pub fn get_local_gum(app: &AppHandle) -> String {
    let path = get_app_file_path(app, "gums.json");
    match std::fs::read_to_string(&path).unwrap_or_default() {
        content if !content.is_empty() => content,
        _ => "unknown".to_string(),
    }
}

pub fn get_gum_version(text: String) -> String {
    let parsed: GumData = serde_json::from_str(&text).unwrap_or(GumData {
        version: "unknown".to_string(),
    });
    parsed.version
}

pub fn write_gums(app: &AppHandle, gums: String) -> Result<(), String> {
    let path = get_app_file_path(app, "gums.json");
    std::fs::write(path, gums).map_err(|e| format!("Erreur écriture fichier: {}", e))
}

pub fn pack_file_exists(app: &AppHandle) -> bool {
    let path = get_app_file_path(app, "pack.json");
    std::fs::metadata(path).is_ok()
}

pub fn create_pack_file(app: &AppHandle) -> Result<(), String> {
    let path = get_app_file_path(app, "pack.json");
    std::fs::write(
        path,
        serde_json::json!({
            "pack": [
                "Always_Done_Swiftly",
                "Arms_Grace",
                "Coagulant",
                "In_Plain_Sight",
                "Stock_Option"
            ]
        })
        .to_string(),
    )
    .map_err(|e| format!("Erreur écriture fichier: {}", e))
}
