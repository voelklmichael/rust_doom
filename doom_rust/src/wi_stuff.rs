//! Rust translation of doomgeneric/wi_stuff.h
//! Intermission.

use crate::doomdef::*;
use crate::d_player::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateenumT {
    NoState = -1,
    StatCount,
    ShowNextLoc,
}

pub fn wi_ticker() {
    todo!("original: WI_Ticker")
}

pub fn wi_drawer() {
    todo!("original: WI_Drawer")
}

pub fn wi_start(wbstartstruct: *mut WbstartstructT) {
    todo!("original: WI_Start")
}

pub fn wi_end() {
    todo!("original: WI_End")
}
