//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Line-of-sight / visibility checks. Uses REJECT and BSP traversal.
//
// Original: p_sight.c

use crate::m_fixed::{fixed_div, Fixed};
use crate::rendering::defs::ML_TWOSIDED;
use crate::rendering::state;
use crate::rendering::{with_r_main_state, with_r_main_state_mut, NF_SUBSECTOR};

use super::p_maputl::{p_divline_side, p_intercept_vector};
use super::p_mobj::Mobj;
use super::Divline;

/// Check if t1 has line of sight to t2.
/// Original: P_CheckSight
pub fn p_check_sight(t1: *const Mobj, t2: *const Mobj) -> bool {
    if t1.is_null() || t2.is_null() {
        return false;
    }

    let sub1_idx = unsafe { (*t1).subsector as usize };
    let sub2_idx = unsafe { (*t2).subsector as usize };

    let reject_ok = state::with_state(|s| {
        let sub1 = s.subsectors.get(sub1_idx);
        let sub2 = s.subsectors.get(sub2_idx);
        match (sub1, sub2) {
            (Some(ss1), Some(ss2)) => {
                let sec1_idx = ss1.sector_idx;
                let sec2_idx = ss2.sector_idx;
                let numsectors = s.sectors.len();
                if sec1_idx < numsectors && sec2_idx < numsectors {
                    let pnum = sec1_idx * numsectors + sec2_idx;
                    let bytenum = pnum >> 3;
                    let bitnum = 1 << (pnum & 7);
                    !s.rejectmatrix.get(bytenum).map_or(false, |&b| b & bitnum != 0)
                } else {
                    true
                }
            }
            _ => true,
        }
    });

    if !reject_ok {
        return false;
    }

    // BSP traversal
    let mut st = SightTrace {
        sightzstart: unsafe { (*t1).z + (*t1).height - ((*t1).height >> 2) },
        topslope: unsafe {
            ((*t2).z + (*t2).height) - ((*t1).z + (*t1).height - ((*t1).height >> 2))
        },
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

    with_r_main_state_mut(|rm| rm.validcount += 1);

    let numnodes = state::with_state(|s| s.numnodes);
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
    let validcount = with_r_main_state(|rm| rm.validcount);

    state::with_state_mut(|s| {
        let sub = match s.subsectors.get(num) {
            Some(sub) => sub,
            None => return true,
        };
        let count = sub.numlines as usize;
        let firstline = sub.firstline as usize;

        for seg_idx in firstline..(firstline + count) {
            let seg = match s.segs.get(seg_idx) {
                Some(seg) => seg,
                None => continue,
            };
            let linedef_idx = seg.linedef_idx;
            let ld = match s.lines.get_mut(linedef_idx) {
                Some(ld) => ld,
                None => continue,
            };
            if ld.validcount == validcount {
                continue;
            }
            ld.validcount = validcount;

            let (v1x, v1y, v2x, v2y) = match (
                s.vertexes.get(seg.v1_idx),
                s.vertexes.get(seg.v2_idx),
            ) {
                (Some(v1), Some(v2)) => (v1.x, v1.y, v2.x, v2.y),
                _ => continue,
            };

            let s1 = p_divline_side(v1x, v1y, &st.strace);
            let s2 = p_divline_side(v2x, v2y, &st.strace);
            if s1 == s2 {
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
                continue;
            }

            if ld.backsector_idx.is_none() {
                return false;
            }
            if (ld.flags & ML_TWOSIDED) == 0 {
                return false;
            }

            let front = match s.sectors.get(seg.frontsector_idx) {
                Some(f) => f,
                None => continue,
            };
            let back = match s.sectors.get(seg.backsector_idx) {
                Some(b) => b,
                None => continue,
            };

            if front.floorheight == back.floorheight && front.ceilingheight == back.ceilingheight {
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
        }

        true
    })
}

/// Returns true if strace crosses the BSP node successfully.
/// Original: P_CrossBSPNode
fn p_cross_bsp_node(st: &mut SightTrace, bspnum: i32) -> bool {
    if (bspnum as u32 & NF_SUBSECTOR as u32) != 0 {
        let ss_num = if bspnum == -1 {
            0
        } else {
            (bspnum as u32 & !(NF_SUBSECTOR as u32)) as usize
        };
        return p_cross_subsector(st, ss_num);
    }

    let bsp = match state::with_state(|s| s.nodes.get(bspnum as usize).cloned()) {
        Some(n) => n,
        None => return true,
    };

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
