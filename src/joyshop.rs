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
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use std::thread::sleep;
use std::time::Instant;

pub fn run_joyshop(config: Arc<RwLock<Box<Config>>>, tx: Sender<String>) {
    let manager = JoyConManager::get_instance();
    let new_device_receiver = match manager.lock() {
        Ok(manager) => manager.new_devices(),
        Err(_) => return,
    };

    new_device_receiver.iter().for_each(|device| {
        println!("Joycon Connected");

        let mut driver = create_driver(&device);
        rumble_for_connect(&mut driver);
        let joycon = StandardFullMode::new(driver).unwrap();
        let config = config.clone();
        let tx = tx.clone();
        std::thread::spawn(move || handle_joycon_input(joycon, config, tx));
    });
}

fn rumble_for_connect(driver: &mut SimpleJoyConDriver) {
    driver
        .rumble((Some(Rumble::new(500.0, 1.0)), Some(Rumble::new(500.0, 1.0))))
        .unwrap();
    let now = Instant::now();
    while now.elapsed().as_millis() < 500 {}
    driver
        .rumble((Some(Rumble::stop()), Some(Rumble::stop())))
        .unwrap();
}

fn create_driver(device: &Arc<Mutex<JoyConDevice>>) -> SimpleJoyConDriver {
    loop {
        let mut driver = match SimpleJoyConDriver::new(device) {
            Ok(d) => d,
            Err(e) => {
                println!("JoyCon init error (will retry):{:?}", e);
                sleep(std::time::Duration::from_millis(100));
                continue;
            }
        };

        match driver.enable_feature(JoyConFeature::Vibration) {
            Ok(_) => return driver,
            Err(e) => {
                println!("JoyCon enable vibration error (will retry):{:?}", e);
                sleep(std::time::Duration::from_millis(500));
                continue;
            }
        }
    }
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

    let mut last_light_updated = Instant::now();

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

        if last_light_updated.elapsed().as_secs() > 10 {
            let (light, flash) = get_light_states(state.common.battery.level);
            joycon.driver_mut().set_player_lights(light, flash).unwrap();
            last_light_updated = Instant::now();
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
        handle_button_action(
            &last_state,
            &state,
            &tx,
            true,
            Buttons::Capture,
            &config.capture,
        );

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
        handle_button_action(&last_state, &state, &tx, false, Buttons::Home, &config.home);

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
    if stick == last_stick {
        return;
    }

    if let Some(_) = stick {
        rumble_for_stick_action(driver);
    }

    let top_right = if is_left {
        &config.left_stick_top_right
    } else {
        &config.right_stick_top_right
    };

    let top = if is_left {
        &config.left_stick_top_center
    } else {
        &config.right_stick_top_center
    };

    let top_left = if is_left {
        &config.left_stick_top_left
    } else {
        &config.right_stick_top_left
    };

    let bottom_left = if is_left {
        &config.left_stick_bottom_left
    } else {
        &config.right_stick_bottom_left
    };

    let bottom = if is_left {
        &config.left_stick_bottom_center
    } else {
        &config.right_stick_bottom_center
    };

    let bottom_right = if is_left {
        &config.left_stick_bottom_right
    } else {
        &config.right_stick_bottom_right
    };

    match last_stick {
        Some(0) => send_ev(top_right, false, &tx),
        Some(1) => send_ev(top, false, &tx),
        Some(2) => send_ev(top_left, false, &tx),
        Some(3) => send_ev(bottom_left, false, &tx),
        Some(4) => send_ev(bottom, false, &tx),
        Some(5) => send_ev(bottom_right, false, &tx),
        _ => {}
    }

    match stick {
        Some(0) => send_ev(top_right, true, &tx),
        Some(1) => send_ev(top, true, &tx),
        Some(2) => send_ev(top_left, true, &tx),
        Some(3) => send_ev(bottom_left, true, &tx),
        Some(4) => send_ev(bottom, true, &tx),
        Some(5) => send_ev(bottom_right, true, &tx),
        _ => {}
    }
}

fn rumble_for_stick_action(driver: &mut SimpleJoyConDriver) {
    driver
        .rumble((Some(Rumble::new(100.0, 1.0)), Some(Rumble::new(100.0, 1.0))))
        .unwrap();

    let now = Instant::now();
    while now.elapsed().as_millis() < 30 {}
    driver
        .rumble((Some(Rumble::stop()), Some(Rumble::stop())))
        .unwrap();
}
