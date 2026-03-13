//! UI and HUD: menus, status bar, heads-up display, intermission.
//!
//! Original C: m_menu, m_controls, m_cheat, m_config, hu_stuff, hu_lib,
//! st_stuff, st_lib, wi_stuff.

pub mod cheat;
pub mod config;
pub mod controls;
pub mod hu_lib;
pub mod hu_stuff;
pub mod menu;
pub mod st_lib;
pub mod st_stuff;
pub mod wi_stuff;

// Re-export commonly used public API
pub use cheat::{cht_check_cheat, cht_get_param, CheatSeq, MAX_CHEAT_LEN, MAX_CHEAT_PARAMS};
pub use config::{
    configdir, m_bind_variable, m_get_float_variable, m_get_int_variable, m_get_save_game_dir,
    m_get_str_variable, m_load_defaults, m_save_defaults, m_save_defaults_alternate,
    m_set_config_dir, m_set_config_filenames, m_set_variable,
};
pub use controls::{
    m_apply_platform_defaults, m_bind_base_controls, m_bind_chat_controls, m_bind_hexen_controls,
    m_bind_heretic_controls, m_bind_map_controls, m_bind_menu_controls, m_bind_strife_controls,
    m_bind_weapon_controls,
};
pub use hu_stuff::{hu_dequeue_chat_char, hu_drawer, hu_erase, hu_init, hu_responder, hu_start, hu_ticker};
pub use menu::{m_drawer, m_init, m_responder, m_start_control_panel, m_ticker};
pub use st_stuff::{st_drawer, st_init, st_responder, st_start, st_ticker};
pub use wi_stuff::{wi_drawer, wi_end, wi_start, wi_ticker};
