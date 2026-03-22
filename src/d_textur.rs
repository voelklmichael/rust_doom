//! Texture typedefs (d_textur.h)
//! Original: d_textur.h

use crate::doomtype::Byte;

// typedef struct { byte width; byte height; byte data; } pic_t
#[repr(C, packed)]
pub struct PicT {
    // byte width
    pub width: Byte,
    // byte height
    pub height: Byte,
    // byte data
    pub data: Byte,
}
