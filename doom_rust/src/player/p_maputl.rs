//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement/collision utility functions, blockmap iterators.
//
// Original: p_maputl.c

use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS, FRACUNIT};
use crate::rendering::defs::{Line, SlopeType, Subsector};
use crate::rendering::{r_point_in_subsector, VALIDCOUNT};
use crate::rendering::state;
use crate::rendering::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};

use super::p_mobj::Mobj;
use super::{Divline, Intercept, MAPBLOCKSHIFT, MAPBTOFRAC, PT_ADDTHINGS, PT_ADDLINES, PT_EARLYOUT};

/// Gives an estimation of distance (not exact).
/// Original: P_AproxDistance
pub fn p_aprox_distance(dx: Fixed, dy: Fixed) -> Fixed {
    let dx = dx.abs();
    let dy = dy.abs();
    if dx < dy {
        dx + dy - (dx >> 1)
    } else {
        dx + dy - (dy >> 1)
    }
}

/// Returns 0 (front) or 1 (back).
/// Original: P_PointOnLineSide
pub fn p_point_on_line_side(x: Fixed, y: Fixed, line: *const Line) -> i32 {
    if line.is_null() {
        return 0;
    }
    let ld = unsafe { &*line };
    let v1_x = unsafe { (*ld.v1).x };
    let v1_y = unsafe { (*ld.v1).y };

    if ld.dx == 0 {
        return if x <= v1_x {
            if ld.dy > 0 {
                1
            } else {
                0
            }
        } else if ld.dy < 0 {
            1
        } else {
            0
        };
    }
    if ld.dy == 0 {
        return if y <= v1_y {
            if ld.dx < 0 {
                1
            } else {
                0
            }
        } else if ld.dx > 0 {
            1
        } else {
            0
        };
    }

    let dx = x - v1_x;
    let dy = y - v1_y;
    let left = fixed_mul(ld.dy >> FRACBITS, dx);
    let right = fixed_mul(dy, ld.dx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

/// Returns 0 (front), 1 (back), or 2 (on line).
/// Original: P_DivlineSide (from p_sight.c, used for intercepts)
pub fn p_divline_side(x: Fixed, y: Fixed, line: &Divline) -> i32 {
    if line.dx == 0 {
        if x == line.x {
            return 2;
        }
        return if x <= line.x {
            if line.dy > 0 {
                1
            } else {
                0
            }
        } else if line.dy < 0 {
            1
        } else {
            0
        };
    }
    if line.dy == 0 {
        if y == line.y {
            return 2; // on line
        }
        return if y <= line.y {
            if line.dx < 0 {
                1
            } else {
                0
            }
        } else if line.dx > 0 {
            1
        } else {
            0
        };
    }

    let dx = x - line.x;
    let dy = y - line.y;
    let left = (line.dy >> FRACBITS) * (dx >> FRACBITS);
    let right = (dy >> FRACBITS) * (line.dx >> FRACBITS);

    if right < left {
        0
    } else if left == right {
        2
    } else {
        1
    }
}

/// Returns 0 (front) or 1 (back). Original: P_PointOnDivlineSide
pub fn p_point_on_divline_side(x: Fixed, y: Fixed, line: &Divline) -> i32 {
    if line.dx == 0 {
        return if x <= line.x {
            if line.dy > 0 {
                1
            } else {
                0
            }
        } else if line.dy < 0 {
            1
        } else {
            0
        };
    }
    if line.dy == 0 {
        return if y <= line.y {
            if line.dx < 0 {
                1
            } else {
                0
            }
        } else if line.dx > 0 {
            1
        } else {
            0
        };
    }

    let dx = x - line.x;
    let dy = y - line.y;

    const SIGN_BIT: i32 = i32::MIN; // 0x8000_0000
    if (line.dy ^ line.dx ^ dx ^ dy) & SIGN_BIT != 0 {
        if (line.dy ^ dx) & SIGN_BIT != 0 {
            return 1;
        }
        return 0;
    }

    let left = fixed_mul(line.dy >> 8, dx >> 8);
    let right = fixed_mul(dy >> 8, line.dx >> 8);

    if right < left {
        0
    } else {
        1
    }
}

/// Fill divline from linedef. Original: P_MakeDivline
pub fn p_make_divline(li: *const Line, dl: &mut Divline) {
    if li.is_null() {
        return;
    }
    let line = unsafe { &*li };
    dl.x = unsafe { (*line.v1).x };
    dl.y = unsafe { (*line.v1).y };
    dl.dx = line.dx;
    dl.dy = line.dy;
}

/// Returns fractional intercept along first divline. Original: P_InterceptVector
pub fn p_intercept_vector(v2: &Divline, v1: &Divline) -> Fixed {
    let den = fixed_mul(v1.dy >> 8, v2.dx) - fixed_mul(v1.dx >> 8, v2.dy);
    if den == 0 {
        return 0;
    }
    let num = fixed_mul((v1.x - v2.x) >> 8, v1.dy) + fixed_mul((v2.y - v1.y) >> 8, v1.dx);
    fixed_div(num, den)
}

/// Returns 0, 1, or -1 if box crosses line. Original: P_BoxOnLineSide
pub fn p_box_on_line_side(tmbox: &[Fixed; 4], ld: *const Line) -> i32 {
    if ld.is_null() {
        return 0;
    }
    let line = unsafe { &*ld };
    let v1_x = unsafe { (*line.v1).x };
    let v1_y = unsafe { (*line.v1).y };

    let (p1, p2) = match line.slopetype {
        SlopeType::Horizontal => {
            let mut p1 = if tmbox[BOXTOP] > v1_y { 1 } else { 0 };
            let mut p2 = if tmbox[BOXBOTTOM] > v1_y { 1 } else { 0 };
            if line.dx < 0 {
                p1 ^= 1;
                p2 ^= 1;
            }
            (p1, p2)
        }
        SlopeType::Vertical => {
            let mut p1 = if tmbox[BOXRIGHT] < v1_x { 1 } else { 0 };
            let mut p2 = if tmbox[BOXLEFT] < v1_x { 1 } else { 0 };
            if line.dy < 0 {
                p1 ^= 1;
                p2 ^= 1;
            }
            (p1, p2)
        }
        SlopeType::Positive => (
            p_point_on_line_side(tmbox[BOXLEFT], tmbox[BOXTOP], ld),
            p_point_on_line_side(tmbox[BOXRIGHT], tmbox[BOXBOTTOM], ld),
        ),
        SlopeType::Negative => (
            p_point_on_line_side(tmbox[BOXRIGHT], tmbox[BOXTOP], ld),
            p_point_on_line_side(tmbox[BOXLEFT], tmbox[BOXBOTTOM], ld),
        ),
    };

    if p1 == p2 {
        p1
    } else {
        -1
    }
}

/// P_LineOpening globals - set by p_line_opening.
pub static mut OPENTOP: Fixed = 0;
pub static mut OPENBOTTOM: Fixed = 0;
pub static mut OPENRANGE: Fixed = 0;
pub static mut LOWFLOOR: Fixed = 0;

/// Sets OPENTOP, OPENBOTTOM, OPENRANGE, LOWFLOOR for two-sided line.
/// Original: P_LineOpening
pub fn p_line_opening(linedef: *const Line) {
    if linedef.is_null() {
        unsafe {
            OPENRANGE = 0;
        }
        return;
    }
    let ld = unsafe { &*linedef };
    if ld.sidenum[1] == -1 {
        unsafe {
            OPENRANGE = 0;
        }
        return;
    }

    let front = ld.frontsector;
    let back = ld.backsector;
    if front.is_null() || back.is_null() {
        unsafe {
            OPENRANGE = 0;
        }
        return;
    }

    let front = unsafe { &*front };
    let back = unsafe { &*back };

    let opentop = if front.ceilingheight < back.ceilingheight {
        front.ceilingheight
    } else {
        back.ceilingheight
    };

    let (openbottom, lowfloor) = if front.floorheight > back.floorheight {
        (front.floorheight, back.floorheight)
    } else {
        (back.floorheight, front.floorheight)
    };

    unsafe {
        OPENTOP = opentop;
        OPENBOTTOM = openbottom;
        OPENRANGE = opentop - openbottom;
        LOWFLOOR = lowfloor;
    }
}

/// Call func for each line in block (x,y). Increment validcount before first call.
/// Original: P_BlockLinesIterator
pub fn p_block_lines_iterator<F>(x: i32, y: i32, mut func: F) -> bool
where
    F: FnMut(*mut Line) -> bool,
{
    let (bmapwidth, bmapheight, blockmap, blockmaplump, lines) = state::with_state(|s| {
        (s.bmapwidth, s.bmapheight, s.blockmap, s.blockmaplump, s.lines)
    });
    if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
        return true;
    }
    if blockmap.is_null() || blockmaplump.is_null() || lines.is_null() {
        return true;
    }
    let validcount = unsafe { VALIDCOUNT };
    let offset_idx = (y * bmapwidth + x) as usize;
    let offset = unsafe { *blockmap.add(offset_idx) } as usize;
    let mut list = unsafe { blockmaplump.add(offset) };
    loop {
        let linedef_idx = unsafe { *list };
        if linedef_idx == -1 {
            break;
        }
        let ld = unsafe { lines.add(linedef_idx as usize) };
        if unsafe { (*ld).validcount } == validcount {
            list = unsafe { list.add(1) };
            continue;
        }
        unsafe {
            (*ld).validcount = validcount;
        }
        if !func(ld) {
            return false;
        }
        list = unsafe { list.add(1) };
    }
    true
}

/// Call func for each thing in block (x,y). Original: P_BlockThingsIterator
pub fn p_block_things_iterator<F>(x: i32, y: i32, mut func: F) -> bool
where
    F: FnMut(*mut Mobj) -> bool,
{
    let (bmapwidth, bmapheight, blocklinks) = state::with_state(|s| {
        (s.bmapwidth, s.bmapheight, s.blocklinks)
    });
    if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
        return true;
    }
    if blocklinks.is_null() {
        return true;
    }
    let idx = (y * bmapwidth + x) as usize;
    let mut mobj = unsafe { *(blocklinks as *mut *mut Mobj).add(idx) };
    while !mobj.is_null() {
        let next = unsafe { (*mobj).bnext };
        if !func(mobj) {
            return false;
        }
        mobj = next;
    }
    true
}

