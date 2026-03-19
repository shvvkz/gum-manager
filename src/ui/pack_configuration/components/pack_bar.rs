use crate::Gum;

use leptos::prelude::*;

#[component]
pub fn PackBar(
    pack: ReadSignal<Vec<String>>,
    gums: ReadSignal<Vec<Gum>>,
    editing_slot: ReadSignal<Option<usize>>,
    set_editing_slot: WriteSignal<Option<usize>>,
    set_preview: WriteSignal<Option<Gum>>,
) -> impl IntoView {
    let get_gum = move |id: &String| gums.get().into_iter().find(|g| &g.id == id);

    view! {
        <div style="display:flex; gap:20px; justify-content:center;">

            {
                move || pack.get().into_iter().enumerate().map(|(i, id)| {

                    let gum = get_gum(&id);

                    let is_active = move || editing_slot.get() == Some(i);

                    view! {
                        <div
                            on:click=move |_| {
                                set_editing_slot.set(Some(i));
                            }

                            style=move || format!("
                                width:90px;
                                height:90px;
                                border-radius:10px;
                                overflow:hidden;
                                cursor:pointer;
                                border:{}px solid {};
                                box-shadow:{};
                                transition:0.2s;

                            ",
                                if is_active() { 3 } else { 2 },
                                if is_active() { "#22c55e" } else { "#f97316" },
                                if is_active() { "0 0 15px #22c55e" } else { "none" }
                            )
                        >

                            {
                                gum.map(|g| {
                                    let g_clone = g.clone();

                                    view! {
                                        <img
                                            src=g.url
                                            style="width:100%; height:100%; object-fit:cover;"

                                            on:mouseenter=move |_| {
                                                set_preview.set(Some(g_clone.clone()));
                                            }
                                        />
                                    }
                                })
                            }

                        </div>
                    }

                }).collect_view()
            }

        </div>
    }
}
