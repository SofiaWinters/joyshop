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
use std::sync::{Arc, RwLock, RwLockReadGuard};
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
    let mut last_left_stick =
        recognize_stick_slot(6, 0, None, &last_state.common.left_analog_stick_data);
    let mut last_right_stick =
        recognize_stick_slot(6, 0, None, &last_state.common.right_analog_stick_data);

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

        handle_button_action(&last_state, &state, &tx, true, Buttons::ZL, &config.zl);
        handle_button_action(&last_state, &state, &tx, true, Buttons::L, &config.l);
        handle_button_action(
            &last_state,
            &state,
            &tx,
            true,
            Buttons::Minus,
            &config.minus,
        );
        handle_button_action(
            &last_state,
            &state,
            &tx,
            true,
            Buttons::LStick,
            &config.left_stick,
        );
        handle_button_action(&last_state, &state, &tx, true, Buttons::Up, &config.up);
        handle_button_action(&last_state, &state, &tx, true, Buttons::Down, &config.down);
        handle_button_action(&last_state, &state, &tx, true, Buttons::Left, &config.left);
        handle_button_action(
            &last_state,
            &state,
            &tx,
            true,
            Buttons::Right,
            &config.right,
        );
        handle_button_action(&last_state, &state, &tx, true, Buttons::SL, &config.left_sl);
        handle_button_action(&last_state, &state, &tx, true, Buttons::SR, &config.left_sr);

        handle_button_action(&last_state, &state, &tx, false, Buttons::ZR, &config.zr);
        handle_button_action(&last_state, &state, &tx, false, Buttons::R, &config.r);
        handle_button_action(&last_state, &state, &tx, false, Buttons::Plus, &config.plus);
        handle_button_action(
            &last_state,
            &state,
            &tx,
            false,
            Buttons::RStick,
            &config.right_stick,
        );
        handle_button_action(&last_state, &state, &tx, false, Buttons::A, &config.a);
        handle_button_action(&last_state, &state, &tx, false, Buttons::B, &config.b);
        handle_button_action(&last_state, &state, &tx, false, Buttons::X, &config.x);
        handle_button_action(&last_state, &state, &tx, false, Buttons::Y, &config.y);
        handle_button_action(&last_state, &state, &tx, false, Buttons::Home, &config.home);
        handle_button_action(
            &last_state,
            &state,
            &tx,
            false,
            Buttons::SL,
            &config.right_sl,
        );
        handle_button_action(
            &last_state,
            &state,
            &tx,
            false,
            Buttons::SR,
            &config.right_sr,
        );

        let left_stick =
            recognize_stick_slot(6, 0, last_left_stick, &state.common.left_analog_stick_data);
        handle_stick_action(
            joycon.driver_mut(),
            last_left_stick,
            left_stick,
            &tx,
            &config,
            true,
        );

        let right_stick = recognize_stick_slot(
            6,
            0,
            last_right_stick,
            &state.common.right_analog_stick_data,
        );
        handle_stick_action(
            joycon.driver_mut(),
            last_right_stick,
            right_stick,
            &tx,
            &config,
            false,
        );

        last_state = state;
        last_left_stick = left_stick;
        last_right_stick = right_stick;
    }
}

fn handle_button_action(
    last_state: &StandardInputReport<IMUData>,
    state: &StandardInputReport<IMUData>,
    tx: &Sender<String>,
    is_left: bool,
    button: Buttons,
    action: &KeyAction,
) {
    if is_button_down(last_state, state, is_left, button) {
        send_ev(action, true, tx);
    }

    if is_button_up(last_state, state, is_left, button) {
        send_ev(action, false, tx);
    }
}

fn handle_stick_action(
    driver: &mut SimpleJoyConDriver,
    last_stick: Option<usize>,
    stick: Option<usize>,
    tx: &Sender<String>,
    config: &RwLockReadGuard<Box<Config>>,
    is_left: bool,
) {
    if stick != last_stick {
        if let Some(_) = stick {
            driver
                .rumble((Some(Rumble::new(100.0, 1.0)), Some(Rumble::new(100.0, 1.0))))
                .unwrap();

            let now = Instant::now();
            while now.elapsed().as_millis() < 30 {}
            driver
                .rumble((Some(Rumble::stop()), Some(Rumble::stop())))
                .unwrap();
        }

        match last_stick {
            Some(0) => {
                if is_left {
                    send_ev(&config.left_stick_top_right, false, &tx);
                } else {
                    send_ev(&config.right_stick_top_right, false, &tx);
                }
            }
            Some(1) => {
                if is_left {
                    send_ev(&config.left_stick_top_center, false, &tx);
                } else {
                    send_ev(&config.right_stick_top_center, false, &tx);
                }
            }
            Some(2) => {
                if is_left {
                    send_ev(&config.left_stick_top_left, false, &tx);
                } else {
                    send_ev(&config.right_stick_top_left, false, &tx);
                }
            }
            Some(3) => {
                if is_left {
                    send_ev(&config.left_stick_bottom_left, false, &tx);
                } else {
                    send_ev(&config.right_stick_bottom_left, false, &tx);
                }
            }
            Some(4) => {
                if is_left {
                    send_ev(&config.left_stick_bottom_center, false, &tx);
                } else {
                    send_ev(&config.right_stick_bottom_center, false, &tx);
                }
            }
            Some(5) => {
                if is_left {
                    send_ev(&config.left_stick_bottom_right, false, &tx);
                } else {
                    send_ev(&config.right_stick_bottom_right, false, &tx);
                }
            }
            _ => {}
        }

        match stick {
            Some(0) => {
                if is_left {
                    send_ev(&config.left_stick_top_right, true, &tx);
                } else {
                    send_ev(&config.right_stick_top_right, true, &tx);
                }
            }
            Some(1) => {
                if is_left {
                    send_ev(&config.left_stick_top_center, true, &tx);
                } else {
                    send_ev(&config.right_stick_top_center, true, &tx);
                }
            }
            Some(2) => {
                if is_left {
                    send_ev(&config.left_stick_top_left, true, &tx);
                } else {
                    send_ev(&config.right_stick_top_left, true, &tx);
                }
            }
            Some(3) => {
                if is_left {
                    send_ev(&config.left_stick_bottom_left, true, &tx);
                } else {
                    send_ev(&config.right_stick_bottom_left, true, &tx);
                }
            }
            Some(4) => {
                if is_left {
                    send_ev(&config.left_stick_bottom_center, true, &tx);
                } else {
                    send_ev(&config.right_stick_bottom_center, true, &tx);
                }
            }
            Some(5) => {
                if is_left {
                    send_ev(&config.left_stick_bottom_right, true, &tx);
                } else {
                    send_ev(&config.right_stick_bottom_right, true, &tx);
                }
            }
            _ => {}
        }
    }
}
