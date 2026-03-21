// doomgeneric/mus2mid.h

pub use crate::doomtype::*;
pub use crate::i_swap::*;
pub use crate::m_misc::*;
pub use crate::memio::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct Mus2midState {
    pub _placeholder: RefCell<()>,
}

impl Mus2midState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn mus2mid(&self, _musinput: *mut Memfile, _midioutput: *mut Memfile) -> Boolean {
        todo!("mus2mid");
    }
}
