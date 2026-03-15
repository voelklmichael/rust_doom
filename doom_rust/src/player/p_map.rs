//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement, collision, shooting, aiming.
//
// Original: p_map.c

use crate::m_fixed::{fixed_mul, Fixed, FRACUNIT};
use std::sync::{Mutex, OnceLock};
use crate::rendering::defs::{Line, ML_BLOCKING, ML_BLOCKMONSTERS, ML_TWOSIDED};
use crate::rendering::state;
use crate::rendering::{r_point_in_subsector, r_point_to_angle2, r_main};
use crate::rendering::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};

use super::p_maputl::{
    p_block_lines_iterator, p_block_things_iterator, p_line_opening, p_point_on_line_side,
    p_set_thing_position, p_unset_thing_position, p_path_traverse,
};
use super::p_mobj::{
    Mobj, MF_DROPOFF, MF_FLOAT, MF_NOCLIP, MF_PICKUP, MF_SOLID, MF_SPECIAL, MF_SHOOTABLE,
    MF_TELEPORT,
};
use super::p_spec::p_cross_special_line;
use super::{MAPBLOCKSHIFT, MAXRADIUS};

/// MAXSPECIALCROSS - spechit buffer size (from p_local.h P_MAP).
pub const MAXSPECIALCROSS: usize = 20;

// =============================================================================
// PMapState - thread-safe via OnceLock + Mutex
// =============================================================================

static P_MAP_STATE: OnceLock<Mutex<PMapState>> = OnceLock::new();

/// Safety: Raw pointers in PMapState are only used while holding the Mutex lock.
unsafe impl Send for PMapState {}

pub struct PMapState {
    pub tmbbox: [Fixed; 4],
    pub tmthing: *mut Mobj,
    pub tmflags: i32,
    pub tmx: Fixed,
    pub tmy: Fixed,
    pub bestslidefrac: Fixed,
    pub secondslidefrac: Fixed,
    pub bestslideline: *mut Line,
    pub secondslideline: *mut Line,
    pub slidemo: *mut Mobj,
    pub tmxmove: Fixed,
    pub tmymove: Fixed,
    // P_Map globals - set during movement checks.
    pub floatok: bool,
    pub tmfloorz: Fixed,
    pub tmceilingz: Fixed,
    pub tmdropoffz: Fixed,
    pub ceilingline: *mut Line,
    pub spechit: [*mut Line; MAXSPECIALCROSS],
    pub numspechit: i32,
    pub linetarget: *mut Mobj,
}

fn get_p_map_state() -> &'static Mutex<PMapState> {
    P_MAP_STATE.get_or_init(|| {
        Mutex::new(PMapState {
            tmbbox: [0; 4],
            tmthing: std::ptr::null_mut(),
            tmflags: 0,
            tmx: 0,
            tmy: 0,
            bestslidefrac: 0,
            secondslidefrac: 0,
            bestslideline: std::ptr::null_mut(),
            secondslideline: std::ptr::null_mut(),
            slidemo: std::ptr::null_mut(),
            tmxmove: 0,
            tmymove: 0,
            floatok: false,
            tmfloorz: 0,
            tmceilingz: 0,
            tmdropoffz: 0,
            ceilingline: std::ptr::null_mut(),
            spechit: [std::ptr::null_mut(); MAXSPECIALCROSS],
            numspechit: 0,
            linetarget: std::ptr::null_mut(),
        })
    })
}

/// Access PMapState.
pub fn with_p_map_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut PMapState) -> R,
{
    let mut guard = get_p_map_state().lock().unwrap();
    f(&mut guard)
}

