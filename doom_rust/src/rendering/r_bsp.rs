// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  BSP traversal, handling of LineSegs for rendering.
//
// Original: r_bsp.h + r_bsp.c

use crate::geometry::{ANG180, ANG90, ANGLETOFINESHIFT, FINEANGLES};
use crate::m_fixed::Fixed;
use crate::rendering::defs::{Seg, SideDef};
use crate::rendering::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::rendering::r_main::{r_point_on_side, r_point_to_angle, NF_SUBSECTOR};
use crate::rendering::r_plane;
use crate::rendering::r_segs;
use crate::rendering::r_sky;
use crate::rendering::r_things;
use crate::rendering::state;
use std::sync::{Mutex, OnceLock};

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
// State (from r_bsp.c) - thread-safe via OnceLock + Mutex
// =============================================================================

struct BspState {
    solidsegs: [ClipRange; MAXSEGS],
    newend: usize,
}

impl Default for BspState {
    fn default() -> Self {
        Self {
            solidsegs: [ClipRange { first: 0, last: 0 }; MAXSEGS],
            newend: 0,
        }
    }
}

static BSP_STATE: OnceLock<Mutex<BspState>> = OnceLock::new();

fn with_bsp_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut BspState) -> R,
{
    let mut guard = BSP_STATE
        .get_or_init(|| Mutex::new(BspState::default()))
        .lock()
        .unwrap();
    f(&mut guard)
}

// =============================================================================
// R_ClipSolidWallSegment
// =============================================================================

fn r_clip_solid_wall_segment(first: i32, last: i32) {
    with_bsp_state(|state| {
        let solidsegs = &mut state.solidsegs[..];
        let newend = &mut state.newend;

        let mut start_idx = 0;
        while solidsegs[start_idx].last < first - 1 {
            start_idx += 1;
        }

        if first < solidsegs[start_idx].first {
            if last < solidsegs[start_idx].first - 1 {
                r_segs::r_store_wall_range(first, last);
                let mut next_idx = *newend;
                *newend += 1;

                while next_idx != start_idx {
                    solidsegs[next_idx] = solidsegs[next_idx - 1];
                    next_idx -= 1;
                }
                solidsegs[next_idx].first = first;
                solidsegs[next_idx].last = last;
                return;
            }
            r_segs::r_store_wall_range(first, solidsegs[start_idx].first - 1);
            solidsegs[start_idx].first = first;
        }

        if last <= solidsegs[start_idx].last {
            return;
        }

        let mut next_idx = start_idx;
        while last >= solidsegs[next_idx + 1].first - 1 {
            r_segs::r_store_wall_range(
                solidsegs[next_idx].last + 1,
                solidsegs[next_idx + 1].first - 1,
            );
            next_idx += 1;

            if last <= solidsegs[next_idx].last {
                solidsegs[start_idx].last = solidsegs[next_idx].last;
                return;
            }
        }

        if last > solidsegs[next_idx].last {
            r_segs::r_store_wall_range(solidsegs[next_idx].last + 1, last);
            solidsegs[start_idx].last = last;

            let mut copy_src = next_idx + 1;
            let mut copy_dst = start_idx + 1;
            while copy_src < *newend {
                solidsegs[copy_dst] = solidsegs[copy_src];
                copy_src += 1;
                copy_dst += 1;
            }
            *newend = copy_dst;
        }
    });
}

// =============================================================================
// R_ClipPassWallSegment
// =============================================================================

fn r_clip_pass_wall_segment(first: i32, last: i32) {
    with_bsp_state(|state| {
        let solidsegs = &state.solidsegs[..];

        let mut start_idx = 0;
        while solidsegs[start_idx].last < first - 1 {
            start_idx += 1;
        }

        if first < solidsegs[start_idx].first {
            if last < solidsegs[start_idx].first - 1 {
                r_segs::r_store_wall_range(first, last);
                return;
            }
            r_segs::r_store_wall_range(first, solidsegs[start_idx].first - 1);
        }

        if last <= solidsegs[start_idx].last {
            return;
        }

        let mut idx = start_idx;
        while last >= solidsegs[idx + 1].first - 1 {
            r_segs::r_store_wall_range(solidsegs[idx].last + 1, solidsegs[idx + 1].first - 1);
            idx += 1;

            if last <= solidsegs[idx].last {
                return;
            }
        }

        r_segs::r_store_wall_range(solidsegs[idx].last + 1, last);
    });
}

