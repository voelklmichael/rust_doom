//! Rust translation of doomgeneric/d_event.h
//! Event handling.

use crate::doomtype::*;
use std::sync::{Arc, Mutex};

/// C typedef: evtype_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: evtype_t
pub enum EvtypeT {
    Keydown,
    Keyup,
    Mouse,
    Joystick,
    Quit,
}

/// C typedef: event_t
#[repr(C)]
/// C typedef: event_t
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
    /// C #define: BT_ATTACK
    pub const BT_ATTACK: i32 = 1;
    /// C #define: BT_USE
    pub const BT_USE: i32 = 2;
    /// C #define: BT_SPECIAL
    pub const BT_SPECIAL: i32 = 128;
    /// C #define: BT_SPECIALMASK
    pub const BT_SPECIALMASK: i32 = 3;
    /// C #define: BT_CHANGE
    pub const BT_CHANGE: i32 = 4;
    /// C #define: BT_WEAPONMASK
    pub const BT_WEAPONMASK: i32 = 8 + 16 + 32;
    /// C #define: BT_WEAPONSHIFT
    pub const BT_WEAPONSHIFT: i32 = 3;
    /// C #define: BTS_PAUSE
    pub const BTS_PAUSE: i32 = 1;
    /// C #define: BTS_SAVEGAME
    pub const BTS_SAVEGAME: i32 = 2;
    /// C #define: BTS_SAVEMASK
    pub const BTS_SAVEMASK: i32 = 4 + 8 + 16;
    /// C #define: BTS_SAVESHIFT
    pub const BTS_SAVESHIFT: i32 = 2;
}

/// C typedef: buttoncode2_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: buttoncode2_t
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
pub fn d_post_event(ev: &mut EventT) {
    todo!("original: D_PostEvent")
}

/// C function: D_PopEvent
pub fn d_pop_event() -> Arc<Mutex<EventT>> {
    todo!("original: D_PopEvent")
}

#[allow(non_camel_case_types)]
/// C typedef: event_t
pub type event_t = EventT;
