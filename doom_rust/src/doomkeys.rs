//! Rust translation of doomgeneric/doomkeys.h
//! Key definitions.

/// C #define: KEY_RIGHTARROW
pub const KEY_RIGHTARROW: i32 = 0xae;
/// C #define: KEY_LEFTARROW
pub const KEY_LEFTARROW: i32 = 0xac;
/// C #define: KEY_UPARROW
pub const KEY_UPARROW: i32 = 0xad;
/// C #define: KEY_DOWNARROW
pub const KEY_DOWNARROW: i32 = 0xaf;
/// C #define: KEY_STRAFE_L
pub const KEY_STRAFE_L: i32 = 0xa0;
/// C #define: KEY_STRAFE_R
pub const KEY_STRAFE_R: i32 = 0xa1;
/// C #define: KEY_USE
pub const KEY_USE: i32 = 0xa2;
/// C #define: KEY_FIRE
pub const KEY_FIRE: i32 = 0xa3;
/// C #define: KEY_ESCAPE
pub const KEY_ESCAPE: i32 = 27;
/// C #define: KEY_ENTER
pub const KEY_ENTER: i32 = 13;
/// C #define: KEY_TAB
pub const KEY_TAB: i32 = 9;
/// C #define: KEY_F1
pub const KEY_F1: i32 = 0x80 + 0x3b;
/// C #define: KEY_F2
pub const KEY_F2: i32 = 0x80 + 0x3c;
/// C #define: KEY_F3
pub const KEY_F3: i32 = 0x80 + 0x3d;
/// C #define: KEY_F4
pub const KEY_F4: i32 = 0x80 + 0x3e;
/// C #define: KEY_F5
pub const KEY_F5: i32 = 0x80 + 0x3f;
/// C #define: KEY_F6
pub const KEY_F6: i32 = 0x80 + 0x40;
/// C #define: KEY_F7
pub const KEY_F7: i32 = 0x80 + 0x41;
/// C #define: KEY_F8
pub const KEY_F8: i32 = 0x80 + 0x42;
/// C #define: KEY_F9
pub const KEY_F9: i32 = 0x80 + 0x43;
/// C #define: KEY_F10
pub const KEY_F10: i32 = 0x80 + 0x44;
/// C #define: KEY_F11
pub const KEY_F11: i32 = 0x80 + 0x57;
/// C #define: KEY_F12
pub const KEY_F12: i32 = 0x80 + 0x58;
/// C #define: KEY_BACKSPACE
pub const KEY_BACKSPACE: i32 = 0x7f;
/// C #define: KEY_PAUSE
pub const KEY_PAUSE: i32 = 0xff;
/// C #define: KEY_EQUALS
pub const KEY_EQUALS: i32 = 0x3d;
/// C #define: KEY_MINUS
pub const KEY_MINUS: i32 = 0x2d;
/// C #define: KEY_RSHIFT
pub const KEY_RSHIFT: i32 = 0x80 + 0x36;
/// C #define: KEY_RCTRL
pub const KEY_RCTRL: i32 = 0x80 + 0x1d;
/// C #define: KEY_RALT
pub const KEY_RALT: i32 = 0x80 + 0x38;
/// C #define: KEY_LALT
pub const KEY_LALT: i32 = KEY_RALT;
/// C #define: KEY_CAPSLOCK
pub const KEY_CAPSLOCK: i32 = 0x80 + 0x3a;
/// C #define: KEY_NUMLOCK
pub const KEY_NUMLOCK: i32 = 0x80 + 0x45;
/// C #define: KEY_SCRLCK
pub const KEY_SCRLCK: i32 = 0x80 + 0x46;
/// C #define: KEY_PRTSCR
pub const KEY_PRTSCR: i32 = 0x80 + 0x59;
/// C #define: KEY_HOME
pub const KEY_HOME: i32 = 0x80 + 0x47;
/// C #define: KEY_END
pub const KEY_END: i32 = 0x80 + 0x4f;
/// C #define: KEY_PGUP
pub const KEY_PGUP: i32 = 0x80 + 0x49;
/// C #define: KEY_PGDN
pub const KEY_PGDN: i32 = 0x80 + 0x51;
/// C #define: KEY_INS
pub const KEY_INS: i32 = 0x80 + 0x52;
/// C #define: KEY_DEL
pub const KEY_DEL: i32 = 0x80 + 0x53;
/// C #define: KEYP_0
pub const KEYP_0: i32 = 0;
/// C #define: KEYP_1
pub const KEYP_1: i32 = KEY_END;
/// C #define: KEYP_2
pub const KEYP_2: i32 = KEY_DOWNARROW;
/// C #define: KEYP_3
pub const KEYP_3: i32 = KEY_PGDN;
/// C #define: KEYP_4
pub const KEYP_4: i32 = KEY_LEFTARROW;
/// C #define: KEYP_5
pub const KEYP_5: i32 = b'5' as i32;
/// C #define: KEYP_6
pub const KEYP_6: i32 = KEY_RIGHTARROW;
/// C #define: KEYP_7
pub const KEYP_7: i32 = KEY_HOME;
/// C #define: KEYP_8
pub const KEYP_8: i32 = KEY_UPARROW;
/// C #define: KEYP_9
pub const KEYP_9: i32 = KEY_PGUP;
/// C #define: KEYP_DIVIDE
pub const KEYP_DIVIDE: i32 = b'/' as i32;
/// C #define: KEYP_PLUS
pub const KEYP_PLUS: i32 = b'+' as i32;
/// C #define: KEYP_MINUS
pub const KEYP_MINUS: i32 = b'-' as i32;
/// C #define: KEYP_MULTIPLY
pub const KEYP_MULTIPLY: i32 = b'*' as i32;
/// C #define: KEYP_PERIOD
pub const KEYP_PERIOD: i32 = 0;
/// C #define: KEYP_EQUALS
pub const KEYP_EQUALS: i32 = KEY_EQUALS;
/// C #define: KEYP_ENTER
pub const KEYP_ENTER: i32 = KEY_ENTER;
