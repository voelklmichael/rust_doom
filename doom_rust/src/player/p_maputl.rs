//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement/collision utility functions, blockmap iterators.
//
// Original: p_maputl.c

use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS, FRACUNIT};
use std::sync::{Mutex, OnceLock};
use crate::rendering::defs::{Line, SlopeType, Vertex};
use crate::rendering::{r_point_in_subsector, with_r_main_state, with_r_main_state_mut};
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
pub fn p_point_on_line_side(x: Fixed, y: Fixed, line: &Line) -> i32 {
    let (v1_x, v1_y) = state::with_state(|s| {
        let v = s.vertexes.get(line.v1_idx).copied().unwrap_or(crate::rendering::defs::Vertex { x: 0, y: 0 });
        (v.x, v.y)
    });

    if line.dx == 0 {
        return if x <= v1_x {
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
        return if y <= v1_y {
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

    let dx = x - v1_x;
    let dy = y - v1_y;
    let left = fixed_mul(line.dy >> FRACBITS, dx);
    let right = fixed_mul(dy, line.dx >> FRACBITS);

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
pub fn p_make_divline(line_idx: usize, dl: &mut Divline) {
    if let Some(line) = state::with_state(|s| s.lines.get(line_idx).cloned()) {
        p_make_divline_from_line(&line, dl);
    }
}

/// Fill divline from Line ref (uses state for vertexes).
fn p_make_divline_from_line(line: &Line, dl: &mut Divline) {
    let (x, y) = state::with_state(|s| {
        let v = s.vertexes.get(line.v1_idx).copied().unwrap_or(Vertex { x: 0, y: 0 });
        (v.x, v.y)
    });
    dl.x = x;
    dl.y = y;
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
pub fn p_box_on_line_side(tmbox: &[Fixed; 4], line: &Line) -> i32 {
    let (v1_x, v1_y) = state::with_state(|s| {
        let v = s.vertexes.get(line.v1_idx).copied().unwrap_or(Vertex { x: 0, y: 0 });
        (v.x, v.y)
    });

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
            p_point_on_line_side(tmbox[BOXLEFT], tmbox[BOXTOP], line),
            p_point_on_line_side(tmbox[BOXRIGHT], tmbox[BOXBOTTOM], line),
        ),
        SlopeType::Negative => (
            p_point_on_line_side(tmbox[BOXRIGHT], tmbox[BOXTOP], line),
            p_point_on_line_side(tmbox[BOXLEFT], tmbox[BOXBOTTOM], line),
        ),
    };

    if p1 == p2 {
        p1
    } else {
        -1
    }
}

/// Sets OPENTOP, OPENBOTTOM, OPENRANGE, LOWFLOOR for two-sided line.
/// Original: P_LineOpening
pub fn p_line_opening(line_idx: usize) {
    let (front, back) = state::with_state(|s| {
        let ld = match s.lines.get(line_idx) {
            Some(l) => l,
            None => return (None, None),
        };
        if ld.sidenum[1] == -1 {
            return (None, None);
        }
        let front = s.sectors.get(ld.frontsector_idx).cloned();
        let back = ld.backsector_idx.and_then(|i| s.sectors.get(i).cloned());
        (front, back)
    });
    let (front, back) = match (front, back) {
        (Some(f), Some(b)) => (f, b),
        _ => {
            with_p_maputl_state(|st| st.openrange = 0);
            return;
        }
    };

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

    with_p_maputl_state(|st| {
        st.opentop = opentop;
        st.openbottom = openbottom;
        st.openrange = opentop - openbottom;
        st.lowfloor = lowfloor;
    });
}

/// Call func for each line in block (x,y). Increment validcount before first call.
/// Original: P_BlockLinesIterator
pub fn p_block_lines_iterator<F>(x: i32, y: i32, mut func: F) -> bool
where
    F: FnMut(usize) -> bool,
{
    let line_indices: Vec<usize> = state::with_state(|s| {
        if x < 0 || y < 0 || x >= s.bmapwidth || y >= s.bmapheight {
            return vec![];
        }
        let blockmaplump = &s.blockmaplump;
        let lines = &s.lines;
        if blockmaplump.len() < 4 || lines.is_empty() {
            return vec![];
        }
        let validcount = with_r_main_state(|rm| rm.validcount);
        let offset_idx = (y * s.bmapwidth + x) as usize;
        let list_start = 4 + offset_idx;
        if list_start >= blockmaplump.len() {
            return vec![];
        }
        let mut list_idx = blockmaplump[list_start] as usize;
        let mut result = Vec::new();
        loop {
            if list_idx >= blockmaplump.len() {
                break;
            }
            let linedef_idx = blockmaplump[list_idx] as i16;
            if linedef_idx == -1 {
                break;
            }
            let line_idx = linedef_idx as usize;
            if line_idx < lines.len() && lines[line_idx].validcount != validcount {
                result.push(line_idx);
            }
            list_idx += 1;
        }
        result
    });
    for line_idx in line_indices {
        state::with_state_mut(|s| {
            if let Some(l) = s.lines.get_mut(line_idx) {
                l.validcount = with_r_main_state(|rm| rm.validcount);
            }
        });
        if !func(line_idx) {
            return false;
        }
    }
    true
}

/// Call func for each thing in block (x,y). Original: P_BlockThingsIterator
pub fn p_block_things_iterator<F>(x: i32, y: i32, mut func: F) -> bool
where
    F: FnMut(*mut Mobj) -> bool,
{
    let (bmapwidth, bmapheight, blocklinks) = state::with_state(|s| {
        (s.bmapwidth, s.bmapheight, s.blocklinks.clone())
    });
    if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
        return true;
    }
    let idx = (y * bmapwidth + x) as usize;
    if idx >= blocklinks.len() {
        return true;
    }
    let mut mobj = unsafe { *(blocklinks.as_ptr().add(idx) as *const *mut Mobj) };
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
            let sub_idx = unsafe { (*thing).subsector as usize };
            crate::rendering::state::with_state_mut(|s| {
                if let Some(sub) = s.subsectors.get(sub_idx) {
                    let sec_idx = sub.sector_idx;
                    if sec_idx < s.sectors.len() {
                        let sec = &mut s.sectors[sec_idx];
                        sec.thinglist = crate::player::mobjs::mobj_index_from_ptr(snext);
                    }
                }
            });
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
            state::with_state_mut(|s| {
                let blockx = (unsafe { (*thing).x } - s.bmaporgx) >> MAPBLOCKSHIFT;
                let blocky = (unsafe { (*thing).y } - s.bmaporgy) >> MAPBLOCKSHIFT;
                if blockx >= 0 && blockx < s.bmapwidth && blocky >= 0 && blocky < s.bmapheight {
                    let idx = (blocky * s.bmapwidth + blockx) as usize;
                    if idx < s.blocklinks.len() {
                        s.blocklinks[idx] = bnext as *mut std::ffi::c_void;
                    }
                }
            });
        }
    }
}

