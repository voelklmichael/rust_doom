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
/// MF_NOSECTOR - don't use sector links (invisible but touchable).
pub const MF_NOSECTOR: i32 = 8;
/// MF_NOBLOCKMAP - don't use blocklinks (inert but displayable).
pub const MF_NOBLOCKMAP: i32 = 16;
/// MF_SOLID - blocks movement.
pub const MF_SOLID: i32 = 2;
/// MF_SPECIAL - touch activates (pickups).
pub const MF_SPECIAL: i32 = 1;
/// MF_SHOOTABLE - can be damaged.
pub const MF_SHOOTABLE: i32 = 4;
/// MF_PICKUP - player can pick up items.
pub const MF_PICKUP: i32 = 0x800;
/// MF_NOCLIP - walk through walls.
pub const MF_NOCLIP: i32 = 0x1000;
/// MF_TELEPORT - don't cross special lines.
pub const MF_TELEPORT: i32 = 0x8000;
/// MF_DROPOFF - allow dropoffs.
pub const MF_DROPOFF: i32 = 0x400;
/// MF_FLOAT - floating (no gravity).
pub const MF_FLOAT: i32 = 0x4000;
/// MF_MISSILE - projectile.
pub const MF_MISSILE: i32 = 0x10000;
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
    /// Next mobj in sector thing list.
    pub snext: *mut Mobj,
    /// Prev mobj in sector thing list.
    pub sprev: *mut Mobj,
    pub angle: u32,
    /// Sprite number (index into sprites[]).
    pub sprite: i32,
    /// Frame number (FF_FRAMEMASK) and flags (FF_FULLBRIGHT).
    pub frame: i32,
    /// Next mobj in blockmap block.
    pub bnext: *mut Mobj,
    /// Prev mobj in blockmap block.
    pub bprev: *mut Mobj,
    /// Subsector containing this thing. Cast from *mut c_void to *mut Subsector.
    pub subsector: *mut std::ffi::c_void,
    /// Floor height at current position.
    pub floorz: Fixed,
    /// Ceiling height at current position.
    pub ceilingz: Fixed,
    /// Mobj flags (MF_SHADOW, MF_NOSECTOR, MF_NOBLOCKMAP, etc.).
    pub flags: i32,
    /// Radius for movement checking.
    pub radius: Fixed,
    /// Height for movement checking.
    pub height: Fixed,
    /// Momentum X.
    pub momx: Fixed,
    /// Momentum Y.
    pub momy: Fixed,
    /// Momentum Z.
    pub momz: Fixed,
}