/// Unlink thing from sector and blockmap. Original: P_UnsetThingPosition
pub fn p_unset_thing_position(thing: *mut Mobj) {
    if thing.is_null() {
        return;
    }
    let flags = unsafe { (*thing).flags };
    if flags & super::p_mobj::MF_NOSECTOR == 0 {
        let snext = unsafe { (*thing).snext };
        let sprev = unsafe { (*thing).sprev };
        if !snext.is_null() {
            unsafe { (*snext).sprev = sprev };
        }
        if !sprev.is_null() {
            unsafe { (*sprev).snext = snext };
        } else {
            let subsector = unsafe { (*thing).subsector } as *mut Subsector;
            if !subsector.is_null() {
                let sector = unsafe { (*subsector).sector };
                if !sector.is_null() {
                    unsafe { (*sector).thinglist = snext };
                }
            }
        }
    }
    if flags & super::p_mobj::MF_NOBLOCKMAP == 0 {
        let bnext = unsafe { (*thing).bnext };
        let bprev = unsafe { (*thing).bprev };
        if !bnext.is_null() {
            unsafe { (*bnext).bprev = bprev };
        }
        if !bprev.is_null() {
            unsafe { (*bprev).bnext = bnext };
        } else {
            let (bmaporgx, bmaporgy, bmapwidth, bmapheight, blocklinks) = state::with_state(|s| {
                (s.bmaporgx, s.bmaporgy, s.bmapwidth, s.bmapheight, s.blocklinks)
            });
            let blockx = (unsafe { (*thing).x } - bmaporgx) >> MAPBLOCKSHIFT;
            let blocky = (unsafe { (*thing).y } - bmaporgy) >> MAPBLOCKSHIFT;
            if blockx >= 0 && blockx < bmapwidth && blocky >= 0 && blocky < bmapheight {
                let idx = (blocky * bmapwidth + blockx) as usize;
                unsafe {
                    *((blocklinks as *mut *mut Mobj).add(idx)) = bnext;
                }
            }
        }
    }
}

