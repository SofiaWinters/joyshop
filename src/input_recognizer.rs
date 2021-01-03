use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use joycon_rs::joycon::input_report_mode::{AnalogStickData, StandardInputReport};
use joycon_rs::joycon::Buttons;
use std::f64::consts::PI;

pub fn is_button_down(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    button: Buttons,
) -> bool {
    !is_button_press(last_state, button) && is_button_press(state, button)
}

pub fn is_button_up(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    button: Buttons,
) -> bool {
    is_button_press(last_state, button) && !is_button_press(state, button)
}

pub fn is_button_press(state: &StandardInputReport<IMUData>, button: Buttons) -> bool {
    state.common.pushed_buttons.contains(button)
}

pub fn recognize_stick_slot(
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