//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  System specific interface stuff.
//
// Original: d_main.h + d_main.c (stub)

use crate::doomdef::Gameaction;
use crate::d_event::d_pop_event;

/// Current game action. Original: gameaction
pub static mut GAMEACTION: Gameaction = Gameaction::Nothing;

/// Read events from all input devices.
/// Original: D_ProcessEvents
pub fn d_process_events() {
    while d_pop_event().is_some() {
        // Consume events; G_Responder would handle them
    }
}

/// Original: D_PageTicker
pub fn d_page_ticker() {
    // Stub
}

/// Original: D_PageDrawer
pub fn d_page_drawer() {
    // Stub
}

/// Original: D_AdvanceDemo
pub fn d_advance_demo() {
    // Stub
}

/// Original: D_DoAdvanceDemo
pub fn d_do_advance_demo() {
    // Stub
}

/// Original: D_StartTitle
pub fn d_start_title() {
    // Stub
}