/// Link thing into sector and blockmap. Original: P_SetThingPosition
pub fn p_set_thing_position(thing: *mut Mobj) {
    if thing.is_null() {
        return;
    }
    let ss = r_point_in_subsector(unsafe { (*thing).x }, unsafe { (*thing).y });
    unsafe {
        (*thing).subsector = ss as *mut std::ffi::c_void;
    }
    let flags = unsafe { (*thing).flags };
    if flags & super::p_mobj::MF_NOSECTOR == 0 {
        let sec = unsafe { (*ss).sector };
        if !sec.is_null() {
            unsafe {
                (*thing).sprev = std::ptr::null_mut();
                (*thing).snext = (*sec).thinglist;
                if !(*sec).thinglist.is_null() {
                    (*(*sec).thinglist).sprev = thing;
                }
                (*sec).thinglist = thing;
            }
        }
    }
    if flags & super::p_mobj::MF_NOBLOCKMAP == 0 {
        let (bmaporgx, bmaporgy, bmapwidth, bmapheight, blocklinks) = state::with_state(|s| {
            (s.bmaporgx, s.bmaporgy, s.bmapwidth, s.bmapheight, s.blocklinks)
        });
        let blockx = (unsafe { (*thing).x } - bmaporgx) >> MAPBLOCKSHIFT;
        let blocky = (unsafe { (*thing).y } - bmaporgy) >> MAPBLOCKSHIFT;
        if blockx >= 0 && blockx < bmapwidth && blocky >= 0 && blocky < bmapheight {
            let idx = (blocky * bmapwidth + blockx) as usize;
            let link = unsafe { (blocklinks as *mut *mut Mobj).add(idx) };
            unsafe {
                (*thing).bprev = std::ptr::null_mut();
                (*thing).bnext = *link;
                if !(*link).is_null() {
                    (*(*link)).bprev = thing;
                }
                *link = thing;
            }
        } else {
            unsafe {
                (*thing).bnext = std::ptr::null_mut();
                (*thing).bprev = std::ptr::null_mut();
            }
        }
    }
}

