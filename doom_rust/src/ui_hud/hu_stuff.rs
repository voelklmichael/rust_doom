//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Heads-up display.
// Original: hu_stuff.h + hu_stuff.c

use crate::doomdef::TICRATE;
use crate::doomtype::Boolean;
use crate::game::d_event::Event;

// =============================================================================
// Public API (from hu_stuff.h)
// =============================================================================

pub const HU_FONTSTART: u8 = b'!';
pub const HU_FONTEND: u8 = b'_';
pub const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize; // 95 - 33 + 1 = 63

pub const HU_BROADCAST: i32 = 5;

pub const HU_MSGX: i32 = 0;
pub const HU_MSGY: i32 = 0;
pub const HU_MSGWIDTH: i32 = 64;
pub const HU_MSGHEIGHT: i32 = 1;

pub const HU_MSGTIMEOUT: i32 = 4 * TICRATE;

pub static mut CHAT_MACROS: [&'static str; 10] = [
    "", "", "", "", "", "", "", "", "", "",
];

pub fn hu_init() {
    // Stub
}

pub fn hu_start() {
    // Stub
}

pub fn hu_responder(_ev: &Event) -> Boolean {
    // Stub
    false
}

pub fn hu_ticker() {
    // Stub
}

pub fn hu_drawer() {
    // Stub
}

pub fn hu_dequeue_chat_char() -> u8 {
    // Stub
    0
}

pub fn hu_erase() {
    // Stub
}
