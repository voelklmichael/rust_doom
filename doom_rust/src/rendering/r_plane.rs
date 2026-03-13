//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Visplane stuff (floor, ceilings).
//
// Original: r_plane.h + r_plane.c (stub)

use crate::m_fixed::Fixed;
use crate::rendering::defs::Visplane;

/// Find or create visplane. Stub: returns null.
pub fn r_find_plane(_height: Fixed, _picnum: i32, _lightlevel: i32) -> *mut Visplane {
    std::ptr::null_mut()
}

/// Initialize planes. Stub.
pub fn r_init_planes() {}

/// Clear planes. Stub.
pub fn r_clear_planes() {}
