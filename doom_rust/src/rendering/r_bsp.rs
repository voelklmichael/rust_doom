//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  BSP traversal, handling of LineSegs for rendering.
//
// Original: r_bsp.h + r_bsp.c

use crate::geometry::{FINEANGLES, ANG90, ANG180, ANGLETOFINESHIFT};
use crate::rendering::defs::{DrawSeg, Line, Seg, Sector, SideDef, MAXDRAWSEGS};
use crate::rendering::m_bbox::{BOXLEFT, BOXRIGHT, BOXBOTTOM, BOXTOP};
use crate::rendering::r_main::{r_point_on_side, r_point_to_angle, NF_SUBSECTOR};
use crate::rendering::r_plane;
use crate::rendering::r_segs;
use crate::rendering::r_sky;
use crate::rendering::r_things;
use crate::rendering::state;
use crate::m_fixed::Fixed;
use std::ptr;

// =============================================================================
// Clip range for solid/pass wall segments
// =============================================================================

#[repr(C)]
#[derive(Clone, Copy)]
struct ClipRange {
    first: i32,
    last: i32,
}

const MAXSEGS: usize = 32;

// =============================================================================
// State (from r_bsp.c)
// =============================================================================

pub static mut CURLINE: *mut Seg = ptr::null_mut();
pub static mut SIDEDEF: *mut SideDef = ptr::null_mut();
pub static mut LINEDEF: *mut Line = ptr::null_mut();
pub static mut FRONTSECTOR: *mut Sector = ptr::null_mut();
pub static mut BACKSECTOR: *mut Sector = ptr::null_mut();

pub static mut DRAWSEGS: [DrawSeg; MAXDRAWSEGS] = unsafe { std::mem::zeroed() };
pub static mut DS_P: *mut DrawSeg = ptr::null_mut();

static mut SOLIDSEGS: [ClipRange; MAXSEGS] = [ClipRange { first: 0, last: 0 }; MAXSEGS];
static mut NEWEND: *mut ClipRange = ptr::null_mut();

// =============================================================================
// R_ClipSolidWallSegment
// =============================================================================

fn r_clip_solid_wall_segment(first: i32, last: i32) {
    unsafe {
        let mut start = SOLIDSEGS.as_mut_ptr();
        while (*start).last < first - 1 {
            start = start.add(1);
        }

        if first < (*start).first {
            if last < (*start).first - 1 {
                r_segs::r_store_wall_range(first, last);
                let mut next = NEWEND;
                NEWEND = NEWEND.add(1);

                while next != start {
                    *next = *next.sub(1);
                    next = next.sub(1);
                }
                (*next).first = first;
                (*next).last = last;
                return;
            }
            r_segs::r_store_wall_range(first, (*start).first - 1);
            (*start).first = first;
        }

        if last <= (*start).last {
            return;
        }

        let mut next = start;
        while last >= (*(next.add(1))).first - 1 {
            r_segs::r_store_wall_range((*next).last + 1, (*(next.add(1))).first - 1);
            next = next.add(1);

            if last <= (*next).last {
                (*start).last = (*next).last;
                break;
            }
        }

        if last > (*next).last {
            r_segs::r_store_wall_range((*next).last + 1, last);
            (*start).last = last;

            let mut next = next.add(1);
            while next != NEWEND {
                start = start.add(1);
                *start = *next;
                next = next.add(1);
            }
            NEWEND = start.add(1);
        }
    }
}

// =============================================================================
// R_ClipPassWallSegment
// =============================================================================

fn r_clip_pass_wall_segment(first: i32, last: i32) {
    unsafe {
        let mut start = SOLIDSEGS.as_mut_ptr();
        while (*start).last < first - 1 {
            start = start.add(1);
        }

        if first < (*start).first {
            if last < (*start).first - 1 {
                r_segs::r_store_wall_range(first, last);
                return;
            }
            r_segs::r_store_wall_range(first, (*start).first - 1);
        }

        if last <= (*start).last {
            return;
        }

        while last >= (*(start.add(1))).first - 1 {
            r_segs::r_store_wall_range((*start).last + 1, (*(start.add(1))).first - 1);
            start = start.add(1);

            if last <= (*start).last {
                return;
            }
        }

        r_segs::r_store_wall_range((*start).last + 1, last);
    }
}

// =============================================================================
// R_ClearClipSegs
// =============================================================================

pub fn r_clear_clip_segs() {
    unsafe {
        SOLIDSEGS[0].first = -0x7fff_ffff;
        SOLIDSEGS[0].last = -1;
        SOLIDSEGS[1].first = state::VIEWWIDTH;
        SOLIDSEGS[1].last = 0x7fff_ffff;
        NEWEND = SOLIDSEGS.as_mut_ptr().add(2);
    }
}

