use crate::Gum;

use leptos::prelude::*;

#[component]
pub fn PreviewPanel(preview: ReadSignal<Option<Gum>>) -> impl IntoView {
    view! {
        <div style="
            flex:1;
            background:#1e293b;
            padding:30px;
            display:flex;
            flex-direction:column;
            justify-content:flex-start;
        ">

            {
                move || {
                    preview.get().map(|g| {

                        let rarity_color = match g.rarity.as_str() {
                            "Classic" => "#9ca3af",
                            "Whimsical" => "#38bdf8",
                            "Mega" => "#22c55e",
                            "Rare Mega" => "#a855f7",
                            "Ultra-Rare Mega" => "#facc15",
                            _ => "#ffffff",
                        };

                        view! {
                            <div style="display:flex; flex-direction:column; gap:10px;">

                                <img
                                    src=g.url
                                    style="
                                        width:100%;
                                        border-radius:10px;
                                    "
                                />

                                <h3 style="margin-top:10px;">
                                    {g.name}
                                </h3>

                                <span style=format!("color:{}; font-weight:bold;", rarity_color)>
                                    {g.rarity}
                                </span>

                                <span style="opacity:0.8;">
                                    {g.activation}
                                </span>

                                <p style="opacity:0.9;">
                                    {g.function}
                                </p>

                            </div>
                        }

                    })
                }
            }

        </div>
    }
}
