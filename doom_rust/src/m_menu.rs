//! Rust translation of doomgeneric/m_menu.h
//! Menu widget stuff, episode selection and such.

use crate::d_event::*;
use crate::doomtype::*;

/// C function: M_Responder
pub fn m_responder(ev: *mut EventT) -> Boolean {
    todo!("original: M_Responder")
}

/// C function: M_Ticker
pub fn m_ticker() {
    todo!("original: M_Ticker")
}

/// C function: M_Drawer
pub fn m_drawer() {
    todo!("original: M_Drawer")
}

/// C function: M_Init
pub fn m_init() {
    todo!("original: M_Init")
}

/// C function: M_StartControlPanel
pub fn m_start_control_panel() {
    todo!("original: M_StartControlPanel")
}

pub static mut detail_level: i32 = 0;
pub static mut screenblocks: i32 = 0;
