use crate::Gum;
use leptos::prelude::*;

use super::{
    components::{
        action_bar::action_bar, filter_bar::FilterBar, gum_grid::GumGrid, header::header,
        pack_bar::PackBar, preview_panel::PreviewPanel, toast::ToastContainer,
    },
    hooks::{use_load_pack, use_toast},
    state::create_pack_configuration_state,
};

#[component]
pub fn PackConfigurationView(gums: ReadSignal<Vec<Gum>>, on_run: Callback<()>) -> impl IntoView {
    let state = create_pack_configuration_state();

    use_load_pack(&state, gums);
    let add_toast = use_toast(&state);

    view! {
        <div style="
            display:flex;
            height:100vh;
            color:white;
            background: linear-gradient(135deg, #0f172a 0%, #020617 100%);
        ">

            <ToastContainer toasts=state.toasts.read_only() />

            <div style="
                flex:3;
                padding:30px;
                display:flex;
                flex-direction:column;
                gap:20px;
            ">

                {header(state.editing_slot.read_only())}

                <PackBar
                    pack=state.pack.read_only()
                    gums=gums
                    editing_slot=state.editing_slot.read_only()
                    set_editing_slot=state.editing_slot.write_only()
                    set_preview=state.preview.write_only()
                />

                {action_bar(state.editing_slot.read_only(), on_run)}

                <FilterBar
                    editing_slot=state.editing_slot.read_only()
                    filter_mode=state.filter_mode.read_only()
                    set_filter_mode=state.filter_mode.write_only()
                />

                <GumGrid
                    gums=gums
                    filter_mode=state.filter_mode.read_only()
                    editing_slot=state.editing_slot.read_only()
                    set_pack=state.pack.write_only()
                    set_editing_slot=state.editing_slot.write_only()
                    set_preview=state.preview.write_only()
                    add_toast=add_toast
                />
            </div>

            <div style="
                flex:1;
                padding:20px;
                background: rgba(30,41,59,0.7);
                backdrop-filter: blur(10px);
            ">
                <PreviewPanel preview=state.preview.read_only() />
            </div>
        </div>
    }
}
