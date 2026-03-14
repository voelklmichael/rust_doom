//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Floor movement (raising/lowering sectors).
//
// Original: p_floor.c

use crate::game::d_think::{ActionF, Thinker};
use crate::m_fixed::{Fixed, FRACUNIT};
use crate::rendering::defs::{Line, Sector};
use crate::z_zone::{z_malloc, PU_LEVSPEC};

use super::p_tick::{p_add_thinker, p_remove_thinker};

/// Floor mover speed. Original: FLOORSPEED
const FLOORSPEED: Fixed = FRACUNIT;

/// Floor mover types (minimal subset).
pub const FLOORTYPE_LOWER_TO_LOWEST: i32 = 1;
pub const FLOORTYPE_LOWER: i32 = 2;
pub const FLOORTYPE_RAISE_TO_NEAREST: i32 = 4;
pub const FLOORTYPE_RAISE: i32 = 5;
pub const FLOORTYPE_RAISE_TO_LOWEST: i32 = 6;

/// Floor mover thinker. Original: floormove_t
#[repr(C)]
pub struct FloorMover {
    pub thinker: Thinker,
    pub floortype: i32,
    pub crush: bool,
    pub sector: *mut Sector,
    pub direction: i32, // 1 = down, -1 = up
    pub newspecial: i32,
    pub texture: i16,
    pub floordestheight: Fixed,
    pub speed: Fixed,
}

/// Find lowest floor height among sectors touching this one. Original: P_FindLowestFloorSurrounding
pub fn find_lowest_floor_surrounding(sec: *mut Sector) -> Fixed {
    if sec.is_null() {
        return 0;
    }
    let mut minheight = unsafe { (*sec).floorheight };
    let linecount = unsafe { (*sec).linecount } as usize;
    let lines = unsafe { (*sec).lines };
    if lines.is_null() {
        return minheight;
    }
    for i in 0..linecount {
        let line = unsafe { *lines.add(i) };
        if line.is_null() {
            continue;
        }
        let other = super::p_spec::get_next_sector(line, sec);
        if !other.is_null() {
            let h = unsafe { (*other).floorheight };
            if h < minheight {
                minheight = h;
            }
        }
    }
    minheight
}

/// Find highest floor height among sectors touching this one. Original: P_FindHighestFloorSurrounding
pub fn find_highest_floor_surrounding(sec: *mut Sector) -> Fixed {
    if sec.is_null() {
        return 0;
    }
    let mut maxheight = unsafe { (*sec).floorheight };
    let linecount = unsafe { (*sec).linecount } as usize;
    let lines = unsafe { (*sec).lines };
    if lines.is_null() {
        return maxheight;
    }
    for i in 0..linecount {
        let line = unsafe { *lines.add(i) };
        if line.is_null() {
            continue;
        }
        let other = super::p_spec::get_next_sector(line, sec);
        if !other.is_null() {
            let h = unsafe { (*other).floorheight };
            if h > maxheight {
                maxheight = h;
            }
        }
    }
    maxheight
}

/// Floor mover thinker. Original: T_MoveFloor
pub unsafe extern "C" fn t_move_floor(floor: *mut ()) {
    let fm = floor as *mut FloorMover;
    if fm.is_null() {
        return;
    }
    let sector = (*fm).sector;
    if sector.is_null() {
        p_remove_thinker(&mut (*fm).thinker as *mut Thinker);
        return;
    }
    let moved = if (*fm).direction > 0 {
        // Moving down
        let mut res = false;
        let newheight = (*sector).floorheight - (*fm).speed;
        if newheight < (*fm).floordestheight {
            (*sector).floorheight = (*fm).floordestheight;
            res = true;
        } else {
            (*sector).floorheight = newheight;
            res = (*fm).floordestheight == newheight;
        }
        res
    } else {
        // Moving up
        let mut res = false;
        let newheight = (*sector).floorheight + (*fm).speed;
        if newheight > (*fm).floordestheight {
            (*sector).floorheight = (*fm).floordestheight;
            res = true;
        } else {
            (*sector).floorheight = newheight;
            res = (*fm).floordestheight == newheight;
        }
        res
    };
    if moved {
        (*sector).specialdata = std::ptr::null_mut();
        p_remove_thinker(&mut (*fm).thinker as *mut Thinker);
        // TODO: S_StartSound for floor stop
    }
}

/// Execute floor special. Original: EV_DoFloor
/// Returns true if a floor mover was started.
pub fn ev_do_floor(line: *const Line, floortype: i32) -> bool {
    if line.is_null() {
        return false;
    }
    let sector = unsafe { (*line).frontsector };
    if sector.is_null() {
        return false;
    }
    // Check if sector already has a floor mover
    if !unsafe { (*sector).specialdata }.is_null() {
        return false;
    }
    let (floordestheight, direction) = match floortype {
        FLOORTYPE_LOWER_TO_LOWEST => {
            let dest = find_lowest_floor_surrounding(sector);
            if dest >= unsafe { (*sector).floorheight } {
                return false;
            }
            (dest, 1)
        }
        FLOORTYPE_LOWER => {
            let dest = unsafe { (*sector).floorheight } - 8 * FRACUNIT;
            if dest < 0 {
                return false;
            }
            (dest, 1)
        }
        FLOORTYPE_RAISE_TO_NEAREST => {
            let dest = find_highest_floor_surrounding(sector);
            if dest <= unsafe { (*sector).floorheight } {
                return false;
            }
            (dest, -1)
        }
        FLOORTYPE_RAISE_TO_LOWEST => {
            let dest = find_lowest_floor_surrounding(sector);
            if dest <= unsafe { (*sector).floorheight } {
                return false;
            }
            (dest, -1)
        }
        FLOORTYPE_RAISE => {
            let dest = unsafe { (*sector).floorheight } + 8 * FRACUNIT;
            (dest, -1)
        }
        _ => return false,
    };
    let fm = z_malloc(
        std::mem::size_of::<FloorMover>(),
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut FloorMover;
    if fm.is_null() {
        return false;
    }
    unsafe {
        (*fm).thinker = Thinker {
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
            function: ActionF {
                acp1: t_move_floor,
            },
        };
        (*fm).floortype = floortype;
        (*fm).crush = false;
        (*fm).sector = sector;
        (*fm).direction = direction;
        (*fm).newspecial = 0;
        (*fm).texture = 0;
        (*fm).floordestheight = floordestheight;
        (*fm).speed = FLOORSPEED;
    }
    unsafe {
        (*sector).specialdata = fm as *mut std::ffi::c_void;
    }
    p_add_thinker(unsafe { &mut (*fm).thinker as *mut Thinker });
    true
}

/// Execute donut (raise floor, lower surrounding). Original: EV_DoDonut
pub fn ev_do_donut(_sector: *mut Sector) -> bool {
    let _ = _sector;
    false
}
