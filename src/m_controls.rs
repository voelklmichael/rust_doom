//! Key bindings (m_controls.h, m_controls.c)
//! Original: m_controls.h, m_controls.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct M_ControlsState {
    pub key_right: Arc<Mutex<i32>>,
    pub key_left: Arc<Mutex<i32>>,
    pub key_up: Arc<Mutex<i32>>,
    pub key_down: Arc<Mutex<i32>>,
    pub key_strafeleft: Arc<Mutex<i32>>,
    pub key_straferight: Arc<Mutex<i32>>,
    pub key_fire: Arc<Mutex<i32>>,
    pub key_use: Arc<Mutex<i32>>,
    pub key_strafe: Arc<Mutex<i32>>,
    pub key_speed: Arc<Mutex<i32>>,
    pub key_jump: Arc<Mutex<i32>>,
    pub key_flyup: Arc<Mutex<i32>>,
    pub key_flydown: Arc<Mutex<i32>>,
    pub key_flycenter: Arc<Mutex<i32>>,
    pub key_lookup: Arc<Mutex<i32>>,
    pub key_lookdown: Arc<Mutex<i32>>,
    pub key_lookcenter: Arc<Mutex<i32>>,
    pub key_invleft: Arc<Mutex<i32>>,
    pub key_invright: Arc<Mutex<i32>>,
    pub key_useartifact: Arc<Mutex<i32>>,
    pub key_message_refresh: Arc<Mutex<i32>>,
    pub key_pause: Arc<Mutex<i32>>,
    pub key_weapon1: Arc<Mutex<i32>>,
    pub key_weapon2: Arc<Mutex<i32>>,
    pub key_weapon3: Arc<Mutex<i32>>,
    pub key_weapon4: Arc<Mutex<i32>>,
    pub key_weapon5: Arc<Mutex<i32>>,
    pub key_weapon6: Arc<Mutex<i32>>,
    pub key_weapon7: Arc<Mutex<i32>>,
    pub key_weapon8: Arc<Mutex<i32>>,
    pub key_demo_quit: Arc<Mutex<i32>>,
    pub key_spy: Arc<Mutex<i32>>,
    pub key_prevweapon: Arc<Mutex<i32>>,
    pub key_nextweapon: Arc<Mutex<i32>>,
    pub key_map_north: Arc<Mutex<i32>>,
    pub key_map_south: Arc<Mutex<i32>>,
    pub key_map_east: Arc<Mutex<i32>>,
    pub key_map_west: Arc<Mutex<i32>>,
    pub key_map_zoomin: Arc<Mutex<i32>>,
    pub key_map_zoomout: Arc<Mutex<i32>>,
    pub key_map_toggle: Arc<Mutex<i32>>,
    pub key_map_maxzoom: Arc<Mutex<i32>>,
    pub key_map_follow: Arc<Mutex<i32>>,
    pub key_map_grid: Arc<Mutex<i32>>,
    pub key_map_mark: Arc<Mutex<i32>>,
    pub key_map_clearmark: Arc<Mutex<i32>>,
    pub key_menu_activate: Arc<Mutex<i32>>,
    pub key_menu_up: Arc<Mutex<i32>>,
    pub key_menu_down: Arc<Mutex<i32>>,
    pub key_menu_left: Arc<Mutex<i32>>,
    pub key_menu_right: Arc<Mutex<i32>>,
    pub key_menu_back: Arc<Mutex<i32>>,
    pub key_menu_forward: Arc<Mutex<i32>>,
    pub key_menu_confirm: Arc<Mutex<i32>>,
    pub key_menu_abort: Arc<Mutex<i32>>,
    pub key_menu_help: Arc<Mutex<i32>>,
    pub key_menu_save: Arc<Mutex<i32>>,
    pub key_menu_load: Arc<Mutex<i32>>,
    pub key_menu_volume: Arc<Mutex<i32>>,
    pub key_menu_detail: Arc<Mutex<i32>>,
    pub key_menu_qsave: Arc<Mutex<i32>>,
    pub key_menu_endgame: Arc<Mutex<i32>>,
    pub key_menu_messages: Arc<Mutex<i32>>,
    pub key_menu_qload: Arc<Mutex<i32>>,
    pub key_menu_quit: Arc<Mutex<i32>>,
    pub key_menu_gamma: Arc<Mutex<i32>>,
    pub key_menu_incscreen: Arc<Mutex<i32>>,
    pub key_menu_decscreen: Arc<Mutex<i32>>,
    pub key_menu_screenshot: Arc<Mutex<i32>>,
    pub mousebfire: Arc<Mutex<i32>>,
    pub mousebstrafe: Arc<Mutex<i32>>,
    pub mousebforward: Arc<Mutex<i32>>,
    pub mousebjump: Arc<Mutex<i32>>,
    pub mousebstrafeleft: Arc<Mutex<i32>>,
    pub mousebstraferight: Arc<Mutex<i32>>,
    pub mousebbackward: Arc<Mutex<i32>>,
    pub mousebuse: Arc<Mutex<i32>>,
    pub mousebprevweapon: Arc<Mutex<i32>>,
    pub mousebnextweapon: Arc<Mutex<i32>>,
    pub joybfire: Arc<Mutex<i32>>,
    pub joybstrafe: Arc<Mutex<i32>>,
    pub joybuse: Arc<Mutex<i32>>,
    pub joybspeed: Arc<Mutex<i32>>,
    pub joybjump: Arc<Mutex<i32>>,
    pub joybstrafeleft: Arc<Mutex<i32>>,
    pub joybstraferight: Arc<Mutex<i32>>,
    pub joybprevweapon: Arc<Mutex<i32>>,
    pub joybnextweapon: Arc<Mutex<i32>>,
    pub joybmenu: Arc<Mutex<i32>>,
    pub dclick_use: Arc<Mutex<i32>>,
}

impl M_ControlsState {
    pub fn m_bind_base_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_heretic_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_hexen_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_strife_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_weapon_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_map_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_menu_controls(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_bind_chat_controls(&self, _num_players: u32) {
        todo!("Basic stage-0 stub")
    }
    pub fn m_apply_platform_defaults(&self) {
        todo!("Basic stage-0 stub")
    }
}
