//! Texture typedefs (d_textur.h)
//! Original: d_textur.h

use crate::doomtype::Byte;

// typedef struct { byte width, height, data; } pic_t
#[repr(C)]
pub struct PicT {
    // byte width
    pub width: Byte,
    // byte height
    pub height: Byte,
    // byte data (flexible array in C - just first element)
    pub data: Byte,
}
