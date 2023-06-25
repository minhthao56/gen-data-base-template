use csv::ReaderBuilder;
use js_sys::{decode_uri_component, JsString};
use leptos::{ev::Event, *};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{FileReader, HtmlInputElement};

use crate::{
    common::types::KeyVal,
    components::{checkbox::Checkbox, header::Header, select::Select},
    helpers::{detect_type_bash_field_name, download_file_csv, generate_csv_string},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (values, set_values) = create_signal(cx, Vec::<KeyVal>::new());

    let handle_change_filed = move |event: Event| {
        let onload = Closure::wrap(Box::new(move |reader_event: Event| {
            let element = reader_event
                .target()
                .unwrap()
                .dyn_into::<FileReader>()
                .unwrap();
            let data = element.result().unwrap();

            let file_string: JsString = data.dyn_into::<JsString>().unwrap();
            let value_string = file_string.as_string().unwrap();

            let mut csv_reader = ReaderBuilder::new().from_reader(value_string.as_bytes());
            let headers = csv_reader.headers().unwrap();

            headers.iter().for_each(|header| {
                let v = detect_type_bash_field_name(header.to_string());
                set_values.update(|values| {
                    values.push(KeyVal {
                        key: header.to_string(),
                        val: v,
                    });
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

    let handle_generate = move |_| {
        log!("handle_generate");
        let csv_string = generate_csv_string(values.get(), 10);
        log!("{}", csv_string);
        let data = decode_uri_component(csv_string.as_str()).unwrap();

        let url = format!("data:text/csv;charset=utf-8,{}", data);
        download_file_csv("test.csv".to_string(), url);
    };

    view! { cx,
        <div class="container mx-auto px-4 max-w-7xl">
           <Header handle_change_filed=handle_change_filed handle_generate=handle_generate/>
            <div class="grid gap-6 grid-cols-3 max-sm:grid-cols-1 max-lg:grid-cols-2">
                <For
                    each=values
                    key=|field| field.key.clone()
                    view=move |cx, key_value| {
                        let (select, set_select) = create_signal(cx, key_value.val.clone());

                        let (key, _) = create_signal(cx, key_value.key.clone());
                        let (checked, set_checked) = create_signal(cx, false);

                        let handel_checkbox = move |event: Event| {
                            let checked_v = event_target_checked(&event);
                            set_checked(checked_v);
                        };
                        let change_selected = move |e: Event| {
                            let value = event_target_value(&e);
                            let cp_value = value.clone();
                            set_select(value);
                            set_values
                                .update(|values| {
                                    values.iter_mut().for_each(|v| {
                                        if v.key == key.get() {
                                            v.val = cp_value.clone();
                                        }
                                    });
                                });
                        };
                        let handle_input_manual = move |e: Event| {
                            let value = event_target_value(&e);
                            set_values
                                .update(|values| {
                                    values.iter_mut().for_each(|v| {
                                        if v.key == key.get() {
                                            v.val = value.clone();
                                        }
                                    });
                                });
                        };
                        view! { cx,
                            <div>
                                <div>
                                    <label class="label font-semibold">{key.get()}</label>
                                </div>
                                <div class="flex justify-between items-center">
                                    <div class = "w-full flex-1">
                                        <Show
                                            when=move || !checked()
                                            fallback=move |cx| {
                                                view! { cx,
                                                    <input
                                                        type="text"
                                                        placeholder="Type here"
                                                        class="input input-bordered input-primary w-full max-w-xs"
                                                        on:change=handle_input_manual
                                                    />
                                                }
                                            }
                                        >
                                            <Select select=select change_selected=change_selected/>
                                        </Show>
                                    </div>
                                    <Checkbox checked=checked handel_checkbox=handel_checkbox/>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
