//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 1993-2008 Raven Software
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Key bindings and control configuration.
// Original: m_controls.h + m_controls.c

use crate::doomkeys;
use crate::ui_hud::config::m_get_int_variable;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// ControlsState - thread-safe via OnceLock + Mutex
// =============================================================================

static CONTROLS_STATE: OnceLock<Mutex<ControlsState>> = OnceLock::new();

pub struct ControlsState {
    // Movement
    pub key_right: i32,
    pub key_left: i32,
    pub key_up: i32,
    pub key_down: i32,
    pub key_strafeleft: i32,
    pub key_straferight: i32,
    pub key_fire: i32,
    pub key_use: i32,
    pub key_strafe: i32,
    pub key_speed: i32,
    pub key_jump: i32,
    // Heretic
    pub key_flyup: i32,
    pub key_flydown: i32,
    pub key_flycenter: i32,
    pub key_lookup: i32,
    pub key_lookdown: i32,
    pub key_lookcenter: i32,
    pub key_invleft: i32,
    pub key_invright: i32,
    pub key_useartifact: i32,
    // Hexen artifacts
    pub key_arti_all: i32,
    pub key_arti_health: i32,
    pub key_arti_poisonbag: i32,
    pub key_arti_blastradius: i32,
    pub key_arti_teleport: i32,
    pub key_arti_teleportother: i32,
    pub key_arti_egg: i32,
    pub key_arti_invulnerability: i32,
    // Strife
    pub key_usehealth: i32,
    pub key_invquery: i32,
    pub key_mission: i32,
    pub key_invpop: i32,
    pub key_invkey: i32,
    pub key_invhome: i32,
    pub key_invend: i32,
    pub key_invuse: i32,
    pub key_invdrop: i32,
    pub key_multi_msgplayer: [i32; 8],
    pub key_message_refresh: i32,
    pub key_pause: i32,
    pub key_demo_quit: i32,
    pub key_spy: i32,
    pub key_multi_msg: i32,
    // Weapon selection
    pub key_weapon1: i32,
    pub key_weapon2: i32,
    pub key_weapon3: i32,
    pub key_weapon4: i32,
    pub key_weapon5: i32,
    pub key_weapon6: i32,
    pub key_weapon7: i32,
    pub key_weapon8: i32,
    pub key_prevweapon: i32,
    pub key_nextweapon: i32,
    // Automap
    pub key_map_north: i32,
    pub key_map_south: i32,
    pub key_map_east: i32,
    pub key_map_west: i32,
    pub key_map_zoomin: i32,
    pub key_map_zoomout: i32,
    pub key_map_toggle: i32,
    pub key_map_maxzoom: i32,
    pub key_map_follow: i32,
    pub key_map_grid: i32,
    pub key_map_mark: i32,
    pub key_map_clearmark: i32,
    // Menu
    pub key_menu_activate: i32,
    pub key_menu_up: i32,
    pub key_menu_down: i32,
    pub key_menu_left: i32,
    pub key_menu_right: i32,
    pub key_menu_back: i32,
    pub key_menu_forward: i32,
    pub key_menu_confirm: i32,
    pub key_menu_abort: i32,
    pub key_menu_help: i32,
    pub key_menu_save: i32,
    pub key_menu_load: i32,
    pub key_menu_volume: i32,
    pub key_menu_detail: i32,
    pub key_menu_qsave: i32,
    pub key_menu_endgame: i32,
    pub key_menu_messages: i32,
    pub key_menu_qload: i32,
    pub key_menu_quit: i32,
    pub key_menu_gamma: i32,
    pub key_menu_incscreen: i32,
    pub key_menu_decscreen: i32,
    pub key_menu_screenshot: i32,
    // Mouse
    pub mousebfire: i32,
    pub mousebstrafe: i32,
    pub mousebforward: i32,
    pub mousebuse: i32,
    pub mousebjump: i32,
    pub mousebstrafeleft: i32,
    pub mousebstraferight: i32,
    pub mousebbackward: i32,
    pub mousebprevweapon: i32,
    pub mousebnextweapon: i32,
    // Joystick
    pub joybfire: i32,
    pub joybstrafe: i32,
    pub joybuse: i32,
    pub joybspeed: i32,
    pub joybjump: i32,
    pub joybstrafeleft: i32,
    pub joybstraferight: i32,
    pub joybprevweapon: i32,
    pub joybnextweapon: i32,
    pub joybmenu: i32,
    pub dclick_use: i32,
}

