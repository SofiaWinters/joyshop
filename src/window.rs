use std::mem::size_of;
use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::um::wingdi::{GetStockObject, WHITE_BRUSH};
use winapi::um::winuser::{
    CreateWindowExW, GetWindowLongW, LoadCursorW, LoadIconW, RegisterClassExW,
    SetLayeredWindowAttributes, SetWindowLongW, ShowWindow, UpdateWindow, CS_HREDRAW, CS_VREDRAW,
    GWL_STYLE, IDC_ARROW, IDI_APPLICATION, LWA_ALPHA, MAKEINTRESOURCEW, SW_NORMAL, WNDCLASSEXW,
    WS_CAPTION, WS_EX_LAYERED, WS_EX_TRANSPARENT,
};

#[derive(Clone, Copy, Debug)]
pub struct Handle {
    hwnd: usize,
}

impl Handle {
    pub fn is_null(self) -> bool {
        self.hwnd == 0
    }
}

impl From<HWND> for Handle {
    fn from(hwnd: HWND) -> Self {
        unsafe { std::mem::transmute(hwnd) }
    }
}

impl From<Handle> for HWND {
    fn from(handle: Handle) -> Self {
        unsafe { std::mem::transmute(handle) }
    }
}

pub struct Window {
    pub handle: Handle,
}

impl Window {
    pub fn create(
        class_name: &str,
        window_title: &str,
        win_proc: unsafe extern "system" fn(
            window_handle: HWND,
            message: UINT,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT,
    ) -> Self {
        Window::register_window_class(class_name, win_proc);
        let window_handle = Window::create_window(window_title, class_name);
        unsafe {
            ShowWindow(window_handle, SW_NORMAL);
            UpdateWindow(window_handle);

            SetLayeredWindowAttributes(window_handle, 0, 0, LWA_ALPHA);
            let mut style = GetWindowLongW(window_handle, GWL_STYLE);
            style = style & !(WS_CAPTION as i32);
            SetWindowLongW(window_handle, GWL_STYLE, style);
        }

        Window {
            handle: window_handle.into(),
        }
    }

    pub fn create_window(window_title: &str, class_name: &str) -> HWND {
        unsafe {
            CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TRANSPARENT,
                to_unicode(class_name).as_ptr(),
                to_unicode(window_title).as_ptr(),
                0,
                300,
                300,
                100,
                100,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        }
    }

    pub fn register_window_class(
        class_name: &str,
        win_proc: unsafe extern "system" fn(
            window_handle: HWND,
            message: UINT,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT,
    ) {
        let win_class_name = to_unicode(class_name).as_ptr();
        let window_class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(win_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: null_mut(),
            hIcon: unsafe { LoadIconW(null_mut(), IDI_APPLICATION) },
            hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
            hbrBackground: unsafe { GetStockObject(WHITE_BRUSH as i32) as HBRUSH },
            lpszMenuName: MAKEINTRESOURCEW(0),
            lpszClassName: win_class_name,
            hIconSm: unsafe { LoadIconW(null_mut(), IDI_APPLICATION) },
        };

        unsafe { RegisterClassExW(&window_class) };
    }
}

pub fn to_unicode(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect::<Vec<_>>()
}