// =============================================================================
// P_PathTraverse - intercept collection and traversal
// =============================================================================

static mut INTERCEPTS: [Intercept; super::MAXINTERCEPTS] = [Intercept {
    frac: 0,
    isaline: false,
    line: std::ptr::null_mut(),
    thing: std::ptr::null_mut(),
}; super::MAXINTERCEPTS];
static mut INTERCEPT_P: usize = 0;
static mut TRACE: Divline = Divline {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};
static mut EARLYOUT: bool = false;

/// Add line intercepts from block. Original: PIT_AddLineIntercepts
fn pit_add_line_intercepts(ld: *mut Line) -> bool {
    if ld.is_null() {
        return true;
    }
    let (s1, s2) = unsafe {
        let (trace_dx, trace_dy) = (TRACE.dx, TRACE.dy);
        if trace_dx > FRACUNIT * 16
            || trace_dy > FRACUNIT * 16
            || trace_dx < -FRACUNIT * 16
            || trace_dy < -FRACUNIT * 16
        {
            let v1_x = (*(*ld).v1).x;
            let v1_y = (*(*ld).v1).y;
            let v2_x = (*(*ld).v2).x;
            let v2_y = (*(*ld).v2).y;
            (
                p_point_on_divline_side(v1_x, v1_y, &TRACE),
                p_point_on_divline_side(v2_x, v2_y, &TRACE),
            )
        } else {
            (
                p_point_on_line_side(TRACE.x, TRACE.y, ld),
                p_point_on_line_side(TRACE.x + TRACE.dx, TRACE.y + TRACE.dy, ld),
            )
        }
    };
    if s1 == s2 {
        return true;
    }
    let mut dl = Divline { x: 0, y: 0, dx: 0, dy: 0 };
    p_make_divline(ld, &mut dl);
    let frac = unsafe { p_intercept_vector(&TRACE, &dl) };
    if frac < 0 {
        return true;
    }
    if unsafe { EARLYOUT } && frac < FRACUNIT && unsafe { (*ld).backsector.is_null() } {
        return false;
    }
    let p = unsafe { INTERCEPT_P };
    if p >= super::MAXINTERCEPTS {
        return true;
    }
    unsafe {
        INTERCEPTS[p].frac = frac;
        INTERCEPTS[p].isaline = true;
        INTERCEPTS[p].line = ld;
        INTERCEPTS[p].thing = std::ptr::null_mut();
        INTERCEPT_P += 1;
    }
    true
}