impl Default for ControlsState {
    fn default() -> Self {
        Self {
            key_right: doomkeys::KEY_RIGHTARROW,
            key_left: doomkeys::KEY_LEFTARROW,
            key_up: doomkeys::KEY_UPARROW,
            key_down: doomkeys::KEY_DOWNARROW,
            key_strafeleft: doomkeys::KEY_STRAFE_L,
            key_straferight: doomkeys::KEY_STRAFE_R,
            key_fire: doomkeys::KEY_FIRE,
            key_use: doomkeys::KEY_USE,
            key_strafe: doomkeys::KEY_RALT,
            key_speed: doomkeys::KEY_RSHIFT,
            key_jump: b'/' as i32,
            key_flyup: doomkeys::KEY_PGUP,
            key_flydown: doomkeys::KEY_INS,
            key_flycenter: doomkeys::KEY_HOME,
            key_lookup: doomkeys::KEY_PGDN,
            key_lookdown: doomkeys::KEY_DEL,
            key_lookcenter: doomkeys::KEY_END,
            key_invleft: b'[' as i32,
            key_invright: b']' as i32,
            key_useartifact: doomkeys::KEY_ENTER,
            key_arti_all: doomkeys::KEY_BACKSPACE,
            key_arti_health: b'\\' as i32,
            key_arti_poisonbag: b'0' as i32,
            key_arti_blastradius: b'9' as i32,
            key_arti_teleport: b'8' as i32,
            key_arti_teleportother: b'7' as i32,
            key_arti_egg: b'6' as i32,
            key_arti_invulnerability: b'5' as i32,
            key_usehealth: b'h' as i32,
            key_invquery: b'q' as i32,
            key_mission: b'w' as i32,
            key_invpop: b'z' as i32,
            key_invkey: b'k' as i32,
            key_invhome: doomkeys::KEY_HOME,
            key_invend: doomkeys::KEY_END,
            key_invuse: doomkeys::KEY_ENTER,
            key_invdrop: doomkeys::KEY_BACKSPACE,
            key_multi_msgplayer: [0; 8],
            key_message_refresh: doomkeys::KEY_ENTER,
            key_pause: doomkeys::KEY_PAUSE,
            key_demo_quit: b'q' as i32,
            key_spy: doomkeys::KEY_F12,
            key_multi_msg: b't' as i32,
            key_weapon1: b'1' as i32,
            key_weapon2: b'2' as i32,
            key_weapon3: b'3' as i32,
            key_weapon4: b'4' as i32,
            key_weapon5: b'5' as i32,
            key_weapon6: b'6' as i32,
            key_weapon7: b'7' as i32,
            key_weapon8: b'8' as i32,
            key_prevweapon: 0,
            key_nextweapon: 0,
            key_map_north: doomkeys::KEY_UPARROW,
            key_map_south: doomkeys::KEY_DOWNARROW,
            key_map_east: doomkeys::KEY_RIGHTARROW,
            key_map_west: doomkeys::KEY_LEFTARROW,
            key_map_zoomin: doomkeys::KEY_EQUALS,
            key_map_zoomout: doomkeys::KEY_MINUS,
            key_map_toggle: doomkeys::KEY_TAB,
            key_map_maxzoom: b'0' as i32,
            key_map_follow: b'f' as i32,
            key_map_grid: b'g' as i32,
            key_map_mark: b'm' as i32,
            key_map_clearmark: b'c' as i32,
            key_menu_activate: doomkeys::KEY_ESCAPE,
            key_menu_up: doomkeys::KEY_UPARROW,
            key_menu_down: doomkeys::KEY_DOWNARROW,
            key_menu_left: doomkeys::KEY_LEFTARROW,
            key_menu_right: doomkeys::KEY_RIGHTARROW,
            key_menu_back: doomkeys::KEY_BACKSPACE,
            key_menu_forward: doomkeys::KEY_ENTER,
            key_menu_confirm: b'y' as i32,
            key_menu_abort: b'n' as i32,
            key_menu_help: doomkeys::KEY_F1,
            key_menu_save: doomkeys::KEY_F2,
            key_menu_load: doomkeys::KEY_F3,
            key_menu_volume: doomkeys::KEY_F4,
            key_menu_detail: doomkeys::KEY_F5,
            key_menu_qsave: doomkeys::KEY_F6,
            key_menu_endgame: doomkeys::KEY_F7,
            key_menu_messages: doomkeys::KEY_F8,
            key_menu_qload: doomkeys::KEY_F9,
            key_menu_quit: doomkeys::KEY_F10,
            key_menu_gamma: doomkeys::KEY_F11,
            key_menu_incscreen: doomkeys::KEY_EQUALS,
            key_menu_decscreen: doomkeys::KEY_MINUS,
            key_menu_screenshot: 0,
            mousebfire: 0,
            mousebstrafe: 1,
            mousebforward: 2,
            mousebuse: -1,
            mousebjump: -1,
            mousebstrafeleft: -1,
            mousebstraferight: -1,
            mousebbackward: -1,
            mousebprevweapon: -1,
            mousebnextweapon: -1,
            joybfire: 0,
            joybstrafe: 1,
            joybuse: 3,
            joybspeed: 2,
            joybjump: -1,
            joybstrafeleft: -1,
            joybstraferight: -1,
            joybprevweapon: -1,
            joybnextweapon: -1,
            joybmenu: -1,
            dclick_use: 1,
        }
    }
}

