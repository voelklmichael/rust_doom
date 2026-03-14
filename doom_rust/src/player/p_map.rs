//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement, collision, shooting, aiming.
//
// Original: p_map.c

use crate::m_fixed::{fixed_mul, Fixed, FRACUNIT};
use crate::rendering::defs::{Line, ML_BLOCKING, ML_BLOCKMONSTERS, ML_TWOSIDED};
use crate::rendering::state;
use crate::rendering::{r_point_in_subsector, r_point_to_angle2, VALIDCOUNT};
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

/// P_Map globals - set during movement checks.
pub static mut FLOATOK: bool = false;
pub static mut TMFLOORZ: Fixed = 0;
pub static mut TMCEILINGZ: Fixed = 0;
pub static mut TMDROPOFFZ: Fixed = 0;
pub static mut CEILINGLINE: *mut Line = std::ptr::null_mut();
pub static mut SPECHIT: [*mut Line; MAXSPECIALCROSS] = [std::ptr::null_mut(); MAXSPECIALCROSS];
pub static mut NUMSPECHIT: i32 = 0;
pub static mut LINETARGET: *mut Mobj = std::ptr::null_mut();

static mut TMBBOX: [Fixed; 4] = [0; 4];
static mut TMTHING: *mut Mobj = std::ptr::null_mut();
static mut TMFLAGS: i32 = 0;
static mut TMX: Fixed = 0;
static mut TMY: Fixed = 0;

// Slide move globals
static mut BESTSLIDEFRAC: Fixed = 0;
static mut SECONDSLIDEFRAC: Fixed = 0;
static mut BESTSLIDELINE: *mut Line = std::ptr::null_mut();
static mut SECONDSLIDELINE: *mut Line = std::ptr::null_mut();
static mut SLIDEMO: *mut Mobj = std::ptr::null_mut();
static mut TMXMOVE: Fixed = 0;
static mut TMYMOVE: Fixed = 0;

fn pit_check_line(ld: *mut Line) -> bool {
    if ld.is_null() {
        return true;
    }
    let (tmbbox, tmthing, tmflags) = unsafe { (TMBBOX, TMTHING, TMFLAGS) };
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
    let (opentop, openbottom, lowfloor) = unsafe {
        (
            super::p_maputl::OPENTOP,
            super::p_maputl::OPENBOTTOM,
            super::p_maputl::LOWFLOOR,
        )
    };
    unsafe {
        if opentop < TMCEILINGZ {
            TMCEILINGZ = opentop;
            CEILINGLINE = ld;
        }
        if openbottom > TMFLOORZ {
            TMFLOORZ = openbottom;
        }
        if lowfloor < TMDROPOFFZ {
            TMDROPOFFZ = lowfloor;
        }
        if ld_ref.special != 0 {
            let n = NUMSPECHIT as usize;
            if n < MAXSPECIALCROSS {
                SPECHIT[n] = ld;
                NUMSPECHIT += 1;
            }
        }
    }
    true
}