/// Link thing into sector and blockmap. Original: P_SetThingPosition
pub fn p_set_thing_position(thing: *mut Mobj) {
    if thing.is_null() {
        return;
    }
    let sub_idx = r_point_in_subsector(unsafe { (*thing).x }, unsafe { (*thing).y });
    unsafe {
        (*thing).subsector = sub_idx as *mut std::ffi::c_void;
    }
    let flags = unsafe { (*thing).flags };
    if flags & super::p_mobj::MF_NOSECTOR == 0 {
        if let Some(mobj_idx) = crate::player::mobjs::mobj_index_from_ptr(thing) {
            let old_head = crate::rendering::state::with_state_mut(|s| {
                if let Some(sub) = s.subsectors.get(sub_idx) {
                    let sec_idx = sub.sector_idx;
                    if sec_idx < s.sectors.len() {
                        let sec = &mut s.sectors[sec_idx];
                        let old = sec.thinglist;
                        sec.thinglist = Some(mobj_idx);
                        old
                    } else {
                        None
                    }
                } else {
                    None
                }
            });
            unsafe {
                (*thing).sprev = std::ptr::null_mut();
                (*thing).snext = crate::player::mobjs::mobj_ptr_from_index(old_head.unwrap_or(crate::player::mobjs::MobjIndex::NULL));
                if let Some(prev_head_idx) = old_head {
                    let prev_head = crate::player::mobjs::mobj_ptr_from_index(prev_head_idx);
                    if !prev_head.is_null() {
                        (*prev_head).sprev = thing;
                    }
                }
            }
        }
    }
    if flags & super::p_mobj::MF_NOBLOCKMAP == 0 {
        state::with_state_mut(|s| {
            let blockx = (unsafe { (*thing).x } - s.bmaporgx) >> MAPBLOCKSHIFT;
            let blocky = (unsafe { (*thing).y } - s.bmaporgy) >> MAPBLOCKSHIFT;
            if blockx >= 0 && blockx < s.bmapwidth && blocky >= 0 && blocky < s.bmapheight {
                let idx = (blocky * s.bmapwidth + blockx) as usize;
                if idx < s.blocklinks.len() {
                    let old_head = s.blocklinks[idx] as *mut Mobj;
                    unsafe {
                        (*thing).bprev = std::ptr::null_mut();
                        (*thing).bnext = old_head;
                        if !old_head.is_null() {
                            (*old_head).bprev = thing;
                        }
                    }
                    s.blocklinks[idx] = thing as *mut std::ffi::c_void;
                }
            } else {
                unsafe {
                    (*thing).bnext = std::ptr::null_mut();
                    (*thing).bprev = std::ptr::null_mut();
                }
            }
        });
    }
}