fn get_controls_state() -> &'static Mutex<ControlsState> {
    CONTROLS_STATE.get_or_init(|| Mutex::new(ControlsState::default()))
}

/// Access ControlsState.
pub fn with_controls_state<F, R>(f: F) -> R
where
    F: FnOnce(&ControlsState) -> R,
{
    let guard = get_controls_state().lock().unwrap();
    f(&guard)
}

/// Mutably access ControlsState.
pub fn with_controls_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut ControlsState) -> R,
{
    let mut guard = get_controls_state().lock().unwrap();
    f(&mut guard)
}

// =============================================================================
// Public API (from m_controls.h) — getters for external use
// =============================================================================

pub fn key_right() -> i32 {
    with_controls_state(|s| s.key_right)
}
pub fn key_left() -> i32 {
    with_controls_state(|s| s.key_left)
}
pub fn key_up() -> i32 {
    with_controls_state(|s| s.key_up)
}
pub fn key_down() -> i32 {
    with_controls_state(|s| s.key_down)
}
pub fn key_strafeleft() -> i32 {
    with_controls_state(|s| s.key_strafeleft)
}
pub fn key_straferight() -> i32 {
    with_controls_state(|s| s.key_straferight)
}
pub fn key_fire() -> i32 {
    with_controls_state(|s| s.key_fire)
}
pub fn key_use() -> i32 {
    with_controls_state(|s| s.key_use)
}
pub fn key_strafe() -> i32 {
    with_controls_state(|s| s.key_strafe)
}
pub fn key_speed() -> i32 {
    with_controls_state(|s| s.key_speed)
}
pub fn key_pause() -> i32 {
    with_controls_state(|s| s.key_pause)
}
pub fn key_menu_activate() -> i32 {
    with_controls_state(|s| s.key_menu_activate)
}
pub fn key_menu_up() -> i32 {
    with_controls_state(|s| s.key_menu_up)
}
pub fn key_menu_down() -> i32 {
    with_controls_state(|s| s.key_menu_down)
}
pub fn key_menu_left() -> i32 {
    with_controls_state(|s| s.key_menu_left)
}
pub fn key_menu_right() -> i32 {
    with_controls_state(|s| s.key_menu_right)
}
pub fn key_menu_back() -> i32 {
    with_controls_state(|s| s.key_menu_back)
}
pub fn key_menu_forward() -> i32 {
    with_controls_state(|s| s.key_menu_forward)
}
pub fn key_menu_confirm() -> i32 {
    with_controls_state(|s| s.key_menu_confirm)
}
pub fn key_menu_abort() -> i32 {
    with_controls_state(|s| s.key_menu_abort)
}
pub fn key_map_north() -> i32 {
    with_controls_state(|s| s.key_map_north)
}
pub fn key_map_south() -> i32 {
    with_controls_state(|s| s.key_map_south)
}
pub fn key_map_east() -> i32 {
    with_controls_state(|s| s.key_map_east)
}
pub fn key_map_west() -> i32 {
    with_controls_state(|s| s.key_map_west)
}
pub fn key_map_zoomin() -> i32 {
    with_controls_state(|s| s.key_map_zoomin)
}
pub fn key_map_zoomout() -> i32 {
    with_controls_state(|s| s.key_map_zoomout)
}
pub fn key_map_toggle() -> i32 {
    with_controls_state(|s| s.key_map_toggle)
}
pub fn key_map_maxzoom() -> i32 {
    with_controls_state(|s| s.key_map_maxzoom)
}
pub fn key_map_follow() -> i32 {
    with_controls_state(|s| s.key_map_follow)
}
pub fn key_map_grid() -> i32 {
    with_controls_state(|s| s.key_map_grid)
}
pub fn key_map_mark() -> i32 {
    with_controls_state(|s| s.key_map_mark)
}
pub fn key_map_clearmark() -> i32 {
    with_controls_state(|s| s.key_map_clearmark)
}

// =============================================================================
// Implementation (from m_controls.c) — M_Bind* read config -> globals
// =============================================================================

