//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 1993-2008 Raven Software
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Key bindings and control configuration.
// Original: m_controls.h + m_controls.c

use crate::doomkeys;

// =============================================================================
// Public API (from m_controls.h) — key globals
// =============================================================================

// Movement
pub static mut KEY_RIGHT: i32 = doomkeys::KEY_RIGHTARROW;
pub static mut KEY_LEFT: i32 = doomkeys::KEY_LEFTARROW;
pub static mut KEY_UP: i32 = doomkeys::KEY_UPARROW;
pub static mut KEY_DOWN: i32 = doomkeys::KEY_DOWNARROW;
pub static mut KEY_STRAFELEFT: i32 = doomkeys::KEY_STRAFE_L;
pub static mut KEY_STRAFERIGHT: i32 = doomkeys::KEY_STRAFE_R;
pub static mut KEY_FIRE: i32 = doomkeys::KEY_FIRE;
pub static mut KEY_USE: i32 = doomkeys::KEY_USE;
pub static mut KEY_STRAFE: i32 = doomkeys::KEY_RALT;
pub static mut KEY_SPEED: i32 = doomkeys::KEY_RSHIFT;

pub static mut KEY_JUMP: i32 = b'/' as i32;

pub static mut KEY_LOOKUP: i32 = doomkeys::KEY_PGDN;
pub static mut KEY_LOOKDOWN: i32 = doomkeys::KEY_DEL;
pub static mut KEY_LOOKCENTER: i32 = doomkeys::KEY_END;

pub static mut KEY_MESSAGE_REFRESH: i32 = doomkeys::KEY_ENTER;
pub static mut KEY_PAUSE: i32 = doomkeys::KEY_PAUSE;
pub static mut KEY_DEMO_QUIT: i32 = b'q' as i32;
pub static mut KEY_SPY: i32 = doomkeys::KEY_F12;

pub static mut KEY_MULTI_MSG: i32 = b't' as i32;

// Weapon selection
pub static mut KEY_WEAPON1: i32 = b'1' as i32;
pub static mut KEY_WEAPON2: i32 = b'2' as i32;
pub static mut KEY_WEAPON3: i32 = b'3' as i32;
pub static mut KEY_WEAPON4: i32 = b'4' as i32;
pub static mut KEY_WEAPON5: i32 = b'5' as i32;
pub static mut KEY_WEAPON6: i32 = b'6' as i32;
pub static mut KEY_WEAPON7: i32 = b'7' as i32;
pub static mut KEY_WEAPON8: i32 = b'8' as i32;

pub static mut KEY_PREVWEAPON: i32 = 0;
pub static mut KEY_NEXTWEAPON: i32 = 0;

// Menu
pub static mut KEY_MENU_ACTIVATE: i32 = doomkeys::KEY_ESCAPE;
pub static mut KEY_MENU_UP: i32 = doomkeys::KEY_UPARROW;
pub static mut KEY_MENU_DOWN: i32 = doomkeys::KEY_DOWNARROW;
pub static mut KEY_MENU_LEFT: i32 = doomkeys::KEY_LEFTARROW;
pub static mut KEY_MENU_RIGHT: i32 = doomkeys::KEY_RIGHTARROW;
pub static mut KEY_MENU_BACK: i32 = doomkeys::KEY_BACKSPACE;
pub static mut KEY_MENU_FORWARD: i32 = doomkeys::KEY_ENTER;
pub static mut KEY_MENU_CONFIRM: i32 = doomkeys::KEY_ENTER;
pub static mut KEY_MENU_ABORT: i32 = doomkeys::KEY_ESCAPE;

// Mouse
pub static mut MOUSEBFIRE: i32 = 0;
pub static mut MOUSEBSTRAFE: i32 = 1;
pub static mut MOUSEBFORWARD: i32 = 2;
pub static mut MOUSEBUSE: i32 = 2;
pub static mut MOUSEBJUMP: i32 = -1;
pub static mut MOUSEBPREVWEAPON: i32 = -1;
pub static mut MOUSEBNEXTWEAPON: i32 = -1;

// Joystick
pub static mut JOYBFIRE: i32 = 0;
pub static mut JOYBSTRAFE: i32 = 1;
pub static mut JOYBUSE: i32 = 2;
pub static mut JOYBSPEED: i32 = 3;
pub static mut JOYBJUMP: i32 = -1;
pub static mut JOYBPREVWEAPON: i32 = -1;
pub static mut JOYBNEXTWEAPON: i32 = -1;
pub static mut JOYBMENU: i32 = -1;

pub static mut DCLICK_USE: i32 = 0;

// =============================================================================
// Implementation (from m_controls.c) — M_Bind* stubs
// =============================================================================

pub fn m_bind_base_controls() {
    // Stub: would bind key_right, key_left, etc. from config
}

pub fn m_bind_heretic_controls() {
    // Stub: Heretic-specific bindings
}

pub fn m_bind_hexen_controls() {
    // Stub: Hexen-specific bindings
}

pub fn m_bind_strife_controls() {
    // Stub: Strife-specific bindings
}

pub fn m_bind_weapon_controls() {
    // Stub: weapon key bindings
}

pub fn m_bind_map_controls() {
    // Stub: automap key bindings
}

pub fn m_bind_menu_controls() {
    // Stub: menu key bindings
}

pub fn m_bind_chat_controls(_num_players: u32) {
    // Stub: multiplayer chat key bindings
}

pub fn m_apply_platform_defaults() {
    // Stub: platform-specific defaults
}
