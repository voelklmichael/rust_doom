//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Line-of-sight / visibility checks. Uses REJECT and BSP traversal.
//
// Original: p_sight.c

use crate::m_fixed::{fixed_div, Fixed};
use crate::rendering::defs::{ML_TWOSIDED, Subsector};
use crate::rendering::state;
use crate::rendering::{NF_SUBSECTOR, VALIDCOUNT};

use super::p_maputl::{p_divline_side, p_intercept_vector};
use super::p_mobj::Mobj;
use super::Divline;

/// Check if t1 has line of sight to t2.
/// Original: P_CheckSight
pub fn p_check_sight(t1: *const Mobj, t2: *const Mobj) -> bool {
    if t1.is_null() || t2.is_null() {
        return false;
    }

    let s1_sector = unsafe { (*t1).subsector };
    let s2_sector = unsafe { (*t2).subsector };
    if s1_sector.is_null() || s2_sector.is_null() {
        return true; // no subsector - assume visible (e.g. MF_NOSECTOR)
    }

    let subsectors = unsafe { state::SUBSECTORS };
    let sectors = unsafe { state::SECTORS };
    let numsectors = unsafe { state::NUMSECTORS };
    let rejectmatrix = unsafe { state::REJECTMATRIX };

    if subsectors.is_null() || sectors.is_null() || numsectors <= 0 {
        return true;
    }

    // REJECT lookup: sector indices from subsector
    let ss1 = unsafe { &*s1_sector.cast::<Subsector>() };
    let ss2 = unsafe { &*s2_sector.cast::<Subsector>() };
    let sec1 = ss1.sector;
    let sec2 = ss2.sector;
    if sec1.is_null() || sec2.is_null() {
        return true;
    }

    let s1 = unsafe { sec1.offset_from(sectors) } as i32;
    let s2 = unsafe { sec2.offset_from(sectors) } as i32;
    if s1 < 0 || s2 < 0 || s1 >= numsectors || s2 >= numsectors {
        return true;
    }

    // Check REJECT table - bit set means cannot see
    if !rejectmatrix.is_null() {
        let pnum = (s1 as usize) * (numsectors as usize) + (s2 as usize);
        let bytenum = pnum >> 3;
        let bitnum = 1 << (pnum & 7);
        if unsafe { *rejectmatrix.add(bytenum) } & bitnum != 0 {
            return false;
        }
    }

    // BSP traversal
    let mut st = SightTrace {
        sightzstart: unsafe { (*t1).z + (*t1).height - ((*t1).height >> 2) },
        topslope: unsafe { ((*t2).z + (*t2).height) - ((*t1).z + (*t1).height - ((*t1).height >> 2)) },
        bottomslope: unsafe { (*t2).z - ((*t1).z + (*t1).height - ((*t1).height >> 2)) },
        strace: Divline {
            x: unsafe { (*t1).x },
            y: unsafe { (*t1).y },
            dx: unsafe { (*t2).x - (*t1).x },
            dy: unsafe { (*t2).y - (*t1).y },
        },
        t2x: unsafe { (*t2).x },
        t2y: unsafe { (*t2).y },
    };

    unsafe {
        VALIDCOUNT += 1;
    }

    let numnodes = unsafe { state::NUMNODES };
    if numnodes <= 0 {
        return true;
    }

    p_cross_bsp_node(&mut st, (numnodes - 1) as i32)
}

/// Sight trace state (C globals: sightzstart, topslope, bottomslope, strace, t2x, t2y)
struct SightTrace {
    sightzstart: Fixed,
    topslope: Fixed,
    bottomslope: Fixed,
    strace: Divline,
    t2x: Fixed,
    t2y: Fixed,
}