pub fn m_bind_base_controls() {
    with_controls_state_mut(|s| {
        s.key_right = m_get_int_variable("key_right");
        s.key_left = m_get_int_variable("key_left");
        s.key_up = m_get_int_variable("key_up");
        s.key_down = m_get_int_variable("key_down");
        s.key_strafeleft = m_get_int_variable("key_strafeleft");
        s.key_straferight = m_get_int_variable("key_straferight");
        s.key_fire = m_get_int_variable("key_fire");
        s.key_use = m_get_int_variable("key_use");
        s.key_strafe = m_get_int_variable("key_strafe");
        s.key_speed = m_get_int_variable("key_speed");
        s.key_pause = m_get_int_variable("key_pause");
        s.key_message_refresh = m_get_int_variable("key_message_refresh");
        s.mousebfire = m_get_int_variable("mouseb_fire");
        s.mousebstrafe = m_get_int_variable("mouseb_strafe");
        s.mousebforward = m_get_int_variable("mouseb_forward");
        s.joybfire = m_get_int_variable("joyb_fire");
        s.joybstrafe = m_get_int_variable("joyb_strafe");
        s.joybuse = m_get_int_variable("joyb_use");
        s.joybspeed = m_get_int_variable("joyb_speed");
        s.joybmenu = m_get_int_variable("joyb_menu_activate");
        s.joybstrafeleft = m_get_int_variable("joyb_strafeleft");
        s.joybstraferight = m_get_int_variable("joyb_straferight");
        s.mousebstrafeleft = m_get_int_variable("mouseb_strafeleft");
        s.mousebstraferight = m_get_int_variable("mouseb_straferight");
        s.mousebuse = m_get_int_variable("mouseb_use");
        s.mousebbackward = m_get_int_variable("mouseb_backward");
        s.dclick_use = m_get_int_variable("dclick_use");
    });
}

pub fn m_bind_heretic_controls() {
    with_controls_state_mut(|s| {
        s.key_flyup = m_get_int_variable("key_flyup");
        s.key_flydown = m_get_int_variable("key_flydown");
        s.key_flycenter = m_get_int_variable("key_flycenter");
        s.key_lookup = m_get_int_variable("key_lookup");
        s.key_lookdown = m_get_int_variable("key_lookdown");
        s.key_lookcenter = m_get_int_variable("key_lookcenter");
        s.key_invleft = m_get_int_variable("key_invleft");
        s.key_invright = m_get_int_variable("key_invright");
        s.key_useartifact = m_get_int_variable("key_useartifact");
    });
}

pub fn m_bind_hexen_controls() {
    with_controls_state_mut(|s| {
        s.key_jump = m_get_int_variable("key_jump");
        s.mousebjump = m_get_int_variable("mouseb_jump");
        s.joybjump = m_get_int_variable("joyb_jump");
        s.key_arti_all = m_get_int_variable("key_arti_all");
        s.key_arti_health = m_get_int_variable("key_arti_health");
        s.key_arti_poisonbag = m_get_int_variable("key_arti_poisonbag");
        s.key_arti_blastradius = m_get_int_variable("key_arti_blastradius");
        s.key_arti_teleport = m_get_int_variable("key_arti_teleport");
        s.key_arti_teleportother = m_get_int_variable("key_arti_teleportother");
        s.key_arti_egg = m_get_int_variable("key_arti_egg");
        s.key_arti_invulnerability = m_get_int_variable("key_arti_invulnerability");
    });
}

pub fn m_bind_strife_controls() {
    with_controls_state_mut(|s| {
        s.key_message_refresh = b'/' as i32;
        s.key_jump = b'a' as i32;
        s.key_lookup = doomkeys::KEY_PGUP;
        s.key_lookdown = doomkeys::KEY_PGDN;
        s.key_invleft = doomkeys::KEY_INS;
        s.key_invright = doomkeys::KEY_DEL;
        s.key_jump = m_get_int_variable("key_jump");
        s.key_lookup = m_get_int_variable("key_lookup");
        s.key_lookdown = m_get_int_variable("key_lookdown");
        s.key_invleft = m_get_int_variable("key_invleft");
        s.key_invright = m_get_int_variable("key_invright");
        s.key_usehealth = m_get_int_variable("key_usehealth");
        s.key_invquery = m_get_int_variable("key_invquery");
        s.key_mission = m_get_int_variable("key_mission");
        s.key_invpop = m_get_int_variable("key_invpop");
        s.key_invkey = m_get_int_variable("key_invkey");
        s.key_invhome = m_get_int_variable("key_invhome");
        s.key_invend = m_get_int_variable("key_invend");
        s.key_invuse = m_get_int_variable("key_invuse");
        s.key_invdrop = m_get_int_variable("key_invdrop");
        s.mousebjump = m_get_int_variable("mouseb_jump");
        s.joybjump = m_get_int_variable("joyb_jump");
    });
}

