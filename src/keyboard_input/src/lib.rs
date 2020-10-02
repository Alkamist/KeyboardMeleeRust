#[macro_use]
extern crate lazy_static;

use serde::{Serialize, Deserialize};

use std::thread;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::{
    ctypes::*,
    shared::{minwindef::*},
    shared::{windef::*},
    um::winuser::*,
};

pub fn start_hook() {
    thread::spawn(move || {
        unsafe {
            SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), 0 as HINSTANCE, 0);
            loop {
                let mut msg: MSG = MSG {
                    hwnd: null_mut(),
                    message: 0,
                    wParam: 0,
                    lParam: 0,
                    time: 0,
                    pt: POINT { x: 0, y: 0 },
                };
                let msg: LPMSG = &mut msg;
                GetMessageA(msg, null_mut(), 0, 0);
                TranslateMessage(msg);
                DispatchMessageA(msg);
            }
        }
    });
}

pub fn key_is_pressed(key: KeyboardKey) -> bool {
    KEYBOARD_STATE.lock().unwrap().key_is_pressed(key)
}

pub fn key_is_blocked(key: KeyboardKey) -> bool {
    KEYBOARD_STATE.lock().unwrap().key_is_blocked(key)
}

pub fn set_key_blocked(key: KeyboardKey, state: bool) {
    KEYBOARD_STATE.lock().unwrap().set_key_blocked(key, state);
}

pub fn set_all_keys_blocked(state: bool) {
    KEYBOARD_STATE.lock().unwrap().set_all_keys_blocked(state);
}

pub fn block_key(key: KeyboardKey) {
    KEYBOARD_STATE.lock().unwrap().set_key_blocked(key, true);
}

pub fn block_all_keys() {
    KEYBOARD_STATE.lock().unwrap().set_all_keys_blocked(true);
}

pub fn unblock_key(key: KeyboardKey) {
    KEYBOARD_STATE.lock().unwrap().set_key_blocked(key, false);
}

pub fn unblock_all_keys() {
    KEYBOARD_STATE.lock().unwrap().set_all_keys_blocked(false);
}

#[derive(Default)]
struct KeyState {
    is_pressed: bool,
    is_blocked: bool
}

macro_rules! keyboard {
    ($($key_name:ident: $keycode:expr),+) => {
        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum KeyboardKey {
            $($key_name),+
        }

        impl KeyboardKey {
            #[allow(dead_code)]
            pub fn to_keycode(&self) -> usize {
                match self {
                    $(KeyboardKey::$key_name => $keycode),+
                }
            }

            #[allow(dead_code)]
            pub fn from_keycode(keycode: usize) -> KeyboardKey {
                match keycode {
                    $($keycode => KeyboardKey::$key_name),+
                    ,
                    _ => panic!("Tried to convert unknown keycode: {}", keycode),
                }
            }
        }

        #[allow(non_snake_case)]
        #[derive(Default)]
        struct KeyboardState {
            $($key_name: KeyState),+
        }

        impl KeyboardState {
            pub fn key_is_pressed(&self, key: KeyboardKey) -> bool {
                match key {
                    $(KeyboardKey::$key_name => self.$key_name.is_pressed),+
                }
            }

            pub fn key_is_blocked(&mut self, key: KeyboardKey) -> bool {
                match key {
                    $(KeyboardKey::$key_name => self.$key_name.is_blocked),+
                }
            }

            fn set_key_pressed(&mut self, key: KeyboardKey, state: bool) {
                match key {
                    $(KeyboardKey::$key_name => self.$key_name.is_pressed = state),+
                }
            }

            fn set_key_blocked(&mut self, key: KeyboardKey, state: bool) {
                match key {
                    $(KeyboardKey::$key_name => self.$key_name.is_blocked = state),+
                }
            }

            fn set_all_keys_blocked(&mut self, state: bool) {
                $(self.$key_name.is_blocked = state);+
            }
        }
    };
}

