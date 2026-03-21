use std::cell::RefCell;

/// Original: m_controls.h — key / mouse / joystick bindings
#[allow(non_camel_case_types)]
pub struct M_ControlsState {
    /// Original: key_right
    pub key_right: RefCell<i32>,
    /// Original: key_left
    pub key_left: RefCell<i32>,
    /// Original: key_up
    pub key_up: RefCell<i32>,
    /// Original: key_down
    pub key_down: RefCell<i32>,
    /// Original: key_strafeleft
    pub key_strafeleft: RefCell<i32>,
    /// Original: key_straferight
    pub key_straferight: RefCell<i32>,
    /// Original: key_fire
    pub key_fire: RefCell<i32>,
    /// Original: key_use
    pub key_use: RefCell<i32>,
    /// Original: key_strafe
    pub key_strafe: RefCell<i32>,
    /// Original: key_speed
    pub key_speed: RefCell<i32>,
    /// Original: key_jump
    pub key_jump: RefCell<i32>,
    /// Original: key_flyup
    pub key_flyup: RefCell<i32>,
    /// Original: key_flydown
    pub key_flydown: RefCell<i32>,
    /// Original: key_flycenter
    pub key_flycenter: RefCell<i32>,
    /// Original: key_lookup
    pub key_lookup: RefCell<i32>,
    /// Original: key_lookdown
    pub key_lookdown: RefCell<i32>,
    /// Original: key_lookcenter
    pub key_lookcenter: RefCell<i32>,
    /// Original: key_invleft
    pub key_invleft: RefCell<i32>,
    /// Original: key_invright
    pub key_invright: RefCell<i32>,
    /// Original: key_useartifact
    pub key_useartifact: RefCell<i32>,
    /// Original: key_usehealth
    pub key_usehealth: RefCell<i32>,
    /// Original: key_invquery
    pub key_invquery: RefCell<i32>,
    /// Original: key_mission
    pub key_mission: RefCell<i32>,
    /// Original: key_invpop
    pub key_invpop: RefCell<i32>,
    /// Original: key_invkey
    pub key_invkey: RefCell<i32>,
    /// Original: key_invhome
    pub key_invhome: RefCell<i32>,
    /// Original: key_invend
    pub key_invend: RefCell<i32>,
    /// Original: key_invuse
    pub key_invuse: RefCell<i32>,
    /// Original: key_invdrop
    pub key_invdrop: RefCell<i32>,
    /// Original: key_message_refresh
    pub key_message_refresh: RefCell<i32>,
    /// Original: key_pause
    pub key_pause: RefCell<i32>,
    /// Original: key_multi_msg
    pub key_multi_msg: RefCell<i32>,
    /// Original: key_multi_msgplayer[8]
    pub key_multi_msgplayer: RefCell<[i32; 8]>,
    /// Original: key_weapon1
    pub key_weapon1: RefCell<i32>,
    /// Original: key_weapon2
    pub key_weapon2: RefCell<i32>,
    /// Original: key_weapon3
    pub key_weapon3: RefCell<i32>,
    /// Original: key_weapon4
    pub key_weapon4: RefCell<i32>,
    /// Original: key_weapon5
    pub key_weapon5: RefCell<i32>,
    /// Original: key_weapon6
    pub key_weapon6: RefCell<i32>,
    /// Original: key_weapon7
    pub key_weapon7: RefCell<i32>,
    /// Original: key_weapon8
    pub key_weapon8: RefCell<i32>,
    /// Original: key_arti_all
    pub key_arti_all: RefCell<i32>,
    /// Original: key_arti_health
    pub key_arti_health: RefCell<i32>,
    /// Original: key_arti_poisonbag
    pub key_arti_poisonbag: RefCell<i32>,
    /// Original: key_arti_blastradius
    pub key_arti_blastradius: RefCell<i32>,
    /// Original: key_arti_teleport
    pub key_arti_teleport: RefCell<i32>,
    /// Original: key_arti_teleportother
    pub key_arti_teleportother: RefCell<i32>,
    /// Original: key_arti_egg
    pub key_arti_egg: RefCell<i32>,
    /// Original: key_arti_invulnerability
    pub key_arti_invulnerability: RefCell<i32>,
    /// Original: key_demo_quit
    pub key_demo_quit: RefCell<i32>,
    /// Original: key_spy
    pub key_spy: RefCell<i32>,
    /// Original: key_prevweapon
    pub key_prevweapon: RefCell<i32>,
    /// Original: key_nextweapon
    pub key_nextweapon: RefCell<i32>,
    /// Original: key_map_north
    pub key_map_north: RefCell<i32>,
    /// Original: key_map_south
    pub key_map_south: RefCell<i32>,
    /// Original: key_map_east
    pub key_map_east: RefCell<i32>,
    /// Original: key_map_west
    pub key_map_west: RefCell<i32>,
    /// Original: key_map_zoomin
    pub key_map_zoomin: RefCell<i32>,
    /// Original: key_map_zoomout
    pub key_map_zoomout: RefCell<i32>,
    /// Original: key_map_toggle
    pub key_map_toggle: RefCell<i32>,
    /// Original: key_map_maxzoom
    pub key_map_maxzoom: RefCell<i32>,
    /// Original: key_map_follow
    pub key_map_follow: RefCell<i32>,
    /// Original: key_map_grid
    pub key_map_grid: RefCell<i32>,
    /// Original: key_map_mark
    pub key_map_mark: RefCell<i32>,
    /// Original: key_map_clearmark
    pub key_map_clearmark: RefCell<i32>,
    /// Original: key_menu_activate
    pub key_menu_activate: RefCell<i32>,
    /// Original: key_menu_up
    pub key_menu_up: RefCell<i32>,
    /// Original: key_menu_down
    pub key_menu_down: RefCell<i32>,
    /// Original: key_menu_left
    pub key_menu_left: RefCell<i32>,
    /// Original: key_menu_right
    pub key_menu_right: RefCell<i32>,
    /// Original: key_menu_back
    pub key_menu_back: RefCell<i32>,
    /// Original: key_menu_forward
    pub key_menu_forward: RefCell<i32>,
    /// Original: key_menu_confirm
    pub key_menu_confirm: RefCell<i32>,
    /// Original: key_menu_abort
    pub key_menu_abort: RefCell<i32>,
    /// Original: key_menu_help
    pub key_menu_help: RefCell<i32>,
    /// Original: key_menu_save
    pub key_menu_save: RefCell<i32>,
    /// Original: key_menu_load
    pub key_menu_load: RefCell<i32>,
    /// Original: key_menu_volume
    pub key_menu_volume: RefCell<i32>,
    /// Original: key_menu_detail
    pub key_menu_detail: RefCell<i32>,
    /// Original: key_menu_qsave
    pub key_menu_qsave: RefCell<i32>,
    /// Original: key_menu_endgame
    pub key_menu_endgame: RefCell<i32>,
    /// Original: key_menu_messages
    pub key_menu_messages: RefCell<i32>,
    /// Original: key_menu_qload
    pub key_menu_qload: RefCell<i32>,
    /// Original: key_menu_quit
    pub key_menu_quit: RefCell<i32>,
    /// Original: key_menu_gamma
    pub key_menu_gamma: RefCell<i32>,
    /// Original: key_menu_incscreen
    pub key_menu_incscreen: RefCell<i32>,
    /// Original: key_menu_decscreen
    pub key_menu_decscreen: RefCell<i32>,
    /// Original: key_menu_screenshot
    pub key_menu_screenshot: RefCell<i32>,
    /// Original: mousebfire
    pub mousebfire: RefCell<i32>,
    /// Original: mousebstrafe
    pub mousebstrafe: RefCell<i32>,
    /// Original: mousebforward
    pub mousebforward: RefCell<i32>,
    /// Original: mousebjump
    pub mousebjump: RefCell<i32>,
    /// Original: mousebstrafeleft
    pub mousebstrafeleft: RefCell<i32>,
    /// Original: mousebstraferight
    pub mousebstraferight: RefCell<i32>,
    /// Original: mousebbackward
    pub mousebbackward: RefCell<i32>,
    /// Original: mousebuse
    pub mousebuse: RefCell<i32>,
    /// Original: mousebprevweapon
    pub mousebprevweapon: RefCell<i32>,
    /// Original: mousebnextweapon
    pub mousebnextweapon: RefCell<i32>,
    /// Original: joybfire
    pub joybfire: RefCell<i32>,
    /// Original: joybstrafe
    pub joybstrafe: RefCell<i32>,
    /// Original: joybuse
    pub joybuse: RefCell<i32>,
    /// Original: joybspeed
    pub joybspeed: RefCell<i32>,
    /// Original: joybjump
    pub joybjump: RefCell<i32>,
    /// Original: joybstrafeleft
    pub joybstrafeleft: RefCell<i32>,
    /// Original: joybstraferight
    pub joybstraferight: RefCell<i32>,
    /// Original: joybprevweapon
    pub joybprevweapon: RefCell<i32>,
    /// Original: joybnextweapon
    pub joybnextweapon: RefCell<i32>,
    /// Original: joybmenu
    pub joybmenu: RefCell<i32>,
    /// Original: dclick_use
    pub dclick_use: RefCell<i32>,
}