/// Add thing intercepts from block. Original: PIT_AddThingIntercepts
fn pit_add_thing_intercepts(thing: *mut Mobj) -> bool {
    if thing.is_null() {
        return true;
    }
    let (x, y, radius) = unsafe { ((*thing).x, (*thing).y, (*thing).radius) };
    let tracepositive = (unsafe { TRACE.dx } ^ unsafe { TRACE.dy }) > 0;
    let (x1, y1, x2, y2) = if tracepositive {
        (x - radius, y + radius, x + radius, y - radius)
    } else {
        (x - radius, y - radius, x + radius, y + radius)
    };
    let (s1, s2) = unsafe {
        (
            p_point_on_divline_side(x1, y1, &TRACE),
            p_point_on_divline_side(x2, y2, &TRACE),
        )
    };
    if s1 == s2 {
        return true;
    }
    let dl = Divline {
        x: x1,
        y: y1,
        dx: x2 - x1,
        dy: y2 - y1,
    };
    let frac = unsafe { p_intercept_vector(&TRACE, &dl) };
    if frac < 0 {
        return true;
    }
    let p = unsafe { INTERCEPT_P };
    if p >= super::MAXINTERCEPTS {
        return true;
    }
    unsafe {
        INTERCEPTS[p].frac = frac;
        INTERCEPTS[p].isaline = false;
        INTERCEPTS[p].line = std::ptr::null_mut();
        INTERCEPTS[p].thing = thing;
        INTERCEPT_P += 1;
    }
    true
}

/// Traverse intercepts in order, call trav for each. Original: P_TraverseIntercepts
fn p_traverse_intercepts<F>(trav: &mut F, maxfrac: Fixed) -> bool
where
    F: FnMut(&mut Intercept) -> bool,
{
    let count = unsafe { INTERCEPT_P };
    if count == 0 {
        return true;
    }
    let mut remaining = count;
    while remaining > 0 {
        let mut dist = i32::MAX;
        let mut in_idx = 0;
        for i in 0..count {
            let frac = unsafe { INTERCEPTS[i].frac };
            if frac < dist {
                dist = frac;
                in_idx = i;
            }
        }
        if dist > maxfrac {
            return true;
        }
        if !trav(unsafe { &mut INTERCEPTS[in_idx] }) {
            return false;
        }
        unsafe {
            INTERCEPTS[in_idx].frac = i32::MAX;
        }
        remaining -= 1;
    }
    true
}