/// Returns true if strace crosses the subsector successfully.
/// Original: P_CrossSubsector
fn p_cross_subsector(st: &mut SightTrace, num: usize) -> bool {
    let subsectors = unsafe { state::SUBSECTORS };
    let segs = unsafe { state::SEGS };
    let validcount = unsafe { VALIDCOUNT };

    if subsectors.is_null() || segs.is_null() {
        return true;
    }

    let sub = unsafe { &*subsectors.add(num) };
    let count = sub.numlines as usize;
    let firstline = sub.firstline as usize;
    let mut seg = unsafe { segs.add(firstline) };

    for _ in 0..count {
        let line = unsafe { (*seg).linedef };
        if line.is_null() {
            seg = unsafe { seg.add(1) };
            continue;
        }

        let ld = unsafe { &*line };
        if ld.validcount == validcount {
            seg = unsafe { seg.add(1) };
            continue;
        }
        unsafe {
            (*line).validcount = validcount;
        }

        let v1 = ld.v1;
        let v2 = ld.v2;
        if v1.is_null() || v2.is_null() {
            seg = unsafe { seg.add(1) };
            continue;
        }

        let v1x = unsafe { (*v1).x };
        let v1y = unsafe { (*v1).y };
        let v2x = unsafe { (*v2).x };
        let v2y = unsafe { (*v2).y };

        let s1 = p_divline_side(v1x, v1y, &st.strace);
        let s2 = p_divline_side(v2x, v2y, &st.strace);
        if s1 == s2 {
            seg = unsafe { seg.add(1) };
            continue;
        }

        let divl = Divline {
            x: v1x,
            y: v1y,
            dx: v2x - v1x,
            dy: v2y - v1y,
        };
        let s1 = p_divline_side(st.strace.x, st.strace.y, &divl);
        let s2 = p_divline_side(st.t2x, st.t2y, &divl);
        if s1 == s2 {
            seg = unsafe { seg.add(1) };
            continue;
        }

        if ld.backsector.is_null() {
            return false;
        }
        if (ld.flags & ML_TWOSIDED) == 0 {
            return false;
        }

        let seg_ref = unsafe { &*seg };
        let front = seg_ref.frontsector;
        let back = seg_ref.backsector;
        if front.is_null() || back.is_null() {
            seg = unsafe { seg.add(1) };
            continue;
        }
        let front = unsafe { &*front };
        let back = unsafe { &*back };

        if front.floorheight == back.floorheight && front.ceilingheight == back.ceilingheight {
            seg = unsafe { seg.add(1) };
            continue;
        }

        let opentop = if front.ceilingheight < back.ceilingheight {
            front.ceilingheight
        } else {
            back.ceilingheight
        };
        let openbottom = if front.floorheight > back.floorheight {
            front.floorheight
        } else {
            back.floorheight
        };

        if openbottom >= opentop {
            return false;
        }

        let frac = p_intercept_vector(&st.strace, &divl);

        if front.floorheight != back.floorheight {
            let slope = fixed_div(openbottom - st.sightzstart, frac);
            if slope > st.bottomslope {
                st.bottomslope = slope;
            }
        }
        if front.ceilingheight != back.ceilingheight {
            let slope = fixed_div(opentop - st.sightzstart, frac);
            if slope < st.topslope {
                st.topslope = slope;
            }
        }

        if st.topslope <= st.bottomslope {
            return false;
        }

        seg = unsafe { seg.add(1) };
    }

    true
}

/// Returns true if strace crosses the BSP node successfully.
/// Original: P_CrossBSPNode
fn p_cross_bsp_node(st: &mut SightTrace, bspnum: i32) -> bool {
    let nodes = unsafe { state::NODES };

    if (bspnum as u32 & NF_SUBSECTOR as u32) != 0 {
        let ss_num = if bspnum == -1 {
            0
        } else {
            (bspnum as u32 & !(NF_SUBSECTOR as u32)) as usize
        };
        return p_cross_subsector(st, ss_num);
    }

    if nodes.is_null() {
        return true;
    }

    let bsp = unsafe { &*nodes.add(bspnum as usize) };

    // Cast node to divline for P_DivlineSide
    let node_dl = Divline {
        x: bsp.x,
        y: bsp.y,
        dx: bsp.dx,
        dy: bsp.dy,
    };

    let mut side = p_divline_side(st.strace.x, st.strace.y, &node_dl);
    if side == 2 {
        side = 0;
    }

    let child = bsp.children[side as usize];
    let child_bspnum = child as i16 as i32;

    if !p_cross_bsp_node(st, child_bspnum) {
        return false;
    }

    let side2 = p_divline_side(st.t2x, st.t2y, &node_dl);
    if side == side2 {
        return true;
    }

    let other_child = bsp.children[(side as usize) ^ 1];
    let other_bspnum = other_child as i16 as i32;
    p_cross_bsp_node(st, other_bspnum)
}
