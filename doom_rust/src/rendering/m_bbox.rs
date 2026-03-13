//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Bounding box coordinate storage and functions.
//
// Original: m_bbox.h / m_bbox.c

use crate::m_fixed::Fixed;

// =============================================================================
// Public API (from .h)
// =============================================================================

/// Bounding box coordinate indices.
pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

/// Bounding box: [top, bottom, left, right] in fixed_t.
pub type Bbox = [Fixed; 4];

/// Clear box to empty state (no points inside).
/// Original: M_ClearBox
pub fn m_clear_box(box_: &mut Bbox) {
    box_[BOXTOP] = i32::MIN;
    box_[BOXRIGHT] = i32::MIN;
    box_[BOXBOTTOM] = i32::MAX;
    box_[BOXLEFT] = i32::MAX;
}

/// Expand box to include point (x, y).
/// Original: M_AddToBox
pub fn m_add_to_box(box_: &mut Bbox, x: Fixed, y: Fixed) {
    if x < box_[BOXLEFT] {
        box_[BOXLEFT] = x;
    } else if x > box_[BOXRIGHT] {
        box_[BOXRIGHT] = x;
    }
    if y < box_[BOXBOTTOM] {
        box_[BOXBOTTOM] = y;
    } else if y > box_[BOXTOP] {
        box_[BOXTOP] = y;
    }
}
