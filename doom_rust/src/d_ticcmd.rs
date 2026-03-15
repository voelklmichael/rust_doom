//! Rust translation of doomgeneric/d_ticcmd.h
//! System specific interface - data sampled per tick.

use crate::doomtype::*;

/// C typedef: ticcmd_t
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TiccmdT {
    pub forwardmove: i8,   // *2048 for move
    pub sidemove: i8,      // *2048 for move
    pub angleturn: i16,    // <<16 for angle delta
    pub chatchar: byte,
    pub buttons: byte,
    pub consistancy: byte, // checks for net game
    pub buttons2: byte,
    pub inventory: i32,
    pub lookfly: byte,     // look/fly up/down/centering
    pub arti: byte,        // artitype_t to use
}

#[allow(non_camel_case_types)]
pub type ticcmd_t = TiccmdT;
