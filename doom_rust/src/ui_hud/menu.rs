//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Menu widget stuff, episode selection.
// Original: m_menu.h + m_menu.c

use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::ui_hud::config::{m_get_int_variable, m_load_defaults, m_set_variable};

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
    unsafe {
        SCREENBLOCKS = m_get_int_variable("screenblocks");
        DETAIL_LEVEL = m_get_int_variable("detailLevel");
    }
}

pub fn m_responder(_ev: &Event) -> Boolean {
    false
}

pub fn m_ticker() {
    // Stub: skull cursor animation
}

pub fn m_drawer() {
    // Stub: draws menus to screen buffer
}

pub fn m_start_control_panel() {
    // Stub: force menu up on keypress
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
