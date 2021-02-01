#![windows_subsystem = "windows"]

use crate::configuration::load_config_or_default;
use crate::joyshop::run_joyshop;
use crossbeam_channel::unbounded;
use std::thread::spawn;

mod battery_light;
mod configuration;
mod input_recognizer;
mod joyshop;
mod key_sender;
mod ui;
mod window;

fn main() {
    let config = load_config_or_default();
    let logic_config = config.clone();
    let (tx, rx) = unbounded::<String>();

    spawn(move || run_joyshop(logic_config, tx));
    ui::process_ui(rx, config.read().unwrap().show_tooltip);
}
