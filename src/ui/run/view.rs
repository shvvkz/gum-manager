use crate::Gum;

use leptos::prelude::*;

use super::{
    components::{header::header, gum_bar::gum_bar},
    hooks::{use_gum_listener, use_load_pack, use_overlay},
    state::create_run_state,
};

#[component]
pub fn RunView(
    gums: ReadSignal<Vec<Gum>>,
    on_back: Callback<()>,
) -> impl IntoView {

    let state = create_run_state();

    use_load_pack(&state);
    use_overlay();
    use_gum_listener(&state);

    view! {
        <div style="
            position:fixed;
            display:flex;
            flex-direction:column;
            gap:8px;
        ">

            {header(state.cycle_count, on_back)}

            {gum_bar(state.pack, gums, state.used_slots)}

        </div>
    }
}