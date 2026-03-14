//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Door movement (vertical, generic).
//
// Original: p_doors.c (stub)

use crate::game::d_think::Thinker;
use crate::m_fixed::Fixed;
use crate::rendering::defs::{Line, Sector};

/// Vertical door thinker. Original: vldoor_t
#[repr(C)]
pub struct Vldoor {
    pub thinker: Thinker,
    pub doortype: i32,
    pub sector: *mut Sector,
    pub topheight: Fixed,
    pub speed: Fixed,
    pub direction: i32,
    pub topwait: i32,
    pub topcountdown: i32,
}

/// Door thinker. Original: T_VerticalDoor (stub - no-op for savegame compatibility)
pub unsafe extern "C" fn t_vertical_door(door: *mut ()) {
    let _ = door;
}

/// Execute door special. Original: EV_DoDoor
/// Returns true if a door mover was started.
pub fn ev_do_door(
    _line: *const Line,
    _doortype: i32,
) -> bool {
    let _ = (_line, _doortype);
    false
}

/// Execute locked door. Original: EV_DoLockedDoor
pub fn ev_do_locked_door(
    _line: *const Line,
    _doortype: i32,
    _key: i32,
) -> bool {
    let _ = (_line, _doortype, _key);
    false
}

