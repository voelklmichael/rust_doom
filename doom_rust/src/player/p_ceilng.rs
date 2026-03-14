//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Ceiling movement (raising/lowering sectors).
//
// Original: p_ceilng.c (stub)

use crate::rendering::defs::Line;

/// Execute ceiling special. Original: EV_DoCeiling
/// Returns true if a ceiling mover was started.
pub fn ev_do_ceiling(
    _line: *const Line,
    _ceilingtype: i32,
) -> bool {
    let _ = (_line, _ceilingtype);
    false
}

// TODO: T_MoveCeiling (thinker), EV_DoCeiling types - require thinker_t, s_sound
