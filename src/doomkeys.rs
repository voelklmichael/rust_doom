//! Key definitions (doomkeys.h)
//! Original: doomkeys.h

// #define KEY_RIGHTARROW 0xae
pub const KEY_RIGHTARROW: i32 = 0xae;
// #define KEY_LEFTARROW 0xac
pub const KEY_LEFTARROW: i32 = 0xac;
// #define KEY_UPARROW 0xad
pub const KEY_UPARROW: i32 = 0xad;
// #define KEY_DOWNARROW 0xaf
pub const KEY_DOWNARROW: i32 = 0xaf;
// #define KEY_STRAFE_L 0xa0
pub const KEY_STRAFE_L: i32 = 0xa0;
// #define KEY_STRAFE_R 0xa1
pub const KEY_STRAFE_R: i32 = 0xa1;
// #define KEY_USE 0xa2
pub const KEY_USE: i32 = 0xa2;
// #define KEY_FIRE 0xa3
pub const KEY_FIRE: i32 = 0xa3;
// #define KEY_ESCAPE 27
pub const KEY_ESCAPE: i32 = 27;
// #define KEY_ENTER 13
pub const KEY_ENTER: i32 = 13;
// #define KEY_TAB 9
pub const KEY_TAB: i32 = 9;
// #define KEY_F1 (0x80+0x3b)
pub const KEY_F1: i32 = 0x80 + 0x3b;
// #define KEY_F2 (0x80+0x3c)
pub const KEY_F2: i32 = 0x80 + 0x3c;
// #define KEY_F3 (0x80+0x3d)
pub const KEY_F3: i32 = 0x80 + 0x3d;
// #define KEY_F4 (0x80+0x3e)
pub const KEY_F4: i32 = 0x80 + 0x3e;
// #define KEY_F5 (0x80+0x3f)
pub const KEY_F5: i32 = 0x80 + 0x3f;
// #define KEY_F6 (0x80+0x40)
pub const KEY_F6: i32 = 0x80 + 0x40;
// #define KEY_F7 (0x80+0x41)
pub const KEY_F7: i32 = 0x80 + 0x41;
// #define KEY_F8 (0x80+0x42)
pub const KEY_F8: i32 = 0x80 + 0x42;
// #define KEY_F9 (0x80+0x43)
pub const KEY_F9: i32 = 0x80 + 0x43;
// #define KEY_F10 (0x80+0x44)
pub const KEY_F10: i32 = 0x80 + 0x44;
// #define KEY_F11 (0x80+0x57)
pub const KEY_F11: i32 = 0x80 + 0x57;
// #define KEY_F12 (0x80+0x58)
pub const KEY_F12: i32 = 0x80 + 0x58;
// #define KEY_BACKSPACE 0x7f
pub const KEY_BACKSPACE: i32 = 0x7f;
// #define KEY_PAUSE 0xff
pub const KEY_PAUSE: i32 = 0xff;
// #define KEY_EQUALS 0x3d
pub const KEY_EQUALS: i32 = 0x3d;
// #define KEY_MINUS 0x2d
pub const KEY_MINUS: i32 = 0x2d;
// #define KEY_RSHIFT (0x80+0x36)
pub const KEY_RSHIFT: i32 = 0x80 + 0x36;
// #define KEY_RCTRL (0x80+0x1d)
pub const KEY_RCTRL: i32 = 0x80 + 0x1d;
// #define KEY_RALT (0x80+0x38)
pub const KEY_RALT: i32 = 0x80 + 0x38;
// #define KEY_LALT KEY_RALT
pub const KEY_LALT: i32 = KEY_RALT;
// #define KEY_CAPSLOCK (0x80+0x3a)
pub const KEY_CAPSLOCK: i32 = 0x80 + 0x3a;
// #define KEY_NUMLOCK (0x80+0x45)
pub const KEY_NUMLOCK: i32 = 0x80 + 0x45;
// #define KEY_SCRLCK (0x80+0x46)
pub const KEY_SCRLCK: i32 = 0x80 + 0x46;
// #define KEY_PRTSCR (0x80+0x59)
pub const KEY_PRTSCR: i32 = 0x80 + 0x59;
// #define KEY_HOME (0x80+0x47)
pub const KEY_HOME: i32 = 0x80 + 0x47;
// #define KEY_END (0x80+0x4f)
pub const KEY_END: i32 = 0x80 + 0x4f;
// #define KEY_PGUP (0x80+0x49)
pub const KEY_PGUP: i32 = 0x80 + 0x49;
// #define KEY_PGDN (0x80+0x51)
pub const KEY_PGDN: i32 = 0x80 + 0x51;
// #define KEY_INS (0x80+0x52)
pub const KEY_INS: i32 = 0x80 + 0x52;
// #define KEY_DEL (0x80+0x53)
pub const KEY_DEL: i32 = 0x80 + 0x53;
// #define KEYP_0 0
pub const KEYP_0: i32 = 0;
// #define KEYP_1 KEY_END
pub const KEYP_1: i32 = KEY_END;
// #define KEYP_2 KEY_DOWNARROW
pub const KEYP_2: i32 = KEY_DOWNARROW;
// #define KEYP_3 KEY_PGDN
pub const KEYP_3: i32 = KEY_PGDN;
// #define KEYP_4 KEY_LEFTARROW
pub const KEYP_4: i32 = KEY_LEFTARROW;
// #define KEYP_5 '5'
pub const KEYP_5: i32 = b'5' as i32;
// #define KEYP_6 KEY_RIGHTARROW
pub const KEYP_6: i32 = KEY_RIGHTARROW;
// #define KEYP_7 KEY_HOME
pub const KEYP_7: i32 = KEY_HOME;
// #define KEYP_8 KEY_UPARROW
pub const KEYP_8: i32 = KEY_UPARROW;
// #define KEYP_9 KEY_PGUP
pub const KEYP_9: i32 = KEY_PGUP;
// #define KEYP_DIVIDE '/'
pub const KEYP_DIVIDE: i32 = b'/' as i32;
// #define KEYP_PLUS '+'
pub const KEYP_PLUS: i32 = b'+' as i32;
// #define KEYP_MINUS '-'
pub const KEYP_MINUS: i32 = KEY_MINUS;
// #define KEYP_MULTIPLY '*'
pub const KEYP_MULTIPLY: i32 = b'*' as i32;
// #define KEYP_PERIOD 0
pub const KEYP_PERIOD: i32 = 0;
// #define KEYP_EQUALS KEY_EQUALS
pub const KEYP_EQUALS: i32 = KEY_EQUALS;
// #define KEYP_ENTER KEY_ENTER
pub const KEYP_ENTER: i32 = KEY_ENTER;
