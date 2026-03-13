//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Map Objects - minimal stub for sound positioning.
//
// Original: p_mobj.h (partial)

use crate::m_fixed::Fixed;

/// Minimal mobj stub for sound - only x, y needed for stereo/volume.
#[derive(Debug, Clone, Copy)]
pub struct Mobj {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub angle: u32,
}
