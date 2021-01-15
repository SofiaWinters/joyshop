#![windows_subsystem = "windows"]

use crate::configuration::load_config_or_default;
use crate::joyshop::run_joyshop;

mod battery_light;
mod configuration;
mod input_recognizer;
mod joyshop;
mod key_sender;

use crossbeam_channel::{unbounded, Receiver};
use once_cell::sync::Lazy;
use std::mem::{size_of, zeroed};
use std::ptr::null_mut;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use win_key_codes::VK_LBUTTON;
use winapi::_core::intrinsics::transmute;
use winapi::_core::time::Duration;
use winapi::shared::minwindef::{FALSE, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HWND, POINT, RECT};
use winapi::um::wingdi::{GetStockObject, SelectObject, DEFAULT_GUI_FONT, WHITE_BRUSH};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, DrawTextW, EndPaint,
    GetCursorPos, GetKeyState, GetMessageW, GetWindowLongW, InvalidateRect, LoadCursorW, LoadIconW,
    PostQuitMessage, RegisterClassExW, SetLayeredWindowAttributes, SetWindowLongW, SetWindowPos,
    ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, DT_CALCRECT, GWL_STYLE,
    HWND_TOP, HWND_TOPMOST, IDC_ARROW, IDI_APPLICATION, LWA_ALPHA, MAKEINTRESOURCEW, MSG,
    PAINTSTRUCT, SWP_NOMOVE, SWP_NOSIZE, SW_NORMAL, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_CAPTION,
    WS_EX_LAYERED, WS_EX_TRANSPARENT,
};

fn main() {
    let config = load_config_or_default();
    let logic_config = config.clone();
    let (tx, rx) = unbounded::<String>();

    spawn(move || run_joyshop(logic_config, tx));
    process_ui(rx);
}

struct WindowState {
    tool: String,
}

static WINDOW_STATE: Lazy<Mutex<WindowState>> = Lazy::new(|| {
    let state = WindowState {
        tool: String::new(),
    };
    Mutex::new(state)
});

fn process_ui(rx: Receiver<String>) {
    unsafe {
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

        let win_class_name = to_unicode("JoyShopWindowClass").as_ptr();
        let window_class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(win_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: null_mut(),
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: GetStockObject(WHITE_BRUSH as i32) as HBRUSH,
            lpszMenuName: MAKEINTRESOURCEW(0),
            lpszClassName: win_class_name,
            hIconSm: LoadIconW(null_mut(), IDI_APPLICATION),
        };

        RegisterClassExW(&window_class);

        let window_handle = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT,
            win_class_name,
            to_unicode("joyshop").as_ptr(),
            0,
            300,
            300,
            100,
            100,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        ShowWindow(window_handle, SW_NORMAL);
        UpdateWindow(window_handle);

        SetLayeredWindowAttributes(window_handle, 0, 0, LWA_ALPHA);
        let mut style = GetWindowLongW(window_handle, GWL_STYLE);
        style = style & !(WS_CAPTION as i32);
        SetWindowLongW(window_handle, GWL_STYLE, style);

        let rawhwnd: usize = std::mem::transmute(window_handle);
        spawn(move || {
            let window_handle: HWND = transmute(rawhwnd);
            let mut last_point: POINT = zeroed();
            loop {
                let mut point: POINT = zeroed();
                GetCursorPos(&mut point);

                match rx.recv_timeout(Duration::from_millis(0)) {
                    Ok(s) => {
                        WINDOW_STATE.lock().unwrap().tool = s;
                        SetLayeredWindowAttributes(window_handle, 0, 120, LWA_ALPHA);
                        InvalidateRect(window_handle, null_mut(), TRUE);
                        last_point = point;
                    }
                    Err(_) => {}
                }

                if GetKeyState(VK_LBUTTON) < 0 {
                    SetLayeredWindowAttributes(window_handle, 0, 0, LWA_ALPHA);
                }

                let sqr_dist = (point.x - last_point.x) * (point.x - last_point.x)
                    + (point.y - last_point.y) * (point.y - last_point.y);
                if sqr_dist >= 10 * 10 {
                    SetLayeredWindowAttributes(window_handle, 0, 0, LWA_ALPHA);
                }

                SetWindowPos(
                    window_handle,
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

fn to_unicode(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect::<Vec<_>>()
}