impl M_ControlsState {
    pub fn new() -> Self {
        Self {
            key_right: RefCell::new(0),
            key_left: RefCell::new(0),
            key_up: RefCell::new(0),
            key_down: RefCell::new(0),
            key_strafeleft: RefCell::new(0),
            key_straferight: RefCell::new(0),
            key_fire: RefCell::new(0),
            key_use: RefCell::new(0),
            key_strafe: RefCell::new(0),
            key_speed: RefCell::new(0),
            key_jump: RefCell::new(0),
            key_flyup: RefCell::new(0),
            key_flydown: RefCell::new(0),
            key_flycenter: RefCell::new(0),
            key_lookup: RefCell::new(0),
            key_lookdown: RefCell::new(0),
            key_lookcenter: RefCell::new(0),
            key_invleft: RefCell::new(0),
            key_invright: RefCell::new(0),
            key_useartifact: RefCell::new(0),
            key_usehealth: RefCell::new(0),
            key_invquery: RefCell::new(0),
            key_mission: RefCell::new(0),
            key_invpop: RefCell::new(0),
            key_invkey: RefCell::new(0),
            key_invhome: RefCell::new(0),
            key_invend: RefCell::new(0),
            key_invuse: RefCell::new(0),
            key_invdrop: RefCell::new(0),
            key_message_refresh: RefCell::new(0),
            key_pause: RefCell::new(0),
            key_multi_msg: RefCell::new(0),
            key_multi_msgplayer: RefCell::new([0; 8]),
            key_weapon1: RefCell::new(0),
            key_weapon2: RefCell::new(0),
            key_weapon3: RefCell::new(0),
            key_weapon4: RefCell::new(0),
            key_weapon5: RefCell::new(0),
            key_weapon6: RefCell::new(0),
            key_weapon7: RefCell::new(0),
            key_weapon8: RefCell::new(0),
            key_arti_all: RefCell::new(0),
            key_arti_health: RefCell::new(0),
            key_arti_poisonbag: RefCell::new(0),
            key_arti_blastradius: RefCell::new(0),
            key_arti_teleport: RefCell::new(0),
            key_arti_teleportother: RefCell::new(0),
            key_arti_egg: RefCell::new(0),
            key_arti_invulnerability: RefCell::new(0),
            key_demo_quit: RefCell::new(0),
            key_spy: RefCell::new(0),
            key_prevweapon: RefCell::new(0),
            key_nextweapon: RefCell::new(0),
            key_map_north: RefCell::new(0),
            key_map_south: RefCell::new(0),
            key_map_east: RefCell::new(0),
            key_map_west: RefCell::new(0),
            key_map_zoomin: RefCell::new(0),
            key_map_zoomout: RefCell::new(0),
            key_map_toggle: RefCell::new(0),
            key_map_maxzoom: RefCell::new(0),
            key_map_follow: RefCell::new(0),
            key_map_grid: RefCell::new(0),
            key_map_mark: RefCell::new(0),
            key_map_clearmark: RefCell::new(0),
            key_menu_activate: RefCell::new(0),
            key_menu_up: RefCell::new(0),
            key_menu_down: RefCell::new(0),
            key_menu_left: RefCell::new(0),
            key_menu_right: RefCell::new(0),
            key_menu_back: RefCell::new(0),
            key_menu_forward: RefCell::new(0),
            key_menu_confirm: RefCell::new(0),
            key_menu_abort: RefCell::new(0),
            key_menu_help: RefCell::new(0),
            key_menu_save: RefCell::new(0),
            key_menu_load: RefCell::new(0),
            key_menu_volume: RefCell::new(0),
            key_menu_detail: RefCell::new(0),
            key_menu_qsave: RefCell::new(0),
            key_menu_endgame: RefCell::new(0),
            key_menu_messages: RefCell::new(0),
            key_menu_qload: RefCell::new(0),
            key_menu_quit: RefCell::new(0),
            key_menu_gamma: RefCell::new(0),
            key_menu_incscreen: RefCell::new(0),
            key_menu_decscreen: RefCell::new(0),
            key_menu_screenshot: RefCell::new(0),
            mousebfire: RefCell::new(0),
            mousebstrafe: RefCell::new(0),
            mousebforward: RefCell::new(0),
            mousebjump: RefCell::new(0),
            mousebstrafeleft: RefCell::new(0),
            mousebstraferight: RefCell::new(0),
            mousebbackward: RefCell::new(0),
            mousebuse: RefCell::new(0),
            mousebprevweapon: RefCell::new(0),
            mousebnextweapon: RefCell::new(0),
            joybfire: RefCell::new(0),
            joybstrafe: RefCell::new(0),
            joybuse: RefCell::new(0),
            joybspeed: RefCell::new(0),
            joybjump: RefCell::new(0),
            joybstrafeleft: RefCell::new(0),
            joybstraferight: RefCell::new(0),
            joybprevweapon: RefCell::new(0),
            joybnextweapon: RefCell::new(0),
            joybmenu: RefCell::new(0),
            dclick_use: RefCell::new(0),
        }
    }
}
