// doomgeneric/r_plane.h

pub use crate::r_data::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_PlaneState {
    pub _placeholder: RefCell<()>,
}

impl R_PlaneState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: R_InitPlanes
    pub fn r_init_planes(&self) {
        todo!("R_InitPlanes");
    }

    // Original: R_ClearPlanes
    pub fn r_clear_planes(&self) {
        todo!("R_ClearPlanes");
    }

    // Original: R_DrawPlanes
    pub fn r_draw_planes(&self) {
        todo!("R_DrawPlanes");
    }
}
