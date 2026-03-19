use leptos::prelude::*;

#[component]
pub fn FilterBar(
    editing_slot: ReadSignal<Option<usize>>,
    filter_mode: ReadSignal<String>,
    set_filter_mode: WriteSignal<String>,
) -> impl IntoView {
    view! {
        {
            move || {
                if editing_slot.get().is_some() {

                    let is_classic = move || filter_mode.get() == "classic";
                    let is_mega = move || filter_mode.get() == "mega";

                    view! {
                        <div style="display:flex; gap:10px;">

                            <button
                                on:click=move |_| set_filter_mode.set("classic".to_string())
                                style=move || format!("
                                    padding:8px 16px;
                                    border-radius:8px;
                                    border:none;
                                    cursor:pointer;
                                    background:{};
                                    color:white;
                                    transition:0.2s;
                                ",
                                    if is_classic() { "#22c55e" } else { "#1e293b" }
                                )
                            >
                                "Classics"
                            </button>

                            <button
                                on:click=move |_| set_filter_mode.set("mega".to_string())
                                style=move || format!("
                                    padding:8px 16px;
                                    border-radius:8px;
                                    border:none;
                                    cursor:pointer;
                                    background:{};
                                    color:white;
                                    transition:0.2s;
                                ",
                                    if is_mega() { "#a855f7" } else { "#1e293b" }
                                )
                            >
                                "Megas"
                            </button>

                        </div>
                    }.into_any()

                } else {
                    ().into_any()
                }
            }
        }
    }
}
