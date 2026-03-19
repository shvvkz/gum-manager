use leptos::prelude::*;
use leptos::reactive::spawn_local;
use crate::tauri::startDrag;

pub fn header(
    cycle_count: RwSignal<i32>,
    on_back: Callback<()>,
) -> impl IntoView {

    view! {
        <div style="
            display:flex;
            align-items:center;
            justify-content:space-between;
        ">

            // 🔥 DRAG ZONE
            <div
                style="
                    flex:1;
                    cursor:grab;
                "
                on:mousedown=move |_| {
                    spawn_local(async {
                        let _ = startDrag().await;
                    });
                }
            >
                <div style="
                    font-size:16px;
                    font-weight:bold;
                    color:#22c55e;
                    background:#020617;
                    padding:6px 10px;
                    border-top-left-radius:6px;
                    border-bottom-left-radius:6px;
                ">
                    {move || format!("Cycles: {}", cycle_count.get())}
                </div>
            </div>

            // 🔥 BOUTON QUIT
            <div
                style="
                    cursor:pointer;
                    color:#ef4444;
                    font-weight:bold;
                    background:#020617;
                    padding:6px 10px;
                    border-top-right-radius:6px;
                    border-bottom-right-radius:6px;
                "
                on:click=move |_| {
                    on_back.run(());
                }
            >
                "⛌"
            </div>

        </div>
    }
}