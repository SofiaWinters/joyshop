use crate::battery_light::get_light_states;
use crate::configuration::{Config, KeyAction};
use crate::input_recognizer::{is_button_down, is_button_up, recognize_stick_slot};
use crate::key_sender::send_ev;
use ::crossbeam_channel::Sender;
use joycon_rs::joycon::input_report_mode::standard_full_mode::IMUData;
use joycon_rs::joycon::input_report_mode::StandardInputReport;
use joycon_rs::joycon::joycon_features::JoyConFeature;
use joycon_rs::prelude::lights::*;
use joycon_rs::prelude::*;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub fn run_joyshop(config: Arc<RwLock<Box<Config>>>, tx: Sender<String>) {
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
            let tx = tx.clone();
            std::thread::spawn(move || handle_joycon_input(joycon, config, tx));
        });
}

fn handle_joycon_input(
    mut joycon: StandardFullMode<SimpleJoyConDriver>,
    config: Arc<RwLock<Box<Config>>>,
    tx: Sender<String>,
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

        handle_button_action(&last_state, &state, &tx, Buttons::ZL, &config.zl);
        handle_button_action(&last_state, &state, &tx, Buttons::L, &config.l);
        handle_button_action(&last_state, &state, &tx, Buttons::Minus, &config.minus);
        handle_button_action(&last_state, &state, &tx, Buttons::LStick, &config.stick);
        handle_button_action(&last_state, &state, &tx, Buttons::Up, &config.up);
        handle_button_action(&last_state, &state, &tx, Buttons::Down, &config.down);
        handle_button_action(&last_state, &state, &tx, Buttons::Left, &config.left);
        handle_button_action(&last_state, &state, &tx, Buttons::Right, &config.right);
        handle_button_action(&last_state, &state, &tx, Buttons::SL, &config.sl);
        handle_button_action(&last_state, &state, &tx, Buttons::SR, &config.sr);

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
                    send_ev(&config.stick_top_right, false, &tx);
                }
                Some(1) => {
                    send_ev(&config.stick_top_center, false, &tx);
                }
                Some(2) => {
                    send_ev(&config.stick_top_left, false, &tx);
                }
                Some(3) => {
                    send_ev(&config.stick_bottom_left, false, &tx);
                }
                Some(4) => {
                    send_ev(&config.stick_bottom_center, false, &tx);
                }
                Some(5) => {
                    send_ev(&config.stick_bottom_right, false, &tx);
                }
                _ => {}
            }

            match stick {
                Some(0) => {
                    send_ev(&config.stick_top_right, true, &tx);
                }
                Some(1) => {
                    send_ev(&config.stick_top_center, true, &tx);
                }
                Some(2) => {
                    send_ev(&config.stick_top_left, true, &tx);
                }
                Some(3) => {
                    send_ev(&config.stick_bottom_left, true, &tx);
                }
                Some(4) => {
                    send_ev(&config.stick_bottom_center, true, &tx);
                }
                Some(5) => {
                    send_ev(&config.stick_bottom_right, true, &tx);
                }
                _ => {}
            }
        }
        last_state = state;
        last_stick = stick;
    }
}

fn handle_button_action(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    tx: &Sender<String>,
    button: Buttons,
    action: &KeyAction,
) {
    if is_button_down(last_state, state, button) {
        send_ev(action, true, tx);
    }

    if is_button_up(last_state, state, button) {
        send_ev(action, false, tx);
    }
}
