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
    pub sector_idx: usize,
    pub direction: i32, // 1 = down, -1 = up
    pub newspecial: i32,
    pub texture: i16,
    pub floordestheight: Fixed,
    pub speed: Fixed,
}

/// Find lowest floor height among sectors touching this one. Original: P_FindLowestFloorSurrounding
pub fn find_lowest_floor_surrounding(sec_idx: usize) -> Fixed {
    crate::rendering::state::with_state(|s| {
        let sec = match s.sectors.get(sec_idx) {
            Some(sec) => sec,
            None => return 0,
        };
        let mut minheight = sec.floorheight;
        for &line_idx in &sec.lines {
            if let Some(other_idx) = super::p_spec::get_next_sector(line_idx, sec_idx) {
                if let Some(other) = s.sectors.get(other_idx) {
                    if other.floorheight < minheight {
                        minheight = other.floorheight;
                    }
                }
            }
        }
        minheight
    })
}

/// Find highest floor height among sectors touching this one. Original: P_FindHighestFloorSurrounding
pub fn find_highest_floor_surrounding(sec_idx: usize) -> Fixed {
    crate::rendering::state::with_state(|s| {
        let sec = match s.sectors.get(sec_idx) {
            Some(sec) => sec,
            None => return 0,
        };
        let mut maxheight = sec.floorheight;
        for &line_idx in &sec.lines {
            if let Some(other_idx) = super::p_spec::get_next_sector(line_idx, sec_idx) {
                if let Some(other) = s.sectors.get(other_idx) {
                    if other.floorheight > maxheight {
                        maxheight = other.floorheight;
                    }
                }
            }
        }
        maxheight
    })
}

/// Floor mover thinker. Original: T_MoveFloor
pub unsafe extern "C" fn t_move_floor(floor: *mut ()) {
    let fm = floor as *mut FloorMover;
    if fm.is_null() {
        return;
    }
    let sector_idx = (*fm).sector_idx;
    let moved = crate::rendering::state::with_state_mut(|s| {
        let sector = match s.sectors.get_mut(sector_idx) {
            Some(sec) => sec,
            None => return false,
        };
        if (*fm).direction > 0 {
            let newheight = sector.floorheight - (*fm).speed;
            if newheight < (*fm).floordestheight {
                sector.floorheight = (*fm).floordestheight;
                true
            } else {
                sector.floorheight = newheight;
                (*fm).floordestheight == newheight
            }
        } else {
            let newheight = sector.floorheight + (*fm).speed;
            if newheight > (*fm).floordestheight {
                sector.floorheight = (*fm).floordestheight;
                true
            } else {
                sector.floorheight = newheight;
                (*fm).floordestheight == newheight
            }
        }
    });
    if moved {
        crate::rendering::state::with_state_mut(|s| {
            if let Some(sec) = s.sectors.get_mut(sector_idx) {
                sec.specialdata = None;
            }
        });
        p_remove_thinker(&mut (*fm).thinker as *mut Thinker);
    }
}

/// Execute floor special. Original: EV_DoFloor
/// Returns true if a floor mover was started.
pub fn ev_do_floor(line_idx: usize, floortype: i32) -> bool {
    let (sector_idx, floorheight) = match crate::rendering::state::with_state(|s| {
        let line = s.lines.get(line_idx)?;
        let sec_idx = line.frontsector_idx;
        let sec = s.sectors.get(sec_idx)?;
        if sec.specialdata.is_some() {
            return None; // already has floor mover
        }
        Some((sec_idx, sec.floorheight))
    }) {
        Some(x) => x,
        None => return false,
    };
    let (floordestheight, direction) = match floortype {
        FLOORTYPE_LOWER_TO_LOWEST => {
            let dest = find_lowest_floor_surrounding(sector_idx);
            if dest >= floorheight {
                return false;
            }
            (dest, 1)
        }
        FLOORTYPE_LOWER => {
            let dest = floorheight - 8 * FRACUNIT;
            if dest < 0 {
                return false;
            }
            (dest, 1)
        }
        FLOORTYPE_RAISE_TO_NEAREST => {
            let dest = find_highest_floor_surrounding(sector_idx);
            if dest <= floorheight {
                return false;
            }
            (dest, -1)
        }
        FLOORTYPE_RAISE_TO_LOWEST => {
            let dest = find_lowest_floor_surrounding(sector_idx);
            if dest <= floorheight {
                return false;
            }
            (dest, -1)
        }
        FLOORTYPE_RAISE => {
            let dest = floorheight + 8 * FRACUNIT;
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
        (*fm).sector_idx = sector_idx;
        (*fm).direction = direction;
        (*fm).newspecial = 0;
        (*fm).texture = 0;
        (*fm).floordestheight = floordestheight;
        (*fm).speed = FLOORSPEED;
    }
    crate::rendering::state::with_state_mut(|s| {
        if let Some(sec) = s.sectors.get_mut(sector_idx) {
            sec.specialdata = Some(fm as usize);
        }
    });
    p_add_thinker(unsafe { &mut (*fm).thinker as *mut Thinker });
    true
}

/// Execute donut (raise floor, lower surrounding). Original: EV_DoDonut
pub fn ev_do_donut(_sector: *mut Sector) -> bool {
    let _ = _sector;
    false
}
