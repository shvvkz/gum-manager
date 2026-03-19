use crate::tauri::get_pack;
use crate::Gum;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::TimeoutFuture;

use super::state::PackConfigurationState;
use super::components::toast::Toast;

pub fn use_load_pack(state: &PackConfigurationState, gums: ReadSignal<Vec<Gum>>) {
    let set_pack = state.pack;
    let set_preview = state.preview;

    spawn_local(async move {
        let p = get_pack().await;
        set_pack.set(p.clone());

        if let Some(first_id) = p.first() {
            if let Some(g) = gums.get().into_iter().find(|g| &g.id == first_id) {
                set_preview.set(Some(g));
            }
        }
    });
}

pub fn use_toast(state: &PackConfigurationState) -> impl Fn(String) + Clone {
    let set_toasts = state.toasts;

    move |msg: String| {
        let clean = msg.replace("JsValue(\"", "").replace("\")", "");
        let id = js_sys::Date::now() as u64;

        set_toasts.set(vec![Toast {
            id,
            message: clean,
            leaving: false,
        }]);

        let set_toasts_clone = set_toasts;

        spawn_local(async move {
            TimeoutFuture::new(2000).await;

            set_toasts_clone.update(|list| {
                if let Some(t) = list.first_mut() {
                    if t.id == id {
                        t.leaving = true;
                    }
                }
            });

            TimeoutFuture::new(300).await;

            set_toasts_clone.update(|list| {
                if let Some(t) = list.first() {
                    if t.id == id {
                        list.clear();
                    }
                }
            });
        });
    }
}