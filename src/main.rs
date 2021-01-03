use crate::configuration::load_config_or_default;
use crate::joyshop::run_joyshop;

mod battery_light;
mod configuration;
mod input_recognizer;
mod joyshop;
mod key_sender;

fn main() {
    let config = load_config_or_default();
    run_joyshop(config);
}
