use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use gum_manager_ui::ui::pack_configuration::view::PackConfigurationView;
use gum_manager_ui::ui::run::view::RunView;
use gum_manager_ui::Gum;

use gum_manager_ui::tauri::invoke;

#[component]
pub fn App() -> impl IntoView {

    let (is_running, set_is_running) = signal(false);
    let (gums, set_gums) = signal::<Vec<Gum>>(vec![]);

    {
        spawn_local(async move {
            if let Ok(val) = invoke("get_gums").await {
                let parsed =
                    serde_wasm_bindgen::from_value::<Vec<Gum>>(val)
                        .unwrap_or_default();
                set_gums.set(parsed);
            }
        });
    }

    let on_run = Callback::new(move |_| {
        set_is_running.set(true);
    });

    let on_back = Callback::new(move |_| {
        set_is_running.set(false);
    });

    view! {
        {
            move || {
                if is_running.get() {
                    view! {
                        <RunView
                            gums=gums
                            on_back=on_back
                        />
                    }.into_any()
                } else {
                    view! {
                        <PackConfigurationView
                            gums=gums
                            on_run=on_run
                        />
                    }.into_any()
                }
            }
        }
    }
}