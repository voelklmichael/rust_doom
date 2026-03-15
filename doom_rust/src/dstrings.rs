//! Rust translation of doomgeneric/dstrings.h
//! DOOM strings, by language.

use crate::d_englsh::*;

pub static SAVEGAMENAME_STR: &str = "doomsav";
pub const NUM_QUITMESSAGES: i32 = 8;

pub static mut doom1_endmsg: [*mut i8; 8] = [core::ptr::null_mut(); 8];
pub static mut doom2_endmsg: [*mut i8; 8] = [core::ptr::null_mut(); 8];
