pub mod helpers;

pub fn handle_pares_json (){

       // let file_name = event_target_value(&event);
        // let type_file = file_name.split(".").last().unwrap().to_string();


   // if type_file == "json" {
            //     let v: Value = serde_json::from_str(&value_string).unwrap();
            //     let v = v.as_object().unwrap();
            //     v.iter().for_each(|(key, v)| {
                    
            //         if v.is_object() {
            //             let v = v.as_object().unwrap();
            //             v.iter().for_each(|(key, v)| {
            //                 set_values.update(|values| {
            //                     values.insert(key.to_string(), "Text".to_string());
            //                 });
            //             });
            //             return;                        
            //         }
            //         if v.is_array() {
            //             let v = v.as_array().unwrap();
            //             v.iter().for_each(|v| {
            //                 if v.is_object() {
            //                     let v = v.as_object().unwrap();
            //                     v.iter().for_each(|(key, v)| {
            //                         set_values.update(|values| {
            //                             values.insert(key.to_string(), "Text".to_string());
            //                         });
            //                     });
            //                     return;                        
            //                 }
            //             });
            //             return;                        
            //         }
            //         set_parents.update(|parents| {
            //             parents.insert(key.to_string(), vec![]);
            //         });
            //         set_values.update(|values| {
            //             values.insert(key.to_string(), "Text".to_string());
            //         });
            //     });
            //     return;
            // }
}

fn apply<F>(f: F) -> i32 where F: Fn() -> i32 { f()  }

