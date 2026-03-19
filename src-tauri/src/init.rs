use reqwest::Client;
use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::services::file::get_app_file_path;

#[derive(Deserialize)]
struct GumData {
    version: String,
}

pub async fn get_remote_gum_version() -> String {
    let client = Client::new();

    let response = match client
        .get("https://raw.githubusercontent.com/shvvkz/gum-manager/refs/heads/main/gum.json")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return "unknown".to_string(),
    };

    let text = match response.text().await {
        Ok(t) => t,
        Err(_) => return "unknown".to_string(),
    };

    let parsed: GumData = match serde_json::from_str(&text) {
        Ok(data) => data,
        Err(_) => return "unknown".to_string(),
    };

    parsed.version
}

pub fn get_local_gum_version(app: &AppHandle) -> String {
    let path = get_app_file_path(app, "gums.json");

    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let parsed: GumData = serde_json::from_str(&text).unwrap_or(GumData {
        version: "unknown".to_string(),
    });
    return parsed.version;
}