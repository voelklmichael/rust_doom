// d_ticcmd.h - ticcmd structure

pub use crate::doomtype::*;

/// Original: ticcmd_t
#[repr(C)]
pub struct TiccmdT {
    pub forwardmove: i8,
    pub sidemove: i8,
    pub angleturn: i16,
    pub chatchar: u8,
    pub buttons: u8,
    pub consistancy: u8,
    pub buttons2: u8,
    pub inventory: i32,
    pub lookfly: u8,
    pub arti: u8,
}

#[allow(non_camel_case_types)]
pub struct D_TiccmdState;

impl D_TiccmdState {
    pub fn new() -> Self {
        Self
    }
}
