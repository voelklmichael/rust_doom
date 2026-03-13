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

// Heretic
pub static mut KEY_FLYUP: i32 = doomkeys::KEY_PGUP;
pub static mut KEY_FLYDOWN: i32 = doomkeys::KEY_INS;
pub static mut KEY_FLYCENTER: i32 = doomkeys::KEY_HOME;
pub static mut KEY_LOOKUP: i32 = doomkeys::KEY_PGDN;
pub static mut KEY_LOOKDOWN: i32 = doomkeys::KEY_DEL;
pub static mut KEY_LOOKCENTER: i32 = doomkeys::KEY_END;
pub static mut KEY_INVLEFT: i32 = b'[' as i32;
pub static mut KEY_INVRIGHT: i32 = b']' as i32;
pub static mut KEY_USEARTIFACT: i32 = doomkeys::KEY_ENTER;

// Hexen artifacts
pub static mut KEY_ARTI_ALL: i32 = doomkeys::KEY_BACKSPACE;
pub static mut KEY_ARTI_HEALTH: i32 = b'\\' as i32;
pub static mut KEY_ARTI_POISONBAG: i32 = b'0' as i32;
pub static mut KEY_ARTI_BLASTRADIUS: i32 = b'9' as i32;
pub static mut KEY_ARTI_TELEPORT: i32 = b'8' as i32;
pub static mut KEY_ARTI_TELEPORTOTHER: i32 = b'7' as i32;
pub static mut KEY_ARTI_EGG: i32 = b'6' as i32;
pub static mut KEY_ARTI_INVULNERABILITY: i32 = b'5' as i32;

// Strife
pub static mut KEY_USEHEALTH: i32 = b'h' as i32;
pub static mut KEY_INVQUERY: i32 = b'q' as i32;
pub static mut KEY_MISSION: i32 = b'w' as i32;
pub static mut KEY_INVPOP: i32 = b'z' as i32;
pub static mut KEY_INVKEY: i32 = b'k' as i32;
pub static mut KEY_INVHOME: i32 = doomkeys::KEY_HOME;
pub static mut KEY_INVEND: i32 = doomkeys::KEY_END;
pub static mut KEY_INVUSE: i32 = doomkeys::KEY_ENTER;
pub static mut KEY_INVDROP: i32 = doomkeys::KEY_BACKSPACE;

pub static mut KEY_MULTI_MSGPLAYER: [i32; 8] = [0; 8];

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
pub static mut KEY_MENU_CONFIRM: i32 = b'y' as i32;
pub static mut KEY_MENU_ABORT: i32 = b'n' as i32;
pub static mut KEY_MENU_HELP: i32 = doomkeys::KEY_F1;
pub static mut KEY_MENU_SAVE: i32 = doomkeys::KEY_F2;
pub static mut KEY_MENU_LOAD: i32 = doomkeys::KEY_F3;
pub static mut KEY_MENU_VOLUME: i32 = doomkeys::KEY_F4;
pub static mut KEY_MENU_DETAIL: i32 = doomkeys::KEY_F5;
pub static mut KEY_MENU_QSAVE: i32 = doomkeys::KEY_F6;
pub static mut KEY_MENU_ENDGAME: i32 = doomkeys::KEY_F7;
pub static mut KEY_MENU_MESSAGES: i32 = doomkeys::KEY_F8;
pub static mut KEY_MENU_QLOAD: i32 = doomkeys::KEY_F9;
pub static mut KEY_MENU_QUIT: i32 = doomkeys::KEY_F10;
pub static mut KEY_MENU_GAMMA: i32 = doomkeys::KEY_F11;
pub static mut KEY_MENU_INCSCREEN: i32 = doomkeys::KEY_EQUALS;
pub static mut KEY_MENU_DECSCREEN: i32 = doomkeys::KEY_MINUS;
pub static mut KEY_MENU_SCREENSHOT: i32 = 0;

