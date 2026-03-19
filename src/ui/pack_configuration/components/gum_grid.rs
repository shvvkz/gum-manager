use crate::tauri::set_gum;
use crate::Gum;

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn GumGrid(
    gums: ReadSignal<Vec<Gum>>,
    filter_mode: ReadSignal<String>,
    editing_slot: ReadSignal<Option<usize>>,
    set_pack: WriteSignal<Vec<String>>,
    set_editing_slot: WriteSignal<Option<usize>>,
    set_preview: WriteSignal<Option<Gum>>,
    add_toast: impl Fn(String) + 'static + Clone + Send + Sync,
) -> impl IntoView {
    let filtered_gums = move || {
        let mode = filter_mode.get();

        gums.get()
            .into_iter()
            .filter(|g| match mode.as_str() {
                "classic" => g.rarity == "Classic" || g.rarity == "Whimsical",
                "mega" => {
                    g.rarity == "Mega" || g.rarity == "Rare Mega" || g.rarity == "Ultra-Rare Mega"
                }
                _ => true,
            })
            .collect::<Vec<_>>()
    };

    view! {
        {
            move || {
                if let Some(index) = editing_slot.get() {

                    view! {
                        <div style="
                            width:100%;
                            max-width:800px;
                            overflow-y:auto;
                            padding-right:10px;
                        ">
                            <div style="
                                display:grid;
                                grid-template-columns: repeat(6, minmax(90px, 1fr));
                                gap:10px;
                            ">

                                {
                                    filtered_gums().into_iter().map(|gum| {

                                        let id = gum.id.clone();
                                        let gum_clone = gum.clone();
                                        let add_toast = add_toast.clone();

                                        view! {
                                            <img
                                                src=gum.url
                                                style="
                                                    width:100%;
                                                    cursor:pointer;
                                                    border-radius:10px;
                                                    transition:0.2s;
                                                "

                                                on:mouseenter=move |_| {
                                                    set_preview.set(Some(gum_clone.clone()));
                                                }

                                                on:click=move |_| {
                                                    let id_clone = id.clone();
                                                    let add_toast_clone = add_toast.clone();

                                                    spawn_local(async move {
                                                        match set_gum(index, id_clone.clone()).await {
                                                            Ok(_) => {
                                                                set_pack.update(|p| p[index] = id_clone);
                                                                set_editing_slot.set(None);
                                                            }
                                                            Err(e) => add_toast_clone(e),
                                                        }
                                                    });
                                                }
                                            />
                                        }

                                    }).collect_view()
                                }

                            </div>
                        </div>
                    }.into_any()

                } else {
                    ().into_any()
                }
            }
        }
    }
}
