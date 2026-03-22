//! ENDOOM screen (i_endoom.h, i_endoom.c)
//! Original: i_endoom.h, i_endoom.c

use crate::doomtype::Byte;

pub struct I_EndoomState;

impl I_EndoomState {
    /// Original: void I_Endoom(byte *data)
    pub fn i_endoom(&self, _data: &[Byte]) {
        // C body: displays text mode ending screen (ORIGCODE/__DJGPP__ paths removed)
        todo!("Basic stage-0 stub")
    }
}
