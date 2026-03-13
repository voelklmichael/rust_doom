//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Intermission screen.
// Original: wi_stuff.h + wi_stuff.c

use crate::doomstat::WbStartStruct;

// =============================================================================
// Public API (from wi_stuff.h)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WiStateEnum {
    NoState = -1,
    StatCount,
    ShowNextLoc,
}

// =============================================================================
// Implementation (from wi_stuff.c) — stubs
// =============================================================================

pub fn wi_start(_wbstartstruct: &WbStartStruct) {
    // Stub
}

pub fn wi_end() {
    // Stub
}

pub fn wi_ticker() {
    // Stub
}

pub fn wi_drawer() {
    // Stub
}
