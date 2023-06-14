use std::{collections::HashMap,};

use csv::ReaderBuilder;
use js_sys::JsString;
use leptos::{ev::Event, *};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FileReader, HtmlInputElement};

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
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

   

    view! { cx,
        <div>
            <div class={"navbar bg-base-100 mb-2"}>
                <a class={"btn btn-ghost normal-case text-xl"}>"Manabie"</a>
            </div>
            <h3 class={"text-xl mb-4"}>"Generate base template (JSON or CSV)"</h3>
            <div class={"flex justify-around"}>
                <input
                    type={"file"}
                    class={"file-input file-input-bordered file-input-neutral w-full max-w-xs"}
                    on:change=change
                />
                <button class={"btn btn-primary"}>"Generate"</button>
            </div>
            <div class={"divider"}></div>
            <div class={"grid gap-4 grid-cols-4 max-sm:grid-cols-2 max-lg:grid-cols-3"}>
                <For
                    each={values}
                    key={|field| field.clone()}
                    view={move |cx, (field, v)| {
                        let is_odd = move || {
                            if v == "Manual" {
                                true
                            } else  {
                                false
                            }
                                
                        };
                        view! { cx,
                                    <div>
                                        
                                        <div>
                                            <label class={"label font-semibold"}>{field.clone()}</label>
                                        </div>
                                        // {move || match v.as_str() {
                                        //     "" if value() == 1 => {
                                        //         // <pre> returns HtmlElement<Pre>
                                        //         view! { cx, <pre>"One"</pre> }.into_any()
                                        //     },
                                        //     false if value() == 2 => {
                                        //         // <p> returns HtmlElement<P>
                                        //         // so we convert into a more generic type
                                        //         view! { cx, <p>"Two"</p> }.into_any()
                                        //     }
                                        //     _ => view! { cx, <textarea>{value()}</textarea> }.into_any()
                                        // }}
                                        <div>
                                            <select
                                                class={"select select-primary w-56 max-w-xs"}
                                                on:change={ move |e| {
                                                    let ve = event_target_value(&e);
                                                    set_values.update( |values| {
                                                        values.insert(field.clone(), ve);
                                                    });
                                                }}
                                            >
                                                <option selected>"Text"</option>
                                                <option>"Number"</option>
                                                <option>"Manual"</option>
                                            </select>
                                        </div>
                                    </div>
                            }
                    }}
                />
            </div>
        </div>
    }
}