// Mouse (C: mousebfire=0, mousebstrafe=1, mousebforward=2, mousebuse=-1)
pub static mut MOUSEBFIRE: i32 = 0;
pub static mut MOUSEBSTRAFE: i32 = 1;
pub static mut MOUSEBFORWARD: i32 = 2;
pub static mut MOUSEBUSE: i32 = -1;
pub static mut MOUSEBJUMP: i32 = -1;
pub static mut MOUSEBSTRAFELEFT: i32 = -1;
pub static mut MOUSEBSTRAFERIGHT: i32 = -1;
pub static mut MOUSEBBACKWARD: i32 = -1;
pub static mut MOUSEBPREVWEAPON: i32 = -1;
pub static mut MOUSEBNEXTWEAPON: i32 = -1;

// Joystick (C: joybfire=0, joybstrafe=1, joybuse=3, joybspeed=2)
pub static mut JOYBFIRE: i32 = 0;
pub static mut JOYBSTRAFE: i32 = 1;
pub static mut JOYBUSE: i32 = 3;
pub static mut JOYBSPEED: i32 = 2;
pub static mut JOYBJUMP: i32 = -1;
pub static mut JOYBSTRAFELEFT: i32 = -1;
pub static mut JOYBSTRAFERIGHT: i32 = -1;
pub static mut JOYBPREVWEAPON: i32 = -1;
pub static mut JOYBNEXTWEAPON: i32 = -1;
pub static mut JOYBMENU: i32 = -1;

pub static mut DCLICK_USE: i32 = 1;

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
        MOUSEBFIRE = m_get_int_variable("mouseb_fire");
        MOUSEBSTRAFE = m_get_int_variable("mouseb_strafe");
        MOUSEBFORWARD = m_get_int_variable("mouseb_forward");
        JOYBFIRE = m_get_int_variable("joyb_fire");
        JOYBSTRAFE = m_get_int_variable("joyb_strafe");
        JOYBUSE = m_get_int_variable("joyb_use");
        JOYBSPEED = m_get_int_variable("joyb_speed");
        JOYBMENU = m_get_int_variable("joyb_menu_activate");
        JOYBSTRAFELEFT = m_get_int_variable("joyb_strafeleft");
        JOYBSTRAFERIGHT = m_get_int_variable("joyb_straferight");
        MOUSEBSTRAFELEFT = m_get_int_variable("mouseb_strafeleft");
        MOUSEBSTRAFERIGHT = m_get_int_variable("mouseb_straferight");
        MOUSEBUSE = m_get_int_variable("mouseb_use");
        MOUSEBBACKWARD = m_get_int_variable("mouseb_backward");
        DCLICK_USE = m_get_int_variable("dclick_use");
    }
}

pub fn m_bind_heretic_controls() {
    unsafe {
        KEY_FLYUP = m_get_int_variable("key_flyup");
        KEY_FLYDOWN = m_get_int_variable("key_flydown");
        KEY_FLYCENTER = m_get_int_variable("key_flycenter");
        KEY_LOOKUP = m_get_int_variable("key_lookup");
        KEY_LOOKDOWN = m_get_int_variable("key_lookdown");
        KEY_LOOKCENTER = m_get_int_variable("key_lookcenter");
        KEY_INVLEFT = m_get_int_variable("key_invleft");
        KEY_INVRIGHT = m_get_int_variable("key_invright");
        KEY_USEARTIFACT = m_get_int_variable("key_useartifact");
    }
}

pub fn m_bind_hexen_controls() {
    unsafe {
        KEY_JUMP = m_get_int_variable("key_jump");
        MOUSEBJUMP = m_get_int_variable("mouseb_jump");
        JOYBJUMP = m_get_int_variable("joyb_jump");
        KEY_ARTI_ALL = m_get_int_variable("key_arti_all");
        KEY_ARTI_HEALTH = m_get_int_variable("key_arti_health");
        KEY_ARTI_POISONBAG = m_get_int_variable("key_arti_poisonbag");
        KEY_ARTI_BLASTRADIUS = m_get_int_variable("key_arti_blastradius");
        KEY_ARTI_TELEPORT = m_get_int_variable("key_arti_teleport");
        KEY_ARTI_TELEPORTOTHER = m_get_int_variable("key_arti_teleportother");
        KEY_ARTI_EGG = m_get_int_variable("key_arti_egg");
        KEY_ARTI_INVULNERABILITY = m_get_int_variable("key_arti_invulnerability");
    }
}