pub fn m_bind_weapon_controls() {
    with_controls_state_mut(|s| {
        s.key_weapon1 = m_get_int_variable("key_weapon1");
        s.key_weapon2 = m_get_int_variable("key_weapon2");
        s.key_weapon3 = m_get_int_variable("key_weapon3");
        s.key_weapon4 = m_get_int_variable("key_weapon4");
        s.key_weapon5 = m_get_int_variable("key_weapon5");
        s.key_weapon6 = m_get_int_variable("key_weapon6");
        s.key_weapon7 = m_get_int_variable("key_weapon7");
        s.key_weapon8 = m_get_int_variable("key_weapon8");
        s.key_prevweapon = m_get_int_variable("key_prevweapon");
        s.key_nextweapon = m_get_int_variable("key_nextweapon");
        s.mousebprevweapon = m_get_int_variable("mouseb_prevweapon");
        s.mousebnextweapon = m_get_int_variable("mouseb_nextweapon");
        s.joybprevweapon = m_get_int_variable("joyb_prevweapon");
        s.joybnextweapon = m_get_int_variable("joyb_nextweapon");
    });
}

pub fn m_bind_map_controls() {
    with_controls_state_mut(|s| {
        s.key_map_north = m_get_int_variable("key_map_north");
        s.key_map_south = m_get_int_variable("key_map_south");
        s.key_map_east = m_get_int_variable("key_map_east");
        s.key_map_west = m_get_int_variable("key_map_west");
        s.key_map_zoomin = m_get_int_variable("key_map_zoomin");
        s.key_map_zoomout = m_get_int_variable("key_map_zoomout");
        s.key_map_toggle = m_get_int_variable("key_map_toggle");
        s.key_map_maxzoom = m_get_int_variable("key_map_maxzoom");
        s.key_map_follow = m_get_int_variable("key_map_follow");
        s.key_map_grid = m_get_int_variable("key_map_grid");
        s.key_map_mark = m_get_int_variable("key_map_mark");
        s.key_map_clearmark = m_get_int_variable("key_map_clearmark");
    });
}

pub fn m_bind_menu_controls() {
    with_controls_state_mut(|s| {
        s.key_menu_activate = m_get_int_variable("key_menu_activate");
        s.key_menu_up = m_get_int_variable("key_menu_up");
        s.key_menu_down = m_get_int_variable("key_menu_down");
        s.key_menu_left = m_get_int_variable("key_menu_left");
        s.key_menu_right = m_get_int_variable("key_menu_right");
        s.key_menu_back = m_get_int_variable("key_menu_back");
        s.key_menu_forward = m_get_int_variable("key_menu_forward");
        s.key_menu_confirm = m_get_int_variable("key_menu_confirm");
        s.key_menu_abort = m_get_int_variable("key_menu_abort");
        s.key_menu_help = m_get_int_variable("key_menu_help");
        s.key_menu_save = m_get_int_variable("key_menu_save");
        s.key_menu_load = m_get_int_variable("key_menu_load");
        s.key_menu_volume = m_get_int_variable("key_menu_volume");
        s.key_menu_detail = m_get_int_variable("key_menu_detail");
        s.key_menu_qsave = m_get_int_variable("key_menu_qsave");
        s.key_menu_endgame = m_get_int_variable("key_menu_endgame");
        s.key_menu_messages = m_get_int_variable("key_menu_messages");
        s.key_menu_qload = m_get_int_variable("key_menu_qload");
        s.key_menu_quit = m_get_int_variable("key_menu_quit");
        s.key_menu_gamma = m_get_int_variable("key_menu_gamma");
        s.key_menu_incscreen = m_get_int_variable("key_menu_incscreen");
        s.key_menu_decscreen = m_get_int_variable("key_menu_decscreen");
        s.key_menu_screenshot = m_get_int_variable("key_menu_screenshot");
        s.key_demo_quit = m_get_int_variable("key_demo_quit");
        s.key_spy = m_get_int_variable("key_spy");
    });
}

pub fn m_bind_chat_controls(num_players: u32) {
    with_controls_state_mut(|s| {
        s.key_multi_msg = m_get_int_variable("key_multi_msg");
        for i in 0..num_players.min(8) {
            s.key_multi_msgplayer[i as usize] =
                m_get_int_variable(&format!("key_multi_msgplayer{}", i + 1));
        }
    });
}

pub fn m_apply_platform_defaults() {
    // Platform-specific defaults — no-op for generic
}

