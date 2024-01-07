use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
pub fn get_php_file_key_values(path: Option<&Path>) -> HashMap<String, Value> {
    let file_content = file_content(path);

    // Remove "<?php", "\r\n\r\n", and "return" from the PHP string
    let valid_json_string: String = format_php_file(file_content);

    // Parse the cleaned PHP array string into a serde_json::Value
    let value: Value = match serde_json::from_str(&valid_json_string) {
        Ok(data) => data,
        Err(err) => {
            println!("Error parsing valid_json_string data: {:?}", err);
            std::process::exit(1)
        }
    };

    // Convert the serde_json::Value into a HashMap<String, Value>
    let json_map: HashMap<String, Value> = map_json(value);
    json_map
}

fn trim_double_quotes(my_string: String) -> String {
    my_string.chars().filter(|&c| c != '"').collect()
}

pub fn differences_hashmaps<'a>(
    map1: &'a HashMap<String, Value>,
    map2: &'a HashMap<String, Value>,
) -> HashMap<String, (String, String)> {
    let mut differences: HashMap<String, (String, String)> = HashMap::new();
    for (key, value1) in map1.iter() {
        if let Some(value2) = map2.get(key) {
            if value1 != value2 {
                differences.insert(
                    key.to_string(),
                    (
                        trim_double_quotes(value1.to_string()),
                        trim_double_quotes(value2.to_string()),
                    ),
                );
            }
        } else {
            differences.insert(
                key.to_string(),
                (trim_double_quotes(value1.to_string()), String::new()),
            );
        }
    }

    for (key, value2) in map2.iter() {
        if !map1.contains_key(key) {
            differences.insert(
                key.to_string(),
                (String::new(), trim_double_quotes(value2.to_string())),
            );
        }
    }

    differences
}

fn format_php_file(file_content: String) -> String {
    // Remove "<?php", "\r\n\r\n", and "return" from the PHP string
    let cleaned_php_string = file_content
        .replace("<?php", "")
        .replace("\r\n\r\n", "")
        .replace("return", "")
        .replace("[", "{")
        .replace("]", "}")
        .replace(";", "")
        .replace("=>", ":");

    cleaned_php_string.replace("'", "\"")
}

fn map_json(value: Value) -> HashMap<String, Value> {
    // Convert the serde_json::Value into a HashMap<String, Value>
    let mut json_map: HashMap<String, Value> = HashMap::new();

    match value {
        Value::Object(obj) => {
            for (key, val) in obj {
                json_map.insert(key, val);
            }
        }
        _ => panic!("Invalid JSON format"),
    }
    json_map
}

pub fn file_content(path: Option<&Path>) -> String {
    let option_path = Some(path).unwrap();
    let path = option_path.unwrap();

    // Open the file
    // Read the content of the file into a String
    fs::read_to_string(path).unwrap()
}