pub fn m_bind_strife_controls() {
    unsafe {
        // Strife-specific defaults before binding (C: M_BindStrifeControls)
        KEY_MESSAGE_REFRESH = b'/' as i32;
        KEY_JUMP = b'a' as i32;
        KEY_LOOKUP = doomkeys::KEY_PGUP;
        KEY_LOOKDOWN = doomkeys::KEY_PGDN;
        KEY_INVLEFT = doomkeys::KEY_INS;
        KEY_INVRIGHT = doomkeys::KEY_DEL;
        // Then read from config (overwrites defaults)
        KEY_JUMP = m_get_int_variable("key_jump");
        KEY_LOOKUP = m_get_int_variable("key_lookup");
        KEY_LOOKDOWN = m_get_int_variable("key_lookdown");
        KEY_INVLEFT = m_get_int_variable("key_invleft");
        KEY_INVRIGHT = m_get_int_variable("key_invright");
        KEY_USEHEALTH = m_get_int_variable("key_usehealth");
        KEY_INVQUERY = m_get_int_variable("key_invquery");
        KEY_MISSION = m_get_int_variable("key_mission");
        KEY_INVPOP = m_get_int_variable("key_invpop");
        KEY_INVKEY = m_get_int_variable("key_invkey");
        KEY_INVHOME = m_get_int_variable("key_invhome");
        KEY_INVEND = m_get_int_variable("key_invend");
        KEY_INVUSE = m_get_int_variable("key_invuse");
        KEY_INVDROP = m_get_int_variable("key_invdrop");
        MOUSEBJUMP = m_get_int_variable("mouseb_jump");
        JOYBJUMP = m_get_int_variable("joyb_jump");
    }
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
        KEY_MENU_HELP = m_get_int_variable("key_menu_help");
        KEY_MENU_SAVE = m_get_int_variable("key_menu_save");
        KEY_MENU_LOAD = m_get_int_variable("key_menu_load");
        KEY_MENU_VOLUME = m_get_int_variable("key_menu_volume");
        KEY_MENU_DETAIL = m_get_int_variable("key_menu_detail");
        KEY_MENU_QSAVE = m_get_int_variable("key_menu_qsave");
        KEY_MENU_ENDGAME = m_get_int_variable("key_menu_endgame");
        KEY_MENU_MESSAGES = m_get_int_variable("key_menu_messages");
        KEY_MENU_QLOAD = m_get_int_variable("key_menu_qload");
        KEY_MENU_QUIT = m_get_int_variable("key_menu_quit");
        KEY_MENU_GAMMA = m_get_int_variable("key_menu_gamma");
        KEY_MENU_INCSCREEN = m_get_int_variable("key_menu_incscreen");
        KEY_MENU_DECSCREEN = m_get_int_variable("key_menu_decscreen");
        KEY_MENU_SCREENSHOT = m_get_int_variable("key_menu_screenshot");
        KEY_DEMO_QUIT = m_get_int_variable("key_demo_quit");
        KEY_SPY = m_get_int_variable("key_spy");
    }
}

pub fn m_bind_chat_controls(num_players: u32) {
    unsafe {
        KEY_MULTI_MSG = m_get_int_variable("key_multi_msg");
        for i in 0..num_players.min(8) {
            KEY_MULTI_MSGPLAYER[i as usize] =
                m_get_int_variable(&format!("key_multi_msgplayer{}", i + 1));
        }
    }
}

pub fn m_apply_platform_defaults() {
    // Platform-specific defaults — no-op for generic
}

