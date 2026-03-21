// doomgeneric/r_bsp.h

pub use crate::doomtype::*;
pub use crate::r_defs::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_BspState {
    pub rw_x: RefCell<i32>,
    pub rw_stopx: RefCell<i32>,
    pub segtextured: RefCell<Boolean>,
    pub markfloor: RefCell<Boolean>,
    pub markceiling: RefCell<Boolean>,
    pub skymap: RefCell<Boolean>,
}

impl R_BspState {
    pub fn new() -> Self {
        Self {
            rw_x: RefCell::new(0),
            rw_stopx: RefCell::new(0),
            segtextured: RefCell::new(Boolean::False),
            markfloor: RefCell::new(Boolean::False),
            markceiling: RefCell::new(Boolean::False),
            skymap: RefCell::new(Boolean::False),
        }
    }

    pub fn r_clear_clip_segs(&self) {
        todo!("R_ClearClipSegs");
    }

    pub fn r_clear_draw_segs(&self) {
        todo!("R_ClearDrawSegs");
    }

    pub fn r_render_bsp_node(&self, _bspnum: i32) {
        todo!("R_RenderBSPNode");
    }
}
