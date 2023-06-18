use std::{collections::HashMap};

use crate::common::{constants::OPTIONS, types::KeyVal};
use csv::Writer;
use rand::Rng;
use uuid::Uuid;
use web_sys::window;

// pub fn handle_pares_json (){

//        // let file_name = event_target_value(&event);
//         // let type_file = file_name.split(".").last().unwrap().to_string();

//    // if type_file == "json" {
//             //     let v: Value = serde_json::from_str(&value_string).unwrap();
//             //     let v = v.as_object().unwrap();
//             //     v.iter().for_each(|(key, v)| {

//             //         if v.is_object() {
//             //             let v = v.as_object().unwrap();
//             //             v.iter().for_each(|(key, v)| {
//             //                 set_values.update(|values| {
//             //                     values.insert(key.to_string(), "Text".to_string());
//             //                 });
//             //             });
//             //             return;
//             //         }
//             //         if v.is_array() {
//             //             let v = v.as_array().unwrap();
//             //             v.iter().for_each(|v| {
//             //                 if v.is_object() {
//             //                     let v = v.as_object().unwrap();
//             //                     v.iter().for_each(|(key, v)| {
//             //                         set_values.update(|values| {
//             //                             values.insert(key.to_string(), "Text".to_string());
//             //                         });
//             //                     });
//             //                     return;
//             //                 }
//             //             });
//             //             return;
//             //         }
//             //         set_parents.update(|parents| {
//             //             parents.insert(key.to_string(), vec![]);
//             //         });
//             //         set_values.update(|values| {
//             //             values.insert(key.to_string(), "Text".to_string());
//             //         });
//             //     });
//             //     return;
//             // }
// }

pub fn random_string_length(length: usize) -> String {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let rand_string: String = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
        .take(length)
        .collect();
    rand_string
}

pub fn random_phone_number() -> String {
    let mut rng = rand::thread_rng();
    let phone_number: String = format!("0{}", rng.gen_range(100000000..999999999));
    phone_number
}

pub fn generate_csv_string(template_csv: Vec<KeyVal>, num_rows: i32) -> String {
    let headers = template_csv
        .iter()
        .map(|key_val| key_val.key.clone())
        .collect::<Vec<String>>();

    let mut wtr = Writer::from_writer(vec![]);

    wtr.write_record(headers).unwrap();

    let mut values = Vec::<Vec<String>>::new();
    for _ in 0..num_rows {
        let mut v = Vec::<String>::new();
        template_csv.iter().for_each(|key_val| {
            let id = Uuid::new_v4();
            let val = key_val.val.clone();
            let s = random_string_length(10);
            v.push(s.to_string());
        });
        values.push(v);
    }

    values.iter().for_each(|row| {
         wtr.write_record(row).unwrap()
    });

    let result = wtr.into_inner().unwrap();
    let result = String::from_utf8(result).unwrap();

    return result;
}

pub fn detect_type_bash_field_name(field_name: String) -> String {
    let map_option_and_type = HashMap::<String, Vec<&str>>::new();

    OPTIONS.iter().for_each(|option| {
        let option = option.to_string();
        let option = option.to_lowercase();
        let option = option.replace(" ", "_");
    });

    return "".to_string();
}

pub fn download_file_csv(file_name: String, url_data: String) {
    let document = window()
        .expect("window is undefined")
        .document()
        .expect("document is undefined");
    let a = document
        .create_element("a")
        .expect("should have <a> element");

    a.set_attribute("href", &url_data).unwrap();
    a.set_attribute("download", &file_name).unwrap();

    let evt = document
        .create_event("MouseEvents")
        .expect("should have MouseEvents");
    evt.init_event("click");

    a.dispatch_event(&evt).unwrap();
}
