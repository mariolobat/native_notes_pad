use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DICT: Mutex<HashMap<String, String>> = Mutex::new(load_dict("es"));
}

fn load_dict(lang: &str) -> HashMap<String, String> {
    let content = match lang {
        "en" => include_str!("locales/en.json"),
        _ => include_str!("locales/es.json"),
    };
    serde_json::from_str(content).unwrap_or_default()
}

pub fn set_language(lang: &str) {
    if let Ok(mut dict) = DICT.lock() {
        *dict = load_dict(lang);
    }
}

pub fn t(key: &str) -> String {
    if let Ok(dict) = DICT.lock() {
        if let Some(val) = dict.get(key) {
            return val.clone();
        }
    }
    key.to_string()
}