/// Trace path from (x1,y1) to (x2,y2), call trav for each intercept.
/// Original: P_PathTraverse
pub fn p_path_traverse<F>(x1: Fixed, y1: Fixed, x2: Fixed, y2: Fixed, flags: i32, mut trav: F) -> bool
where
    F: FnMut(&mut Intercept) -> bool,
{
    unsafe {
        VALIDCOUNT += 1;
        INTERCEPT_P = 0;
        EARLYOUT = (flags & PT_EARLYOUT) != 0;
    }
    let (bmaporgx, bmaporgy, bmapwidth, bmapheight) = state::with_state(|s| {
        (s.bmaporgx, s.bmaporgy, s.bmapwidth, s.bmapheight)
    });
    let mut x1 = x1;
    let mut y1 = y1;
    if ((x1 - bmaporgx) & (super::MAPBMASK as i32)) == 0 {
        x1 += FRACUNIT;
    }
    if ((y1 - bmaporgy) & (super::MAPBMASK as i32)) == 0 {
        y1 += FRACUNIT;
    }
    unsafe {
        TRACE.x = x1;
        TRACE.y = y1;
        TRACE.dx = x2 - x1;
        TRACE.dy = y2 - y1;
    }
    let x1_off = x1 - bmaporgx;
    let y1_off = y1 - bmaporgy;
    let xt1 = x1_off >> MAPBLOCKSHIFT;
    let yt1 = y1_off >> MAPBLOCKSHIFT;
    let x2_off = x2 - bmaporgx;
    let y2_off = y2 - bmaporgy;
    let xt2 = x2_off >> MAPBLOCKSHIFT;
    let yt2 = y2_off >> MAPBLOCKSHIFT;

    let (mapxstep, partial_x, ystep) = if xt2 > xt1 {
        let partial = FRACUNIT - ((x1_off >> MAPBTOFRAC) & (FRACUNIT - 1));
        let ystep = fixed_div(y2_off - y1_off, (x2_off - x1_off).abs());
        (1, partial, ystep)
    } else if xt2 < xt1 {
        let partial = (x1_off >> MAPBTOFRAC) & (FRACUNIT - 1);
        let ystep = fixed_div(y2_off - y1_off, (x2_off - x1_off).abs());
        (-1, partial, ystep)
    } else {
        (0, FRACUNIT, 256 * FRACUNIT)
    };
    let mut yintercept = (y1_off >> MAPBTOFRAC) + fixed_mul(partial_x, ystep);

    let (mapystep, partial_y, xstep) = if yt2 > yt1 {
        let partial = FRACUNIT - ((y1_off >> MAPBTOFRAC) & (FRACUNIT - 1));
        let xstep = fixed_div(x2_off - x1_off, (y2_off - y1_off).abs());
        (1, partial, xstep)
    } else if yt2 < yt1 {
        let partial = (y1_off >> MAPBTOFRAC) & (FRACUNIT - 1);
        let xstep = fixed_div(x2_off - x1_off, (y2_off - y1_off).abs());
        (-1, partial, xstep)
    } else {
        (0, FRACUNIT, 256 * FRACUNIT)
    };
    let mut xintercept = (x1_off >> MAPBTOFRAC) + fixed_mul(partial_y, xstep);

    let mut mapx = xt1;
    let mut mapy = yt1;
    for _count in 0..64 {
        if (flags & PT_ADDLINES) != 0 {
            if !p_block_lines_iterator(mapx, mapy, pit_add_line_intercepts) {
                return false;
            }
        }
        if (flags & PT_ADDTHINGS) != 0 {
            if !p_block_things_iterator(mapx, mapy, pit_add_thing_intercepts) {
                return false;
            }
        }
        if mapx == xt2 && mapy == yt2 {
            break;
        }
        if (yintercept >> FRACBITS) == mapy {
            yintercept += ystep;
            mapx += mapxstep;
        } else if (xintercept >> FRACBITS) == mapx {
            xintercept += xstep;
            mapy += mapystep;
        }
    }
    p_traverse_intercepts(&mut trav, FRACUNIT)
}