// =============================================================================
// R_ClearDrawSegs
// =============================================================================

pub fn r_clear_draw_segs() {
    unsafe {
        DS_P = DRAWSEGS.as_mut_ptr();
    }
}

// =============================================================================
// R_AddLine
// =============================================================================

fn r_add_line(line: *mut Seg) {
    let viewangle = unsafe { state::VIEWANGLE };
    let clipangle = unsafe { state::CLIPANGLE };

    unsafe {
        CURLINE = line;

        let angle1 = r_point_to_angle((*(*line).v1).x, (*(*line).v1).y);
        let angle2 = r_point_to_angle((*(*line).v2).x, (*(*line).v2).y);

        let span = angle1.wrapping_sub(angle2);
        if span >= ANG180 {
            return;
        }

        state::RW_ANGLE1 = angle1 as i32;
        let mut angle1 = angle1.wrapping_sub(viewangle);
        let mut angle2 = angle2.wrapping_sub(viewangle);

        let two_clip = clipangle.wrapping_add(clipangle);

        let mut tspan = angle1.wrapping_add(clipangle);
        if tspan > two_clip {
            tspan = tspan.wrapping_sub(two_clip);
            if tspan >= span {
                return;
            }
            angle1 = clipangle;
        }
        tspan = clipangle.wrapping_sub(angle2);
        if tspan > two_clip {
            tspan = tspan.wrapping_sub(two_clip);
            if tspan >= span {
                return;
            }
            angle2 = clipangle.wrapping_neg();
        }

        let angle1_idx = ((angle1 + ANG90) >> ANGLETOFINESHIFT) as usize;
        let angle2_idx = ((angle2 + ANG90) >> ANGLETOFINESHIFT) as usize;
        let x1 = state::VIEWANGLETOX[angle1_idx.min(FINEANGLES / 2 - 1)];
        let x2 = state::VIEWANGLETOX[angle2_idx.min(FINEANGLES / 2 - 1)];

        if x1 == x2 {
            return;
        }

        BACKSECTOR = (*line).backsector;

        if BACKSECTOR.is_null() {
            r_clip_solid_wall_segment(x1, x2 - 1);
            return;
        }

        let front = FRONTSECTOR;
        let back = BACKSECTOR;

        if (*back).ceilingheight <= (*front).floorheight
            || (*back).floorheight >= (*front).ceilingheight
        {
            r_clip_solid_wall_segment(x1, x2 - 1);
            return;
        }

        if (*back).ceilingheight != (*front).ceilingheight
            || (*back).floorheight != (*front).floorheight
        {
            r_clip_pass_wall_segment(x1, x2 - 1);
            return;
        }

        let sidedef = (*line).sidedef;
        if (*back).ceilingpic == (*front).ceilingpic
            && (*back).floorpic == (*front).floorpic
            && (*back).lightlevel == (*front).lightlevel
            && (*sidedef).midtexture == 0
        {
            return;
        }

        r_clip_pass_wall_segment(x1, x2 - 1);
    }
}

// =============================================================================
// R_CheckBBox
// =============================================================================

const CHECKCOORD: [[i32; 4]; 12] = [
    [3, 0, 2, 1],
    [3, 0, 2, 0],
    [3, 1, 2, 0],
    [0, 0, 0, 0],
    [2, 0, 2, 1],
    [0, 0, 0, 0],
    [3, 1, 3, 0],
    [0, 0, 0, 0],
    [2, 0, 3, 1],
    [2, 1, 3, 1],
    [2, 1, 3, 0],
    [0, 0, 0, 0],
];

