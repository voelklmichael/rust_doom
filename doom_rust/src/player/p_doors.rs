//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Door movement (vertical, generic).
//
// Original: p_doors.c (stub)

use crate::rendering::defs::Line;

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

// TODO: T_VerticalDoor (thinker) - require thinker_t, s_sound
