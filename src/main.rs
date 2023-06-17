use csv::ReaderBuilder;
use js_sys::JsString;
use leptos::{ev::Event, *};
// use serde_json::{Result, Value};
use std::collections::HashMap;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FileReader, HtmlInputElement};

mod components;
use components::select::Select;


fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (values, set_values) = create_signal(cx, HashMap::<String, String>::new());

    let handle_change_filed = move |event: Event| {
        
        let onload = Closure::wrap(Box::new(move |reader_event: Event| {
            let element = reader_event.target().unwrap().dyn_into::<FileReader>().unwrap();
            let data = element.result().unwrap();
    
            let file_string: JsString = data.dyn_into::<JsString>().unwrap();
            let value_string = file_string.as_string().unwrap();
        
            let mut csv_reader = ReaderBuilder::new().from_reader(value_string.as_bytes());
            let headers = csv_reader.headers().unwrap();
    
            headers.iter().for_each(|header| {
                set_values.update(|values| {
                    values.insert(header.to_string(), "Text".to_string());
                });
            });
        }) as Box<dyn FnMut(_)>);
    
        let e = event_target::<HtmlInputElement>(&event);
        let file = e.files().unwrap().get(0).unwrap();
        let file_reader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();
        file_reader.read_as_binary_string(&file).unwrap();
        file_reader.set_onloadend(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    };

    let handle_generate = move |_| log!("values: {:?}", values.get());

    view! { cx,
        <div class="container">
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
            <div class="grid gap-4 grid-cols-4 max-sm:grid-cols-2 max-lg:grid-cols-3">
                <For
                    each=values
                    key=|field| field.clone()
                    view=move |cx, (field, v)| {
                        let (select, set_select) = create_signal(cx, v);
                        let (key, _) = create_signal(cx, field);
                        let (checked, set_checked) = create_signal(cx, false);
                        let handel_change_manual = move |event: Event| {
                            let checked_v = event_target_checked(&event);
                            set_checked(checked_v);
                        };
                        let change_selected = move |e: Event| {
                            let value = event_target_value(&e);
                            let cp_value = value.clone();
                            set_select(value);
                            set_values
                                .update(|values| {
                                    values.insert(key.get(), cp_value);
                                });
                        };
                        let handle_input_manual = move |e: Event| {
                            let value = event_target_value(&e);
                            set_values
                                .update(|values| {
                                    values.insert(key.get(), value);
                                });
                        };
                        view! { cx,
                            <div>
                                <div>
                                    <label class="label font-semibold">{key.get()}</label>
                                </div>
                                <div class="flex justify-start items-center">
                                    <Show
                                        when=move || !checked()
                                        fallback=move |cx| {
                                            view! { cx,
                                                <input
                                                    type="text"
                                                    placeholder="Type here"
                                                    class="input input-bordered input-primary  w-56 max-w-xs"
                                                    on:change=handle_input_manual
                                                />
                                            }
                                        }
                                    >
                                    <Select
                                        select = select
                                        change_selected = change_selected
                                    />
                                    </Show>
                                    <label class="cursor-pointer label">
                                        <input
                                            type="checkbox"
                                            class="toggle toggle-sm"
                                            on:change=handel_change_manual
                                            checked=checked
                                        />
                                        <span class="label-text ml-2">"Manual"</span>
                                    </label>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
