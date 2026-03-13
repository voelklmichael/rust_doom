//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Map Objects - things in the world (monsters, items, etc.).
//
// Original: p_mobj.h (partial)

use crate::m_fixed::Fixed;

/// MF_SHADOW - render as shadow (invisible player).
pub const MF_SHADOW: i32 = 32;
/// FF_FRAMEMASK - frame index mask.
pub const FF_FRAMEMASK: i32 = 0x7fff;
/// FF_FULLBRIGHT - full bright sprite.
pub const FF_FULLBRIGHT: i32 = 0x8000;

/// Map object - thing in the world (monster, item, projectile, etc.).
#[repr(C)]
#[derive(Debug)]
pub struct Mobj {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub angle: u32,
    /// Next mobj in sector thing list.
    pub snext: *mut Mobj,
    /// Sprite number (index into sprites[]).
    pub sprite: i32,
    /// Frame number (FF_FRAMEMASK) and flags (FF_FULLBRIGHT).
    pub frame: i32,
    /// Mobj flags (MF_SHADOW, etc.).
    pub flags: i32,
}
