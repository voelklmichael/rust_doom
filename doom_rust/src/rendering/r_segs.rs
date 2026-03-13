//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Clipping: columns, horizontal spans, sky columns.
//
// Original: r_segs.h + r_segs.c

use crate::geometry::{finesine, ANG90, ANGLETOFINESHIFT};
use crate::m_fixed::{fixed_mul, FRACUNIT};
use crate::rendering::defs::{SIL_BOTH, SIL_BOTTOM, SIL_TOP, MAXDRAWSEGS};
use crate::rendering::r_main::{r_point_to_dist, r_scale_from_global_angle};
use crate::rendering::r_sky;
use crate::rendering::state;
use std::ptr;

const LIGHTSEGSHIFT: i32 = 4;

/// Store wall range for rendering. Populates drawsegs; actual drawing is stubbed.
pub fn r_store_wall_range(start: i32, stop: i32) {
    unsafe {
        let ds_p = state::DS_P;
        if ds_p.is_null() {
            return;
        }
        if ds_p as *const _ >= state::DRAWSEGS.as_ptr().add(MAXDRAWSEGS) {
            return;
        }

        let viewwidth = state::VIEWWIDTH;
        if start >= viewwidth || start > stop {
            return;
        }

        let curline = state::CURLINE;
        let frontsector = state::FRONTSECTOR;
        let backsector = state::BACKSECTOR;

        if curline.is_null() || frontsector.is_null() {
            return;
        }

        let sidedef = (*curline).sidedef;
        let linedef = (*curline).linedef;

        if !linedef.is_null() {
            (*linedef).flags |= crate::rendering::defs::ML_MAPPED;
        }
        state::SIDEDEF = sidedef;
        state::LINEDEF = linedef;

        let rw_angle1 = state::RW_ANGLE1 as u32;
        let rw_normalangle = (*curline).angle + ANG90;
        state::RW_NORMALANGLE = rw_normalangle;
        let mut offsetangle = rw_normalangle.wrapping_sub(rw_angle1);
        if offsetangle > ANG90 {
            offsetangle = ANG90;
        }
        let distangle = ANG90 - offsetangle;
        let hyp = r_point_to_dist((*(*curline).v1).x, (*(*curline).v1).y);
        let sineval = finesine((distangle >> ANGLETOFINESHIFT) as usize);
        state::RW_DISTANCE = fixed_mul(hyp, sineval);

        (*ds_p).x1 = start;
        (*ds_p).x2 = stop;
        (*ds_p).curline = curline;

        let viewangle = state::VIEWANGLE;
        (*ds_p).scale1 = r_scale_from_global_angle(viewangle + state::XTOVIEWANGLE[start as usize]);
        if stop > start {
            (*ds_p).scale2 =
                r_scale_from_global_angle(viewangle + state::XTOVIEWANGLE[stop as usize]);
            (*ds_p).scalestep = ((*ds_p).scale2 - (*ds_p).scale1) / (stop - start);
        } else {
            (*ds_p).scale2 = (*ds_p).scale1;
        }

        let viewz = state::VIEWZ;
        let _worldtop = (*frontsector).ceilingheight - viewz;

        (*ds_p).maskedtexturecol = ptr::null_mut();

        if backsector.is_null() {
            (*ds_p).silhouette = SIL_BOTH;
            (*ds_p).bsilheight = i32::MAX;
            (*ds_p).tsilheight = i32::MIN;
            (*ds_p).sprtopclip = ptr::null_mut();
            (*ds_p).sprbottomclip = ptr::null_mut();
        } else {
            (*ds_p).sprtopclip = ptr::null_mut();
            (*ds_p).sprbottomclip = ptr::null_mut();
            (*ds_p).silhouette = 0;

            if (*frontsector).floorheight > (*backsector).floorheight {
                (*ds_p).silhouette = SIL_BOTTOM;
                (*ds_p).bsilheight = (*frontsector).floorheight;
            } else if (*backsector).floorheight > viewz {
                (*ds_p).silhouette = SIL_BOTTOM;
                (*ds_p).bsilheight = i32::MAX;
            }

            if (*frontsector).ceilingheight < (*backsector).ceilingheight {
                (*ds_p).silhouette |= SIL_TOP;
                (*ds_p).tsilheight = (*frontsector).ceilingheight;
            } else if (*backsector).ceilingheight < viewz {
                (*ds_p).silhouette |= SIL_TOP;
                (*ds_p).tsilheight = i32::MIN;
            }

            if (*backsector).ceilingheight <= (*frontsector).floorheight {
                (*ds_p).sprbottomclip = ptr::null_mut();
                (*ds_p).bsilheight = i32::MAX;
                (*ds_p).silhouette |= SIL_BOTTOM;
            }

            if (*backsector).floorheight >= (*frontsector).ceilingheight {
                (*ds_p).sprtopclip = ptr::null_mut();
                (*ds_p).tsilheight = i32::MIN;
                (*ds_p).silhouette |= SIL_TOP;
            }

            let skyflatnum = r_sky::SKYFLATNUM;
            if (*frontsector).ceilingpic as i32 == skyflatnum
                && (*backsector).ceilingpic as i32 == skyflatnum
            {
                // worldtop = worldhigh handled in texture logic
            }

            if (*sidedef).midtexture != 0 {
                (*ds_p).maskedtexturecol = ptr::null_mut();
            }
        }

        state::DS_P = ds_p.add(1);
    }
}
