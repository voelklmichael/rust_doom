//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Special effects - texture animation, sector specials, line specials.
//
// Original: p_spec.h / p_spec.c (partial)

use crate::rendering::defs::{Line, ML_TWOSIDED, Sector};

use super::p_ceilng::ev_do_ceiling;
use super::p_doors::ev_do_door;
use super::p_floor::ev_do_floor;
use super::p_lights::{ev_start_light_flickering, ev_start_light_strobing};
use super::p_mobj::Mobj;
use super::p_plats::ev_do_plat;
use super::p_switch::p_use_special_line as p_switch_use_line;
use super::p_telept::ev_teleport;

/// MO_TELEPORTMAN - thing type for teleport destination (matches MT_TELEPORTMAN in info).
pub const MO_TELEPORTMAN: i32 = 5;

/// Return sector index on the other side of the line. None if not two-sided.
/// Original: getNextSector
pub fn get_next_sector(line_idx: usize, sec_idx: usize) -> Option<usize> {
    crate::rendering::state::with_state(|s| {
        let ld = s.lines.get(line_idx)?;
        if (ld.flags & ML_TWOSIDED) == 0 {
            return None;
        }
        let back_idx = ld.backsector_idx?;
        if ld.frontsector_idx == sec_idx {
            Some(back_idx)
        } else {
            Some(ld.frontsector_idx)
        }
    })
}

/// Legacy: get sector pointer from index (for code still using *mut Sector).
#[allow(dead_code)]
fn _sector_ptr_from_idx(_idx: usize) -> *mut Sector {
    std::ptr::null_mut()
}

/// Called when thing crosses a special line. Dispatches to EV_* stubs.
/// Original: P_CrossSpecialLine
pub fn p_cross_special_line(line_index: i32, oldside: i32, thing: *mut Mobj) {
    if thing.is_null() || line_index < 0 {
        return;
    }
    let line_idx = line_index as usize;
    ev_do_line_special(line_idx, oldside, thing);
}

/// Use (activate) a special line. Dispatches to p_switch or EV_* stubs.
/// Original: P_UseSpecialLine
pub fn p_use_special_line(thing: *mut Mobj, line_idx: usize) -> bool {
    if thing.is_null() {
        return false;
    }
    p_switch_use_line(thing, line_idx) || ev_do_line_special(line_idx, 0, thing)
}

/// Shoot (projectile hits) a special line. Original: P_ShootSpecialLine
pub fn p_shoot_special_line(thing: *mut Mobj, line_idx: usize) -> bool {
    if thing.is_null() {
        return false;
    }
    ev_do_line_special(line_idx, 0, thing)
}

/// Dispatch line special to appropriate EV_* function.
fn ev_do_line_special(line_idx: usize, side: i32, thing: *mut Mobj) -> bool {
    let special = crate::rendering::state::with_state(|s| {
        s.lines.get(line_idx).map(|l| l.special as i32)
    });
    let special = match special {
        Some(s) => s,
        None => return false,
    };
    match special {
        1..=23 => ev_do_floor(line_idx, special),
        26..=28 => ev_do_ceiling(line_idx, special),
        29..=34 => ev_do_door(line_idx, special),
        35..=38 => ev_do_plat(line_idx, special, 0),
        8 | 9 => ev_start_light_strobing(line_idx),
        10 | 11 => ev_start_light_flickering(line_idx),
        39 | 97 | 125 | 126 => ev_teleport(line_idx, side, thing),
        52 => {
            crate::game::g_game::g_exit_level();
            true
        }
        124 => {
            crate::game::g_game::g_secret_exit_level();
            true
        }
        _ => p_switch_use_line(thing, line_idx),
    }
}

// TODO: twoSided, getSector, getSide - require sector.lines array (populated in full p_setup)
// TODO: P_InitPicAnims, P_SpawnSpecials, P_UpdateSpecials - require animdefs, w_wad
// TODO: P_PlayerInSpecialSector - require player_t
// TODO: P_FindLowestFloorSurrounding, P_FindHighestFloorSurrounding, etc.
// TODO: EV_DoDonut, EV_DoFloor, EV_DoCeiling, etc.
