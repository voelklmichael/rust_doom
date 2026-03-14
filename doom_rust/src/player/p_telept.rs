//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Teleporter logic.
//
// Original: p_telept.c

use crate::rendering::defs::{Line, Sector};
use crate::rendering::state;

use super::p_mobj::Mobj;
use super::p_map::p_teleport_move;
use super::p_spec::MO_TELEPORTMAN;

/// Execute teleport special. Original: EV_Teleport
/// Finds sectors with line tag, then a MO_TELEPORTMAN thing in those sectors.
/// Returns true if thing was teleported.
pub fn ev_teleport(line: *const Line, _side: i32, thing: *mut Mobj) -> bool {
    if line.is_null() || thing.is_null() {
        return false;
    }
    let tag = unsafe { (*line).tag } as i32;
    let dest = find_teleport_dest(tag);
    if dest.is_null() {
        return false;
    }
    let (dest_x, dest_y) = unsafe { ((*dest).x, (*dest).y) };
    if !p_teleport_move(thing, dest_x, dest_y) {
        return false;
    }
    unsafe {
        (*thing).z = (*thing).floorz;
        (*thing).momx = 0;
        (*thing).momy = 0;
        (*thing).momz = 0;
    }
    // TODO: S_StartSound(thing, sfx_telept)
    true
}

/// Find a MO_TELEPORTMAN in a sector with the given tag. Returns first found.
fn find_teleport_dest(tag: i32) -> *mut Mobj {
    let sectors = unsafe { state::SECTORS };
    let numsectors = unsafe { state::NUMSECTORS };
    if sectors.is_null() || numsectors <= 0 {
        return std::ptr::null_mut();
    }
    for i in 0..(numsectors as usize) {
        let sec = unsafe { sectors.add(i) };
        if unsafe { (*sec).tag as i32 } != tag {
            continue;
        }
        let mo = find_teleportman_in_sector(sec);
        if !mo.is_null() {
            return mo;
        }
    }
    std::ptr::null_mut()
}

/// Find first MO_TELEPORTMAN in sector's thinglist.
fn find_teleportman_in_sector(sec: *mut Sector) -> *mut Mobj {
    if sec.is_null() {
        return std::ptr::null_mut();
    }
    let mut mo = unsafe { (*sec).thinglist };
    while !mo.is_null() {
        if unsafe { (*mo).type_ } == MO_TELEPORTMAN {
            return mo;
        }
        mo = unsafe { (*mo).snext };
    }
    std::ptr::null_mut()
}

// NOTE: P_TeleportMove is in p_map.rs