fn pit_check_line(ld: *mut Line) -> bool {
    if ld.is_null() {
        return true;
    }
    let (tmbbox, tmthing, tmflags) = with_p_map_state(|st| (st.tmbbox, st.tmthing, st.tmflags));
    let ld_ref = unsafe { &*ld };
    if tmbbox[BOXRIGHT] <= ld_ref.bbox[BOXLEFT]
        || tmbbox[BOXLEFT] >= ld_ref.bbox[BOXRIGHT]
        || tmbbox[BOXTOP] <= ld_ref.bbox[BOXBOTTOM]
        || tmbbox[BOXBOTTOM] >= ld_ref.bbox[BOXTOP]
    {
        return true;
    }
    if super::p_maputl::p_box_on_line_side(&tmbbox, ld) != -1 {
        return true;
    }
    if ld_ref.backsector.is_null() {
        return false;
    }
    if (tmflags & super::p_mobj::MF_MISSILE) == 0 {
        if (ld_ref.flags & ML_BLOCKING) != 0 {
            return false;
        }
        // Block monsters on ML_BLOCKMONSTERS lines (assume non-player for now)
        if (ld_ref.flags & ML_BLOCKMONSTERS) != 0 {
            return false;
        }
    }
    p_line_opening(ld);
    let (opentop, openbottom, lowfloor) = super::p_maputl::with_p_maputl_state(|st| {
        (st.opentop, st.openbottom, st.lowfloor)
    });
    with_p_map_state(|st| {
        if opentop < st.tmceilingz {
            st.tmceilingz = opentop;
            st.ceilingline = ld;
        }
        if openbottom > st.tmfloorz {
            st.tmfloorz = openbottom;
        }
        if lowfloor < st.tmdropoffz {
            st.tmdropoffz = lowfloor;
        }
        if ld_ref.special != 0 {
            let n = st.numspechit as usize;
            if n < MAXSPECIALCROSS {
                st.spechit[n] = ld;
                st.numspechit += 1;
            }
        }
    });
    true
}

fn pit_check_thing(thing: *mut Mobj) -> bool {
    if thing.is_null() {
        return true;
    }
    let (tmthing, tmx, tmy, tmflags) =
        with_p_map_state(|st| (st.tmthing, st.tmx, st.tmy, st.tmflags));
    if (unsafe { (*thing).flags } & (MF_SOLID | MF_SPECIAL | MF_SHOOTABLE)) == 0 {
        return true;
    }
    let blockdist = unsafe { (*thing).radius + (*tmthing).radius };
    if (unsafe { (*thing).x } - tmx).abs() >= blockdist
        || (unsafe { (*thing).y } - tmy).abs() >= blockdist
    {
        return true;
    }
    if thing == tmthing {
        return true;
    }
    if (unsafe { (*thing).flags } & MF_SPECIAL) != 0 {
        let solid = (unsafe { (*thing).flags } & MF_SOLID) != 0;
        if (tmflags & MF_PICKUP) != 0 {
            super::p_inter::p_touch_special_thing(thing, tmthing);
        }
        return !solid;
    }
    !(unsafe { (*thing).flags } & MF_SOLID != 0)
}