/// Update a single control global from config. Called by m_set_variable when setting int vars.
/// Implements M_BindVariable-style pointer binding: config changes propagate to controls.
pub fn m_update_control_from_config(name: &str, value: i32) {
    let name = name.to_lowercase();
    with_controls_state_mut(|s| {
        match name.as_str() {
            "key_right" => s.key_right = value,
            "key_left" => s.key_left = value,
            "key_up" => s.key_up = value,
            "key_down" => s.key_down = value,
            "key_strafeleft" => s.key_strafeleft = value,
            "key_straferight" => s.key_straferight = value,
            "key_fire" => s.key_fire = value,
            "key_use" => s.key_use = value,
            "key_strafe" => s.key_strafe = value,
            "key_speed" => s.key_speed = value,
            "key_pause" => s.key_pause = value,
            "key_message_refresh" => s.key_message_refresh = value,
            "key_demo_quit" => s.key_demo_quit = value,
            "key_spy" => s.key_spy = value,
            "key_multi_msg" => s.key_multi_msg = value,
            "key_weapon1" => s.key_weapon1 = value,
            "key_weapon2" => s.key_weapon2 = value,
            "key_weapon3" => s.key_weapon3 = value,
            "key_weapon4" => s.key_weapon4 = value,
            "key_weapon5" => s.key_weapon5 = value,
            "key_weapon6" => s.key_weapon6 = value,
            "key_weapon7" => s.key_weapon7 = value,
            "key_weapon8" => s.key_weapon8 = value,
            "key_prevweapon" => s.key_prevweapon = value,
            "key_nextweapon" => s.key_nextweapon = value,
            "key_menu_activate" => s.key_menu_activate = value,
            "key_menu_up" => s.key_menu_up = value,
            "key_menu_down" => s.key_menu_down = value,
            "key_menu_left" => s.key_menu_left = value,
            "key_menu_right" => s.key_menu_right = value,
            "key_menu_back" => s.key_menu_back = value,
            "key_menu_forward" => s.key_menu_forward = value,
            "key_menu_confirm" => s.key_menu_confirm = value,
            "key_menu_abort" => s.key_menu_abort = value,
            "mouseb_fire" => s.mousebfire = value,
            "mouseb_strafe" => s.mousebstrafe = value,
            "mouseb_forward" => s.mousebforward = value,
            "joyb_fire" => s.joybfire = value,
            "joyb_strafe" => s.joybstrafe = value,
            "joyb_use" => s.joybuse = value,
            "joyb_speed" => s.joybspeed = value,
            "key_map_north" => s.key_map_north = value,
            "key_map_south" => s.key_map_south = value,
            "key_map_east" => s.key_map_east = value,
            "key_map_west" => s.key_map_west = value,
            "key_map_zoomin" => s.key_map_zoomin = value,
            "key_map_zoomout" => s.key_map_zoomout = value,
            "key_map_toggle" => s.key_map_toggle = value,
            "key_map_maxzoom" => s.key_map_maxzoom = value,
            "key_map_follow" => s.key_map_follow = value,
            "key_map_grid" => s.key_map_grid = value,
            "key_map_mark" => s.key_map_mark = value,
            "key_map_clearmark" => s.key_map_clearmark = value,
            "mouseb_prevweapon" => s.mousebprevweapon = value,
            "mouseb_nextweapon" => s.mousebnextweapon = value,
            "joyb_prevweapon" => s.joybprevweapon = value,
            "joyb_nextweapon" => s.joybnextweapon = value,
            "joyb_menu_activate" => s.joybmenu = value,
            "joyb_strafeleft" => s.joybstrafeleft = value,
            "joyb_straferight" => s.joybstraferight = value,
            "mouseb_strafeleft" => s.mousebstrafeleft = value,
            "mouseb_straferight" => s.mousebstraferight = value,
            "mouseb_use" => s.mousebuse = value,
            "mouseb_backward" => s.mousebbackward = value,
            "mouseb_jump" => s.mousebjump = value,
            "joyb_jump" => s.joybjump = value,
            "dclick_use" => s.dclick_use = value,
            "key_flyup" => s.key_flyup = value,
            "key_flydown" => s.key_flydown = value,
            "key_flycenter" => s.key_flycenter = value,
            "key_lookup" => s.key_lookup = value,
            "key_lookdown" => s.key_lookdown = value,
            "key_lookcenter" => s.key_lookcenter = value,
            "key_invleft" => s.key_invleft = value,
            "key_invright" => s.key_invright = value,
            "key_useartifact" => s.key_useartifact = value,
            "key_arti_all" => s.key_arti_all = value,
            "key_arti_health" => s.key_arti_health = value,
            "key_arti_poisonbag" => s.key_arti_poisonbag = value,
            "key_arti_blastradius" => s.key_arti_blastradius = value,
            "key_arti_teleport" => s.key_arti_teleport = value,
            "key_arti_teleportother" => s.key_arti_teleportother = value,
            "key_arti_egg" => s.key_arti_egg = value,
            "key_arti_invulnerability" => s.key_arti_invulnerability = value,
            "key_usehealth" => s.key_usehealth = value,
            "key_invquery" => s.key_invquery = value,
            "key_mission" => s.key_mission = value,
            "key_invpop" => s.key_invpop = value,
            "key_invkey" => s.key_invkey = value,
            "key_invhome" => s.key_invhome = value,
            "key_invend" => s.key_invend = value,
            "key_invuse" => s.key_invuse = value,
            "key_invdrop" => s.key_invdrop = value,
            "key_menu_help" => s.key_menu_help = value,
            "key_menu_save" => s.key_menu_save = value,
            "key_menu_load" => s.key_menu_load = value,
            "key_menu_volume" => s.key_menu_volume = value,
            "key_menu_detail" => s.key_menu_detail = value,
            "key_menu_qsave" => s.key_menu_qsave = value,
            "key_menu_endgame" => s.key_menu_endgame = value,
            "key_menu_messages" => s.key_menu_messages = value,
            "key_menu_qload" => s.key_menu_qload = value,
            "key_menu_quit" => s.key_menu_quit = value,
            "key_menu_gamma" => s.key_menu_gamma = value,
            "key_menu_incscreen" => s.key_menu_incscreen = value,
            "key_menu_decscreen" => s.key_menu_decscreen = value,
            "key_menu_screenshot" => s.key_menu_screenshot = value,
            n => {
                if n.starts_with("key_multi_msgplayer") {
                    if let Ok(idx) = n["key_multi_msgplayer".len()..].parse::<usize>() {
                        if (1..=8).contains(&idx) {
                            s.key_multi_msgplayer[idx - 1] = value;
                        }
                    }
                }
            }
        }
    });
}

