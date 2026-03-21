// doomgeneric/i_endoom.h

pub use crate::config::*;
pub use crate::doomtype::*;
pub use crate::i_video::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct I_EndoomState {
    pub _placeholder: RefCell<()>,
}

impl I_EndoomState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: I_Endoom
    pub fn i_endoom(&self, _data: *mut Byte) {
        todo!("I_Endoom");
    }
}