/// Check if thing can occupy position (x, y). Original: P_CheckPosition
pub fn p_check_position(thing: *mut Mobj, x: Fixed, y: Fixed) -> bool {
    if thing.is_null() {
        return false;
    }
    let (bmaporgx, bmaporgy) = state::with_state(|s| (s.bmaporgx, s.bmaporgy));
    with_p_map_state(|st| {
        st.tmthing = thing;
        st.tmflags = unsafe { (*thing).flags };
        st.tmx = x;
        st.tmy = y;
        st.tmbbox[BOXBOTTOM as usize] = y - unsafe { (*thing).radius };
        st.tmbbox[BOXRIGHT as usize] = x + unsafe { (*thing).radius };
        st.tmbbox[BOXTOP as usize] = y + unsafe { (*thing).radius };
        st.tmbbox[BOXLEFT as usize] = x - unsafe { (*thing).radius };
    });
    let newsubsec = r_point_in_subsector(x, y);
    let sec = unsafe { (*newsubsec).sector };
    with_p_map_state(|st| {
        st.ceilingline = std::ptr::null_mut();
        st.tmfloorz = if sec.is_null() { 0 } else { unsafe { (*sec).floorheight } };
        st.tmdropoffz = st.tmfloorz;
        st.tmceilingz = if sec.is_null() { 0 } else { unsafe { (*sec).ceilingheight } };
        st.numspechit = 0;
    });
    r_main::with_r_main_state_mut(|rm| rm.validcount += 1);
    if (unsafe { (*thing).flags } & MF_NOCLIP) != 0 {
        return true;
    }
    let (boxleft, boxright, boxbottom, boxtop) =
        with_p_map_state(|st| (st.tmbbox[BOXLEFT as usize], st.tmbbox[BOXRIGHT as usize], st.tmbbox[BOXBOTTOM as usize], st.tmbbox[BOXTOP as usize]));
    let xl = (boxleft - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
    let xh = (boxright - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
    let yl = (boxbottom - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
    let yh = (boxtop - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
    for bx in xl..=xh {
        for by in yl..=yh {
            if !p_block_things_iterator(bx, by, pit_check_thing) {
                return false;
            }
        }
    }
    let (boxleft, boxright, boxbottom, boxtop) =
        with_p_map_state(|st| (st.tmbbox[BOXLEFT as usize], st.tmbbox[BOXRIGHT as usize], st.tmbbox[BOXBOTTOM as usize], st.tmbbox[BOXTOP as usize]));
    let xl = (boxleft - bmaporgx) >> MAPBLOCKSHIFT;
    let xh = (boxright - bmaporgx) >> MAPBLOCKSHIFT;
    let yl = (boxbottom - bmaporgy) >> MAPBLOCKSHIFT;
    let yh = (boxtop - bmaporgy) >> MAPBLOCKSHIFT;
    for bx in xl..=xh {
        for by in yl..=yh {
            if !p_block_lines_iterator(bx, by, pit_check_line) {
                return false;
            }
        }
    }
    true
}

/// Try to move thing to (x, y). Original: P_TryMove
pub fn p_try_move(thing: *mut Mobj, x: Fixed, y: Fixed) -> bool {
    if thing.is_null() {
        return false;
    }
    with_p_map_state(|st| st.floatok = false);
    if !p_check_position(thing, x, y) {
        return false;
    }
    let (tmfloorz, tmceilingz, tmdropoffz) =
        with_p_map_state(|st| (st.tmfloorz, st.tmceilingz, st.tmdropoffz));
    if (unsafe { (*thing).flags } & MF_NOCLIP) == 0 {
        if tmceilingz - tmfloorz < (unsafe { (*thing).height }) {
            return false;
        }
        with_p_map_state(|st| st.floatok = true);
        if (unsafe { (*thing).flags } & MF_TELEPORT) == 0
            && tmceilingz - (unsafe { (*thing).z }) < (unsafe { (*thing).height })
        {
            return false;
        }
        if (unsafe { (*thing).flags } & MF_TELEPORT) == 0
            && tmfloorz - (unsafe { (*thing).z }) > 24 * FRACUNIT
        {
            return false;
        }
        if (unsafe { (*thing).flags } & (MF_DROPOFF | MF_FLOAT)) == 0
            && tmfloorz - tmdropoffz > 24 * FRACUNIT
        {
            return false;
        }
    }
    p_unset_thing_position(thing);
    let oldx = unsafe { (*thing).x };
    let oldy = unsafe { (*thing).y };
    unsafe {
        (*thing).floorz = tmfloorz;
        (*thing).ceilingz = tmceilingz;
        (*thing).x = x;
        (*thing).y = y;
    }
    p_set_thing_position(thing);
    if (unsafe { (*thing).flags } & (MF_TELEPORT | MF_NOCLIP)) == 0 {
        let lines = state::with_state(|s| s.lines);
        with_p_map_state(|st| {
            while st.numspechit > 0 {
                st.numspechit -= 1;
                let idx = st.numspechit as usize;
                let ld = st.spechit[idx];
                if !ld.is_null() && !lines.is_null() {
                    let line_index = ld.offset_from(lines) as i32;
                    let side = p_point_on_line_side(unsafe { (*thing).x }, unsafe { (*thing).y }, ld);
                    let oldside = p_point_on_line_side(oldx, oldy, ld);
                    if side != oldside && unsafe { (*ld).special } != 0 {
                        p_cross_special_line(line_index, oldside, thing);
                    }
                }
            }
        });
    }
    true
}

/// Teleport thing to (x, y). Original: P_TeleportMove
pub fn p_teleport_move(thing: *mut Mobj, x: Fixed, y: Fixed) -> bool {
    if thing.is_null() {
        return false;
    }
    // Simplified: no PIT_StompThing - just move
    p_unset_thing_position(thing);
    let newsubsec = r_point_in_subsector(x, y);
    let sec = unsafe { (*newsubsec).sector };
    unsafe {
        (*thing).floorz = if sec.is_null() { 0 } else { (*sec).floorheight };
        (*thing).ceilingz = if sec.is_null() { 0 } else { (*sec).ceilingheight };
        (*thing).x = x;
        (*thing).y = y;
    }
    p_set_thing_position(thing);
    true
}

fn ptr_slide_traverse(in_: &mut super::Intercept) -> bool {
    if !in_.isaline {
        return true;
    }
    let li = in_.line;
    if li.is_null() {
        return true;
    }
    let ld = unsafe { &*li };
    let slidemo = with_p_map_state(|st| st.slidemo);
    if slidemo.is_null() {
        return true;
    }
    let mut is_blocking = false;
    if (ld.flags & ML_TWOSIDED) == 0 {
        if p_point_on_line_side(unsafe { (*slidemo).x }, unsafe { (*slidemo).y }, li) != 0 {
            return true; // don't hit back side of one-sided
        }
        is_blocking = true;
    } else {
        p_line_opening(li);
        let (opentop, openbottom, openrange) = super::p_maputl::with_p_maputl_state(|st| {
            (st.opentop, st.openbottom, st.openrange)
        });
        if openrange < unsafe { (*slidemo).height } {
            is_blocking = true;
        } else if opentop - unsafe { (*slidemo).z } < unsafe { (*slidemo).height } {
            is_blocking = true;
        } else if openbottom - unsafe { (*slidemo).z } > 24 * FRACUNIT {
            is_blocking = true;
        }
    }
    if is_blocking {
        with_p_map_state(|st| {
            if in_.frac < st.bestslidefrac {
                st.secondslidefrac = st.bestslidefrac;
                st.secondslideline = st.bestslideline;
                st.bestslidefrac = in_.frac;
                st.bestslideline = li;
            }
        });
        return false;
    }
    true
}

/// Slide thing along walls. Original: P_SlideMove
pub fn p_slide_move(mo: *mut Mobj) {
    if mo.is_null() {
        return;
    }
    with_p_map_state(|st| st.slidemo = mo);
    let mut hitcount = 0;
    loop {
        hitcount += 1;
        if hitcount == 3 {
            if !p_try_move(mo, unsafe { (*mo).x }, unsafe { (*mo).y + (*mo).momy }) {
                p_try_move(mo, unsafe { (*mo).x + (*mo).momx }, unsafe { (*mo).y });
            }
            return;
        }
        let (leadx, trailx) = if unsafe { (*mo).momx } > 0 {
            (
                unsafe { (*mo).x + (*mo).radius },
                unsafe { (*mo).x - (*mo).radius },
            )
        } else {
            (
                unsafe { (*mo).x - (*mo).radius },
                unsafe { (*mo).x + (*mo).radius },
            )
        };
        let (leady, traily) = if unsafe { (*mo).momy } > 0 {
            (
                unsafe { (*mo).y + (*mo).radius },
                unsafe { (*mo).y - (*mo).radius },
            )
        } else {
            (
                unsafe { (*mo).y - (*mo).radius },
                unsafe { (*mo).y + (*mo).radius },
            )
        };
        with_p_map_state(|st| st.bestslidefrac = FRACUNIT + 1);
        p_path_traverse(
            leadx,
            leady,
            leadx + unsafe { (*mo).momx },
            leady + unsafe { (*mo).momy },
            super::PT_ADDLINES,
            ptr_slide_traverse,
        );
        p_path_traverse(
            trailx,
            leady,
            trailx + unsafe { (*mo).momx },
            leady + unsafe { (*mo).momy },
            super::PT_ADDLINES,
            ptr_slide_traverse,
        );
        p_path_traverse(
            leadx,
            traily,
            leadx + unsafe { (*mo).momx },
            traily + unsafe { (*mo).momy },
            super::PT_ADDLINES,
            ptr_slide_traverse,
        );
        let bestslidefrac = with_p_map_state(|st| st.bestslidefrac);
        if bestslidefrac == FRACUNIT + 1 {
            continue;
        }
        let bestslidefrac = bestslidefrac - 0x800;
        if bestslidefrac > 0 {
            let newx = fixed_mul(unsafe { (*mo).momx }, bestslidefrac);
            let newy = fixed_mul(unsafe { (*mo).momy }, bestslidefrac);
            if !p_try_move(mo, unsafe { (*mo).x } + newx, unsafe { (*mo).y } + newy) {
                continue;
            }
        }
        let bestslidefrac = FRACUNIT - (bestslidefrac + 0x800);
        let bestslidefrac = if bestslidefrac > FRACUNIT {
            FRACUNIT
        } else {
            bestslidefrac
        };
        if bestslidefrac <= 0 {
            return;
        }
        let mut tmxmove = fixed_mul(unsafe { (*mo).momx }, bestslidefrac);
        let mut tmymove = fixed_mul(unsafe { (*mo).momy }, bestslidefrac);
        let (bestlideline, tmxmove_after, tmymove_after) =
            with_p_map_state(|st| (st.bestslideline, st.tmxmove, st.tmymove));
        if !bestlideline.is_null() {
            p_hit_slide_line(bestlideline);
            tmxmove = tmxmove_after;
            tmymove = tmymove_after;
        }
        unsafe {
            (*mo).momx = tmxmove;
            (*mo).momy = tmymove;
        }
        if !p_try_move(mo, unsafe { (*mo).x } + tmxmove, unsafe { (*mo).y } + tmymove) {
            continue;
        }
        return;
    }
}

fn p_hit_slide_line(ld: *mut Line) {
    if ld.is_null() {
        return;
    }
    use crate::geometry::{finecosine, finesine, ANG180, ANGLETOFINESHIFT};
    let ld_ref = unsafe { &*ld };
    match ld_ref.slopetype {
        crate::rendering::defs::SlopeType::Horizontal => {
            with_p_map_state(|st| st.tmymove = 0);
            return;
        }
        crate::rendering::defs::SlopeType::Vertical => {
            with_p_map_state(|st| st.tmxmove = 0);
            return;
        }
        _ => {}
    }
    let slidemo = with_p_map_state(|st| st.slidemo);
    let side = p_point_on_line_side(unsafe { (*slidemo).x }, unsafe { (*slidemo).y }, ld);
    let lineangle = r_point_to_angle2(0, 0, ld_ref.dx, ld_ref.dy);
    let lineangle = if side == 1 {
        lineangle.wrapping_add(ANG180)
    } else {
        lineangle
    };
    let (tmxmove, tmymove) = with_p_map_state(|st| (st.tmxmove, st.tmymove));
    let moveangle = r_point_to_angle2(0, 0, tmxmove, tmymove);
    let mut deltaangle = moveangle.wrapping_sub(lineangle);
    if deltaangle > ANG180 {
        deltaangle = deltaangle.wrapping_add(ANG180);
    }
    let lineangle_idx = (lineangle >> ANGLETOFINESHIFT) as usize;
    let deltaangle_idx = (deltaangle >> ANGLETOFINESHIFT) as usize;
    let movelen = super::p_maputl::p_aprox_distance(tmxmove, tmymove);
    let newlen = fixed_mul(movelen, finecosine(deltaangle_idx));
    with_p_map_state(|st| {
        st.tmxmove = fixed_mul(newlen, finecosine(lineangle_idx));
        st.tmymove = fixed_mul(newlen, finesine(lineangle_idx));
    });
}

/// Use lines in range. Original: P_UseLines
pub fn p_use_lines(_player: *mut std::ffi::c_void) {
    // TODO: requires player_t, P_BlockLinesIterator
}

/// Change sector (crush damage, etc.). Original: P_ChangeSector
pub fn p_change_sector(_sector: *mut std::ffi::c_void, _crunch: bool) -> bool {
    false
}

/// Aim line attack, return slope. Original: P_AimLineAttack
pub fn p_aim_line_attack(_t1: *mut Mobj, _angle: u32, _distance: Fixed) -> Fixed {
    0
}

/// Line attack (hitscan). Original: P_LineAttack
pub fn p_line_attack(
    _t1: *mut Mobj,
    _angle: u32,
    _distance: Fixed,
    _slope: Fixed,
    _damage: i32,
) {
    // TODO: requires P_PathTraverse, PIT_CheckLine, P_DamageMobj
}

/// Radius attack (explosion). Original: P_RadiusAttack
pub fn p_radius_attack(_spot: *mut Mobj, _source: *mut Mobj, _damage: i32) {
    // TODO: requires P_BlockThingsIterator, P_DamageMobj
}
