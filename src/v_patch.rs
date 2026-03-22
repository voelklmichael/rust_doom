//! Patch/sprite structures (v_patch.h)
//! Original: v_patch.h

use crate::doomtype::Byte;

// typedef struct { short width, height, leftoffset, topoffset; int columnofs[8]; } PACKEDATTR patch_t
#[repr(C, packed)]
pub struct PatchT {
    // short width
    pub width: i16,
    // short height
    pub height: i16,
    // short leftoffset
    pub leftoffset: i16,
    // short topoffset
    pub topoffset: i16,
    // int columnofs[8]
    pub columnofs: [i32; 8],
}

// typedef struct { byte topdelta; byte length; } PACKEDATTR post_t
#[repr(C, packed)]
pub struct PostT {
    // byte topdelta
    pub topdelta: Byte,
    // byte length
    pub length: Byte,
}

// typedef post_t column_t
pub type ColumnT = PostT;
