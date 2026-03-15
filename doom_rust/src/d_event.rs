//! Rust translation of doomgeneric/d_event.h
//! Event handling.

use crate::doomtype::*;

/// C typedef: evtype_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvtypeT {
    Keydown,
    Keyup,
    Mouse,
    Joystick,
    Quit,
}

/// C typedef: event_t
#[repr(C)]
pub struct EventT {
    pub ev_type: EvtypeT,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub data4: i32,
}

/// C typedef: buttoncode_t - bit flags/constants (C enum allows duplicate values)
#[allow(non_camel_case_types)]
pub mod buttoncode_t {
    pub const BT_ATTACK: i32 = 1;
    pub const BT_USE: i32 = 2;
    pub const BT_SPECIAL: i32 = 128;
    pub const BT_SPECIALMASK: i32 = 3;
    pub const BT_CHANGE: i32 = 4;
    pub const BT_WEAPONMASK: i32 = 8 + 16 + 32;
    pub const BT_WEAPONSHIFT: i32 = 3;
    pub const BTS_PAUSE: i32 = 1;
    pub const BTS_SAVEGAME: i32 = 2;
    pub const BTS_SAVEMASK: i32 = 4 + 8 + 16;
    pub const BTS_SAVESHIFT: i32 = 2;
}

/// C typedef: buttoncode2_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buttoncode2T {
    Lookup = 1,
    Lookdown = 2,
    Centerview = 4,
    Invuse = 8,
    Invdrop = 16,
    Jump = 32,
    Health = 128,
}

/// C function: D_PostEvent
pub fn d_post_event(ev: *mut EventT) {
    todo!("original: D_PostEvent")
}

/// C function: D_PopEvent
pub fn d_pop_event() -> *mut EventT {
    todo!("original: D_PopEvent")
}

#[allow(non_camel_case_types)]
pub type event_t = EventT;
