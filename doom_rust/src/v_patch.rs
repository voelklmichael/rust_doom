//! Rust translation of doomgeneric/v_patch.h

use crate::doomtype::*;

/// C typedef: patch_t
#[repr(C, packed)]
pub struct PatchT {
    pub width: i16,
    pub height: i16,
    pub leftoffset: i16,
    pub topoffset: i16,
    pub columnofs: [i32; 8],
}

/// C typedef: post_t
#[repr(C, packed)]
pub struct PostT {
    pub topdelta: byte,
    pub length: byte,
}

/// C typedef: column_t
pub type ColumnT = PostT;
