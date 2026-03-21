// d_textur.h - texture typedefs

pub use crate::doomtype::*;

/// Original: pic_t
#[repr(C)]
pub struct PicT {
    pub width: u8,
    pub height: u8,
    pub data: u8,
}

#[allow(non_camel_case_types)]
pub struct D_TexturState;

impl D_TexturState {
    pub fn new() -> Self {
        Self
    }
}