// =============================================================================
// PMaputlState - thread-safe via OnceLock + Mutex
// =============================================================================

static P_MAPUTL_STATE: OnceLock<Mutex<PMaputlState>> = OnceLock::new();

/// Safety: Raw pointers in PMaputlState (Intercept) are only used while holding the Mutex lock.
unsafe impl Send for PMaputlState {}

pub struct PMaputlState {
    pub intercepts: [Intercept; super::MAXINTERCEPTS],
    pub intercept_p: usize,
    pub trace: Divline,
    pub earlyout: bool,
    /// P_LineOpening globals - set by p_line_opening.
    pub opentop: Fixed,
    pub openbottom: Fixed,
    pub openrange: Fixed,
    pub lowfloor: Fixed,
}

fn default_intercepts() -> [Intercept; super::MAXINTERCEPTS] {
    [Intercept {
        frac: 0,
        isaline: false,
        line_idx: None,
        thing: std::ptr::null_mut(),
    }; super::MAXINTERCEPTS]
}

fn get_p_maputl_state() -> &'static Mutex<PMaputlState> {
    P_MAPUTL_STATE.get_or_init(|| {
        Mutex::new(PMaputlState {
            intercepts: default_intercepts(),
            intercept_p: 0,
            trace: Divline { x: 0, y: 0, dx: 0, dy: 0 },
            earlyout: false,
            opentop: 0,
            openbottom: 0,
            openrange: 0,
            lowfloor: 0,
        })
    })
}

/// Access PMaputlState.
pub fn with_p_maputl_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut PMaputlState) -> R,
{
    let mut guard = get_p_maputl_state().lock().unwrap();
    f(&mut guard)
}

