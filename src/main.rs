use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use joycon_rs::joycon::input_report_mode::{AnalogStickData, BatteryLevel, StandardInputReport};
use joycon_rs::prelude::lights::*;
use joycon_rs::prelude::*;
use std::convert::TryInto;
use std::f64::consts::PI;
use std::mem::{size_of, zeroed};
use win_key_codes::{
    VK_B, VK_CONTROL, VK_E, VK_LMENU, VK_N, VK_O, VK_OEM_4, VK_OEM_6, VK_R, VK_S, VK_SHIFT, VK_Z,
};
use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, VK_SPACE};

fn main() {
    let manager = JoyConManager::get_instance();
    let devices = match manager.lock() {
        Ok(manager) => manager.new_devices(),
        Err(_) => return,
    };

    devices
        .iter()
        .flat_map(|device| SimpleJoyConDriver::new(&device))
        .for_each(|driver| {
            let joycon = StandardFullMode::new(driver).unwrap();
            std::thread::spawn(move || process_loop(joycon));
        });
}

fn judge_stick(
    slot_count: usize,
    deg_offset: i32,
    last_result: Option<usize>,
    stick: &AnalogStickData,
) -> Option<usize> {
    const OFFSET: f64 = -4096.0 / 2.0;
    const DEAD_ZONE_DOWN: f64 = 800.0;
    const DEAD_ZONE_UP: f64 = 500.0;

    let x = stick.horizontal as f64 + OFFSET;
    let y = stick.vertical as f64 + OFFSET;

    let mut deg = (y).atan2(x) / PI * 180.0 + deg_offset as f64;
    while deg < 0.0 {
        deg += 360.0;
    }

    while deg >= 360.0 {
        deg -= 360.0;
    }

    let sqr_dist = x * x + y * y;

    if let Some(_) = last_result {
        if sqr_dist <= DEAD_ZONE_UP * DEAD_ZONE_UP {
            return None;
        }
    } else {
        if sqr_dist <= DEAD_ZONE_DOWN * DEAD_ZONE_DOWN {
            return None;
        }
    }

    let socket_degrees = 360 / slot_count;
    for i in 0..slot_count {
        let from = socket_degrees * i;
        let to = socket_degrees * (i + 1);
        if from as f64 <= deg && deg < to as f64 {
            return Some(i);
        }
    }

    return None;
}

fn process_loop(mut joycon: StandardFullMode<SimpleJoyConDriver>) {
    let mut last_state: StandardInputReport<IMUData> = joycon.read_input_report().unwrap();
    let mut last_stick = judge_stick(6, 0, None, &last_state.common.left_analog_stick_data);
    loop {
        let state = joycon.read_input_report().unwrap();
        if state.common.battery.level != last_state.common.battery.level {
            let (light, flash) = get_light_states_from_battery_level(state.common.battery.level);
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

        let stick = judge_stick(6, 0, last_stick, &state.common.left_analog_stick_data);
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

fn is_button_down(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    button: Buttons,
) -> bool {
    !is_button_press(last_state, button) && is_button_press(state, button)
}

fn is_button_up(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    button: Buttons,
) -> bool {
    is_button_press(last_state, button) && !is_button_press(state, button)
}

fn is_button_press(state: &StandardInputReport<IMUData>, button: Buttons) -> bool {
    state.common.pushed_buttons.contains(button)
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

const EMPTY_LIGHT: [LightUp; 0] = [];
const EMPTY_FLASH: [Flash; 4] = [Flash::LED0, Flash::LED1, Flash::LED2, Flash::LED3];
const CRITICAL_LIGHT: [LightUp; 0] = [];
const CRITICAL_FLASH: [Flash; 1] = [Flash::LED3];
const LOW_LIGHT: [LightUp; 2] = [LightUp::LED2, LightUp::LED3];
const LOW_FLASH: [Flash; 0] = [];
const MEDIUM_LIGHT: [LightUp; 3] = [LightUp::LED1, LightUp::LED2, LightUp::LED3];
const MEDIUM_FLASH: [Flash; 0] = [];
const FULL_LIGHT: [LightUp; 4] = [LightUp::LED0, LightUp::LED1, LightUp::LED2, LightUp::LED3];
const FULL_FLASH: [Flash; 0] = [];

fn get_light_states_from_battery_level(
    battery_level: BatteryLevel,
) -> (&'static [LightUp], &'static [Flash]) {
    match battery_level {
        BatteryLevel::Empty => (&EMPTY_LIGHT, &EMPTY_FLASH),
        BatteryLevel::Critical => (&CRITICAL_LIGHT, &CRITICAL_FLASH),
        BatteryLevel::Low => (&LOW_LIGHT, &LOW_FLASH),
        BatteryLevel::Medium => (&MEDIUM_LIGHT, &MEDIUM_FLASH),
        BatteryLevel::Full => (&FULL_LIGHT, &FULL_FLASH),
    }
}
