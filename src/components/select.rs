use leptos::{ev::Event, *};

#[component]
pub fn Select(
    cx: Scope,
    select: ReadSignal<String>,
    change_selected:  impl FnMut(Event) + 'static,
) -> impl IntoView {
    view! { cx,
        <div>
            <select
                class="select select-primary w-56 max-w-xs"
                on:change=change_selected
                value=select
            >
                {vec!["Text", "Number"]
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
        </div>
    }
}
#[component]
fn ProgressBar<F>(
    cx: Scope,
    #[prop(default = 100)]
    max: u16,
    progress: F
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! { cx,
        <progress
            max=max
            value=progress
        />
    }
}