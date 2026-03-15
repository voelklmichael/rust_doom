//! Rust translation of doomgeneric/m_controls.h
//! Key and control bindings.

pub static mut key_right: i32 = 0;
pub static mut key_left: i32 = 0;
pub static mut key_up: i32 = 0;
pub static mut key_down: i32 = 0;
pub static mut key_strafeleft: i32 = 0;
pub static mut key_straferight: i32 = 0;
pub static mut key_fire: i32 = 0;
pub static mut key_use: i32 = 0;
pub static mut key_strafe: i32 = 0;
pub static mut key_speed: i32 = 0;
pub static mut key_jump: i32 = 0;
pub static mut key_flyup: i32 = 0;
pub static mut key_flydown: i32 = 0;
pub static mut key_flycenter: i32 = 0;
pub static mut key_lookup: i32 = 0;
pub static mut key_lookdown: i32 = 0;
pub static mut key_lookcenter: i32 = 0;
pub static mut key_invleft: i32 = 0;
pub static mut key_invright: i32 = 0;
pub static mut key_useartifact: i32 = 0;
pub static mut key_usehealth: i32 = 0;
pub static mut key_invquery: i32 = 0;
pub static mut key_mission: i32 = 0;
pub static mut key_invpop: i32 = 0;
pub static mut key_invkey: i32 = 0;
pub static mut key_invhome: i32 = 0;
pub static mut key_invend: i32 = 0;
pub static mut key_invuse: i32 = 0;
pub static mut key_invdrop: i32 = 0;
pub static mut key_message_refresh: i32 = 0;
pub static mut key_pause: i32 = 0;
pub static mut key_multi_msg: i32 = 0;
pub static mut key_multi_msgplayer: [i32; 8] = [0; 8];
pub static mut key_weapon1: i32 = 0;
pub static mut key_weapon2: i32 = 0;
pub static mut key_weapon3: i32 = 0;
pub static mut key_weapon4: i32 = 0;
pub static mut key_weapon5: i32 = 0;
pub static mut key_weapon6: i32 = 0;
pub static mut key_weapon7: i32 = 0;
pub static mut key_weapon8: i32 = 0;
pub static mut key_arti_all: i32 = 0;
pub static mut key_arti_health: i32 = 0;
pub static mut key_arti_poisonbag: i32 = 0;
pub static mut key_arti_blastradius: i32 = 0;
pub static mut key_arti_teleport: i32 = 0;
pub static mut key_arti_teleportother: i32 = 0;
pub static mut key_arti_egg: i32 = 0;
pub static mut key_arti_invulnerability: i32 = 0;
pub static mut key_demo_quit: i32 = 0;
pub static mut key_spy: i32 = 0;
pub static mut key_prevweapon: i32 = 0;
pub static mut key_nextweapon: i32 = 0;
pub static mut key_map_north: i32 = 0;
pub static mut key_map_south: i32 = 0;
pub static mut key_map_east: i32 = 0;
pub static mut key_map_west: i32 = 0;
pub static mut key_map_zoomin: i32 = 0;
pub static mut key_map_zoomout: i32 = 0;
pub static mut key_map_toggle: i32 = 0;
pub static mut key_map_maxzoom: i32 = 0;
pub static mut key_map_follow: i32 = 0;
pub static mut key_map_grid: i32 = 0;
pub static mut key_map_mark: i32 = 0;
pub static mut key_map_clearmark: i32 = 0;
pub static mut key_menu_activate: i32 = 0;
pub static mut key_menu_up: i32 = 0;
pub static mut key_menu_down: i32 = 0;
pub static mut key_menu_left: i32 = 0;
pub static mut key_menu_right: i32 = 0;
pub static mut key_menu_back: i32 = 0;
pub static mut key_menu_forward: i32 = 0;
pub static mut key_menu_confirm: i32 = 0;
pub static mut key_menu_abort: i32 = 0;
pub static mut key_menu_help: i32 = 0;
pub static mut key_menu_save: i32 = 0;
pub static mut key_menu_load: i32 = 0;
pub static mut key_menu_volume: i32 = 0;
pub static mut key_menu_detail: i32 = 0;
pub static mut key_menu_qsave: i32 = 0;
pub static mut key_menu_endgame: i32 = 0;
pub static mut key_menu_messages: i32 = 0;
pub static mut key_menu_qload: i32 = 0;
pub static mut key_menu_quit: i32 = 0;
pub static mut key_menu_gamma: i32 = 0;
pub static mut key_menu_incscreen: i32 = 0;
pub static mut key_menu_decscreen: i32 = 0;
pub static mut key_menu_screenshot: i32 = 0;
pub static mut mousebfire: i32 = 0;
pub static mut mousebstrafe: i32 = 0;
pub static mut mousebforward: i32 = 0;
pub static mut mousebjump: i32 = 0;
pub static mut mousebstrafeleft: i32 = 0;
pub static mut mousebstraferight: i32 = 0;
pub static mut mousebbackward: i32 = 0;
pub static mut mousebuse: i32 = 0;
pub static mut mousebprevweapon: i32 = 0;
pub static mut mousebnextweapon: i32 = 0;
pub static mut joybfire: i32 = 0;
pub static mut joybstrafe: i32 = 0;
pub static mut joybuse: i32 = 0;
pub static mut joybspeed: i32 = 0;
pub static mut joybjump: i32 = 0;
pub static mut joybstrafeleft: i32 = 0;
pub static mut joybstraferight: i32 = 0;
pub static mut joybprevweapon: i32 = 0;
pub static mut joybnextweapon: i32 = 0;
pub static mut joybmenu: i32 = 0;
pub static mut dclick_use: i32 = 0;

/// C function: M_BindBaseControls
pub fn m_bind_base_controls() {
    todo!("original: M_BindBaseControls")
}

/// C function: M_BindHereticControls
pub fn m_bind_heretic_controls() {
    todo!("original: M_BindHereticControls")
}

/// C function: M_BindHexenControls
pub fn m_bind_hexen_controls() {
    todo!("original: M_BindHexenControls")
}

/// C function: M_BindStrifeControls
pub fn m_bind_strife_controls() {
    todo!("original: M_BindStrifeControls")
}

/// C function: M_BindWeaponControls
pub fn m_bind_weapon_controls() {
    todo!("original: M_BindWeaponControls")
}

/// C function: M_BindMapControls
pub fn m_bind_map_controls() {
    todo!("original: M_BindMapControls")
}

/// C function: M_BindMenuControls
pub fn m_bind_menu_controls() {
    todo!("original: M_BindMenuControls")
}

/// C function: M_BindChatControls
pub fn m_bind_chat_controls(num_players: u32) {
    todo!("original: M_BindChatControls")
}

/// C function: M_ApplyPlatformDefaults
pub fn m_apply_platform_defaults() {
    todo!("original: M_ApplyPlatformDefaults")
}
