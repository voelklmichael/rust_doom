// doomgeneric/r_local.h — aggregate refresh API

pub use crate::r_bsp::*;
pub use crate::r_data::*;
pub use crate::r_draw::*;
pub use crate::r_main::*;
pub use crate::r_plane::*;
pub use crate::r_segs::*;
pub use crate::r_things::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_LocalState {
    pub _placeholder: RefCell<()>,
}

impl R_LocalState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }
}
