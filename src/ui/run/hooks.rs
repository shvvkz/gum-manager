use crate::tauri::{get_pack, invoke, setOverlay};
use leptos::{prelude::*, reactive::spawn_local};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use super::state::RunState;

pub fn use_load_pack(state: &RunState) {
    let set_pack = state.pack;

    spawn_local(async move {
        let data = get_pack().await;
        set_pack.set(data);
    });
}

pub fn use_overlay() {
    Effect::new(move |_| {
        spawn_local(async {
            let _ = invoke("register_shortcut").await;
            let _ = setOverlay(true).await;
        });
    });

    on_cleanup(move || {
        spawn_local(async {
            let _ = invoke("unregister_shortcut").await;
            let _ = setOverlay(false).await;
        });
    });
}

pub fn use_gum_listener(state: &RunState) {
    let set_used_slots = state.used_slots;
    let set_cycle_count = state.cycle_count;

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();

        let handler = Closure::wrap(Box::new(move |event: JsValue| {
            let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload"));

            if let Ok(payload) = payload {
                if let Ok(index) = serde_wasm_bindgen::from_value::<usize>(payload) {

                    set_used_slots.update(|list| {

                        if list.contains(&index) {
                            return;
                        }

                        if list.len() >= 4 {
                            list.clear();
                            set_cycle_count.update(|c| *c += 1);
                            return;
                        }

                        list.push(index);
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);

        let _ = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__"))
            .and_then(|tauri| js_sys::Reflect::get(&tauri, &JsValue::from_str("event")))
            .and_then(|event| js_sys::Reflect::get(&event, &JsValue::from_str("listen")))
            .and_then(|listen| {
                let listen_fn = listen.dyn_into::<js_sys::Function>()?;

                listen_fn.call2(
                    &JsValue::NULL,
                    &JsValue::from_str("use-gum"),
                    handler.as_ref().unchecked_ref(),
                )
            });

        handler.forget();
    });
}