#![windows_subsystem = "windows"]

pub mod state;
pub mod i18n;
pub mod editor;
pub mod ui;

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::Controls::Dialogs::*,
    Win32::UI::HiDpi::*,
    Win32::UI::Shell::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

use crate::state::*;
use crate::i18n::*;
use crate::editor::*;
use crate::ui::*;

pub const ID_EDIT: i32 = 101;
// File
pub const ID_FILE_NEW: u32 = 102;
pub const ID_FILE_OPEN: u32 = 103;
pub const ID_FILE_SAVE: u32 = 104;
pub const ID_FILE_SAVE_AS: u32 = 105;
pub const ID_FILE_CLOSE: u32 = 106;
pub const ID_FILE_EXIT: u32 = 107;
// Edit
pub const ID_EDIT_UNDO: u32 = 201;
pub const ID_EDIT_CUT: u32 = 202;
pub const ID_EDIT_COPY: u32 = 203;
pub const ID_EDIT_PASTE: u32 = 204;
pub const ID_EDIT_REDO: u32 = 205;
// Help
pub const ID_HELP_ABOUT: u32 = 301;
pub const ID_HELP_SUPPORT: u32 = 302;
// Format
pub const ID_FORMAT_WORD_WRAP: u32 = 401;
pub const ID_FORMAT_FONT: u32 = 402;
// View
pub const ID_VIEW_ZOOM_IN: u32 = 501;
pub const ID_VIEW_ZOOM_OUT: u32 = 502;
pub const ID_VIEW_ZOOM_RESET: u32 = 503;
pub const ID_VIEW_LANG_ES: u32 = 601;
pub const ID_VIEW_LANG_EN: u32 = 602;

pub fn get_default_zoom() -> usize {
    unsafe {
        let height = GetSystemMetrics(SM_CYSCREEN);
        if height <= 1080 {
            100
        } else if height <= 1440 {
            125
        } else {
            150
        }
    }
}
pub const DEFAULT_ZOOM_DEN: isize = 100; // El divisor base

// Edit control styles
const ES_LEFT: u32 = 0x0000;
const ES_MULTILINE: u32 = 0x0004;
const ES_AUTOVSCROLL: u32 = 0x0040;
const ES_WANTRETURN: u32 = 0x1000;
const ES_AUTOHSCROLL: u32 = 0x0080;

