//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 1993-2008 Raven Software
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Key bindings and control configuration.
// Original: m_controls.h + m_controls.c

use crate::doomkeys;
use crate::ui_hud::config::m_get_int_variable;

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

// Automap
pub static mut KEY_MAP_NORTH: i32 = doomkeys::KEY_UPARROW;
pub static mut KEY_MAP_SOUTH: i32 = doomkeys::KEY_DOWNARROW;
pub static mut KEY_MAP_EAST: i32 = doomkeys::KEY_RIGHTARROW;
pub static mut KEY_MAP_WEST: i32 = doomkeys::KEY_LEFTARROW;
pub static mut KEY_MAP_ZOOMIN: i32 = doomkeys::KEY_EQUALS;
pub static mut KEY_MAP_ZOOMOUT: i32 = doomkeys::KEY_MINUS;
pub static mut KEY_MAP_TOGGLE: i32 = doomkeys::KEY_TAB;
pub static mut KEY_MAP_MAXZOOM: i32 = b'0' as i32;
pub static mut KEY_MAP_FOLLOW: i32 = b'f' as i32;
pub static mut KEY_MAP_GRID: i32 = b'g' as i32;
pub static mut KEY_MAP_MARK: i32 = b'm' as i32;
pub static mut KEY_MAP_CLEARMARK: i32 = b'c' as i32;

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
// Implementation (from m_controls.c) — M_Bind* read config -> globals
// =============================================================================

pub fn m_bind_base_controls() {
    unsafe {
        KEY_RIGHT = m_get_int_variable("key_right");
        KEY_LEFT = m_get_int_variable("key_left");
        KEY_UP = m_get_int_variable("key_up");
        KEY_DOWN = m_get_int_variable("key_down");
        KEY_STRAFELEFT = m_get_int_variable("key_strafeleft");
        KEY_STRAFERIGHT = m_get_int_variable("key_straferight");
        KEY_FIRE = m_get_int_variable("key_fire");
        KEY_USE = m_get_int_variable("key_use");
        KEY_STRAFE = m_get_int_variable("key_strafe");
        KEY_SPEED = m_get_int_variable("key_speed");
        KEY_PAUSE = m_get_int_variable("key_pause");
        KEY_MESSAGE_REFRESH = m_get_int_variable("key_message_refresh");
        KEY_DEMO_QUIT = m_get_int_variable("key_demo_quit");
        KEY_SPY = m_get_int_variable("key_spy");
        KEY_MULTI_MSG = m_get_int_variable("key_multi_msg");
        MOUSEBFIRE = m_get_int_variable("mouseb_fire");
        MOUSEBSTRAFE = m_get_int_variable("mouseb_strafe");
        MOUSEBFORWARD = m_get_int_variable("mouseb_forward");
        JOYBFIRE = m_get_int_variable("joyb_fire");
        JOYBSTRAFE = m_get_int_variable("joyb_strafe");
        JOYBUSE = m_get_int_variable("joyb_use");
        JOYBSPEED = m_get_int_variable("joyb_speed");
    }
}

pub fn m_bind_heretic_controls() {
    // Heretic-specific: key_flyup, key_lookup, etc. — use defaults for now
}

pub fn m_bind_hexen_controls() {
    // Hexen-specific: key_jump, etc. — use defaults for now
}

pub fn m_bind_strife_controls() {
    // Strife-specific — use defaults for now
}

pub fn m_bind_weapon_controls() {
    unsafe {
        KEY_WEAPON1 = m_get_int_variable("key_weapon1");
        KEY_WEAPON2 = m_get_int_variable("key_weapon2");
        KEY_WEAPON3 = m_get_int_variable("key_weapon3");
        KEY_WEAPON4 = m_get_int_variable("key_weapon4");
        KEY_WEAPON5 = m_get_int_variable("key_weapon5");
        KEY_WEAPON6 = m_get_int_variable("key_weapon6");
        KEY_WEAPON7 = m_get_int_variable("key_weapon7");
        KEY_WEAPON8 = m_get_int_variable("key_weapon8");
        KEY_PREVWEAPON = m_get_int_variable("key_prevweapon");
        KEY_NEXTWEAPON = m_get_int_variable("key_nextweapon");
        MOUSEBPREVWEAPON = m_get_int_variable("mouseb_prevweapon");
        MOUSEBNEXTWEAPON = m_get_int_variable("mouseb_nextweapon");
        JOYBPREVWEAPON = m_get_int_variable("joyb_prevweapon");
        JOYBNEXTWEAPON = m_get_int_variable("joyb_nextweapon");
    }
}

pub fn m_bind_map_controls() {
    unsafe {
        KEY_MAP_NORTH = m_get_int_variable("key_map_north");
        KEY_MAP_SOUTH = m_get_int_variable("key_map_south");
        KEY_MAP_EAST = m_get_int_variable("key_map_east");
        KEY_MAP_WEST = m_get_int_variable("key_map_west");
        KEY_MAP_ZOOMIN = m_get_int_variable("key_map_zoomin");
        KEY_MAP_ZOOMOUT = m_get_int_variable("key_map_zoomout");
        KEY_MAP_TOGGLE = m_get_int_variable("key_map_toggle");
        KEY_MAP_MAXZOOM = m_get_int_variable("key_map_maxzoom");
        KEY_MAP_FOLLOW = m_get_int_variable("key_map_follow");
        KEY_MAP_GRID = m_get_int_variable("key_map_grid");
        KEY_MAP_MARK = m_get_int_variable("key_map_mark");
        KEY_MAP_CLEARMARK = m_get_int_variable("key_map_clearmark");
    }
}

