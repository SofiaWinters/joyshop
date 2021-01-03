use crate::input_recognizer::{is_button_down, is_button_up, recognize_stick_slot};
use battery_light::get_light_states;
use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use joycon_rs::joycon::input_report_mode::StandardInputReport;
use joycon_rs::prelude::lights::*;
use joycon_rs::prelude::*;
use std::convert::TryInto;
use std::mem::{size_of, zeroed};
use win_key_codes::{
    VK_B, VK_CONTROL, VK_E, VK_LMENU, VK_N, VK_O, VK_OEM_4, VK_OEM_6, VK_R, VK_S, VK_SHIFT, VK_Z,
};
use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, VK_SPACE};

mod battery_light;
mod input_recognizer;

fn main() {
    let manager = JoyConManager::get_instance();
    let new_device_receiver = match manager.lock() {
        Ok(manager) => manager.new_devices(),
        Err(_) => return,
    };

    new_device_receiver
        .iter()
        .flat_map(|device| SimpleJoyConDriver::new(&device))
        .for_each(|driver| {
            let joycon = StandardFullMode::new(driver).unwrap();
            std::thread::spawn(move || process_loop(joycon));
        });
}

fn process_loop(mut joycon: StandardFullMode<SimpleJoyConDriver>) {
    let mut last_state: StandardInputReport<IMUData> = joycon.read_input_report().unwrap();
    let mut last_stick =
        recognize_stick_slot(6, 0, None, &last_state.common.left_analog_stick_data);
    let (light, flash) = get_light_states(last_state.common.battery.level);
    joycon.driver_mut().set_player_lights(light, flash).unwrap();

    loop {
        let state = joycon.read_input_report().unwrap();
        if state.common.battery.level != last_state.common.battery.level {
            let (light, flash) = get_light_states(state.common.battery.level);
            joycon.driver_mut().set_player_lights(light, flash).unwrap();
        }

        if is_button_down(&last_state, &state, Buttons::SL) {
            send_input(VK_CONTROL, true);
            send_input(VK_Z, true);
            send_input(VK_Z, false);
            send_input(VK_CONTROL, false);
            println!("Undo");
        }

        if is_button_down(&last_state, &state, Buttons::SR) {
            send_input(VK_CONTROL, true);
            send_input(VK_SHIFT, true);
            send_input(VK_Z, true);
            send_input(VK_Z, false);
            send_input(VK_SHIFT, false);
            send_input(VK_CONTROL, false);
            println!("Redo");
        }

        if is_button_down(&last_state, &state, Buttons::Up) {
            send_input(VK_LMENU, true);
            println!("Alt Press");
        }

        if is_button_up(&last_state, &state, Buttons::Up) {
            send_input(VK_LMENU, false);
            println!("Alt Release");
        }

        if is_button_down(&last_state, &state, Buttons::Right) {
            send_input(VK_OEM_6, true); // ]
            send_input(VK_OEM_6, false);
            println!("Larger");
        }

        if is_button_down(&last_state, &state, Buttons::Left) {
            send_input(VK_OEM_4, true); // [
            send_input(VK_OEM_4, false);
            println!("Smaller");
        }

        if is_button_down(&last_state, &state, Buttons::ZL) {
            send_input(VK_E, true);
            println!("Eraser Press");
        }

        if is_button_up(&last_state, &state, Buttons::ZL) {
            send_input(VK_E, false);
            println!("Eraser Release");
        }

        if is_button_down(&last_state, &state, Buttons::L) {
            send_input(VK_SHIFT, true);
            println!("Shift Press");
        }

        if is_button_up(&last_state, &state, Buttons::L) {
            send_input(VK_SHIFT, false);
            println!("Shift Release");
        }

        if is_button_down(&last_state, &state, Buttons::Minus) {
            send_input(VK_CONTROL, true);
            send_input(VK_S, true);
            send_input(VK_S, false);
            send_input(VK_CONTROL, false);
            println!("Save");
        }

        let stick = recognize_stick_slot(6, 0, last_stick, &state.common.left_analog_stick_data);
        if stick != last_stick {
            match last_stick {
                Some(3) => {
                    send_input(VK_R, false);
                    println!("Rotate Release");
                }
                Some(4) => {
                    send_input(VK_SPACE, false);
                    println!("Grab Release");
                }
                Some(5) => {
                    send_input(VK_Z, false);
                    println!("Zoom Release");
                }
                _ => {}
            }

            match stick {
                Some(0) => {
                    send_input(VK_O, true);
                    send_input(VK_O, false);
                    println!("Dodge");
                }
                Some(1) => {
                    send_input(VK_B, true);
                    send_input(VK_B, false);
                    println!("Brush");
                }
                Some(2) => {
                    send_input(VK_N, true);
                    send_input(VK_N, false);
                    println!("Finger");
                }
                Some(3) => {
                    send_input(VK_R, true);
                    println!("Rotate Press");
                }
                Some(4) => {
                    send_input(VK_SPACE, true);
                    println!("Grab Press");
                }
                Some(5) => {
                    send_input(VK_Z, true);
                    println!("Zoom Press");
                }
                _ => {}
            }
        }

        last_state = state;
        last_stick = stick;
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