keyboard!(
    ControlBreak: 3,
    Backspace: 8,
    Tab: 9,
    Clear: 12,
    Enter: 13,
    Shift: 16,
    Control: 17,
    Alt: 18,
    Pause: 19,
    CapsLock: 20,
    IMEKana: 21,
    IMEJunja: 23,
    IMEFinal: 24,
    IMEHanja: 25,
    Escape: 27,
    IMEConvert: 28,
    IMENonConvert: 29,
    IMEAccept: 30,
    IMEModeChange: 31,
    Space: 32,
    PageUp: 33,
    PageDown: 34,
    End: 35,
    Home: 36,
    LeftArrow: 37,
    UpArrow: 38,
    RightArrow: 39,
    DownArrow: 40,
    Select: 41,
    Print: 42,
    Execute: 43,
    PrintScreen: 44,
    Insert: 45,
    Delete: 46,
    Help: 47,
    Key0: 48,
    Key1: 49,
    Key2: 50,
    Key3: 51,
    Key4: 52,
    Key5: 53,
    Key6: 54,
    Key7: 55,
    Key8: 56,
    Key9: 57,
    A: 65,
    B: 66,
    C: 67,
    D: 68,
    E: 69,
    F: 70,
    G: 71,
    H: 72,
    I: 73,
    J: 74,
    K: 75,
    L: 76,
    M: 77,
    N: 78,
    O: 79,
    P: 80,
    Q: 81,
    R: 82,
    S: 83,
    T: 84,
    U: 85,
    V: 86,
    W: 87,
    X: 88,
    Y: 89,
    Z: 90,
    LeftWindows: 91,
    RightWindows: 92,
    Applications: 93,
    Sleep: 95,
    NumPad0: 96,
    NumPad1: 97,
    NumPad2: 98,
    NumPad3: 99,
    NumPad4: 100,
    NumPad5: 101,
    NumPad6: 102,
    NumPad7: 103,
    NumPad8: 104,
    NumPad9: 105,
    NumPadMultiply: 106,
    NumPadAdd: 107,
    NumPadSeparator: 108,
    NumPadSubtract: 109,
    NumPadDecimal: 110,
    NumPadDivide: 111,
    F1: 112,
    F2: 113,
    F3: 114,
    F4: 115,
    F5: 116,
    F6: 117,
    F7: 118,
    F8: 119,
    F9: 120,
    F10: 121,
    F11: 122,
    F12: 123,
    F13: 124,
    F14: 125,
    F15: 126,
    F16: 127,
    F17: 128,
    F18: 129,
    F20: 130,
    F21: 131,
    F22: 132,
    F23: 133,
    F24: 134,
    NumLock: 144,
    ScrollLock: 145,
    LeftShift: 160,
    RightShift: 161,
    LeftControl: 162,
    RightControl: 163,
    LeftAlt: 164,
    RightAlt: 165,
    BrowserBack: 166,
    BrowserForward: 167,
    BrowserRefresh: 168,
    BrowserStop: 169,
    BrowserSearch: 170,
    BrowserFavorites: 171,
    BrowserHome: 172,
    BrowserMute: 173,
    VolumeDown: 174,
    VolumeUp: 175,
    MediaNextTrack: 176,
    MediaPreviousTrack: 177,
    MediaStop: 178,
    MediaPlay: 179,
    StartMail: 180,
    MediaSelect: 181,
    LaunchApplication1: 182,
    LaunchApplication2: 183,
    Semicolon: 186,
    Equals: 187,
    Comma: 188,
    Minus: 189,
    Period: 190,
    Slash: 191,
    Grave: 192,
    LeftBracket: 219,
    BackSlash: 220,
    RightBracket: 221,
    Apostrophe: 222,
    IMEProcess: 229
);

lazy_static! {
    static ref KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState::default());
}

unsafe extern "system" fn keyboard_hook(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mut block_key_press = false;

    match w_param as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            let key_code = (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode;
            let keyboard_key = KeyboardKey::from_keycode(key_code as usize);
            let mut keyboard = KEYBOARD_STATE.lock().unwrap();
            keyboard.set_key_pressed(keyboard_key, true);
            block_key_press = keyboard.key_is_blocked(keyboard_key);
        },
        WM_KEYUP | WM_SYSKEYUP => {
            let key_code = (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode;
            let keyboard_key = KeyboardKey::from_keycode(key_code as usize);
            let mut keyboard = KEYBOARD_STATE.lock().unwrap();
            keyboard.set_key_pressed(keyboard_key, false);
            block_key_press = keyboard.key_is_blocked(keyboard_key);
        },
        _ => ()
    }

    if block_key_press { 1 }
    else { CallNextHookEx(null_mut(), code, w_param, l_param) }
}