fn r_check_bbox(bspcoord: &[Fixed; 4]) -> bool {
    let viewx = unsafe { state::VIEWX };
    let viewy = unsafe { state::VIEWY };
    let viewangle = unsafe { state::VIEWANGLE };
    let clipangle = unsafe { state::CLIPANGLE };

    let boxx = if viewx <= bspcoord[BOXLEFT as usize] {
        0
    } else if viewx < bspcoord[BOXRIGHT as usize] {
        1
    } else {
        2
    };

    let boxy = if viewy >= bspcoord[BOXTOP as usize] {
        0
    } else if viewy > bspcoord[BOXBOTTOM as usize] {
        1
    } else {
        2
    };

    let boxpos = (boxy << 2) + boxx;
    if boxpos == 5 {
        return true;
    }

    let coords = &CHECKCOORD[boxpos];
    if coords[0] == 0 {
        return true;
    }

    let x1 = bspcoord[coords[0] as usize];
    let y1 = bspcoord[coords[1] as usize];
    let x2 = bspcoord[coords[2] as usize];
    let y2 = bspcoord[coords[3] as usize];

    let mut angle1 = r_point_to_angle(x1, y1).wrapping_sub(viewangle);
    let mut angle2 = r_point_to_angle(x2, y2).wrapping_sub(viewangle);
    let span = angle1.wrapping_sub(angle2);

    if span >= ANG180 {
        return true;
    }

    let two_clip = clipangle.wrapping_add(clipangle);

    let mut tspan = angle1.wrapping_add(clipangle);
    if tspan > two_clip {
        tspan = tspan.wrapping_sub(two_clip);
        if tspan >= span {
            return false;
        }
        angle1 = clipangle;
    }
    tspan = clipangle.wrapping_sub(angle2);
    if tspan > two_clip {
        tspan = tspan.wrapping_sub(two_clip);
        if tspan >= span {
            return false;
        }
        angle2 = clipangle.wrapping_neg();
    }

    let angle1_idx = ((angle1 + ANG90) >> ANGLETOFINESHIFT) as usize;
    let angle2_idx = ((angle2 + ANG90) >> ANGLETOFINESHIFT) as usize;
    let sx1 = unsafe { state::VIEWANGLETOX[angle1_idx.min(FINEANGLES / 2 - 1)] };
    let mut sx2 = unsafe { state::VIEWANGLETOX[angle2_idx.min(FINEANGLES / 2 - 1)] };

    if sx1 == sx2 {
        return false;
    }
    sx2 -= 1;

    unsafe {
        let mut start = SOLIDSEGS.as_ptr();
        while (*start).last < sx2 {
            start = start.add(1);
        }

        if sx1 >= (*start).first && sx2 <= (*start).last {
            return false;
        }
    }

    true
}

// =============================================================================
// R_Subsector
// =============================================================================

fn r_subsector(num: usize) {
    unsafe {
        state::SSCOUNT += 1;

        let numsubsectors = state::NUMSUBSECTORS as usize;
        if num >= numsubsectors {
            return;
        }

        let subsectors = state::SUBSECTORS;
        let segs = state::SEGS;
        if subsectors.is_null() || segs.is_null() {
            return;
        }

        let sub = &*subsectors.add(num);
        FRONTSECTOR = (*sub).sector;

        let count = (*sub).numlines as usize;
        let mut line = segs.add((*sub).firstline as usize);

        let viewz = state::VIEWZ;
        let floorplane = if (*FRONTSECTOR).floorheight < viewz {
            r_plane::r_find_plane(
                (*FRONTSECTOR).floorheight,
                (*FRONTSECTOR).floorpic as i32,
                (*FRONTSECTOR).lightlevel as i32,
            )
        } else {
            std::ptr::null_mut()
        };
        state::FLOORPLANE = floorplane;

        let skyflatnum = r_sky::SKYFLATNUM;
        let ceilingplane = if (*FRONTSECTOR).ceilingheight > viewz
            || (*FRONTSECTOR).ceilingpic as i32 == skyflatnum
        {
            r_plane::r_find_plane(
                (*FRONTSECTOR).ceilingheight,
                (*FRONTSECTOR).ceilingpic as i32,
                (*FRONTSECTOR).lightlevel as i32,
            )
        } else {
            std::ptr::null_mut()
        };
        state::CEILINGPLANE = ceilingplane;

        r_things::r_add_sprites(FRONTSECTOR);

        for _ in 0..count {
            r_add_line(line);
            line = line.add(1);
        }
    }
}

// =============================================================================
// R_RenderBSPNode
// =============================================================================

pub fn r_render_bsp_node(bspnum: i32) {
    let numnodes = unsafe { state::NUMNODES };
    let nodes = unsafe { state::NODES };

    if bspnum & (NF_SUBSECTOR as i32) != 0 {
        let num = if bspnum == -1 {
            0
        } else {
            (bspnum as u32 & !(NF_SUBSECTOR as u32)) as usize
        };
        r_subsector(num);
        return;
    }

    let bspnum = bspnum as usize;
    if nodes.is_null() || bspnum >= numnodes as usize {
        return;
    }

    unsafe {
        let bsp = nodes.add(bspnum);
        let side = r_point_on_side(state::VIEWX, state::VIEWY, bsp);

        r_render_bsp_node((*bsp).children[side as usize] as i32);

        let other_side = side ^ 1;
        let bbox = &(*bsp).bbox[other_side as usize];
        if r_check_bbox(bbox) {
            r_render_bsp_node((*bsp).children[other_side as usize] as i32);
        }
    }
}