/// Add line intercepts from block. Original: PIT_AddLineIntercepts
fn pit_add_line_intercepts(line_idx: usize) -> bool {
    let line = match state::with_state(|s| s.lines.get(line_idx).cloned()) {
        Some(l) => l,
        None => return true,
    };
    with_p_maputl_state(|st| {
    let (s1, s2) = {
        let (trace_dx, trace_dy) = (st.trace.dx, st.trace.dy);
        if trace_dx > FRACUNIT * 16
            || trace_dy > FRACUNIT * 16
            || trace_dx < -FRACUNIT * 16
            || trace_dy < -FRACUNIT * 16
        {
            let (v1_x, v1_y, v2_x, v2_y) = state::with_state(|s| {
                let v1 = s.vertexes.get(line.v1_idx).copied().unwrap_or(Vertex { x: 0, y: 0 });
                let v2 = s.vertexes.get(line.v2_idx).copied().unwrap_or(Vertex { x: 0, y: 0 });
                (v1.x, v1.y, v2.x, v2.y)
            });
            (
                p_point_on_divline_side(v1_x, v1_y, &st.trace),
                p_point_on_divline_side(v2_x, v2_y, &st.trace),
            )
        } else {
            (
                p_point_on_line_side(st.trace.x, st.trace.y, &line),
                p_point_on_line_side(st.trace.x + st.trace.dx, st.trace.y + st.trace.dy, &line),
            )
        }
    };
    if s1 == s2 {
        return true;
    }
    let mut dl = Divline { x: 0, y: 0, dx: 0, dy: 0 };
    p_make_divline_from_line(&line, &mut dl);
    let frac = p_intercept_vector(&st.trace, &dl);
    if frac < 0 {
        return true;
    }
    if st.earlyout && frac < FRACUNIT && line.backsector_idx.is_none() {
        return false;
    }
    let p = st.intercept_p;
    if p >= super::MAXINTERCEPTS {
        return true;
    }
    st.intercepts[p].frac = frac;
    st.intercepts[p].isaline = true;
    st.intercepts[p].line_idx = Some(line_idx);
    st.intercepts[p].thing = std::ptr::null_mut();
    st.intercept_p += 1;
    true
    })
}

/// Add thing intercepts from block. Original: PIT_AddThingIntercepts
fn pit_add_thing_intercepts(thing: *mut Mobj) -> bool {
    if thing.is_null() {
        return true;
    }
    with_p_maputl_state(|st| {
    let (x, y, radius) = unsafe { ((*thing).x, (*thing).y, (*thing).radius) };
    let tracepositive = (st.trace.dx ^ st.trace.dy) > 0;
    let (x1, y1, x2, y2) = if tracepositive {
        (x - radius, y + radius, x + radius, y - radius)
    } else {
        (x - radius, y - radius, x + radius, y + radius)
    };
    let (s1, s2) = (
        p_point_on_divline_side(x1, y1, &st.trace),
        p_point_on_divline_side(x2, y2, &st.trace),
    );
    if s1 == s2 {
        return true;
    }
    let dl = Divline {
        x: x1,
        y: y1,
        dx: x2 - x1,
        dy: y2 - y1,
    };
    let frac = p_intercept_vector(&st.trace, &dl);
    if frac < 0 {
        return true;
    }
    let p = st.intercept_p;
    if p >= super::MAXINTERCEPTS {
        return true;
    }
    st.intercepts[p].frac = frac;
    st.intercepts[p].isaline = false;
    st.intercepts[p].line_idx = None;
    st.intercepts[p].thing = thing;
    st.intercept_p += 1;
    true
    })
}

/// Traverse intercepts in order, call trav for each. Original: P_TraverseIntercepts
fn p_traverse_intercepts<F>(trav: &mut F, maxfrac: Fixed) -> bool
where
    F: FnMut(&mut Intercept) -> bool,
{
    with_p_maputl_state(|st| {
        let count = st.intercept_p;
        if count == 0 {
            return true;
        }
        let mut remaining = count;
        while remaining > 0 {
            let mut dist = i32::MAX;
            let mut in_idx = 0;
            for i in 0..count {
                let frac = st.intercepts[i].frac;
                if frac < dist {
                    dist = frac;
                    in_idx = i;
                }
            }
            if dist > maxfrac {
                return true;
            }
            if !trav(&mut st.intercepts[in_idx]) {
                return false;
            }
            st.intercepts[in_idx].frac = i32::MAX;
            remaining -= 1;
        }
        true
    })
}

/// Trace path from (x1,y1) to (x2,y2), call trav for each intercept.
/// Original: P_PathTraverse
pub fn p_path_traverse<F>(x1: Fixed, y1: Fixed, x2: Fixed, y2: Fixed, flags: i32, mut trav: F) -> bool
where
    F: FnMut(&mut Intercept) -> bool,
{
    with_r_main_state_mut(|rm| rm.validcount += 1);
    with_p_maputl_state(|st| {
        st.intercept_p = 0;
        st.earlyout = (flags & PT_EARLYOUT) != 0;
    });
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
    with_p_maputl_state(|st| {
        st.trace.x = x1;
        st.trace.y = y1;
        st.trace.dx = x2 - x1;
        st.trace.dy = y2 - y1;
    });
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
