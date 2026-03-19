use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/tauri.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn invoke(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn setOverlay(enabled: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn startDrag() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn invoke_with_args(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn registerShortcut() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn unregisterShortcut() -> Result<JsValue, JsValue>;
}

pub async fn get_pack() -> Vec<String> {
    match invoke("get_pack").await {
        Ok(val) => serde_wasm_bindgen::from_value(val).unwrap_or_default(),
        Err(_) => vec![],
    }
}

pub async fn set_gum(index: usize, gum_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "index": index,
        "gumId": gum_id
    }))
    .unwrap();

    match invoke_with_args("set_gum_in_pack", args).await {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(s) = e.as_string() {
                Err(s)
            } else {
                Err("Erreur inconnue".to_string())
            }
        }
    }
}
