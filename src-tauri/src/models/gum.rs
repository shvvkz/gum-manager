use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gum {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub activation: String,
    pub function: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GumList {
    pub gums: Vec<Gum>,
}
