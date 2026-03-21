// doomkeys.h - key definitions
// No dependencies (leaf module)

// Original: #define KEY_RIGHTARROW 0xae
pub const KEY_RIGHTARROW: i32 = 0xae;
pub const KEY_LEFTARROW: i32 = 0xac;
pub const KEY_UPARROW: i32 = 0xad;
pub const KEY_DOWNARROW: i32 = 0xaf;
pub const KEY_STRAFE_L: i32 = 0xa0;
pub const KEY_STRAFE_R: i32 = 0xa1;
pub const KEY_USE: i32 = 0xa2;
pub const KEY_FIRE: i32 = 0xa3;
pub const KEY_ESCAPE: i32 = 27;
pub const KEY_ENTER: i32 = 13;
pub const KEY_TAB: i32 = 9;

// Original: #define KEY_F1 (0x80+0x3b)
pub const KEY_F1: i32 = 0x80 + 0x3b;
pub const KEY_F2: i32 = 0x80 + 0x3c;
pub const KEY_F3: i32 = 0x80 + 0x3d;
pub const KEY_F4: i32 = 0x80 + 0x3e;
pub const KEY_F5: i32 = 0x80 + 0x3f;
pub const KEY_F6: i32 = 0x80 + 0x40;
pub const KEY_F7: i32 = 0x80 + 0x41;
pub const KEY_F8: i32 = 0x80 + 0x42;
pub const KEY_F9: i32 = 0x80 + 0x43;
pub const KEY_F10: i32 = 0x80 + 0x44;
pub const KEY_F11: i32 = 0x80 + 0x57;
pub const KEY_F12: i32 = 0x80 + 0x58;

pub const KEY_BACKSPACE: i32 = 0x7f;
pub const KEY_PAUSE: i32 = 0xff;
pub const KEY_EQUALS: i32 = 0x3d;
pub const KEY_MINUS: i32 = 0x2d;
pub const KEY_RSHIFT: i32 = 0x80 + 0x36;
pub const KEY_RCTRL: i32 = 0x80 + 0x1d;
pub const KEY_RALT: i32 = 0x80 + 0x38;
pub const KEY_LALT: i32 = KEY_RALT;

pub const KEY_CAPSLOCK: i32 = 0x80 + 0x3a;
pub const KEY_NUMLOCK: i32 = 0x80 + 0x45;
pub const KEY_SCRLCK: i32 = 0x80 + 0x46;
pub const KEY_PRTSCR: i32 = 0x80 + 0x59;

pub const KEY_HOME: i32 = 0x80 + 0x47;
pub const KEY_END: i32 = 0x80 + 0x4f;
pub const KEY_PGUP: i32 = 0x80 + 0x49;
pub const KEY_PGDN: i32 = 0x80 + 0x51;
pub const KEY_INS: i32 = 0x80 + 0x52;
pub const KEY_DEL: i32 = 0x80 + 0x53;

pub const KEYP_0: i32 = 0;
pub const KEYP_1: i32 = KEY_END;
pub const KEYP_2: i32 = KEY_DOWNARROW;
pub const KEYP_3: i32 = KEY_PGDN;
pub const KEYP_4: i32 = KEY_LEFTARROW;
pub const KEYP_5: i32 = b'5' as i32;
pub const KEYP_6: i32 = KEY_RIGHTARROW;
pub const KEYP_7: i32 = KEY_HOME;
pub const KEYP_8: i32 = KEY_UPARROW;
pub const KEYP_9: i32 = KEY_PGUP;

pub const KEYP_DIVIDE: i32 = b'/' as i32;
pub const KEYP_PLUS: i32 = b'+' as i32;
pub const KEYP_MINUS: i32 = b'-' as i32;
pub const KEYP_MULTIPLY: i32 = b'*' as i32;
pub const KEYP_PERIOD: i32 = 0;
pub const KEYP_EQUALS: i32 = KEY_EQUALS;
pub const KEYP_ENTER: i32 = KEY_ENTER;

#[allow(non_camel_case_types)]
pub struct DoomkeysState;

impl DoomkeysState {
    pub fn new() -> Self {
        Self
    }
}
