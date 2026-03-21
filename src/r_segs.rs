// doomgeneric/r_segs.h

pub use crate::r_defs::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_SegsState {
    pub _placeholder: RefCell<()>,
}

impl R_SegsState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: R_RenderMaskedSegRange
    pub fn r_render_masked_seg_range(&self, _ds: *mut DrawsegT, _x1: i32, _x2: i32) {
        todo!("R_RenderMaskedSegRange");
    }
}
