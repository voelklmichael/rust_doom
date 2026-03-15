//! Rust translation of doomgeneric/m_bbox.h
//! Bounding box coordinate storage and functions.

use crate::m_fixed::*;

/// Bounding box coordinates (C enum)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: bbox_coord_t
pub enum BboxCoord {
    BoxTop,
    BoxBottom,
    BoxLeft,
    BoxRight,
}

/// C function: M_ClearBox
pub fn m_clear_box(box_: *mut FixedT) {
    todo!("original: M_ClearBox")
}

/// C function: M_AddToBox
pub fn m_add_to_box(box_: *mut FixedT, x: FixedT, y: FixedT) {
    todo!("original: M_AddToBox")
}
