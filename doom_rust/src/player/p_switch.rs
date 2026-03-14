//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Switch/button logic - usable linedefs.
//
// Original: p_switch.c (stub)

use crate::rendering::defs::Line;
use super::p_mobj::Mobj;

/// Use (activate) a special line. Original: P_UseSpecialLine
/// Returns true if line was activated.
pub fn p_use_special_line(
    _thing: *mut Mobj,
    _line: *const Line,
) -> bool {
    let _ = (_thing, _line);
    false
}

/// Change switch texture to "on" state. Original: P_ChangeSwitchTexture
pub fn p_change_switch_texture(
    _line: *const Line,
    _useagain: bool,
) {
    let _ = (_line, _useagain);
}

// TODO: Button list, switch animation - require g_game, deh_main, s_sound
