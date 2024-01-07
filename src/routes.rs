use rocket::fs::TempFile;
use rocket::{form::Form, serde::json::Json};
use serde_json::Value;
use std::collections::HashMap;

#[path = "responses.rs"]
mod responses;

#[path = "./utils/utils.rs"]
mod utils;
#[derive(FromForm, Debug)]
pub struct Upload<'f> {
    file: TempFile<'f>,
}
#[derive(FromForm, Debug)]
pub struct UploadFiles<'f> {
    file: TempFile<'f>,
    file2: TempFile<'f>,
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello World!"
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub async fn upload_file(
    form: Form<Upload<'_>>,
) -> Result<Json<responses::SuccessResponse<responses::EmptyTradKeys>>, responses::Error> {

    let json = utils::file_content(form.file.path());
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
        data: responses::EmptyTradKeys {
            empty_trad_keys: empty_trad_key_array,
        },
        code: 200,
    }))
}

#[post(
    "/convert_php_files_to_json",
    format = "multipart/form-data",
    data = "<form>"
)]
pub async fn convert_php_files_to_json(
    form: Form<UploadFiles<'_>>,
) -> Result<Json<responses::SuccessResponse<responses::EmptyTradKeysHashMap>>, responses::Error> {

    let json_map: HashMap<String, Value> = utils::get_php_file_key_values(form.file.path());
    let json_map2: HashMap<String, Value> = utils::get_php_file_key_values(form.file2.path());

    let mut diff: HashMap<String, (String, String)> = HashMap::new();
    if json_map.ne(&json_map2) {
        diff = utils::differences_hashmaps(&json_map, &json_map2);
    }

    Ok(Json(responses::SuccessResponse {
        data: responses::EmptyTradKeysHashMap {
            empty_or_missing_trad_keys: diff,
        },
        code: 200,
    }))
}





