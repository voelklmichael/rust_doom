//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Menu widget stuff, episode selection.
// Original: m_menu.h + m_menu.c

use crate::doomstat::MENUACTIVE;
use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::ui_hud::config::{m_get_int_variable, m_load_defaults, m_set_variable};
use crate::ui_hud::controls::{
    m_bind_base_controls, m_bind_map_controls, m_bind_menu_controls, m_bind_weapon_controls,
};

// =============================================================================
// Public API (from m_menu.h)
// =============================================================================

pub static mut DETAIL_LEVEL: i32 = 0;
pub static mut SCREENBLOCKS: i32 = 10;

// =============================================================================
// Implementation (from m_menu.c)
// =============================================================================

pub fn m_init() {
    m_load_defaults();
    m_bind_base_controls();
    m_bind_weapon_controls();
    m_bind_map_controls();
    m_bind_menu_controls();
    unsafe {
        SCREENBLOCKS = m_get_int_variable("screenblocks");
        DETAIL_LEVEL = m_get_int_variable("detailLevel");
    }
}

/// Handle menu key input. Returns true if event was consumed.
pub fn m_responder(ev: &Event) -> Boolean {
    use crate::game::d_event::EvType;
    use crate::ui_hud::controls::{KEY_MENU_ABORT, KEY_MENU_ACTIVATE};
    if ev.ev_type != EvType::KeyDown {
        return false;
    }
    let key = ev.data2;
    unsafe {
        if MENUACTIVE {
            if key == KEY_MENU_ABORT {
                MENUACTIVE = false;
                return true;
            }
        } else if key == KEY_MENU_ACTIVATE {
            m_start_control_panel();
            return true;
        }
    }
    false
}

pub fn m_ticker() {
    // Stub: skull cursor animation
}

pub fn m_drawer() {
    // Stub: draws menus to screen buffer
}

/// Force menu up on keypress. Does nothing if menu already active.
pub fn m_start_control_panel() {
    unsafe {
        if MENUACTIVE {
            return;
        }
        MENUACTIVE = true;
    }
}

/// Sync screenblocks to config and optionally trigger view size update.
pub fn m_set_screenblocks(val: i32) {
    unsafe {
        SCREENBLOCKS = val;
    }
    m_set_variable("screenblocks", &val.to_string());
}

/// Sync detail level to config.
pub fn m_set_detail_level(val: i32) {
    unsafe {
        DETAIL_LEVEL = val;
    }
    m_set_variable("detailLevel", &val.to_string());
}
