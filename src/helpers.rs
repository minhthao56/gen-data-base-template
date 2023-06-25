use std::collections::HashMap;

use crate::common::{constants::OPTIONS, types::KeyVal};
use csv::Writer;
use rand::Rng;
use uuid::Uuid;
use web_sys::window;
use chrono::prelude::*;

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

pub fn random_email() -> String {
    let email: String = format!(
        "{}@{}.{}",
        random_string_length(10),
        random_string_length(5),
        random_string_length(3)
    );
    email
}
pub fn random_time() -> String {
    let mut rng = rand::thread_rng();
    let time: String = format!(
        "{}:{}:{}",
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60)
    );
    time
}

pub fn now_date(fmt: &str   ) -> String {
    let now = Utc::now();
    now.format(fmt).to_string();
    return now.to_string();
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
            let val = key_val.val.clone();

            if val == OPTIONS[0] {
                let s = random_string_length(10);
                v.push(s);
                return;
            }
            if val == OPTIONS[1]{
                let s = rand::thread_rng().gen_range(1..1000).to_string();
                v.push(s);
                return;
            }
            if val == OPTIONS[2]{
                let s = now_date("%Y/%m/%d");
                v.push(s);
                return;
            }
            if val == OPTIONS[3]{
                let s = now_date("%d/%m/%Y");
                v.push(s);
                return;
            }
            if val == OPTIONS[4] {
                let s = now_date("%m-%d-%Y");
                v.push(s);
                return;
            }
            if val == OPTIONS[5]{
                let id = Uuid::new_v4();
                v.push(id.to_string());
                return;
            }
            if val == OPTIONS[6]{
                let s = random_time();
                v.push(s);
                return;
            }
            if val == OPTIONS[7]{
                v.push("true".to_string());
                return;
            }
            if val == OPTIONS[8]{
                v.push(val.to_string());
                return;
            }
            if val == OPTIONS[9]{
                v.push(random_email());
                return;
            }
            if val == OPTIONS[10]{
                v.push(random_phone_number());
                return;
            }
            
        });
        values.push(v);
    }

    values.iter().for_each(|row| wtr.write_record(row).unwrap());

    let result = wtr.into_inner().unwrap();
    let result = String::from_utf8(result).unwrap();

    return result;
}

pub fn build_map_detect_type() -> HashMap<String, Vec<&'static str>> {
    let mut map_option_and_detect_type = HashMap::<String, Vec<&str>>::new();
    let is_text = vec![
        "name",
        "contact",
        "remarks",
        "address",
        "note",
        "stress",
        "title",
        "description",
        "author",
    ];
    let is_number = vec![
        "age", "amount", "quantity", "price", "total", "count", "weight",
    ];
    let is_date = vec!["birthday", "date", "day", "dob", "created_at", "updated_at"];
    let is_id = vec![
        "id", "uuid", "guid", "key", "code", "serial", "number", "index", "order",
    ];
    let is_time = vec!["time", "hour", "minute"];
    let is_boolean = vec![
        "is",
        "has",
        "can",
        "should",
        "will",
        "did",
        "do",
        "done",
        "completed",
    ];
    let is_email = vec!["email", "mail", "e-mail", "outlook", "gmail", "yahoo"];
    let is_phone = vec!["phone", "mobile", "telephone"];

    map_option_and_detect_type.insert(OPTIONS[0].to_owned(), is_text);
    map_option_and_detect_type.insert(OPTIONS[1].to_owned(), is_number);
    map_option_and_detect_type.insert(OPTIONS[2].to_owned(), is_date.clone());
    map_option_and_detect_type.insert(OPTIONS[3].to_owned(), is_date.clone());
    map_option_and_detect_type.insert(OPTIONS[4].to_owned(), is_date);
    map_option_and_detect_type.insert(OPTIONS[5].to_owned(), is_id);
    map_option_and_detect_type.insert(OPTIONS[6].to_owned(), is_time);
    map_option_and_detect_type.insert(OPTIONS[7].to_owned(), is_boolean);
    map_option_and_detect_type.insert(OPTIONS[9].to_owned(), is_email);
    map_option_and_detect_type.insert(OPTIONS[10].to_owned(), is_phone);
    return map_option_and_detect_type;
}

pub fn detect_type_bash_field_name(field_name: String) -> String {
    let map_option_and_detect_type = build_map_detect_type();
    let mut type_field = OPTIONS[0].to_string();

    map_option_and_detect_type.iter().for_each(|(key_type, detect_types)| {
        detect_types.iter().for_each(|detect_type| {
            if field_name.contains(detect_type) {
                type_field = key_type.to_string();
            }
        });
    });
    return type_field;
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
