use serde_json::Value;
use tauri::AppHandle;

use crate::models::gum::GumList;
use crate::services::db::get_db_path;

pub fn get_gums(app: &AppHandle) -> Result<Vec<crate::models::gum::Gum>, String> {
    let path = get_db_path(app);

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Erreur lecture fichier: {}", e))?;

    let parsed: GumList =
        serde_json::from_str(&content).map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    Ok(parsed.gums)
}

pub fn get_pack(app: &AppHandle) -> Result<Vec<String>, String> {
    let path = get_db_path(app);

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Erreur lecture fichier: {}", e))?;

    let parsed: Value =
        serde_json::from_str(&content).map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    let pack = parsed
        .get("pack")
        .and_then(|p| p.as_array())
        .ok_or_else(|| "Clé 'pack' manquante".to_string())?
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(pack)
}

pub fn set_gum_in_pack(app: &AppHandle, index: usize, gum_id: String) -> Result<(), String> {
    let path = get_db_path(app);

    let content =
        std::fs::read_to_string(&path).map_err(|e| format!("Erreur lecture fichier: {}", e))?;

    let mut parsed: Value =
        serde_json::from_str(&content).map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    let pack = parsed
        .get_mut("pack")
        .and_then(|p| p.as_array_mut())
        .ok_or_else(|| "Clé 'pack' manquante".to_string())?;

    if index >= pack.len() {
        return Err("Index hors limites".to_string());
    }

    for (i, id) in pack.iter().filter_map(|v| v.as_str()).enumerate() {
        if id == gum_id && i != index {
            return Err("Gobblegum already in your pack.".to_string());
        }
    }

    pack[index] = Value::String(gum_id);

    let new_content = serde_json::to_string_pretty(&parsed)
        .map_err(|e| format!("Erreur sérialisation JSON: {}", e))?;

    std::fs::write(path, new_content).map_err(|e| format!("Erreur écriture fichier: {}", e))?;

    Ok(())
}
