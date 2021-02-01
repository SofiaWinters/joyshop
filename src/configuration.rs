use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::{Arc, RwLock};
use winapi::_core::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    None = 0x00,
    Cancel = 0x03,

    MouseLeftButton = 0x01,
    MouseRightButton = 0x02,
    MouseMiddleButton = 0x04,
    MouseX1Button = 0x05,
    MouseX2Button = 0x06,

    BackSpace = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Enter = 0x0D,
    Shift = 0x10,
    Control = 0x11,
    Alt = 0x12,
    Pause = 0x13,
    CapsLock = 0x14,
    ImeKanaOrHangul = 0x15,
    ImeJunja = 0x17,
    ImeFinal = 0x18,
    ImeKanjiOrHanja = 0x19,

    Escape = 0x1B,
    ImeConvert = 0x1C,
    ImeNonConvert = 0x1D,
    ImeAccept = 0x1E,
    ImeModeChange = 0x1F,
    Space = 0x20,
    PageUp = 0x21,
    PageDown = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    PrintScreen = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,

    Zero = 0x30,
    One = 0x31,
    Two = 0x32,
    Three = 0x33,
    Four = 0x34,
    Five = 0x35,
    Six = 0x36,
    Seven = 0x37,
    Eight = 0x38,
    Nine = 0x39,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    LeftWindows = 0x5B,
    RightWindows = 0x5C,
    Application = 0x5D,
    Sleep = 0x5F,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,

    AsteriskOrMultiply = 0x6A,
    PlusOrAdd = 0x6B,
    Separator = 0x6C,
    MinusOrSubstract = 0x6D,
    PeriodOrDecimal = 0x6E,
    SlashOrDivide = 0x6F,

    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,

    NumLock = 0x90,
    Scroll = 0x91,
    LeftShift = 0xA0,
    RightShift = 0xA1,
    LeftControl = 0xA2,
    RightControl = 0xA3,
    LeftAlt = 0xA4,
    RightAlt = 0xA5,
    BrowserBack = 0xA6,
    BrowserForward = 0xA7,
    BrowserRefresh = 0xA8,
    BrowserStop = 0xA9,
    BrowserSearch = 0xAA,
    BrowserFavorites = 0xAB,
    BrowserHome = 0xAC,

    VolumeMute = 0xAD,
    VolumeDown = 0xAE,
    VolumeUp = 0xAF,

    MediaNextTrack = 0xB0,
    MediaPrevTrack = 0xB1,
    MediaStop = 0xB2,
    MediaPlayPause = 0xB3,
    LaunchMail = 0xB4,
    LaunchMediaSelect = 0xB5,
    LaunchApp1 = 0xB6,
    LaunchApp2 = 0xB7,

    Oem1SemiColonOrColon = 0xBA,
    OemPlus = 0xBB,
    OemComma = 0xBC,
    OemMinus = 0xBD,
    OemPeriod = 0xBE,
    Oem2SlashOrQuestion = 0xBF,
    Oem3BacktickOrTilda = 0xC0,
    Oem4OpenSquareOrCurlyBracket = 0xDB,
    Oem5BackslashOrPipe = 0xDC,
    Oem6CloseSquareOrCurlyBracket = 0xDD,
    Oem7SingleOrDoubleQuote = 0xDE,
    Oem8 = 0xDF,
    Oem102 = 0xE2,
    ImeProcessKey = 0xE5,
    Packet = 0xE7,
    Attn = 0xF6,
    CrSel = 0xF7,
    ExSel = 0xF8,
    EraseEof = 0xF9,
    Play = 0xFA,
    Zoom = 0xFB,
    Pa1 = 0xFD,
    OemClear = 0xFE,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyCombination {
    pub name: String,
    pub key: Key,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KeyAction {
    None,
    KeyHold(KeyCombination),
    KeyClick(KeyCombination),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub show_tooltip: bool,

    pub zl: KeyAction,
    pub l: KeyAction,
    pub minus: KeyAction,
    pub left_stick: KeyAction,
    pub left_stick_top_left: KeyAction,
    pub left_stick_top_center: KeyAction,
    pub left_stick_top_right: KeyAction,
    pub left_stick_bottom_left: KeyAction,
    pub left_stick_bottom_center: KeyAction,
    pub left_stick_bottom_right: KeyAction,
    pub up: KeyAction,
    pub down: KeyAction,
    pub left: KeyAction,
    pub right: KeyAction,
    pub capture: KeyAction,
    pub left_sl: KeyAction,
    pub left_sr: KeyAction,

    pub zr: KeyAction,
    pub r: KeyAction,
    pub plus: KeyAction,
    pub right_stick: KeyAction,
    pub right_stick_top_left: KeyAction,
    pub right_stick_top_center: KeyAction,
    pub right_stick_top_right: KeyAction,
    pub right_stick_bottom_left: KeyAction,
    pub right_stick_bottom_center: KeyAction,
    pub right_stick_bottom_right: KeyAction,
    pub a: KeyAction,
    pub b: KeyAction,
    pub x: KeyAction,
    pub y: KeyAction,
    pub home: KeyAction,
    pub right_sl: KeyAction,
    pub right_sr: KeyAction,
}

pub fn load_config_or_default() -> Arc<RwLock<Box<Config>>> {
    let path = "settings.json";
    let exists = Path::new(path).exists();

    let config = if exists {
        match read_to_string(path) {
            Ok(json) => match serde_json::from_str::<Config>(&json) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    println!("invalid config file error: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("couldn't load file error: {}", e);
                None
            }
        }
    } else {
        None
    };

    let config = config.unwrap_or(create_default());
    if !exists {
        std::fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
    }

    Arc::new(RwLock::new(Box::new(config)))
}

fn create_default() -> Config {
    Config {
        show_tooltip: true,

        zl: KeyAction::KeyHold(KeyCombination {
            name: "Eraser".into(),
            key: Key::E,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        l: KeyAction::KeyHold(KeyCombination {
            name: "Shift".into(),
            key: Key::LeftShift,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        minus: KeyAction::KeyClick(KeyCombination {
            name: "Save".into(),
            key: Key::S,
            ctrl: true,
            alt: false,
            shift: false,
        }),
        left_stick: KeyAction::None,
        left_stick_top_left: KeyAction::KeyClick(KeyCombination {
            name: "Finger".into(),
            key: Key::N,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        left_stick_top_center: KeyAction::KeyClick(KeyCombination {
            name: "Brush".into(),
            key: Key::B,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        left_stick_top_right: KeyAction::KeyClick(KeyCombination {
            name: "Dodge".into(),
            key: Key::O,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        left_stick_bottom_left: KeyAction::KeyHold(KeyCombination {
            name: "Rotate".into(),
            key: Key::R,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        left_stick_bottom_center: KeyAction::KeyHold(KeyCombination {
            name: "Grab".into(),
            key: Key::Space,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        left_stick_bottom_right: KeyAction::KeyHold(KeyCombination {
            name: "Zoom".into(),
            key: Key::Z,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        up: KeyAction::KeyHold(KeyCombination {
            name: "Alt".into(),
            key: Key::LeftAlt,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        down: KeyAction::None,
        left: KeyAction::KeyClick(KeyCombination {
            name: "Smaller Brush".into(),
            key: Key::Oem4OpenSquareOrCurlyBracket,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        right: KeyAction::KeyClick(KeyCombination {
            name: "Larger Brush".into(),
            key: Key::Oem6CloseSquareOrCurlyBracket,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        capture: KeyAction::None,
        left_sl: KeyAction::KeyClick(KeyCombination {
            name: "Undo".into(),
            key: Key::Z,
            ctrl: true,
            alt: false,
            shift: false,
        }),
        left_sr: KeyAction::KeyClick(KeyCombination {
            name: "Redo".into(),
            key: Key::Z,
            ctrl: true,
            alt: false,
            shift: true,
        }),

        zr: KeyAction::KeyHold(KeyCombination {
            name: "Eraser".into(),
            key: Key::E,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        r: KeyAction::KeyHold(KeyCombination {
            name: "Shift".into(),
            key: Key::LeftShift,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        plus: KeyAction::KeyClick(KeyCombination {
            name: "Save".into(),
            key: Key::S,
            ctrl: true,
            alt: false,
            shift: false,
        }),
        right_stick: KeyAction::None,
        right_stick_top_left: KeyAction::KeyClick(KeyCombination {
            name: "Finger".into(),
            key: Key::N,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        right_stick_top_center: KeyAction::KeyClick(KeyCombination {
            name: "Brush".into(),
            key: Key::B,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        right_stick_top_right: KeyAction::KeyClick(KeyCombination {
            name: "Dodge".into(),
            key: Key::O,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        right_stick_bottom_left: KeyAction::KeyHold(KeyCombination {
            name: "Rotate".into(),
            key: Key::R,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        right_stick_bottom_center: KeyAction::KeyHold(KeyCombination {
            name: "Grab".into(),
            key: Key::Space,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        right_stick_bottom_right: KeyAction::KeyHold(KeyCombination {
            name: "Zoom".into(),
            key: Key::Z,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        x: KeyAction::KeyHold(KeyCombination {
            name: "Alt".into(),
            key: Key::LeftAlt,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        b: KeyAction::None,
        y: KeyAction::KeyClick(KeyCombination {
            name: "Smaller Brush".into(),
            key: Key::Oem4OpenSquareOrCurlyBracket,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        a: KeyAction::KeyClick(KeyCombination {
            name: "Larger Brush".into(),
            key: Key::Oem6CloseSquareOrCurlyBracket,
            ctrl: false,
            alt: false,
            shift: false,
        }),
        home: KeyAction::None,
        right_sl: KeyAction::KeyClick(KeyCombination {
            name: "Undo".into(),
            key: Key::Z,
            ctrl: true,
            alt: false,
            shift: false,
        }),
        right_sr: KeyAction::KeyClick(KeyCombination {
            name: "Redo".into(),
            key: Key::Z,
            ctrl: true,
            alt: false,
            shift: true,
        }),
    }
}
