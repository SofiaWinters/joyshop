use crate::window::{to_unicode, Window};
use crossbeam_channel::Receiver;
use std::mem::zeroed;
use std::ptr::null_mut;
use std::thread::{sleep, spawn};
use win_key_codes::VK_LBUTTON;
use winapi::_core::time::Duration;
use winapi::shared::minwindef::{FALSE, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::um::wingdi::{GetStockObject, SelectObject, DEFAULT_GUI_FONT};
use winapi::um::winuser::{
    BeginPaint, DefWindowProcW, DispatchMessageW, DrawTextW, EndPaint, GetCursorPos, GetKeyState,
    GetMessageW, InvalidateRect, PostQuitMessage, SetLayeredWindowAttributes, SetWindowPos,
    TranslateMessage, DT_CALCRECT, HWND_TOP, HWND_TOPMOST, LWA_ALPHA, MSG, PAINTSTRUCT, SWP_NOMOVE,
    SWP_NOSIZE, WM_DESTROY, WM_PAINT,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct WindowState {
    pub tool: String,
}

pub static WINDOW_STATE: Lazy<Mutex<WindowState>> = Lazy::new(|| {
    let state = WindowState {
        tool: String::new(),
    };
    Mutex::new(state)
});

pub fn process_ui(rx: Receiver<String>) {
    unsafe {
        let win_class_name = "JoyShopWindowClass";
        let window = Window::create(win_class_name, "Joyshop", win_proc);
        let window_handle = window.handle;
        spawn(move || {
            let mut last_point: POINT = zeroed();
            loop {
                let mut point: POINT = zeroed();
                GetCursorPos(&mut point);

                match rx.recv_timeout(Duration::from_millis(0)) {
                    Ok(s) => {
                        WINDOW_STATE.lock().unwrap().tool = s;
                        SetLayeredWindowAttributes(window_handle.into(), 0, 120, LWA_ALPHA);
                        InvalidateRect(window_handle.into(), null_mut(), TRUE);
                        last_point = point;
                    }
                    Err(_) => {}
                }

                if GetKeyState(VK_LBUTTON) < 0 {
                    SetLayeredWindowAttributes(window_handle.into(), 0, 0, LWA_ALPHA);
                }

                let sqr_dist = (point.x - last_point.x) * (point.x - last_point.x)
                    + (point.y - last_point.y) * (point.y - last_point.y);
                if sqr_dist >= 10 * 10 {
                    SetLayeredWindowAttributes(window_handle.into(), 0, 0, LWA_ALPHA);
                }

                SetWindowPos(
                    window_handle.into(),
                    HWND_TOPMOST,
                    point.x,
                    point.y + 30,
                    0,
                    0,
                    SWP_NOSIZE,
                );
                sleep(Duration::from_millis(1));
            }
        });

        let mut msg: MSG = zeroed();
        loop {
            if GetMessageW(&mut msg, null_mut(), 0, 0) == FALSE {
                return;
            }

            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
}

unsafe extern "system" fn win_proc(
    window_handle: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_PAINT => {
            let tool = &WINDOW_STATE.lock().unwrap().tool;
            let mut ps: PAINTSTRUCT = zeroed();
            let hdc = BeginPaint(window_handle, &mut ps);
            let mut rect: RECT = zeroed();

            let font = GetStockObject(DEFAULT_GUI_FONT as i32);
            SelectObject(hdc, font);

            DrawTextW(
                hdc,
                to_unicode(&tool).as_ptr(),
                tool.len() as i32,
                &mut rect,
                DT_CALCRECT,
            );

            SetWindowPos(
                window_handle,
                HWND_TOP,
                0,
                0,
                rect.right - rect.left,
                rect.bottom - rect.top,
                SWP_NOMOVE,
            );

            DrawTextW(
                hdc,
                to_unicode(&tool).as_ptr(),
                tool.len() as i32,
                &mut rect,
                0,
            );

            EndPaint(window_handle, &ps);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            wparam as isize
        }
        _ => DefWindowProcW(window_handle, message, wparam, lparam),
    }
}