// =============================================================================
// R_ClearClipSegs
// =============================================================================

pub fn r_clear_clip_segs() {
    with_bsp_state(|state| {
        state.solidsegs[0].first = -0x7fff_ffff;
        state.solidsegs[0].last = -1;
        state.solidsegs[1].first = state::with_state(|s| s.viewwidth);
        state.solidsegs[1].last = 0x7fff_ffff;
        state.newend = 2;
    });
}

// =============================================================================
// R_ClearDrawSegs
// =============================================================================

pub fn r_clear_draw_segs() {
    state::with_state_mut(|s| s.ds_p = s.drawsegs.as_mut_ptr());
}

// =============================================================================
// R_AddLine
// =============================================================================

fn r_add_line(seg_idx: usize) {
    let action = state::with_state_mut(|s| {
        let viewangle = s.viewangle;
        let clipangle = s.clipangle;
        let seg = match s.segs.get(seg_idx) {
            Some(seg) => seg,
            None => return None,
        };

        s.curline_idx = Some(seg_idx);

        let (v1x, v1y, v2x, v2y) = {
            let v1 = s
                .vertexes
                .get(seg.v1_idx)
                .copied()
                .unwrap_or(crate::rendering::defs::Vertex { x: 0, y: 0 });
            let v2 = s
                .vertexes
                .get(seg.v2_idx)
                .copied()
                .unwrap_or(crate::rendering::defs::Vertex { x: 0, y: 0 });
            (v1.x, v1.y, v2.x, v2.y)
        };

        let angle1 = r_point_to_angle(v1x, v1y);
        let angle2 = r_point_to_angle(v2x, v2y);

        let span = angle1.wrapping_sub(angle2);
        if span >= ANG180 {
            return None;
        }

        s.rw_angle1 = angle1 as i32;
        let mut angle1 = angle1.wrapping_sub(viewangle);
        let mut angle2 = angle2.wrapping_sub(viewangle);

        let two_clip = clipangle.wrapping_add(clipangle);

        let mut tspan = angle1.wrapping_add(clipangle);
        if tspan > two_clip {
            tspan = tspan.wrapping_sub(two_clip);
            if tspan >= span {
                return None;
            }
            angle1 = clipangle;
        }
        tspan = clipangle.wrapping_sub(angle2);
        if tspan > two_clip {
            tspan = tspan.wrapping_sub(two_clip);
            if tspan >= span {
                return None;
            }
            angle2 = clipangle.wrapping_neg();
        }

        let angle1_idx = ((angle1 + ANG90) >> ANGLETOFINESHIFT) as usize;
        let angle2_idx = ((angle2 + ANG90) >> ANGLETOFINESHIFT) as usize;
        let x1 = s.viewangletox[angle1_idx.min(FINEANGLES / 2 - 1)];
        let x2 = s.viewangletox[angle2_idx.min(FINEANGLES / 2 - 1)];

        if x1 == x2 {
            return None;
        }

        s.backsector_idx = Some(seg.backsector_idx);

        let front_idx = s.frontsector_idx.unwrap_or(0);
        let back_idx = seg.backsector_idx;

        return match (s.sectors.get(front_idx), s.sectors.get(back_idx)) {
            (Some(front), Some(back)) => {
                let solid = back.ceilingheight <= front.floorheight
                    || back.floorheight >= front.ceilingheight;
                let pass = back.ceilingheight != front.ceilingheight
                    || back.floorheight != front.floorheight;
                let skip = s
                    .sides
                    .get(seg.sidedef_idx)
                    .map(|sd| {
                        back.ceilingpic == front.ceilingpic
                            && back.floorpic == front.floorpic
                            && back.lightlevel == front.lightlevel
                            && sd.midtexture == 0
                    })
                    .unwrap_or(false);
                Some((x1, x2, solid, pass, skip))
            }
            _ => Some((x1, x2, true, false, false)),
        };
    });

    if let Some((x1, x2, solid, pass, skip)) = action {
        if solid {
            r_clip_solid_wall_segment(x1, x2 - 1);
        } else if pass || !skip {
            r_clip_pass_wall_segment(x1, x2 - 1);
        }
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
    let viewx = state::with_state(|s| s.viewx);
    let viewy = state::with_state(|s| s.viewy);
    let viewangle = state::with_state(|s| s.viewangle);
    let clipangle = state::with_state(|s| s.clipangle);

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
    let sx1 = state::with_state(|s| s.viewangletox[angle1_idx.min(FINEANGLES / 2 - 1)]);
    let mut sx2 = state::with_state(|s| s.viewangletox[angle2_idx.min(FINEANGLES / 2 - 1)]);

    if sx1 == sx2 {
        return false;
    }
    sx2 -= 1;

    with_bsp_state(|state| {
        let solidsegs = &state.solidsegs[..];
        let mut start_idx = 0;
        while solidsegs[start_idx].last < sx2 {
            start_idx += 1;
        }

        if sx1 >= solidsegs[start_idx].first && sx2 <= solidsegs[start_idx].last {
            return false;
        }
        true
    })
}

// =============================================================================
// R_Subsector
// =============================================================================

fn r_subsector(num: usize) {
    state::with_state_mut(|s| s.sscount += 1);

    let (sub, frontsector_idx, firstline, count) = state::with_state(|s| {
        let numsubsectors = s.numsubsectors as usize;
        if num >= numsubsectors {
            return (None, 0, 0, 0);
        }
        let sub = s.subsectors.get(num).cloned();
        let (fs, fl, c) = sub
            .as_ref()
            .map(|sub| {
                (
                    sub.sector_idx,
                    sub.firstline as usize,
                    sub.numlines as usize,
                )
            })
            .unwrap_or((0, 0, 0));
        (sub, fs, fl, c)
    });

    if sub.is_none() || count == 0 {
        return;
    }

    state::with_state_mut(|s| s.frontsector_idx = Some(frontsector_idx));

    let (floorplane, ceilingplane) = state::with_state(|s| {
        let viewz = s.viewz;
        let front = s.sectors.get(frontsector_idx);
        let floorplane = front
            .filter(|f| f.floorheight < viewz)
            .map(|f| r_plane::r_find_plane(f.floorheight, f.floorpic as i32, f.lightlevel as i32))
            .unwrap_or(std::ptr::null_mut());
        let skyflatnum = r_sky::with_r_sky_state(|rs| rs.skyflatnum);
        let ceilingplane = front
            .filter(|f| f.ceilingheight > viewz || f.ceilingpic as i32 == skyflatnum)
            .map(|f| {
                r_plane::r_find_plane(f.ceilingheight, f.ceilingpic as i32, f.lightlevel as i32)
            })
            .unwrap_or(std::ptr::null_mut());
        (floorplane, ceilingplane)
    });
    state::with_state_mut(|s| {
        s.floorplane = floorplane;
        s.ceilingplane = ceilingplane;
    });

    r_things::r_add_sprites(frontsector_idx);

    for i in 0..count {
        r_add_line(firstline + i);
    }
}

// =============================================================================
// R_RenderBSPNode
// =============================================================================

pub fn r_render_bsp_node(bspnum: i32) {
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
    let (child0, child1, bbox1) = state::with_state(|s| {
        if bspnum >= s.nodes.len() {
            return (0u16, 0u16, None);
        }
        let bsp = &s.nodes[bspnum];
        let side = r_point_on_side(s.viewx, s.viewy, bsp);
        let other_side = side ^ 1;
        (
            bsp.children[side as usize],
            bsp.children[other_side as usize],
            Some(bsp.bbox[other_side as usize]),
        )
    });

    if let Some(bbox) = bbox1 {
        r_render_bsp_node(child0 as i32);
        if r_check_bbox(&bbox) {
            r_render_bsp_node(child1 as i32);
        }
    }
}
