use leptos::prelude::*;

#[derive(Clone)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub leaving: bool,
}

#[component]
pub fn ToastContainer(toasts: ReadSignal<Vec<Toast>>) -> impl IntoView {
    view! {
        <div style="
            position:fixed;
            top:20px;
            right:20px;
            display:flex;
            flex-direction:column;
            gap:10px;
            z-index:999;
        ">

            {
                move || {
                    toasts.get().into_iter().map(|toast| {

                        view! {
                            <div
                                style=move || format!("
                                    background:#1e293b;
                                    color:white;
                                    padding:10px 16px;
                                    border-radius:8px;
                                    box-shadow:0 5px 15px rgba(0,0,0,0.3);
                                    transform:translateX({});
                                    opacity:{};
                                    transition: all 0.3s ease;
                                    border: 2px solid #334155;
                                    border-left: 4px solid #be1414;

                                    ",
                                    if toast.leaving { "120%" } else { "0%" },
                                    if toast.leaving { "0" } else { "1" }
                                )
                            >
                                {toast.message.clone()}
                            </div>
                        }

                    }).collect_view()
                }
            }

        </div>
    }
}