/// Update a single control global from config. Called by m_set_variable when setting int vars.
/// Implements M_BindVariable-style pointer binding: config changes propagate to controls.
pub fn m_update_control_from_config(name: &str, value: i32) {
    let name = name.to_lowercase();
    unsafe {
        match name.as_str() {
            "key_right" => KEY_RIGHT = value,
            "key_left" => KEY_LEFT = value,
            "key_up" => KEY_UP = value,
            "key_down" => KEY_DOWN = value,
            "key_strafeleft" => KEY_STRAFELEFT = value,
            "key_straferight" => KEY_STRAFERIGHT = value,
            "key_fire" => KEY_FIRE = value,
            "key_use" => KEY_USE = value,
            "key_strafe" => KEY_STRAFE = value,
            "key_speed" => KEY_SPEED = value,
            "key_pause" => KEY_PAUSE = value,
            "key_message_refresh" => KEY_MESSAGE_REFRESH = value,
            "key_demo_quit" => KEY_DEMO_QUIT = value,
            "key_spy" => KEY_SPY = value,
            "key_multi_msg" => KEY_MULTI_MSG = value,
            "key_weapon1" => KEY_WEAPON1 = value,
            "key_weapon2" => KEY_WEAPON2 = value,
            "key_weapon3" => KEY_WEAPON3 = value,
            "key_weapon4" => KEY_WEAPON4 = value,
            "key_weapon5" => KEY_WEAPON5 = value,
            "key_weapon6" => KEY_WEAPON6 = value,
            "key_weapon7" => KEY_WEAPON7 = value,
            "key_weapon8" => KEY_WEAPON8 = value,
            "key_prevweapon" => KEY_PREVWEAPON = value,
            "key_nextweapon" => KEY_NEXTWEAPON = value,
            "key_menu_activate" => KEY_MENU_ACTIVATE = value,
            "key_menu_up" => KEY_MENU_UP = value,
            "key_menu_down" => KEY_MENU_DOWN = value,
            "key_menu_left" => KEY_MENU_LEFT = value,
            "key_menu_right" => KEY_MENU_RIGHT = value,
            "key_menu_back" => KEY_MENU_BACK = value,
            "key_menu_forward" => KEY_MENU_FORWARD = value,
            "key_menu_confirm" => KEY_MENU_CONFIRM = value,
            "key_menu_abort" => KEY_MENU_ABORT = value,
            "mouseb_fire" => MOUSEBFIRE = value,
            "mouseb_strafe" => MOUSEBSTRAFE = value,
            "mouseb_forward" => MOUSEBFORWARD = value,
            "joyb_fire" => JOYBFIRE = value,
            "joyb_strafe" => JOYBSTRAFE = value,
            "joyb_use" => JOYBUSE = value,
            "joyb_speed" => JOYBSPEED = value,
            "key_map_north" => KEY_MAP_NORTH = value,
            "key_map_south" => KEY_MAP_SOUTH = value,
            "key_map_east" => KEY_MAP_EAST = value,
            "key_map_west" => KEY_MAP_WEST = value,
            "key_map_zoomin" => KEY_MAP_ZOOMIN = value,
            "key_map_zoomout" => KEY_MAP_ZOOMOUT = value,
            "key_map_toggle" => KEY_MAP_TOGGLE = value,
            "key_map_maxzoom" => KEY_MAP_MAXZOOM = value,
            "key_map_follow" => KEY_MAP_FOLLOW = value,
            "key_map_grid" => KEY_MAP_GRID = value,
            "key_map_mark" => KEY_MAP_MARK = value,
            "key_map_clearmark" => KEY_MAP_CLEARMARK = value,
            "mouseb_prevweapon" => MOUSEBPREVWEAPON = value,
            "mouseb_nextweapon" => MOUSEBNEXTWEAPON = value,
            "joyb_prevweapon" => JOYBPREVWEAPON = value,
            "joyb_nextweapon" => JOYBNEXTWEAPON = value,
            "joyb_menu_activate" => JOYBMENU = value,
            "joyb_strafeleft" => JOYBSTRAFELEFT = value,
            "joyb_straferight" => JOYBSTRAFERIGHT = value,
            "mouseb_strafeleft" => MOUSEBSTRAFELEFT = value,
            "mouseb_straferight" => MOUSEBSTRAFERIGHT = value,
            "mouseb_use" => MOUSEBUSE = value,
            "mouseb_backward" => MOUSEBBACKWARD = value,
            "mouseb_jump" => MOUSEBJUMP = value,
            "joyb_jump" => JOYBJUMP = value,
            "dclick_use" => DCLICK_USE = value,
            "key_flyup" => KEY_FLYUP = value,
            "key_flydown" => KEY_FLYDOWN = value,
            "key_flycenter" => KEY_FLYCENTER = value,
            "key_lookup" => KEY_LOOKUP = value,
            "key_lookdown" => KEY_LOOKDOWN = value,
            "key_lookcenter" => KEY_LOOKCENTER = value,
            "key_invleft" => KEY_INVLEFT = value,
            "key_invright" => KEY_INVRIGHT = value,
            "key_useartifact" => KEY_USEARTIFACT = value,
            "key_arti_all" => KEY_ARTI_ALL = value,
            "key_arti_health" => KEY_ARTI_HEALTH = value,
            "key_arti_poisonbag" => KEY_ARTI_POISONBAG = value,
            "key_arti_blastradius" => KEY_ARTI_BLASTRADIUS = value,
            "key_arti_teleport" => KEY_ARTI_TELEPORT = value,
            "key_arti_teleportother" => KEY_ARTI_TELEPORTOTHER = value,
            "key_arti_egg" => KEY_ARTI_EGG = value,
            "key_arti_invulnerability" => KEY_ARTI_INVULNERABILITY = value,
            "key_usehealth" => KEY_USEHEALTH = value,
            "key_invquery" => KEY_INVQUERY = value,
            "key_mission" => KEY_MISSION = value,
            "key_invpop" => KEY_INVPOP = value,
            "key_invkey" => KEY_INVKEY = value,
            "key_invhome" => KEY_INVHOME = value,
            "key_invend" => KEY_INVEND = value,
            "key_invuse" => KEY_INVUSE = value,
            "key_invdrop" => KEY_INVDROP = value,
            "key_menu_help" => KEY_MENU_HELP = value,
            "key_menu_save" => KEY_MENU_SAVE = value,
            "key_menu_load" => KEY_MENU_LOAD = value,
            "key_menu_volume" => KEY_MENU_VOLUME = value,
            "key_menu_detail" => KEY_MENU_DETAIL = value,
            "key_menu_qsave" => KEY_MENU_QSAVE = value,
            "key_menu_endgame" => KEY_MENU_ENDGAME = value,
            "key_menu_messages" => KEY_MENU_MESSAGES = value,
            "key_menu_qload" => KEY_MENU_QLOAD = value,
            "key_menu_quit" => KEY_MENU_QUIT = value,
            "key_menu_gamma" => KEY_MENU_GAMMA = value,
            "key_menu_incscreen" => KEY_MENU_INCSCREEN = value,
            "key_menu_decscreen" => KEY_MENU_DECSCREEN = value,
            "key_menu_screenshot" => KEY_MENU_SCREENSHOT = value,
            n => {
                if n.starts_with("key_multi_msgplayer") {
                    if let Ok(idx) = n["key_multi_msgplayer".len()..].parse::<usize>() {
                        if (1..=8).contains(&idx) {
                            KEY_MULTI_MSGPLAYER[idx - 1] = value;
                        }
                    }
                }
            }
        }
    }
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
        m_set_variable("joyb_menu_activate", &JOYBMENU.to_string());
        m_set_variable("joyb_strafeleft", &JOYBSTRAFELEFT.to_string());
        m_set_variable("joyb_straferight", &JOYBSTRAFERIGHT.to_string());
        m_set_variable("mouseb_strafeleft", &MOUSEBSTRAFELEFT.to_string());
        m_set_variable("mouseb_straferight", &MOUSEBSTRAFERIGHT.to_string());
        m_set_variable("mouseb_use", &MOUSEBUSE.to_string());
        m_set_variable("mouseb_backward", &MOUSEBBACKWARD.to_string());
        m_set_variable("mouseb_jump", &MOUSEBJUMP.to_string());
        m_set_variable("joyb_jump", &JOYBJUMP.to_string());
        m_set_variable("dclick_use", &DCLICK_USE.to_string());
        m_set_variable("key_flyup", &KEY_FLYUP.to_string());
        m_set_variable("key_flydown", &KEY_FLYDOWN.to_string());
        m_set_variable("key_flycenter", &KEY_FLYCENTER.to_string());
        m_set_variable("key_lookup", &KEY_LOOKUP.to_string());
        m_set_variable("key_lookdown", &KEY_LOOKDOWN.to_string());
        m_set_variable("key_lookcenter", &KEY_LOOKCENTER.to_string());
        m_set_variable("key_invleft", &KEY_INVLEFT.to_string());
        m_set_variable("key_invright", &KEY_INVRIGHT.to_string());
        m_set_variable("key_useartifact", &KEY_USEARTIFACT.to_string());
        m_set_variable("key_jump", &KEY_JUMP.to_string());
        m_set_variable("key_arti_all", &KEY_ARTI_ALL.to_string());
        m_set_variable("key_arti_health", &KEY_ARTI_HEALTH.to_string());
        m_set_variable("key_arti_poisonbag", &KEY_ARTI_POISONBAG.to_string());
        m_set_variable("key_arti_blastradius", &KEY_ARTI_BLASTRADIUS.to_string());
        m_set_variable("key_arti_teleport", &KEY_ARTI_TELEPORT.to_string());
        m_set_variable("key_arti_teleportother", &KEY_ARTI_TELEPORTOTHER.to_string());
        m_set_variable("key_arti_egg", &KEY_ARTI_EGG.to_string());
        m_set_variable("key_arti_invulnerability", &KEY_ARTI_INVULNERABILITY.to_string());
        m_set_variable("key_usehealth", &KEY_USEHEALTH.to_string());
        m_set_variable("key_invquery", &KEY_INVQUERY.to_string());
        m_set_variable("key_mission", &KEY_MISSION.to_string());
        m_set_variable("key_invpop", &KEY_INVPOP.to_string());
        m_set_variable("key_invkey", &KEY_INVKEY.to_string());
        m_set_variable("key_invhome", &KEY_INVHOME.to_string());
        m_set_variable("key_invend", &KEY_INVEND.to_string());
        m_set_variable("key_invuse", &KEY_INVUSE.to_string());
        m_set_variable("key_invdrop", &KEY_INVDROP.to_string());
        m_set_variable("key_menu_help", &KEY_MENU_HELP.to_string());
        m_set_variable("key_menu_save", &KEY_MENU_SAVE.to_string());
        m_set_variable("key_menu_load", &KEY_MENU_LOAD.to_string());
        m_set_variable("key_menu_volume", &KEY_MENU_VOLUME.to_string());
        m_set_variable("key_menu_detail", &KEY_MENU_DETAIL.to_string());
        m_set_variable("key_menu_qsave", &KEY_MENU_QSAVE.to_string());
        m_set_variable("key_menu_endgame", &KEY_MENU_ENDGAME.to_string());
        m_set_variable("key_menu_messages", &KEY_MENU_MESSAGES.to_string());
        m_set_variable("key_menu_qload", &KEY_MENU_QLOAD.to_string());
        m_set_variable("key_menu_quit", &KEY_MENU_QUIT.to_string());
        m_set_variable("key_menu_gamma", &KEY_MENU_GAMMA.to_string());
        m_set_variable("key_menu_incscreen", &KEY_MENU_INCSCREEN.to_string());
        m_set_variable("key_menu_decscreen", &KEY_MENU_DECSCREEN.to_string());
        m_set_variable("key_menu_screenshot", &KEY_MENU_SCREENSHOT.to_string());
        for (i, v) in KEY_MULTI_MSGPLAYER.iter().enumerate() {
            m_set_variable(&format!("key_multi_msgplayer{}", i + 1), &v.to_string());
        }
    }
}
