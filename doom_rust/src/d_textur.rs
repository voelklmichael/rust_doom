//! Rust translation of doomgeneric/d_textur.h
//! Typedefs related to textures.

use crate::doomtype::*;

/// C typedef: pic_t - flexible array member at end

#[repr(C)]
pub struct PicT {
    pub width: byte,
    pub height: byte,
    pub data: u8, // C flexible array member: byte data[]
}