fn main() -> Result<()> {
    // Configuración inicial del idioma basada en el sistema operativo
    unsafe {
        // Obtenemos el idioma principal del usuario
        let lang_id = windows::Win32::Globalization::GetUserDefaultUILanguage();
        let primary_lang = lang_id & 0x3FF; // 0x0A es Español, 0x09 es Inglés
        
        if primary_lang == 0x0A {
            set_current_lang("es");
            set_language("es");
        } else {
            set_current_lang("en");
            set_language("en");
        }
    }

    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        let _ = LoadLibraryW(w!("MsftEdit.dll"));

        let instance = GetModuleHandleW(None)?;
        let window_class = w!("MinimalNotepadClass");

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbWndExtra: 0,
            cbClsExtra: 0,
            hIcon: LoadIconW(Some(instance.into()), PCWSTR(1usize as *mut u16)).unwrap_or_default(),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as usize as *mut _),
            lpszMenuName: PCWSTR::null(),
        };

        if RegisterClassW(&wc) == 0 {
            return Err(Error::from_thread());
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            PCWSTR::null(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1280,
            768,
            None,
            None,
            Some(instance.into()),
            None,
        )?;

        update_window_title(hwnd, false);

        let accelerators = [
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: 'N' as u16, cmd: ID_FILE_NEW as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: 'O' as u16, cmd: ID_FILE_OPEN as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: 'S' as u16, cmd: ID_FILE_SAVE as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: 'Z' as u16, cmd: ID_EDIT_UNDO as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: 'Y' as u16, cmd: ID_EDIT_REDO as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: VK_OEM_PLUS.0, cmd: ID_VIEW_ZOOM_IN as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: VK_OEM_MINUS.0, cmd: ID_VIEW_ZOOM_OUT as u16 },
            ACCEL { fVirt: FVIRTKEY | FCONTROL, key: '0' as u16, cmd: ID_VIEW_ZOOM_RESET as u16 },
        ];
        let haccel = CreateAcceleratorTableW(&accelerators)?;

        let mut message = MSG::default();

        while GetMessageW(&mut message, None, 0, 0).into() {
            if TranslateAcceleratorW(hwnd, haccel, &mut message) == 0 {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                let instance = GetModuleHandleW(None).unwrap();

                // Removed WS_BORDER to avoid the hard black visual border
                let mut style = WS_CHILD | WS_VISIBLE | WS_VSCROLL
                    | WINDOW_STYLE(ES_LEFT) | WINDOW_STYLE(ES_MULTILINE) 
                    | WINDOW_STYLE(ES_AUTOVSCROLL) | WINDOW_STYLE(ES_WANTRETURN);
                
                if !*WORD_WRAP.lock().unwrap() {
                    style |= WS_HSCROLL | WINDOW_STYLE(ES_AUTOHSCROLL as u32);
                }

                let edit_hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("RichEdit50W"),
                    None,
                    style,
                    0, 0, 0, 0,
                    Some(window),
                    Some(HMENU(ID_EDIT as _)),
                    Some(instance.into()),
                    None,
                ).unwrap();

                let font_ptr = {
                    let mut chosen_font = HFONT::default();
                    if let Ok(font_lock) = CURRENT_FONT.lock() {
                        if let Some(logfont) = &*font_lock {
                            chosen_font = CreateFontIndirectW(logfont);
                        } else {
                            // Crear Consolas por defecto
                            let mut lf = LOGFONTW::default();
                            lf.lfHeight = -20; // Tamaño aproximado
                            lf.lfWeight = 400; // Normal
                            let name = w!("Consolas");
                            let len = name.as_wide().len().min(lf.lfFaceName.len() - 1);
                            lf.lfFaceName[..len].copy_from_slice(&name.as_wide()[..len]);
                            chosen_font = CreateFontIndirectW(&lf);
                        }
                    }
                    chosen_font.0 as _
                };
                
                SendMessageW(edit_hwnd, WM_SETFONT, Some(WPARAM(font_ptr)), Some(LPARAM(1)));
                SendMessageW(edit_hwnd, 0x0435 /* EM_EXLIMITTEXT */, Some(WPARAM(0)), Some(LPARAM(-1)));
                SendMessageW(edit_hwnd, 0x04E1 /* EM_SETZOOM */, Some(WPARAM(get_default_zoom())), Some(LPARAM(DEFAULT_ZOOM_DEN)));
                
                // Activar notificaciones de cambio para el asterisco
                let _ = SendMessageW(edit_hwnd, 0x0445 /* EM_SETEVENTMASK */, None, Some(LPARAM(0x0001 /* ENM_CHANGE */ as isize)));

                DragAcceptFiles(window, true);
                build_menu(window);

                LRESULT(0)
            }
            WM_SIZE => {
                if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                    let mut rect = RECT::default();
                    let _ = GetClientRect(window, &mut rect);
                    let _ = MoveWindow(edit_hwnd, 0, 0, rect.right - rect.left, rect.bottom - rect.top, true);
                    
                    // Añadir margenes (padding) internos
                    let mut edit_rect = rect;
                    let padding = 2;
                    edit_rect.left += padding;
                    edit_rect.top += padding;
                    edit_rect.right -= padding;
                    edit_rect.bottom -= padding;
                    let _ = SendMessageW(edit_hwnd, 0x00B3 /* EM_SETRECT */, None, Some(LPARAM(&edit_rect as *const _ as isize)));
                }
                LRESULT(0)
            }
            WM_CLOSE => {
                if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                    if is_edit_modified(edit_hwnd) {
                        let res = MessageBoxW(
                            Some(window),
                            PCWSTR(w_string(&t("msg_save_text")).as_ptr()),
                            PCWSTR(w_string(&t("msg_save_title")).as_ptr()),
                            MB_YESNOCANCEL | MB_ICONWARNING,
                        );
                        
                        match res {
                            IDYES => {
                                // Save and then close
                                let path_opt = { CURRENT_FILE.lock().unwrap().clone() };
                                if let Some(path) = path_opt {
                                    let content = get_edit_text(edit_hwnd);
                                    let _ = std::fs::write(&path, content);
                                } else {
                                    save_file_as(window, edit_hwnd);
                                    // If user canceled save dialog! We shouldn't close.
                                    // is_edit_modified should ideally track save success.
                                    // For now check if it saved:
                                    if is_edit_modified(edit_hwnd) {
                                        return LRESULT(0);
                                    }
                                }
                            }
                            IDNO => {
                                // proceed
                            }
                            _ => {
                                // IDCANCEL
                                return LRESULT(0);
                            }
                        }
                    }
                }
                DefWindowProcW(window, message, wparam, lparam)
            }
            WM_COMMAND => {
                let id = wparam.0 as u32 & 0xFFFF;
                
                // Detectar cambios en el texto para el asterisco
                if id == ID_EDIT as u32 {
                    let code = (wparam.0 >> 16) as u16;
                    if code == 0x0300 /* EN_CHANGE */ {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            update_window_title(window, is_edit_modified(edit_hwnd));
                        }
                    }
                }

                match id {
                    ID_FILE_NEW => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            // verify un-saved modifications
                            if is_edit_modified(edit_hwnd) {
                                let res = MessageBoxW(
                                    Some(window),
                                    PCWSTR(w_string(&t("msg_save_text")).as_ptr()),
                                    PCWSTR(w_string(&t("msg_save_title")).as_ptr()),
                                    MB_YESNOCANCEL | MB_ICONWARNING,
                                );
                                if res == IDYES {
                                    let path_opt = { CURRENT_FILE.lock().unwrap().clone() };
                                    if let Some(path) = path_opt {
                                        let content = get_edit_text(edit_hwnd);
                                        let _ = std::fs::write(&path, content);
                                    } else {
                                        save_file_as(window, edit_hwnd);
                                    }
                                } else if res == IDCANCEL {
                                    return LRESULT(0);
                                }
                            }
                            
                            set_edit_text(edit_hwnd, "");
                            set_edit_modified(edit_hwnd, false);
                            if let Ok(mut file_lock) = CURRENT_FILE.lock() {
                                *file_lock = None;
                            }
                            update_window_title(window, false);
                            // Re-aplicar zoom
                            let _ = SendMessageW(edit_hwnd, 0x04E1 /* EM_SETZOOM */, Some(WPARAM(get_default_zoom())), Some(LPARAM(DEFAULT_ZOOM_DEN)));
                        }
                    }
                    ID_FILE_OPEN => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            if is_edit_modified(edit_hwnd) {
                                let res = MessageBoxW(
                                    Some(window),
                                    PCWSTR(w_string(&t("msg_save_text")).as_ptr()),
                                    PCWSTR(w_string(&t("msg_save_title")).as_ptr()),
                                    MB_YESNOCANCEL | MB_ICONWARNING,
                                );
                                if res == IDYES {
                                    let path_opt = { CURRENT_FILE.lock().unwrap().clone() };
                                    if let Some(path) = path_opt {
                                        let content = get_edit_text(edit_hwnd);
                                        let _ = std::fs::write(&path, content);
                                    } else {
                                        save_file_as(window, edit_hwnd);
                                        if is_edit_modified(edit_hwnd) { return LRESULT(0); }
                                    }
                                } else if res == IDCANCEL {
                                    return LRESULT(0);
                                }
                            }
                            open_file(window, edit_hwnd);
                            // Re-aplicar zoom
                            let _ = SendMessageW(edit_hwnd, 0x04E1 /* EM_SETZOOM */, Some(WPARAM(get_default_zoom())), Some(LPARAM(DEFAULT_ZOOM_DEN)));
                        }
                    }
                    ID_FILE_SAVE => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let path_opt = { CURRENT_FILE.lock().unwrap().clone() };
                            if let Some(path) = path_opt {
                                let content = get_edit_text(edit_hwnd);
                                let _ = std::fs::write(&path, content);
                                set_edit_modified(edit_hwnd, false);
                            } else {
                                save_file_as(window, edit_hwnd);
                            }
                        }
                    }
                    ID_FILE_SAVE_AS => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            save_file_as(window, edit_hwnd);
                        }
                    }
                    ID_FILE_CLOSE => {
                        let _ = SendMessageW(window, WM_COMMAND, Some(WPARAM(ID_FILE_NEW as _)), Some(LPARAM(0)));
                    }
                    ID_FILE_EXIT => {
                        let _ = SendMessageW(window, WM_CLOSE, Some(WPARAM(0)), Some(LPARAM(0)));
                    }
                    ID_EDIT_UNDO => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let _ = SendMessageW(edit_hwnd, WM_UNDO, Some(WPARAM(0)), Some(LPARAM(0)));
                        }
                    }
                    ID_EDIT_CUT => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let _ = SendMessageW(edit_hwnd, WM_CUT, Some(WPARAM(0)), Some(LPARAM(0)));
                        }
                    }
                    ID_EDIT_COPY => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let _ = SendMessageW(edit_hwnd, WM_COPY, Some(WPARAM(0)), Some(LPARAM(0)));
                        }
                    }
                    ID_EDIT_PASTE => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let _ = SendMessageW(edit_hwnd, WM_PASTE, Some(WPARAM(0)), Some(LPARAM(0)));
                        }
                    }
                    ID_EDIT_REDO => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let _ = SendMessageW(edit_hwnd, 0x0454 /* EM_REDO */, Some(WPARAM(0)), Some(LPARAM(0)));
                        }
                    }
                    ID_HELP_ABOUT => {
                        let _ = MessageBoxW(
                            Some(window),
                            PCWSTR(w_string(&t("msg_about_text")).as_ptr()),
                            PCWSTR(w_string(&t("msg_about_title")).as_ptr()),
                            MB_OK | MB_ICONINFORMATION,
                        );
                    }
                    ID_HELP_SUPPORT => {
                        let _ = ShellExecuteW(
                            None,
                            w!("open"),
                            w!("https://ko-fi.com/mariolobato"),
                            PCWSTR::null(),
                            PCWSTR::null(),
                            SW_SHOWNORMAL,
                        );
                    }
                    ID_FORMAT_WORD_WRAP => {
                        let mut wrap = WORD_WRAP.lock().unwrap();
                        *wrap = !*wrap;
                        
                        let hmenu = GetMenu(window);
                        let state = if *wrap { MF_CHECKED } else { MF_UNCHECKED };
                        let _ = CheckMenuItem(hmenu, ID_FORMAT_WORD_WRAP, (MF_BYCOMMAND | state).0 as u32);
                        
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let line_width = if *wrap { 0 } else { 1000000 };
                            let _ = SendMessageW(edit_hwnd, 0x0448 /* EM_SETTARGETDEVICE */, None, Some(LPARAM(line_width)));
                        }
                    }
                    ID_FORMAT_FONT => {
                        let mut logfont = if let Ok(font_lock) = CURRENT_FONT.lock() {
                            if let Some(lf) = &*font_lock {
                                *lf
                            } else {
                                LOGFONTW::default()
                            }
                        } else {
                            LOGFONTW::default()
                        };
                        
                        let mut cf = CHOOSEFONTW {
                            lStructSize: std::mem::size_of::<CHOOSEFONTW>() as u32,
                            hwndOwner: window,
                            lpLogFont: &mut logfont,
                            Flags: CF_SCREENFONTS | CF_INITTOLOGFONTSTRUCT,
                            ..Default::default()
                        };
                        
                        if ChooseFontW(&mut cf).as_bool() {
                            if let Ok(mut font_lock) = CURRENT_FONT.lock() {
                                *font_lock = Some(logfont);
                            }
                            if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                                let new_font = CreateFontIndirectW(&logfont);
                                SendMessageW(edit_hwnd, WM_SETFONT, Some(WPARAM(new_font.0 as _)), Some(LPARAM(1)));
                                let _ = InvalidateRect(Some(edit_hwnd), None, true);
                            }
                        }
                    }
                    ID_VIEW_ZOOM_IN => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let mut num: u32 = 0;
                            let mut den: u32 = 0;
                            let _ = SendMessageW(edit_hwnd, 0x04E0 /* EM_GETZOOM */, Some(WPARAM(&mut num as *mut u32 as usize)), Some(LPARAM(&mut den as *mut u32 as isize)));
                            if den == 0 { num = 100; den = 100; }
                            if num < 500 {
                                let _ = SendMessageW(edit_hwnd, 0x04E1 /* EM_SETZOOM */, Some(WPARAM((num + 10) as usize)), Some(LPARAM(den as isize)));
                            }
                        }
                    }
                    ID_VIEW_ZOOM_OUT => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let mut num: u32 = 0;
                            let mut den: u32 = 0;
                            let _ = SendMessageW(edit_hwnd, 0x04E0 /* EM_GETZOOM */, Some(WPARAM(&mut num as *mut u32 as usize)), Some(LPARAM(&mut den as *mut u32 as isize)));
                            if den == 0 { num = 100; den = 100; }
                            if num > 20 {
                                let _ = SendMessageW(edit_hwnd, 0x04E1 /* EM_SETZOOM */, Some(WPARAM((num - 10) as usize)), Some(LPARAM(den as isize)));
                            }
                        }
                    }
                    ID_VIEW_ZOOM_RESET => {
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            let _ = SendMessageW(edit_hwnd, 0x04E1 /* EM_SETZOOM */, Some(WPARAM(get_default_zoom())), Some(LPARAM(DEFAULT_ZOOM_DEN)));
                        }
                    }
                    ID_VIEW_LANG_ES => {
                        set_current_lang("es");
                        set_language("es");
                        build_menu(window);
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            update_window_title(window, is_edit_modified(edit_hwnd));
                        }
                    }
                    ID_VIEW_LANG_EN => {
                        set_current_lang("en");
                        set_language("en");
                        build_menu(window);
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            update_window_title(window, is_edit_modified(edit_hwnd));
                        }
                    }
                    _ => {}
                }
                LRESULT(0)
            }
            WM_DROPFILES => {
                let hdrop = HDROP(wparam.0 as *mut _);
                let mut filename = vec![0u16; 260];
                let count = DragQueryFileW(hdrop, 0xFFFFFFFF, None);
                if count > 0 {
                    DragQueryFileW(hdrop, 0, Some(&mut filename));
                    if let Some(end) = filename.iter().position(|&c| c == 0) {
                        let path = String::from_utf16_lossy(&filename[..end]);
                        if let Ok(edit_hwnd) = GetDlgItem(Some(window), ID_EDIT) {
                            if let Ok(content) = crate::editor::read_file_string(&path) {
                                let normalized = content.replace("\r\n", "\n").replace("\n", "\r\n");
                                set_edit_text(edit_hwnd, &normalized);
                                if let Ok(mut file_lock) = CURRENT_FILE.lock() {
                                    *file_lock = Some(path);
                                }
                                set_edit_modified(edit_hwnd, false);
                                update_window_title(window, false);
                            }
                        }
                    }
                }
                DragFinish(hdrop);
                LRESULT(0)
            }
            WM_MOUSEWHEEL => {
                let delta = (wparam.0 >> 16) as i16;
                let keys = (wparam.0 & 0xFFFF) as u16;
                if (keys & 0x0008 /* MK_CONTROL */) != 0 {
                    if delta > 0 {
                        let _ = SendMessageW(window, WM_COMMAND, Some(WPARAM(ID_VIEW_ZOOM_IN as usize)), Some(LPARAM(0)));
                    } else {
                        let _ = SendMessageW(window, WM_COMMAND, Some(WPARAM(ID_VIEW_ZOOM_OUT as usize)), Some(LPARAM(0)));
                    }
                    return LRESULT(0);
                }
                DefWindowProcW(window, message, wparam, lparam)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
