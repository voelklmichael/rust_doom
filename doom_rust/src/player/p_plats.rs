//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Platform movement (up/down, by sector).
//
// Original: p_plats.c (stub)

use crate::rendering::defs::Line;

/// Execute platform special. Original: EV_DoPlat
/// Returns true if a platform mover was started.
pub fn ev_do_plat(
    _line: *const Line,
    _plattype: i32,
    _amount: i32,
) -> bool {
    let _ = (_line, _plattype, _amount);
    false
}

// TODO: T_PlatRaise (thinker) - require thinker_t, s_sound
