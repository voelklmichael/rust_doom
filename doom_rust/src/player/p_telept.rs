//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Teleporter logic.
//
// Original: p_telept.c

use crate::m_fixed::Fixed;
use crate::player::mobjs::{mobj_ptr_from_index, MobjIndex};
use crate::rendering::state;

use super::p_mobj::Mobj;
use super::p_map::p_teleport_move;
use super::p_spec::MO_TELEPORTMAN;

/// Execute teleport special. Original: EV_Teleport
/// Finds sectors with line tag, then a MO_TELEPORTMAN thing in those sectors.
/// Returns true if thing was teleported.
pub fn ev_teleport(line_idx: usize, _side: i32, thing: *mut Mobj) -> bool {
    if thing.is_null() {
        return false;
    }
    let tag = match state::with_state(|s| s.lines.get(line_idx).map(|l| l.tag as i32)) {
        Some(t) => t,
        None => return false,
    };
    let (dest_x, dest_y) = match find_teleport_dest(tag) {
        Some((x, y)) => (x, y),
        None => return false,
    };
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

/// Find a MO_TELEPORTMAN in a sector with the given tag. Returns (x, y) of first found.
fn find_teleport_dest(tag: i32) -> Option<(Fixed, Fixed)> {
    state::with_state(|s| {
        for (sec_idx, sec) in s.sectors.iter().enumerate() {
            if sec.tag as i32 != tag {
                continue;
            }
            if let Some((x, y)) = find_teleportman_in_sector(sec_idx) {
                return Some((x, y));
            }
        }
        None
    })
}

/// Find first MO_TELEPORTMAN in sector's thinglist. Returns (x, y).
fn find_teleportman_in_sector(sec_idx: usize) -> Option<(Fixed, Fixed)> {
    let mut mo_idx: Option<MobjIndex> =
        state::with_state(|s| s.sectors.get(sec_idx).and_then(|sec| sec.thinglist));
    while let Some(idx) = mo_idx {
        if idx.is_null() {
            break;
        }
        let ptr = mobj_ptr_from_index(idx);
        if ptr.is_null() {
            break;
        }
        if unsafe { (*ptr).type_ } == MO_TELEPORTMAN {
            return Some(unsafe { ((*ptr).x, (*ptr).y) });
        }
        mo_idx = crate::player::mobjs::mobj_index_from_ptr(unsafe { (*ptr).snext });
    }
    None
}

// NOTE: P_TeleportMove is in p_map.rs
