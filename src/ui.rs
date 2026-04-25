use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::state::{CURRENT_FILE, WORD_WRAP};
use crate::i18n::t;
use crate::{
    ID_FILE_NEW, ID_FILE_OPEN, ID_FILE_SAVE, ID_FILE_SAVE_AS, ID_FILE_CLOSE, ID_FILE_EXIT,
    ID_EDIT_UNDO, ID_EDIT_CUT, ID_EDIT_COPY, ID_EDIT_PASTE, ID_EDIT_REDO,
    ID_HELP_ABOUT, ID_HELP_SUPPORT, ID_FORMAT_WORD_WRAP, ID_FORMAT_FONT,
    ID_VIEW_ZOOM_IN, ID_VIEW_ZOOM_OUT, ID_VIEW_ZOOM_RESET,
    ID_VIEW_LANG_ES, ID_VIEW_LANG_EN,
};

pub fn w_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn update_window_title(hwnd: HWND, modified: bool) {
    if let Ok(file_lock) = CURRENT_FILE.lock() {
        let mut title = if let Some(path) = &*file_lock {
            let file_name = std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path);
            let format = t("title_format");
            format.replace("{}", file_name)
        } else {
            t("title_untitled")
        };

        if modified {
            title = format!("*{}", title);
        }

        let wstr = w_string(&title);
        unsafe {
            let _ = SetWindowTextW(hwnd, PCWSTR(wstr.as_ptr()));
        }
    }
}

pub fn build_menu(window: HWND) {
    unsafe {
        let hmenu = CreateMenu().unwrap();
        
        let hfile = CreatePopupMenu().unwrap();
        let _ = AppendMenuW(hfile, MF_STRING, ID_FILE_NEW as usize, PCWSTR(w_string(&t("file_new")).as_ptr()));
        let _ = AppendMenuW(hfile, MF_STRING, ID_FILE_OPEN as usize, PCWSTR(w_string(&t("file_open")).as_ptr()));
        let _ = AppendMenuW(hfile, MF_STRING, ID_FILE_SAVE as usize, PCWSTR(w_string(&t("file_save")).as_ptr()));
        let _ = AppendMenuW(hfile, MF_STRING, ID_FILE_SAVE_AS as usize, PCWSTR(w_string(&t("file_save_as")).as_ptr()));
        let _ = AppendMenuW(hfile, MF_SEPARATOR, 0, None);
        let _ = AppendMenuW(hfile, MF_STRING, ID_FILE_CLOSE as usize, PCWSTR(w_string(&t("file_close")).as_ptr()));
        let _ = AppendMenuW(hfile, MF_SEPARATOR, 0, None);
        let _ = AppendMenuW(hfile, MF_STRING, ID_FILE_EXIT as usize, PCWSTR(w_string(&t("file_exit")).as_ptr()));

        let hedit = CreatePopupMenu().unwrap();
        let _ = AppendMenuW(hedit, MF_STRING, ID_EDIT_UNDO as usize, PCWSTR(w_string(&t("edit_undo")).as_ptr()));
        let _ = AppendMenuW(hedit, MF_STRING, ID_EDIT_REDO as usize, PCWSTR(w_string(&t("edit_redo")).as_ptr()));
        let _ = AppendMenuW(hedit, MF_SEPARATOR, 0, None);
        let _ = AppendMenuW(hedit, MF_STRING, ID_EDIT_CUT as usize, PCWSTR(w_string(&t("edit_cut")).as_ptr()));
        let _ = AppendMenuW(hedit, MF_STRING, ID_EDIT_COPY as usize, PCWSTR(w_string(&t("edit_copy")).as_ptr()));
        let _ = AppendMenuW(hedit, MF_STRING, ID_EDIT_PASTE as usize, PCWSTR(w_string(&t("edit_paste")).as_ptr()));

        let hhelp = CreatePopupMenu().unwrap();
        let _ = AppendMenuW(hhelp, MF_STRING, ID_HELP_SUPPORT as usize, PCWSTR(w_string(&t("help_support")).as_ptr()));
        let _ = AppendMenuW(hhelp, MF_SEPARATOR, 0, None);
        let _ = AppendMenuW(hhelp, MF_STRING, ID_HELP_ABOUT as usize, PCWSTR(w_string(&t("help_about")).as_ptr()));
        
        let hformat = CreatePopupMenu().unwrap();
        let mut wrap_flags = MF_STRING;
        if *WORD_WRAP.lock().unwrap() {
            wrap_flags |= MF_CHECKED;
        }
        let _ = AppendMenuW(hformat, wrap_flags, ID_FORMAT_WORD_WRAP as usize, PCWSTR(w_string(&t("format_wordwrap")).as_ptr()));
        let _ = AppendMenuW(hformat, MF_STRING, ID_FORMAT_FONT as usize, PCWSTR(w_string(&t("format_font")).as_ptr()));

        let hview = CreatePopupMenu().unwrap();
        let _ = AppendMenuW(hview, MF_STRING, ID_VIEW_ZOOM_IN as usize, PCWSTR(w_string(&t("view_zoom_in")).as_ptr()));
        let _ = AppendMenuW(hview, MF_STRING, ID_VIEW_ZOOM_OUT as usize, PCWSTR(w_string(&t("view_zoom_out")).as_ptr()));
        let _ = AppendMenuW(hview, MF_STRING, ID_VIEW_ZOOM_RESET as usize, PCWSTR(w_string(&t("view_zoom_reset")).as_ptr()));
        
        let _ = AppendMenuW(hview, MF_SEPARATOR, 0, None);
        let hlang = CreatePopupMenu().unwrap();
        let _ = AppendMenuW(hlang, MF_STRING, ID_VIEW_LANG_ES as usize, PCWSTR(w_string(&t("lang_es")).as_ptr()));
        let _ = AppendMenuW(hlang, MF_STRING, ID_VIEW_LANG_EN as usize, PCWSTR(w_string(&t("lang_en")).as_ptr()));
        let _ = AppendMenuW(hview, MF_POPUP, hlang.0 as usize, PCWSTR(w_string(&t("view_languages")).as_ptr()));

        // Aquí es donde añadimos los menús a la barra principal.
        // Hemos añadido espacios ("   {}   ") alrededor del texto para que estén más separados entre sí.
        let _ = AppendMenuW(hmenu, MF_POPUP, hfile.0 as usize, PCWSTR(w_string(&format!("{} ", t("file_menu"))).as_ptr()));
        let _ = AppendMenuW(hmenu, MF_POPUP, hedit.0 as usize, PCWSTR(w_string(&format!("{} ", t("edit_menu"))).as_ptr()));
        let _ = AppendMenuW(hmenu, MF_POPUP, hformat.0 as usize, PCWSTR(w_string(&format!("{} ", t("format_menu"))).as_ptr()));
        let _ = AppendMenuW(hmenu, MF_POPUP, hview.0 as usize, PCWSTR(w_string(&format!("{} ", t("view_menu"))).as_ptr()));
        let _ = AppendMenuW(hmenu, MF_POPUP, hhelp.0 as usize, PCWSTR(w_string(&format!("{} ", t("help_menu"))).as_ptr()));

        let old_menu = GetMenu(window);
        let _ = SetMenu(window, Some(hmenu));
        if !old_menu.is_invalid() {
            let _ = DestroyMenu(old_menu);
        }
    }
}
