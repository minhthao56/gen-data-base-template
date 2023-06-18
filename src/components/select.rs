use leptos::{ev::Event, *};

use crate::common::constants::OPTIONS;

#[component]
pub fn Select<F>(
    cx: Scope,
    select: ReadSignal<String>,
    change_selected: F  ,
) -> impl IntoView
where
    F: FnMut(Event) -> () + 'static,
 {
    view! { cx,
        <select
            class="select select-primary w-full max-w-xs"
            on:change=change_selected
            value=select
        >
            {OPTIONS
                .into_iter()
                .map(|option| {
                    view! { cx,
                        <option value=option selected=if option == select.get() { true } else { false }>
                            {option}
                        </option>
                    }
                })
                .collect::<Vec<_>>()}
        </select>
    }
}