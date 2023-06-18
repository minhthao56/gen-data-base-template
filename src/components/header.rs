use leptos::{ev::Event,ev::MouseEvent, *};

#[component]
pub fn Header<F, S>(
    cx: Scope,
    handle_change_filed: F,
    handle_generate:S
   
) -> impl IntoView
where
    F: FnMut(Event) -> () + 'static,
    S: FnMut(MouseEvent) -> () + 'static,
 {
    view! { cx,
        <>
            <div class="navbar bg-base-100 mb-2">
                <a class="btn btn-ghost normal-case text-xl">"Manabie"</a>
            </div>
            <h3 class="text-xl mb-4">"Generate base template (JSON or CSV)"</h3>
            <div class="flex justify-around">
                <input
                    type="file"
                    class="file-input file-input-bordered file-input-neutral w-full max-w-xs"
                    on:change=handle_change_filed
                    accept="application/json, text/csv"
                />
                <button class="btn btn-primary" on:click=handle_generate>
                    "Generate"
                </button>
            </div>
            <div class="divider"></div>
        </>
    }
}