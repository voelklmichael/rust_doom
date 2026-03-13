//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement, collision, shooting, aiming.
//
// Original: p_map.c (stub - full impl requires blockmap)

use crate::m_fixed::Fixed;
use crate::rendering::defs::Line;

use super::p_mobj::Mobj;

/// MAXSPECIALCROSS - spechit buffer size (from p_local.h P_MAP).
pub const MAXSPECIALCROSS: usize = 20;

/// P_Map globals - set during movement checks.
pub static mut FLOATOK: bool = false;
pub static mut TMFLOORZ: Fixed = 0;
pub static mut TMCEILINGZ: Fixed = 0;
pub static mut CEILINGLINE: *mut Line = std::ptr::null_mut();
pub static mut SPECHIT: [*mut Line; MAXSPECIALCROSS] = [std::ptr::null_mut(); MAXSPECIALCROSS];
pub static mut NUMSPECHIT: i32 = 0;
pub static mut LINETARGET: *mut Mobj = std::ptr::null_mut();

/// Check if thing can occupy position (x, y). Original: P_CheckPosition
/// Stub: returns false until blockmap is implemented.
pub fn p_check_position(_thing: *mut Mobj, _x: Fixed, _y: Fixed) -> bool {
    false
}

/// Try to move thing to (x, y). Original: P_TryMove
/// Stub: returns false until blockmap is implemented.
pub fn p_try_move(_thing: *mut Mobj, _x: Fixed, _y: Fixed) -> bool {
    false
}

/// Teleport thing to (x, y). Original: P_TeleportMove
/// Stub: returns false until blockmap is implemented.
pub fn p_teleport_move(_thing: *mut Mobj, _x: Fixed, _y: Fixed) -> bool {
    false
}

/// Slide thing along walls. Original: P_SlideMove
pub fn p_slide_move(_mo: *mut Mobj) {
    // TODO: requires P_TryMove, slide logic
}

/// Use lines in range. Original: P_UseLines
pub fn p_use_lines(_player: *mut std::ffi::c_void) {
    // TODO: requires player_t, P_BlockLinesIterator
}

/// Change sector (crush damage, etc.). Original: P_ChangeSector
pub fn p_change_sector(_sector: *mut std::ffi::c_void, _crunch: bool) -> bool {
    false
}

/// Aim line attack, return slope. Original: P_AimLineAttack
pub fn p_aim_line_attack(_t1: *mut Mobj, _angle: u32, _distance: Fixed) -> Fixed {
    0
}

/// Line attack (hitscan). Original: P_LineAttack
pub fn p_line_attack(
    _t1: *mut Mobj,
    _angle: u32,
    _distance: Fixed,
    _slope: Fixed,
    _damage: i32,
) {
    // TODO: requires P_PathTraverse, PIT_CheckLine, P_DamageMobj
}

/// Radius attack (explosion). Original: P_RadiusAttack
pub fn p_radius_attack(_spot: *mut Mobj, _source: *mut Mobj, _damage: i32) {
    // TODO: requires P_BlockThingsIterator, P_DamageMobj
}