/// Sync current key/mouse/joy values to config. Call before M_SaveDefaults.
pub fn m_sync_controls_to_config() {
    use crate::ui_hud::config::m_set_variable;
    with_controls_state(|s| {
        m_set_variable("key_right", &s.key_right.to_string());
        m_set_variable("key_left", &s.key_left.to_string());
        m_set_variable("key_up", &s.key_up.to_string());
        m_set_variable("key_down", &s.key_down.to_string());
        m_set_variable("key_strafeleft", &s.key_strafeleft.to_string());
        m_set_variable("key_straferight", &s.key_straferight.to_string());
        m_set_variable("key_fire", &s.key_fire.to_string());
        m_set_variable("key_use", &s.key_use.to_string());
        m_set_variable("key_strafe", &s.key_strafe.to_string());
        m_set_variable("key_speed", &s.key_speed.to_string());
        m_set_variable("key_pause", &s.key_pause.to_string());
        m_set_variable("key_message_refresh", &s.key_message_refresh.to_string());
        m_set_variable("key_demo_quit", &s.key_demo_quit.to_string());
        m_set_variable("key_spy", &s.key_spy.to_string());
        m_set_variable("key_multi_msg", &s.key_multi_msg.to_string());
        m_set_variable("key_weapon1", &s.key_weapon1.to_string());
        m_set_variable("key_weapon2", &s.key_weapon2.to_string());
        m_set_variable("key_weapon3", &s.key_weapon3.to_string());
        m_set_variable("key_weapon4", &s.key_weapon4.to_string());
        m_set_variable("key_weapon5", &s.key_weapon5.to_string());
        m_set_variable("key_weapon6", &s.key_weapon6.to_string());
        m_set_variable("key_weapon7", &s.key_weapon7.to_string());
        m_set_variable("key_weapon8", &s.key_weapon8.to_string());
        m_set_variable("key_prevweapon", &s.key_prevweapon.to_string());
        m_set_variable("key_nextweapon", &s.key_nextweapon.to_string());
        m_set_variable("key_menu_activate", &s.key_menu_activate.to_string());
        m_set_variable("key_menu_up", &s.key_menu_up.to_string());
        m_set_variable("key_menu_down", &s.key_menu_down.to_string());
        m_set_variable("key_menu_left", &s.key_menu_left.to_string());
        m_set_variable("key_menu_right", &s.key_menu_right.to_string());
        m_set_variable("key_menu_back", &s.key_menu_back.to_string());
        m_set_variable("key_menu_forward", &s.key_menu_forward.to_string());
        m_set_variable("key_menu_confirm", &s.key_menu_confirm.to_string());
        m_set_variable("key_menu_abort", &s.key_menu_abort.to_string());
        m_set_variable("mouseb_fire", &s.mousebfire.to_string());
        m_set_variable("mouseb_strafe", &s.mousebstrafe.to_string());
        m_set_variable("mouseb_forward", &s.mousebforward.to_string());
        m_set_variable("joyb_fire", &s.joybfire.to_string());
        m_set_variable("joyb_strafe", &s.joybstrafe.to_string());
        m_set_variable("joyb_use", &s.joybuse.to_string());
        m_set_variable("joyb_speed", &s.joybspeed.to_string());
        m_set_variable("key_map_north", &s.key_map_north.to_string());
        m_set_variable("key_map_south", &s.key_map_south.to_string());
        m_set_variable("key_map_east", &s.key_map_east.to_string());
        m_set_variable("key_map_west", &s.key_map_west.to_string());
        m_set_variable("key_map_zoomin", &s.key_map_zoomin.to_string());
        m_set_variable("key_map_zoomout", &s.key_map_zoomout.to_string());
        m_set_variable("key_map_toggle", &s.key_map_toggle.to_string());
        m_set_variable("key_map_maxzoom", &s.key_map_maxzoom.to_string());
        m_set_variable("key_map_follow", &s.key_map_follow.to_string());
        m_set_variable("key_map_grid", &s.key_map_grid.to_string());
        m_set_variable("key_map_mark", &s.key_map_mark.to_string());
        m_set_variable("key_map_clearmark", &s.key_map_clearmark.to_string());
        m_set_variable("joyb_menu_activate", &s.joybmenu.to_string());
        m_set_variable("joyb_strafeleft", &s.joybstrafeleft.to_string());
        m_set_variable("joyb_straferight", &s.joybstraferight.to_string());
        m_set_variable("mouseb_strafeleft", &s.mousebstrafeleft.to_string());
        m_set_variable("mouseb_straferight", &s.mousebstraferight.to_string());
        m_set_variable("mouseb_use", &s.mousebuse.to_string());
        m_set_variable("mouseb_backward", &s.mousebbackward.to_string());
        m_set_variable("mouseb_jump", &s.mousebjump.to_string());
        m_set_variable("joyb_jump", &s.joybjump.to_string());
        m_set_variable("dclick_use", &s.dclick_use.to_string());
        m_set_variable("key_flyup", &s.key_flyup.to_string());
        m_set_variable("key_flydown", &s.key_flydown.to_string());
        m_set_variable("key_flycenter", &s.key_flycenter.to_string());
        m_set_variable("key_lookup", &s.key_lookup.to_string());
        m_set_variable("key_lookdown", &s.key_lookdown.to_string());
        m_set_variable("key_lookcenter", &s.key_lookcenter.to_string());
        m_set_variable("key_invleft", &s.key_invleft.to_string());
        m_set_variable("key_invright", &s.key_invright.to_string());
        m_set_variable("key_useartifact", &s.key_useartifact.to_string());
        m_set_variable("key_jump", &s.key_jump.to_string());
        m_set_variable("key_arti_all", &s.key_arti_all.to_string());
        m_set_variable("key_arti_health", &s.key_arti_health.to_string());
        m_set_variable("key_arti_poisonbag", &s.key_arti_poisonbag.to_string());
        m_set_variable("key_arti_blastradius", &s.key_arti_blastradius.to_string());
        m_set_variable("key_arti_teleport", &s.key_arti_teleport.to_string());
        m_set_variable("key_arti_teleportother", &s.key_arti_teleportother.to_string());
        m_set_variable("key_arti_egg", &s.key_arti_egg.to_string());
        m_set_variable("key_arti_invulnerability", &s.key_arti_invulnerability.to_string());
        m_set_variable("key_usehealth", &s.key_usehealth.to_string());
        m_set_variable("key_invquery", &s.key_invquery.to_string());
        m_set_variable("key_mission", &s.key_mission.to_string());
        m_set_variable("key_invpop", &s.key_invpop.to_string());
        m_set_variable("key_invkey", &s.key_invkey.to_string());
        m_set_variable("key_invhome", &s.key_invhome.to_string());
        m_set_variable("key_invend", &s.key_invend.to_string());
        m_set_variable("key_invuse", &s.key_invuse.to_string());
        m_set_variable("key_invdrop", &s.key_invdrop.to_string());
        m_set_variable("key_menu_help", &s.key_menu_help.to_string());
        m_set_variable("key_menu_save", &s.key_menu_save.to_string());
        m_set_variable("key_menu_load", &s.key_menu_load.to_string());
        m_set_variable("key_menu_volume", &s.key_menu_volume.to_string());
        m_set_variable("key_menu_detail", &s.key_menu_detail.to_string());
        m_set_variable("key_menu_qsave", &s.key_menu_qsave.to_string());
        m_set_variable("key_menu_endgame", &s.key_menu_endgame.to_string());
        m_set_variable("key_menu_messages", &s.key_menu_messages.to_string());
        m_set_variable("key_menu_qload", &s.key_menu_qload.to_string());
        m_set_variable("key_menu_quit", &s.key_menu_quit.to_string());
        m_set_variable("key_menu_gamma", &s.key_menu_gamma.to_string());
        m_set_variable("key_menu_incscreen", &s.key_menu_incscreen.to_string());
        m_set_variable("key_menu_decscreen", &s.key_menu_decscreen.to_string());
        m_set_variable("key_menu_screenshot", &s.key_menu_screenshot.to_string());
        for (i, v) in s.key_multi_msgplayer.iter().enumerate() {
            m_set_variable(&format!("key_multi_msgplayer{}", i + 1), &v.to_string());
        }
    });
}
