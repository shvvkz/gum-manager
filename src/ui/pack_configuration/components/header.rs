use leptos::prelude::*;

pub fn header(editing_slot: ReadSignal<Option<usize>>) -> impl IntoView {
    view! {
        <div style="display:flex; flex-direction:column; gap:8px;">
            <h1 style="
                font-size:28px;
                font-weight:bold;
                background: linear-gradient(90deg, #22c55e, #4ade80);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            ">
                "Gum Manager"
            </h1>

            <span style="opacity:0.7;">
                {
                    move || {
                        if editing_slot.get().is_some() {
                            "Choose a GobbleGum to replace your selection"
                        } else {
                            "Customize your loadout and optimize your strategy"
                        }
                    }
                }
            </span>
        </div>
    }
}