pub fn m_bind_menu_controls() {
    unsafe {
        KEY_MENU_ACTIVATE = m_get_int_variable("key_menu_activate");
        KEY_MENU_UP = m_get_int_variable("key_menu_up");
        KEY_MENU_DOWN = m_get_int_variable("key_menu_down");
        KEY_MENU_LEFT = m_get_int_variable("key_menu_left");
        KEY_MENU_RIGHT = m_get_int_variable("key_menu_right");
        KEY_MENU_BACK = m_get_int_variable("key_menu_back");
        KEY_MENU_FORWARD = m_get_int_variable("key_menu_forward");
        KEY_MENU_CONFIRM = m_get_int_variable("key_menu_confirm");
        KEY_MENU_ABORT = m_get_int_variable("key_menu_abort");
    }
}

pub fn m_bind_chat_controls(_num_players: u32) {
    unsafe {
        KEY_MULTI_MSG = m_get_int_variable("key_multi_msg");
    }
}

pub fn m_apply_platform_defaults() {
    // Platform-specific defaults — no-op for generic
}

/// Sync current key/mouse/joy values to config. Call before M_SaveDefaults.
pub fn m_sync_controls_to_config() {
    use crate::ui_hud::config::m_set_variable;
    unsafe {
        m_set_variable("key_right", &KEY_RIGHT.to_string());
        m_set_variable("key_left", &KEY_LEFT.to_string());
        m_set_variable("key_up", &KEY_UP.to_string());
        m_set_variable("key_down", &KEY_DOWN.to_string());
        m_set_variable("key_strafeleft", &KEY_STRAFELEFT.to_string());
        m_set_variable("key_straferight", &KEY_STRAFERIGHT.to_string());
        m_set_variable("key_fire", &KEY_FIRE.to_string());
        m_set_variable("key_use", &KEY_USE.to_string());
        m_set_variable("key_strafe", &KEY_STRAFE.to_string());
        m_set_variable("key_speed", &KEY_SPEED.to_string());
        m_set_variable("key_pause", &KEY_PAUSE.to_string());
        m_set_variable("key_message_refresh", &KEY_MESSAGE_REFRESH.to_string());
        m_set_variable("key_demo_quit", &KEY_DEMO_QUIT.to_string());
        m_set_variable("key_spy", &KEY_SPY.to_string());
        m_set_variable("key_multi_msg", &KEY_MULTI_MSG.to_string());
        m_set_variable("key_weapon1", &KEY_WEAPON1.to_string());
        m_set_variable("key_weapon2", &KEY_WEAPON2.to_string());
        m_set_variable("key_weapon3", &KEY_WEAPON3.to_string());
        m_set_variable("key_weapon4", &KEY_WEAPON4.to_string());
        m_set_variable("key_weapon5", &KEY_WEAPON5.to_string());
        m_set_variable("key_weapon6", &KEY_WEAPON6.to_string());
        m_set_variable("key_weapon7", &KEY_WEAPON7.to_string());
        m_set_variable("key_weapon8", &KEY_WEAPON8.to_string());
        m_set_variable("key_prevweapon", &KEY_PREVWEAPON.to_string());
        m_set_variable("key_nextweapon", &KEY_NEXTWEAPON.to_string());
        m_set_variable("key_menu_activate", &KEY_MENU_ACTIVATE.to_string());
        m_set_variable("key_menu_up", &KEY_MENU_UP.to_string());
        m_set_variable("key_menu_down", &KEY_MENU_DOWN.to_string());
        m_set_variable("key_menu_left", &KEY_MENU_LEFT.to_string());
        m_set_variable("key_menu_right", &KEY_MENU_RIGHT.to_string());
        m_set_variable("key_menu_back", &KEY_MENU_BACK.to_string());
        m_set_variable("key_menu_forward", &KEY_MENU_FORWARD.to_string());
        m_set_variable("key_menu_confirm", &KEY_MENU_CONFIRM.to_string());
        m_set_variable("key_menu_abort", &KEY_MENU_ABORT.to_string());
        m_set_variable("mouseb_fire", &MOUSEBFIRE.to_string());
        m_set_variable("mouseb_strafe", &MOUSEBSTRAFE.to_string());
        m_set_variable("mouseb_forward", &MOUSEBFORWARD.to_string());
        m_set_variable("joyb_fire", &JOYBFIRE.to_string());
        m_set_variable("joyb_strafe", &JOYBSTRAFE.to_string());
        m_set_variable("joyb_use", &JOYBUSE.to_string());
        m_set_variable("joyb_speed", &JOYBSPEED.to_string());
        m_set_variable("key_map_north", &KEY_MAP_NORTH.to_string());
        m_set_variable("key_map_south", &KEY_MAP_SOUTH.to_string());
        m_set_variable("key_map_east", &KEY_MAP_EAST.to_string());
        m_set_variable("key_map_west", &KEY_MAP_WEST.to_string());
        m_set_variable("key_map_zoomin", &KEY_MAP_ZOOMIN.to_string());
        m_set_variable("key_map_zoomout", &KEY_MAP_ZOOMOUT.to_string());
        m_set_variable("key_map_toggle", &KEY_MAP_TOGGLE.to_string());
        m_set_variable("key_map_maxzoom", &KEY_MAP_MAXZOOM.to_string());
        m_set_variable("key_map_follow", &KEY_MAP_FOLLOW.to_string());
        m_set_variable("key_map_grid", &KEY_MAP_GRID.to_string());
        m_set_variable("key_map_mark", &KEY_MAP_MARK.to_string());
        m_set_variable("key_map_clearmark", &KEY_MAP_CLEARMARK.to_string());
    }
}
