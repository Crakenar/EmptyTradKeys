use serde_json::Value;
use std::fs;
use rocket::{form::Form, serde::json::Json};
use rocket::fs::TempFile;

#[path = "responses.rs"]
mod responses;

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>,
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub async fn upload_file(form: Form<Upload<'_>>) -> Result<Json<responses::SuccessResponse<responses::EmptyTradKeys>>, responses::Error> {
    let option_path = Some(form.file.path()).unwrap();
    let path = option_path.unwrap();

    let x = fs::read_to_string(path).unwrap();
    let json: Value = serde_json::from_str(&x).unwrap();
    let parsed_array: Vec<Value> = serde_json::from_str(&json.to_string()).unwrap();


    let mut empty_trad_key_array: Vec<String> = Vec::new();
    for object in &parsed_array {
        let object_parsed_data: Value = serde_json::from_str(&object.to_string()).unwrap();

        for (key, value) in object_parsed_data.as_object().unwrap() {
            match value {
                Value::String(s) => {
                    if s.is_empty() {
                        empty_trad_key_array.push(key.to_owned())
                    }
                }
                Value::Null => empty_trad_key_array.push(key.to_owned()),
                Value::Array(arr) => {
                    for element in arr {
                        if let Value::Object(obj) = element {
                            for (child_key, child_value) in obj {
                                match child_value {
                                    Value::String(s) => {
                                        if s.is_empty() {
                                            empty_trad_key_array.push(child_key.to_owned())
                                        }
                                    }
                                    Value::Null => empty_trad_key_array.push(key.to_owned()),
                                    _ => {
                                        // code for handling any other variant not explicitly covered
                                        continue;
                                    }
                                };
                            }
                        };
                    }
                }
                _ => {
                    continue;
                    // code for handling any other variant not explicitly covered
                }
            }
        }
    }
    Ok(Json(responses::SuccessResponse {
        data: responses::EmptyTradKeys { empty_trad_keys: empty_trad_key_array },
        code: 200,
    }))
}