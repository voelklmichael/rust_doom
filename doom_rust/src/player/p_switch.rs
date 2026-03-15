//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Switch/button logic - usable linedefs.
//
// Original: p_switch.c (partial)

use super::p_mobj::Mobj;

/// Use (activate) a special line. Original: P_UseSpecialLine
/// Returns true if line was activated.
pub fn p_use_special_line(thing: *mut Mobj, line_idx: usize) -> bool {
    if thing.is_null() {
        return false;
    }
    let special = crate::rendering::state::with_state(|s| {
        s.lines.get(line_idx).map(|l| l.special as i32)
    });
    let special = match special {
        Some(s) => s,
        None => return false,
    };
    match special {
        11 => {
            p_change_switch_texture(line_idx, false);
            crate::game::g_game::g_exit_level();
            true
        }
        51 => {
            p_change_switch_texture(line_idx, false);
            crate::game::g_game::g_secret_exit_level();
            true
        }
        _ => false,
    }
}

/// Change switch texture to "on" state. Original: P_ChangeSwitchTexture
pub fn p_change_switch_texture(_line_idx: usize, _useagain: bool) {
    let _ = (_line_idx, _useagain);
}

// TODO: Button list, switch animation - require g_game, deh_main, s_sound
