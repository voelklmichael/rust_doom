//! Rust translation of doomgeneric/d_main.h
//! System specific interface stuff.

use crate::doomdef::*;

/// C function: D_ProcessEvents
pub fn d_process_events() {
    todo!("original: D_ProcessEvents")
}

/// C function: D_PageTicker
pub fn d_page_ticker() {
    todo!("original: D_PageTicker")
}

/// C function: D_PageDrawer
pub fn d_page_drawer() {
    todo!("original: D_PageDrawer")
}

/// C function: D_AdvanceDemo
pub fn d_advance_demo() {
    todo!("original: D_AdvanceDemo")
}

/// C function: D_DoAdvanceDemo
pub fn d_do_advance_demo() {
    todo!("original: D_DoAdvanceDemo")
}

/// C function: D_StartTitle
pub fn d_start_title() {
    todo!("original: D_StartTitle")
}

pub static mut gameaction: GameactionT = GameactionT::Nothing;
