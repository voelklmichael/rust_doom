//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Status bar code.
// Original: st_stuff.h + st_stuff.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::ui_hud::cheat::CheatSeq;

// =============================================================================
// Public API (from st_stuff.h)
// =============================================================================

pub const ST_HEIGHT: i32 = 32;
pub const ST_WIDTH: i32 = SCREENWIDTH;
pub const ST_Y: i32 = SCREENHEIGHT - ST_HEIGHT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum StStateEnum {
    #[default]
    AutomapState,
    FirstPersonState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum StChatStateEnum {
    #[default]
    StartChatState,
    WaitDestState,
    GetChatState,
}

pub static mut ST_BACKING_SCREEN: *mut u8 = std::ptr::null_mut();

pub static mut CHEAT_MUS: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_GOD: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_AMMO: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_AMMONOKEY: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_NOCLIP: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_CLEV: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_MYPOS: CheatSeq = CheatSeq::EMPTY;

// =============================================================================
// Implementation (from st_stuff.c) — stubs
// =============================================================================

pub fn st_init() {
    // Stub
}

pub fn st_start() {
    // Stub
}

pub fn st_responder(_ev: &Event) -> Boolean {
    // Stub
    false
}

pub fn st_ticker() {
    // Stub
}

pub fn st_drawer(_fullscreen: bool, _refresh: bool) {
    // Stub
}
