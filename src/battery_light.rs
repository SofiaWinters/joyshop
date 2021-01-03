use joycon_rs::joycon::input_report_mode::BatteryLevel;
use joycon_rs::joycon::lights::{Flash, LightUp};

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

pub fn get_light_states(battery_level: BatteryLevel) -> (&'static [LightUp], &'static [Flash]) {
    match battery_level {
        BatteryLevel::Empty => (&EMPTY_LIGHT, &EMPTY_FLASH),
        BatteryLevel::Critical => (&CRITICAL_LIGHT, &CRITICAL_FLASH),
        BatteryLevel::Low => (&LOW_LIGHT, &LOW_FLASH),
        BatteryLevel::Medium => (&MEDIUM_LIGHT, &MEDIUM_FLASH),
        BatteryLevel::Full => (&FULL_LIGHT, &FULL_FLASH),
    }
}
