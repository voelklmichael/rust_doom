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

/// Return sector on the other side of the line. NULL if not two-sided.
/// Original: getNextSector
pub fn get_next_sector(line: *const Line, sec: *const Sector) -> *mut Sector {
    if line.is_null() || sec.is_null() {
        return std::ptr::null_mut();
    }
    let ld = unsafe { &*line };
    if (ld.flags & ML_TWOSIDED) == 0 {
        return std::ptr::null_mut();
    }
    let front = ld.frontsector;
    let back = ld.backsector;
    if front.is_null() || back.is_null() {
        return std::ptr::null_mut();
    }
    if std::ptr::eq(front, sec) {
        back
    } else {
        front
    }
}

/// Called when thing crosses a special line. Dispatches to EV_* stubs.
/// Original: P_CrossSpecialLine
pub fn p_cross_special_line(line_index: i32, oldside: i32, thing: *mut Mobj) {
    if thing.is_null() {
        return;
    }
    let lines = unsafe { crate::rendering::state::LINES };
    if lines.is_null() || line_index < 0 {
        return;
    }
    let line = unsafe { lines.add(line_index as usize) };
    ev_do_line_special(line, oldside, thing);
}

/// Use (activate) a special line. Dispatches to p_switch or EV_* stubs.
/// Original: P_UseSpecialLine
pub fn p_use_special_line(thing: *mut Mobj, line: *const Line) -> bool {
    if thing.is_null() || line.is_null() {
        return false;
    }
    p_switch_use_line(thing, line) || ev_do_line_special(line, 0, thing)
}

/// Shoot (projectile hits) a special line. Original: P_ShootSpecialLine
pub fn p_shoot_special_line(thing: *mut Mobj, line: *const Line) -> bool {
    if thing.is_null() || line.is_null() {
        return false;
    }
    ev_do_line_special(line, 0, thing)
}

/// Dispatch line special to appropriate EV_* function. Stub - calls ev_do_* which return false.
fn ev_do_line_special(line: *const Line, side: i32, thing: *mut Mobj) -> bool {
    if line.is_null() {
        return false;
    }
    let special = unsafe { (*line).special as i32 };
    let _ = (side, thing);
    match special {
        1..=23 => ev_do_floor(line, special),
        26..=28 => ev_do_ceiling(line, special),
        29..=34 => ev_do_door(line, special),
        35..=38 => ev_do_plat(line, special, 0),
        8 | 9 => ev_start_light_strobing(line),
        10 | 11 => ev_start_light_flickering(line),
        39 | 97 | 125 | 126 => ev_teleport(line, side, thing),
        _ => p_switch_use_line(thing, line),
    }
}

// TODO: twoSided, getSector, getSide - require sector.lines array (populated in full p_setup)
// TODO: P_InitPicAnims, P_SpawnSpecials, P_UpdateSpecials - require animdefs, w_wad
// TODO: P_PlayerInSpecialSector - require player_t
// TODO: P_FindLowestFloorSurrounding, P_FindHighestFloorSurrounding, etc.
// TODO: EV_DoDonut, EV_DoFloor, EV_DoCeiling, etc.
