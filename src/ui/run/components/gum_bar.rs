use crate::Gum;
use leptos::prelude::*;

pub fn gum_bar(
    pack: RwSignal<Vec<String>>,
    gums: ReadSignal<Vec<Gum>>,
    used_slots: RwSignal<Vec<usize>>,
) -> impl IntoView {
    let get_gum = move |id: &String| gums.get().into_iter().find(|g| &g.id == id);

    view! {
        <div style="display:flex; gap:10px;">
            {
                move || {
                    pack.get().into_iter().enumerate().map(|(i, id)| {
                        get_gum(&id).map(|gum| {

                            let is_used = move || used_slots.get().contains(&i);

                            view! {
                                <img
                                    src=gum.url
                                    style=move || format!(
                                        "
                                        width:80px;
                                        height:80px;
                                        opacity:{};
                                        filter:{};
                                        transform:{};
                                        transition: all 0.2s ease;
                                        ",
                                        if is_used() { "0.4" } else { "1" },
                                        if is_used() { "grayscale(100%)" } else { "none" },
                                        if is_used() { "scale(0.9)" } else { "scale(1)" }
                                    )
                                />
                            }
                        })
                    }).collect_view()
                }
            }
        </div>
    }
}
