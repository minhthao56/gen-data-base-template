use std::{collections::HashMap, hash::Hash};

use csv::ReaderBuilder;
use js_sys::JsString;
use leptos::{ev::Event, *};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FileReader, HtmlInputElement};

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

struct LableValue {
    label: String,
    value: String,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (fields, set_fields) = create_signal(cx, Vec::<String>::new());

    let (values, set_values) = create_signal(cx, HashMap::<String, String>::new());

    let change = move |event: Event| {
        let onload = Closure::wrap(Box::new(move |event: Event| {
            let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
            let data = element.result().unwrap();
            let file_string: JsString = data.dyn_into::<JsString>().unwrap();
            let t = file_string.as_string().unwrap();
            let mut csv_reader = ReaderBuilder::new().from_reader(t.as_bytes());
            let headers = csv_reader.headers().unwrap();

            headers.iter().for_each(|header| {
                set_fields.update(move |fields| fields.push(header.to_string()));
                set_values.update(move |values| {
                    values.insert(header.to_string(), "text".to_string());
                });
            });
        }) as Box<dyn FnMut(_)>);

        let e = event_target::<HtmlInputElement>(&event);
        let _file = e.files().unwrap().get(0).unwrap();
        let filereader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();
        filereader.read_as_binary_string(&_file).unwrap();
        filereader.set_onloadend(Some(onload.as_ref().unchecked_ref()));

        onload.forget();
    };

    let change_select = move |e: Event| {

        let v = event_target_value(&e);
        log!("v: {}", v);
       

    };

    view! { cx,
        <div>
        <div class="navbar bg-base-100 mb-2">
            <a class="btn btn-ghost normal-case text-xl">"Manabie"</a>
        </div>
        <h3 class = "text-xl mb-4">"Generate base template (JSON or CSV)"</h3>
        <div class = "flex justify-around">
            <input
                type="file"
                class="file-input file-input-bordered file-input-neutral w-full max-w-xs"
                on:change = change
            />
            <button class="btn btn-primary">"Gen"</button>
        </div>
            <div class="divider"></div>
            <div class="grid gap-4 grid-cols-4 max-sm:grid-cols-2 max-lg:grid-cols-3">
                <For
                    each=fields
                    key=|field| field.clone()
                    view=move |cx, field| {
                        view! { cx,
                            <div >
                                <div>
                                    <label class="label font-semibold">{field}</label>
                                </div>
                                <div>
                                    <select class="select select-primary w-56 max-w-xs" on:change = change_select>
                                    <option selected>"Text"</option>
                                    <option>"Number"</option>
                                    <option>"Manual"</option>
                                    </select>
                                </div>
                            </div>

                        }
                    }
                />
            </div>

        </div>
    }
}
