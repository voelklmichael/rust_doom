//! Rust translation of doomgeneric/r_plane.h
//! Refresh, visplane stuff (floor, ceilings).

use crate::i_video::*;
use crate::m_fixed::*;
use crate::r_defs::*;

pub static mut lastopening: *mut i16 = std::ptr::null_mut();

/// C typedef: planefunction_t
pub type PlanefunctionT = fn(i32, i32);

pub static mut floorfunc: Option<PlanefunctionT> = None;
pub static mut ceilingfunc_t: Option<PlanefunctionT> = None;

pub static mut floorclip: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];
pub static mut ceilingclip: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];

pub static mut yslope: [FixedT; crate::i_video::SCREENHEIGHT as usize] =
    [0; crate::i_video::SCREENHEIGHT as usize];
pub static mut distscale: [FixedT; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];

/// C function: R_InitPlanes
pub fn r_init_planes() {
    todo!("original: R_InitPlanes")
}

/// C function: R_ClearPlanes
pub fn r_clear_planes() {
    todo!("original: R_ClearPlanes")
}

/// C function: R_MapPlane
pub fn r_map_plane(y: i32, x1: i32, x2: i32) {
    todo!("original: R_MapPlane")
}

/// C function: R_MakeSpans
pub fn r_make_spans(x: i32, t1: i32, b1: i32, t2: i32, b2: i32) {
    todo!("original: R_MakeSpans")
}

/// C function: R_DrawPlanes
pub fn r_draw_planes() {
    todo!("original: R_DrawPlanes")
}

/// C function: R_FindPlane
pub fn r_find_plane(
    height: FixedT,
    picnum: i32,
    lightlevel: i32,
) -> *mut VisplaneT {
    todo!("original: R_FindPlane")
}

/// C function: R_CheckPlane
pub fn r_check_plane(pl: &mut VisplaneT, start: i32, stop: i32) -> *mut VisplaneT {
    todo!("original: R_CheckPlane")
}