fn pit_check_thing(thing: *mut Mobj) -> bool {
    if thing.is_null() {
        return true;
    }
    let (tmthing, tmx, tmy, tmflags) = unsafe { (TMTHING, TMX, TMY, TMFLAGS) };
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
    unsafe {
        TMTHING = thing;
        TMFLAGS = (*thing).flags;
        TMX = x;
        TMY = y;
        TMBBOX[BOXTOP] = y + (*thing).radius;
        TMBBOX[BOXBOTTOM] = y - (*thing).radius;
        TMBBOX[BOXRIGHT] = x + (*thing).radius;
        TMBBOX[BOXLEFT] = x - (*thing).radius;
    }
    let newsubsec = r_point_in_subsector(x, y);
    unsafe {
        CEILINGLINE = std::ptr::null_mut();
        let sec = (*newsubsec).sector;
        TMFLOORZ = if sec.is_null() { 0 } else { (*sec).floorheight };
        TMDROPOFFZ = TMFLOORZ;
        TMCEILINGZ = if sec.is_null() { 0 } else { (*sec).ceilingheight };
        VALIDCOUNT += 1;
        NUMSPECHIT = 0;
    }
    if (unsafe { (*thing).flags } & MF_NOCLIP) != 0 {
        return true;
    }
    let xl = (unsafe { TMBBOX[BOXLEFT] } - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
    let xh = (unsafe { TMBBOX[BOXRIGHT] } - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
    let yl = (unsafe { TMBBOX[BOXBOTTOM] } - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
    let yh = (unsafe { TMBBOX[BOXTOP] } - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
    for bx in xl..=xh {
        for by in yl..=yh {
            if !p_block_things_iterator(bx, by, pit_check_thing) {
                return false;
            }
        }
    }
    let xl = (unsafe { TMBBOX[BOXLEFT] } - bmaporgx) >> MAPBLOCKSHIFT;
    let xh = (unsafe { TMBBOX[BOXRIGHT] } - bmaporgx) >> MAPBLOCKSHIFT;
    let yl = (unsafe { TMBBOX[BOXBOTTOM] } - bmaporgy) >> MAPBLOCKSHIFT;
    let yh = (unsafe { TMBBOX[BOXTOP] } - bmaporgy) >> MAPBLOCKSHIFT;
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
    unsafe {
        FLOATOK = false;
    }
    if !p_check_position(thing, x, y) {
        return false;
    }
    if (unsafe { (*thing).flags } & MF_NOCLIP) == 0 {
        if unsafe { TMCEILINGZ - TMFLOORZ } < (unsafe { (*thing).height }) {
            return false;
        }
        unsafe {
            FLOATOK = true;
        }
        if (unsafe { (*thing).flags } & MF_TELEPORT) == 0
            && unsafe { TMCEILINGZ - (*thing).z } < (unsafe { (*thing).height })
        {
            return false;
        }
        if (unsafe { (*thing).flags } & MF_TELEPORT) == 0
            && unsafe { TMFLOORZ - (*thing).z } > 24 * FRACUNIT
        {
            return false;
        }
        if (unsafe { (*thing).flags } & (MF_DROPOFF | MF_FLOAT)) == 0
            && unsafe { TMFLOORZ - TMDROPOFFZ } > 24 * FRACUNIT
        {
            return false;
        }
    }
    p_unset_thing_position(thing);
    let oldx = unsafe { (*thing).x };
    let oldy = unsafe { (*thing).y };
    unsafe {
        (*thing).floorz = TMFLOORZ;
        (*thing).ceilingz = TMCEILINGZ;
        (*thing).x = x;
        (*thing).y = y;
    }
    p_set_thing_position(thing);
    if (unsafe { (*thing).flags } & (MF_TELEPORT | MF_NOCLIP)) == 0 {
        let lines = state::with_state(|s| s.lines);
        while unsafe { NUMSPECHIT } > 0 {
            unsafe {
                NUMSPECHIT -= 1;
                let idx = NUMSPECHIT as usize;
                let ld = SPECHIT[idx];
                if !ld.is_null() && !lines.is_null() {
                    let line_index = ld.offset_from(lines) as i32;
                    let side = p_point_on_line_side((*thing).x, (*thing).y, ld);
                    let oldside = p_point_on_line_side(oldx, oldy, ld);
                    if side != oldside && (*ld).special != 0 {
                        p_cross_special_line(line_index, oldside, thing);
                    }
                }
            }
        }
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
    let slidemo = unsafe { SLIDEMO };
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
        let opentop = unsafe { super::p_maputl::OPENTOP };
        let openbottom = unsafe { super::p_maputl::OPENBOTTOM };
        if unsafe { super::p_maputl::OPENRANGE } < unsafe { (*slidemo).height } {
            is_blocking = true;
        } else if opentop - unsafe { (*slidemo).z } < unsafe { (*slidemo).height } {
            is_blocking = true;
        } else if openbottom - unsafe { (*slidemo).z } > 24 * FRACUNIT {
            is_blocking = true;
        }
    }
    if is_blocking {
        if in_.frac < unsafe { BESTSLIDEFRAC } {
            unsafe {
                SECONDSLIDEFRAC = BESTSLIDEFRAC;
                SECONDSLIDELINE = BESTSLIDELINE;
                BESTSLIDEFRAC = in_.frac;
                BESTSLIDELINE = li;
            }
        }
        return false;
    }
    true
}

/// Slide thing along walls. Original: P_SlideMove
pub fn p_slide_move(mo: *mut Mobj) {
    if mo.is_null() {
        return;
    }
    unsafe {
        SLIDEMO = mo;
    }
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
        unsafe {
            BESTSLIDEFRAC = FRACUNIT + 1;
        }
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
        if unsafe { BESTSLIDEFRAC } == FRACUNIT + 1 {
            continue;
        }
        let bestslidefrac = unsafe { BESTSLIDEFRAC } - 0x800;
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
        let bestlideline = unsafe { BESTSLIDELINE };
        if !bestlideline.is_null() {
            p_hit_slide_line(bestlideline);
            tmxmove = unsafe { TMXMOVE };
            tmymove = unsafe { TMYMOVE };
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
            unsafe { TMYMOVE = 0 };
            return;
        }
        crate::rendering::defs::SlopeType::Vertical => {
            unsafe { TMXMOVE = 0 };
            return;
        }
        _ => {}
    }
    let side = p_point_on_line_side(unsafe { (*SLIDEMO).x }, unsafe { (*SLIDEMO).y }, ld);
    let lineangle = r_point_to_angle2(0, 0, ld_ref.dx, ld_ref.dy);
    let lineangle = if side == 1 {
        lineangle.wrapping_add(ANG180)
    } else {
        lineangle
    };
    let moveangle = r_point_to_angle2(0, 0, unsafe { TMXMOVE }, unsafe { TMYMOVE });
    let mut deltaangle = moveangle.wrapping_sub(lineangle);
    if deltaangle > ANG180 {
        deltaangle = deltaangle.wrapping_add(ANG180);
    }
    let lineangle_idx = (lineangle >> ANGLETOFINESHIFT) as usize;
    let deltaangle_idx = (deltaangle >> ANGLETOFINESHIFT) as usize;
    let movelen = super::p_maputl::p_aprox_distance(unsafe { TMXMOVE }, unsafe { TMYMOVE });
    let newlen = fixed_mul(movelen, finecosine(deltaangle_idx));
    unsafe {
        TMXMOVE = fixed_mul(newlen, finecosine(lineangle_idx));
        TMYMOVE = fixed_mul(newlen, finesine(lineangle_idx));
    }
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
