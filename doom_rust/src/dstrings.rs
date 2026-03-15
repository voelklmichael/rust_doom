//! Rust translation of doomgeneric/dstrings.h
//! DOOM strings, by language.

use crate::d_englsh::*;

pub static SAVEGAMENAME_STR: &str = "doomsav";
/// C #define: NUM_QUITMESSAGES
pub const NUM_QUITMESSAGES: i32 = 8;

pub static mut doom1_endmsg: [&str; 8] = [""; 8];
pub static mut doom2_endmsg: [&str; 8] = [""; 8];
