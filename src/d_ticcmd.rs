// d_ticcmd.h - ticcmd structure

pub use crate::doomtype::*;

/// Original: ticcmd_t
#[repr(C)]
pub struct TiccmdT {
    // Original: forwardmove
    pub forwardmove: i8,
    // Original: sidemove
    pub sidemove: i8,
    // Original: angleturn
    pub angleturn: i16,
    // Original: chatchar
    pub chatchar: u8,
    // Original: buttons
    pub buttons: u8,
    // Original: consistancy
    pub consistancy: u8,
    // Original: buttons2
    pub buttons2: u8,
    // Original: inventory
    pub inventory: i32,
    // Original: lookfly
    pub lookfly: u8,
    // Original: arti
    pub arti: u8,
}

#[allow(non_camel_case_types)]
pub struct D_TiccmdState;

impl D_TiccmdState {
    pub fn new() -> Self {
        Self
    }
}
