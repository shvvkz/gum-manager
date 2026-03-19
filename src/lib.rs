pub mod tauri;
pub mod ui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gum {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub activation: String,
    pub function: String,
    pub url: String,
}
