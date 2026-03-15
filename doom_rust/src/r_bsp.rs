//! Rust translation of doomgeneric/r_bsp.h
//! Refresh module, BSP traversal and handling.

use crate::doomtype::*;
use crate::r_defs::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static mut curline: *mut SegT = std::ptr::null_mut();
pub static mut sidedef: *mut SideT = std::ptr::null_mut();
pub static mut linedef: *mut LineT = std::ptr::null_mut();
pub static mut frontsector: *mut SectorT = std::ptr::null_mut();
pub static mut backsector: *mut SectorT = std::ptr::null_mut();

pub static mut rw_x: i32 = 0;
pub static mut rw_stopx: i32 = 0;
pub static mut segtextured: boolean = crate::doomtype::Boolean::False;
pub static mut markfloor: boolean = crate::doomtype::Boolean::False;
pub static mut markceiling: boolean = crate::doomtype::Boolean::False;
pub static mut skymap: boolean = crate::doomtype::Boolean::False;

pub static DRAWSEGS: Lazy<Mutex<[DrawsegT; MAXDRAWSEGS]>> = Lazy::new(|| {
    Mutex::new(std::array::from_fn(|_| DrawsegT::new()))
});
pub static mut ds_p: *mut DrawsegT = std::ptr::null_mut();

pub static mut hscalelight: *mut *mut LighttableT = std::ptr::null_mut();
pub static mut vscalelight: *mut *mut LighttableT = std::ptr::null_mut();
pub static mut dscalelight: *mut *mut LighttableT = std::ptr::null_mut();

/// C typedef: drawfunc_t
pub type DrawfuncT = fn(i32, i32);

/// C function: R_ClearClipSegs
pub fn r_clear_clip_segs() {
    todo!("original: R_ClearClipSegs")
}

/// C function: R_ClearDrawSegs
pub fn r_clear_draw_segs() {
    todo!("original: R_ClearDrawSegs")
}

/// C function: R_RenderBSPNode
pub fn r_render_bsp_node(bspnum: i32) {
    todo!("original: R_RenderBSPNode")
}
