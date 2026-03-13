//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Special effects - texture animation, sector specials, line specials.
//
// Original: p_spec.h / p_spec.c (partial)

use crate::rendering::defs::{Line, ML_TWOSIDED, Sector};

/// MO_TELEPORTMAN - thing type for teleport destination.
pub const MO_TELEPORTMAN: i32 = 14;

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

// TODO: twoSided, getSector, getSide - require sector.lines array (populated in full p_setup)
// TODO: P_InitPicAnims, P_SpawnSpecials, P_UpdateSpecials - require animdefs, w_wad
// TODO: P_UseSpecialLine, P_ShootSpecialLine, P_CrossSpecialLine - require g_game
// TODO: P_PlayerInSpecialSector - require player_t
// TODO: P_FindLowestFloorSurrounding, P_FindHighestFloorSurrounding, etc.
// TODO: EV_DoDonut, EV_DoFloor, EV_DoCeiling, etc.
