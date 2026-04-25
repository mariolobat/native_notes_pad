use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Controls::Dialogs::*,
    Win32::Globalization::*,
};

use crate::state::CURRENT_FILE;
use crate::i18n::t;
use crate::ui::update_window_title;

pub fn get_edit_text(hwnd: HWND) -> String {
    unsafe {
        let len = GetWindowTextLengthW(hwnd);
        if len > 0 {
            let mut buf = vec![0u16; (len + 1) as usize];
            GetWindowTextW(hwnd, &mut buf);
            if let Some(end) = buf.iter().position(|&x| x == 0) {
                buf.truncate(end);
            }
            String::from_utf16_lossy(&buf)
        } else {
            String::new()
        }
    }
}

// Función auxiliar para leer archivos compatibles con UTF-8, UTF-16 y ASCII/ANSI
pub fn read_file_string(path: &str) -> std::io::Result<String> {
    let bytes = std::fs::read(path)?;
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        let u16_data: Vec<u16> = bytes[2..].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        return Ok(String::from_utf16_lossy(&u16_data));
    } else if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        let u16_data: Vec<u16> = bytes[2..].chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
        return Ok(String::from_utf16_lossy(&u16_data));
    }
    // Detectar y omitir UTF-8 BOM si existe
    let mut utf8_data = bytes.as_slice();
    if utf8_data.len() >= 3 && utf8_data[0] == 0xEF && utf8_data[1] == 0xBB && utf8_data[2] == 0xBF {
        utf8_data = &utf8_data[3..];
    }

    // Intentar UTF-8 primero (que nativamente incluye también ASCII puro)
    match String::from_utf8(utf8_data.to_vec()) {
        Ok(s) => Ok(s),
        Err(_) => {
            // Si falla, decodificar como ASCII/ANSI local (CP_ACP)
            unsafe {
                let required_size = MultiByteToWideChar(CP_ACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0), &bytes, None);
                if required_size > 0 {
                    let mut wide_buf = vec![0u16; required_size as usize];
                    MultiByteToWideChar(CP_ACP, MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0), &bytes, Some(&mut wide_buf));
                    Ok(String::from_utf16_lossy(&wide_buf))
                } else {
                    // Fallback a pérdida
                    Ok(String::from_utf8_lossy(&bytes).into_owned())
                }
            }
        }
    }
}

pub fn set_edit_text(hwnd: HWND, text: &str) {
    let wstr: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let _ = SetWindowTextW(hwnd, PCWSTR(wstr.as_ptr()));
    }
}

pub fn is_edit_modified(hwnd: HWND) -> bool {
    unsafe {
        let result = SendMessageW(hwnd, 0x00B8 /* EM_GETMODIFY */, None, None);
        result.0 != 0
    }
}

pub fn set_edit_modified(hwnd: HWND, modified: bool) {
    unsafe {
        let _ = SendMessageW(hwnd, 0x00B9 /* EM_SETMODIFY */, Some(WPARAM(modified as usize)), None);
    }
}

pub fn open_file(hwnd_owner: HWND, hwnd_edit: HWND) {
    unsafe {
        let mut filename = vec![0u16; 260];
        let filter_str = t("filter_text");
        let filter: Vec<u16> = filter_str.replace('|', "\0").encode_utf16().chain(std::iter::once(0)).chain(std::iter::once(0)).collect();

        let mut ofn = OPENFILENAMEW {
            lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: hwnd_owner,
            lpstrFilter: PCWSTR(filter.as_ptr()),
            lpstrFile: PWSTR(filename.as_mut_ptr()),
            nMaxFile: filename.len() as u32,
            Flags: OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST,
            ..Default::default()
        };

        if GetOpenFileNameW(&mut ofn).as_bool() {
            if let Some(end) = filename.iter().position(|&c| c == 0) {
                let path = String::from_utf16_lossy(&filename[..end]);
                if let Ok(content) = read_file_string(&path) {
                    let normalized = content.replace("\r\n", "\n").replace("\n", "\r\n");
                    set_edit_text(hwnd_edit, &normalized);
                    if let Ok(mut file_lock) = CURRENT_FILE.lock() {
                        *file_lock = Some(path);
                    }
                    set_edit_modified(hwnd_edit, false);
                    update_window_title(hwnd_owner, false);
                }
            }
        }
    }
}

pub fn save_file_as(hwnd_owner: HWND, hwnd_edit: HWND) {
    unsafe {
        let mut filename = vec![0u16; 260];
        let filter_str = t("filter_text");
        let filter: Vec<u16> = filter_str.replace('|', "\0").encode_utf16().chain(std::iter::once(0)).chain(std::iter::once(0)).collect();

        let mut ofn = OPENFILENAMEW {
            lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: hwnd_owner,
            lpstrFilter: PCWSTR(filter.as_ptr()),
            lpstrFile: PWSTR(filename.as_mut_ptr()),
            nMaxFile: filename.len() as u32,
            Flags: OFN_PATHMUSTEXIST | OFN_OVERWRITEPROMPT,
            lpstrDefExt: w!("txt"),
            ..Default::default()
        };

        if GetSaveFileNameW(&mut ofn).as_bool() {
            if let Some(end) = filename.iter().position(|&c| c == 0) {
                let path = String::from_utf16_lossy(&filename[..end]);
                let content = get_edit_text(hwnd_edit);
                let _ = std::fs::write(&path, content);
                if let Ok(mut file_lock) = CURRENT_FILE.lock() {
                    *file_lock = Some(path);
                }
                set_edit_modified(hwnd_edit, false);
                update_window_title(hwnd_owner, false);
            }
        }
    }
}
