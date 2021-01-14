use crate::battery_light::get_light_states;
use crate::configuration::Config;
use crate::input_recognizer::{is_button_down, is_button_up, recognize_stick_slot};
use crate::key_sender::send_ev;
use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use joycon_rs::joycon::input_report_mode::StandardInputReport;
use joycon_rs::joycon::joycon_features::JoyConFeature;
use joycon_rs::prelude::lights::*;
use joycon_rs::prelude::*;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub fn run_joyshop(config: Arc<RwLock<Box<Config>>>) {
    let manager = JoyConManager::get_instance();
    let new_device_receiver = match manager.lock() {
        Ok(manager) => manager.new_devices(),
        Err(_) => return,
    };

    new_device_receiver
        .iter()
        .flat_map(|device| SimpleJoyConDriver::new(&device))
        .for_each(|mut driver| {
            println!("Joycon Connected");
            driver.enable_feature(JoyConFeature::Vibration).unwrap();
            let joycon = StandardFullMode::new(driver).unwrap();
            let config = config.clone();
            std::thread::spawn(move || handle_joycon_input(joycon, config));
        });
}

fn handle_joycon_input(
    mut joycon: StandardFullMode<SimpleJoyConDriver>,
    config: Arc<RwLock<Box<Config>>>,
) {
    let mut last_state: StandardInputReport<IMUData> = joycon.read_input_report().unwrap();
    let mut last_stick =
        recognize_stick_slot(6, 0, None, &last_state.common.left_analog_stick_data);
    let (light, flash) = get_light_states(last_state.common.battery.level);
    joycon.driver_mut().set_player_lights(light, flash).unwrap();

    loop {
        let config = match config.read() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let state = match joycon.read_input_report() {
            Ok(s) => s,
            Err(e) => {
                println!("Joycon error occurred: {:?}", e);
                match e {
                    JoyConError::Disconnected => break,
                    _ => continue,
                }
            }
        };

        if state.common.battery.level != last_state.common.battery.level {
            let (light, flash) = get_light_states(state.common.battery.level);
            joycon.driver_mut().set_player_lights(light, flash).unwrap();
        }

        if is_button_down(&last_state, &state, Buttons::ZL) {
            send_ev(&config.zl, true);
        }

        if is_button_up(&last_state, &state, Buttons::ZL) {
            send_ev(&config.zl, false);
        }

        if is_button_down(&last_state, &state, Buttons::L) {
            send_ev(&config.l, true);
        }

        if is_button_up(&last_state, &state, Buttons::L) {
            send_ev(&config.l, false);
        }

        if is_button_down(&last_state, &state, Buttons::Minus) {
            send_ev(&config.minus, true);
        }

        if is_button_up(&last_state, &state, Buttons::Minus) {
            send_ev(&config.minus, false);
        }

        if is_button_down(&last_state, &state, Buttons::LStick) {
            send_ev(&config.stick, true);
        }

        if is_button_up(&last_state, &state, Buttons::LStick) {
            send_ev(&config.stick, false);
        }

        if is_button_down(&last_state, &state, Buttons::Up) {
            send_ev(&config.up, true);
        }

        if is_button_up(&last_state, &state, Buttons::Up) {
            send_ev(&config.up, false);
        }

        if is_button_down(&last_state, &state, Buttons::Down) {
            send_ev(&config.down, true);
        }

        if is_button_up(&last_state, &state, Buttons::Down) {
            send_ev(&config.down, false);
        }

        if is_button_down(&last_state, &state, Buttons::Left) {
            send_ev(&config.left, true);
        }

        if is_button_up(&last_state, &state, Buttons::Left) {
            send_ev(&config.left, false);
        }

        if is_button_down(&last_state, &state, Buttons::Right) {
            send_ev(&config.right, true);
        }

        if is_button_up(&last_state, &state, Buttons::Right) {
            send_ev(&config.right, false);
        }

        if is_button_down(&last_state, &state, Buttons::Capture) {
            send_ev(&config.capture, true);
        }

        if is_button_up(&last_state, &state, Buttons::Capture) {
            send_ev(&config.capture, false);
        }

        if is_button_down(&last_state, &state, Buttons::SL) {
            send_ev(&config.sl, true);
        }

        if is_button_up(&last_state, &state, Buttons::SL) {
            send_ev(&config.sl, false);
        }

        if is_button_down(&last_state, &state, Buttons::SR) {
            send_ev(&config.sr, true);
        }

        if is_button_up(&last_state, &state, Buttons::SR) {
            send_ev(&config.sr, false);
        }

        let stick = recognize_stick_slot(6, 0, last_stick, &state.common.left_analog_stick_data);
        if stick != last_stick {
            if let Some(_) = stick {
                joycon
                    .driver_mut()
                    .rumble((Some(Rumble::new(100.0, 1.0)), None))
                    .unwrap();

                let now = Instant::now();
                while now.elapsed().as_millis() < 30 {}
                joycon
                    .driver_mut()
                    .rumble((Some(Rumble::stop()), None))
                    .unwrap();
            }

            match last_stick {
                Some(0) => {
                    send_ev(&config.stick_top_right, false);
                }
                Some(1) => {
                    send_ev(&config.stick_top_center, false);
                }
                Some(2) => {
                    send_ev(&config.stick_top_left, false);
                }
                Some(3) => {
                    send_ev(&config.stick_bottom_left, false);
                }
                Some(4) => {
                    send_ev(&config.stick_bottom_center, false);
                }
                Some(5) => {
                    send_ev(&config.stick_bottom_right, false);
                }
                _ => {}
            }

            match stick {
                Some(0) => {
                    send_ev(&config.stick_top_right, true);
                }
                Some(1) => {
                    send_ev(&config.stick_top_center, true);
                }
                Some(2) => {
                    send_ev(&config.stick_top_left, true);
                }
                Some(3) => {
                    send_ev(&config.stick_bottom_left, true);
                }
                Some(4) => {
                    send_ev(&config.stick_bottom_center, true);
                }
                Some(5) => {
                    send_ev(&config.stick_bottom_right, true);
                }
                _ => {}
            }
        }
        last_state = state;
        last_stick = stick;
    }
}
