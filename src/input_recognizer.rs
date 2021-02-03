use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use joycon_rs::joycon::input_report_mode::{AnalogStickData, StandardInputReport};
use joycon_rs::joycon::Buttons;
use std::f64::consts::PI;

pub fn is_button_down(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    is_left: bool,
    button: Buttons,
) -> bool {
    !is_button_press(last_state, is_left, button) && is_button_press(state, is_left, button)
}

pub fn is_button_up(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    is_left: bool,
    button: Buttons,
) -> bool {
    is_button_press(last_state, is_left, button) && !is_button_press(state, is_left, button)
}

pub fn is_button_press(
    state: &StandardInputReport<IMUData>,
    is_left: bool,
    button: Buttons,
) -> bool {
    if is_left {
        state.common.pushed_buttons.left.contains(&button)
            || state.common.pushed_buttons.shared.contains(&button)
    } else {
        state.common.pushed_buttons.right.contains(&button)
            || state.common.pushed_buttons.shared.contains(&button)
    }
}

pub fn recognize_stick_slot(
    slot_count: usize,
    deg_offset: i32,
    last_result: Option<usize>,
    stick: &AnalogStickData,
) -> Option<usize> {
    const OFFSET: f64 = -4096.0 / 2.0;
    const DEAD_ZONE_DOWN: f64 = 1000.0;
    const DEAD_ZONE_UP: f64 = 900.0;

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
