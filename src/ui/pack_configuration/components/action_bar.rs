use leptos::prelude::*;

pub fn action_bar(
    editing_slot: ReadSignal<Option<usize>>,
    on_run: Callback<()>,
) -> impl IntoView {

    view! {
        {
            move || {
                if editing_slot.get().is_none() {
                    view! {
                        <div style="
                            display:flex;
                            gap:15px;
                            margin-top:20px;
                        ">

                            <button
                                on:click=move |_| on_run.run(())
                                style="
                                    flex:1;
                                    background: linear-gradient(135deg, #22c55e, #16a34a);
                                    color:white;
                                    padding:14px;
                                    border:none;
                                    border-radius:10px;
                                    font-size:16px;
                                    font-weight:bold;
                                    cursor:pointer;
                                "
                            >
                                "Start"
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