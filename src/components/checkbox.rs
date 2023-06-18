use leptos::{ev::Event, *};

#[component]
pub fn Checkbox<F>(
    cx: Scope,
    checked: ReadSignal<bool>,
    handel_checkbox: F,
   
) -> impl IntoView
where
    F: FnMut(Event) -> () + 'static,

 {
    view! { cx,
        <label class="cursor-pointer label">
            <input
                type="checkbox"
                class="toggle toggle-sm"
                on:change=handel_checkbox
                checked=checked
            />
            <span class="label-text ml-2">"Manual"</span>
        </label>
    }
}