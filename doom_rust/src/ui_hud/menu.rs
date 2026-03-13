//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Menu widget stuff, episode selection.
// Original: m_menu.h + m_menu.c

use crate::doomtype::Boolean;
use crate::game::d_event::Event;

// =============================================================================
// Public API (from m_menu.h)
// =============================================================================

pub static mut DETAIL_LEVEL: i32 = 0;
pub static mut SCREENBLOCKS: i32 = 10;

// =============================================================================
// Implementation (from m_menu.c) — stubs
// =============================================================================

pub fn m_init() {
    // Stub: would load config file
}

pub fn m_responder(_ev: &Event) -> Boolean {
    // Stub
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
