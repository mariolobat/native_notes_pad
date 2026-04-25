use std::sync::Mutex;
use windows::Win32::Graphics::Gdi::LOGFONTW;

pub static CURRENT_FILE: Mutex<Option<String>> = Mutex::new(None);
pub static WORD_WRAP: Mutex<bool> = Mutex::new(false);
pub static CURRENT_FONT: Mutex<Option<LOGFONTW>> = Mutex::new(None);

pub fn get_current_lang() -> String {
    let lang = CURRENT_LANG.lock().unwrap();
    lang.clone()
}

pub fn set_current_lang(l: &str) {
    let mut lang = CURRENT_LANG.lock().unwrap();
    *lang = l.to_string();
}

pub static CURRENT_LANG: Mutex<String> = Mutex::new(String::new());
