//! Tic command (d_ticcmd.h)
//! Original: d_ticcmd.h

use crate::doomtype::Byte;

// typedef struct { ... } ticcmd_t
#[repr(C)]
pub struct TiccmdT {
    // signed char forwardmove
    pub forwardmove: i8,
    // signed char sidemove
    pub sidemove: i8,
    // short angleturn
    pub angleturn: i16,
    // byte chatchar
    pub chatchar: Byte,
    // byte buttons
    pub buttons: Byte,
    // byte consistancy
    pub consistancy: Byte,
    // byte buttons2
    pub buttons2: Byte,
    // int inventory
    pub inventory: i32,
    // byte lookfly
    pub lookfly: Byte,
    // byte arti
    pub arti: Byte,
}
