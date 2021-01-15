use crate::configuration::{Key, KeyAction};
use crossbeam_channel::Sender;
use std::convert::TryInto;
use std::mem::{size_of, zeroed};
use win_key_codes::{VK_CONTROL, VK_MENU, VK_SHIFT};
use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP};

pub fn send_ev(key_action: &KeyAction, is_down: bool, tx: &Sender<String>) {
    if is_down {
        match key_action {
            KeyAction::None => {}
            KeyAction::KeyHold(k) => {
                if k.alt {
                    send_input(VK_MENU, true);
                }
                if k.shift {
                    send_input(VK_SHIFT, true);
                }
                if k.ctrl {
                    send_input(VK_CONTROL, true);
                }

                if k.key != Key::None {
                    send_input(k.key as i32, true);
                }

                println!("[Hold] {}", &k.name);
                tx.send(k.name.clone()).unwrap();
            }
            KeyAction::KeyClick(k) => {
                if k.alt {
                    send_input(VK_MENU, true);
                }
                if k.shift {
                    send_input(VK_SHIFT, true);
                }
                if k.ctrl {
                    send_input(VK_CONTROL, true);
                }

                if k.key != Key::None {
                    send_input(k.key as i32, true);
                    send_input(k.key as i32, false);
                }

                if k.alt {
                    send_input(VK_MENU, false);
                }
                if k.shift {
                    send_input(VK_SHIFT, false);
                }
                if k.ctrl {
                    send_input(VK_CONTROL, false);
                }

                println!("[Click] {}", &k.name);
                tx.send(k.name.clone()).unwrap();
            }
        }
    } else {
        match key_action {
            KeyAction::None => {}
            KeyAction::KeyHold(k) => {
                if k.key != Key::None {
                    send_input(k.key as i32, false);
                }

                if k.alt {
                    send_input(VK_MENU, false);
                }
                if k.shift {
                    send_input(VK_SHIFT, false);
                }
                if k.ctrl {
                    send_input(VK_CONTROL, false);
                }

                println!("[Release] {}", &k.name);
            }
            KeyAction::KeyClick(_) => {}
        }
    }
}

fn send_input(key: i32, down: bool) {
    let mut input = unsafe { zeroed::<INPUT>() };
    input.type_ = INPUT_KEYBOARD;
    let mut ki = unsafe { input.u.ki_mut() };
    ki.wVk = key as u16;

    if down {
        ki.dwFlags = 0;
    } else {
        ki.dwFlags = KEYEVENTF_KEYUP;
    }

    let mut inputs = vec![input];
    unsafe {
        SendInput(
            inputs.len().try_into().unwrap(),
            inputs.as_mut_ptr(),
            size_of::<INPUT>().try_into().unwrap(),
        )
    };
}
