// TODO(UNSAFE_ELIMINATION): Remove when migrated to Vec + indices
#[allow(unsafe_code)]
//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Clipping: columns, horizontal spans, sky columns.
//
// Original: r_segs.h + r_segs.c

use crate::geometry::{finesine, finecosine, finetangent, ANG90, ANG180, ANGLETOFINESHIFT};
use crate::m_fixed::{fixed_mul, Fixed, FRACBITS};
use crate::rendering::defs::{Line, Seg, SideDef, SIL_BOTH, SIL_BOTTOM, SIL_TOP, MAXDRAWSEGS};
use crate::rendering::r_data::r_get_column;
use crate::rendering::r_draw::{colfunc, with_r_draw_state_mut};
use crate::rendering::r_main::{
    r_point_to_dist, r_scale_from_global_angle, with_r_main_state, LIGHTLEVELS, LIGHTSCALESHIFT,
    MAXLIGHTSCALE,
};
use crate::rendering::r_plane::{r_check_plane, with_plane_state};
use crate::rendering::r_sky;
use crate::rendering::state;
use std::ptr::{self, addr_of};
use std::sync::{Mutex, OnceLock};

const LIGHTSEGSHIFT: i32 = 4;
const HEIGHTBITS: i32 = 12;
const HEIGHTUNIT: i32 = 1 << HEIGHTBITS;

/// SHRT_MAX - used as sentinel for masked texture column.
const SHRT_MAX: i16 = 0x7fff;

// =============================================================================
// State (from r_segs.c) - thread-safe via OnceLock + Mutex
// =============================================================================

struct SegState {
    segtextured: bool,
    markfloor: bool,
    markceiling: bool,
    maskedtexture: bool,
    toptexture: i32,
    bottomtexture: i32,
    midtexture: i32,
    rw_x: i32,
    rw_stopx: i32,
    rw_centerangle: u32,
    rw_offset: Fixed,
    rw_scale: Fixed,
    rw_scalestep: Fixed,
    rw_midtexturemid: Fixed,
    rw_topexturemid: Fixed,
    rw_bottomtexturemid: Fixed,
    worldtop: Fixed,
    worldbottom: Fixed,
    worldhigh: Fixed,
    worldlow: Fixed,
    pixhigh: Fixed,
    pixlow: Fixed,
    pixhighstep: Fixed,
    pixlowstep: Fixed,
    topfrac: Fixed,
    topstep: Fixed,
    bottomfrac: Fixed,
    bottomstep: Fixed,
    walllights: *mut *mut u8,
    maskedtexturecol: *mut i16,
}

impl Default for SegState {
    fn default() -> Self {
        Self {
            segtextured: false,
            markfloor: false,
            markceiling: false,
            maskedtexture: false,
            toptexture: 0,
            bottomtexture: 0,
            midtexture: 0,
            rw_x: 0,
            rw_stopx: 0,
            rw_centerangle: 0,
            rw_offset: 0,
            rw_scale: 0,
            rw_scalestep: 0,
            rw_midtexturemid: 0,
            rw_topexturemid: 0,
            rw_bottomtexturemid: 0,
            worldtop: 0,
            worldbottom: 0,
            worldhigh: 0,
            worldlow: 0,
            pixhigh: 0,
            pixlow: 0,
            pixhighstep: 0,
            pixlowstep: 0,
            topfrac: 0,
            topstep: 0,
            bottomfrac: 0,
            bottomstep: 0,
            walllights: ptr::null_mut(),
            maskedtexturecol: ptr::null_mut(),
        }
    }
}

unsafe impl Send for SegState {}

static SEG_STATE: OnceLock<Mutex<SegState>> = OnceLock::new();

fn with_seg_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut SegState) -> R,
{
    let mut guard = SEG_STATE
        .get_or_init(|| Mutex::new(SegState::default()))
        .lock()
        .unwrap();
    f(&mut guard)
}

// =============================================================================
// R_RenderSegLoop - core loop, draws walls and marks floor/ceiling
// =============================================================================

fn r_render_seg_loop(
    ps: &mut crate::rendering::r_plane::PlaneState,
    ss: &mut SegState,
) {
    let viewheight = state::with_state(|s| s.viewheight);
    let floorplane = state::with_state(|s| s.floorplane);
    let ceilingplane = state::with_state(|s| s.ceilingplane);

    let floorclip = &mut ps.floorclip;
    let ceilingclip = &mut ps.ceilingclip;

    let mut rw_x = ss.rw_x;
    unsafe {
    let stopx = ss.rw_stopx;
    let mut rw_scale = ss.rw_scale;
    let mut topfrac = ss.topfrac;
    let mut bottomfrac = ss.bottomfrac;
    let mut pixhigh = ss.pixhigh;
    let mut pixlow = ss.pixlow;

    while rw_x < stopx {
        let rw_x_usize = rw_x as usize;
        let yl = (topfrac >> HEIGHTBITS) as i32;
        let mut yl = yl;
        if yl < ceilingclip[rw_x_usize] as i32 + 1 {
            yl = ceilingclip[rw_x_usize] as i32 + 1;
        }

        if ss.markceiling {
            let top = ceilingclip[rw_x_usize] as i32 + 1;
            let mut bottom = yl - 1;
            if bottom >= floorclip[rw_x_usize] as i32 {
                bottom = floorclip[rw_x_usize] as i32 - 1;
            }
            if top <= bottom && !ceilingplane.is_null() {
                unsafe {
                    (*ceilingplane).top[rw_x_usize] = top as u8;
                    (*ceilingplane).bottom[rw_x_usize] = bottom as u8;
                }
            }
        }

        let mut yh = (bottomfrac >> HEIGHTBITS) as i32;
        if yh >= floorclip[rw_x_usize] as i32 {
            yh = floorclip[rw_x_usize] as i32 - 1;
        }

        if ss.markfloor {
            let top = yh + 1;
            let mut bottom = floorclip[rw_x_usize] as i32 - 1;
            if top <= ceilingclip[rw_x_usize] as i32 {
                bottom = ceilingclip[rw_x_usize] as i32 + 1;
            }
            if top <= bottom && !floorplane.is_null() {
                unsafe {
                    (*floorplane).top[rw_x_usize] = top as u8;
                    (*floorplane).bottom[rw_x_usize] = bottom as u8;
                }
            }
        }

        let texturecolumn: i32 = if ss.segtextured {
            let angle = (ss.rw_centerangle
                .wrapping_add(state::with_state(|s| s.xtoviewangle[rw_x_usize])))
                >> ANGLETOFINESHIFT;
            let tanval = finetangent(angle as usize);
            (ss.rw_offset - fixed_mul(tanval, state::with_state(|s| s.rw_distance))) >> FRACBITS
        } else {
            0
        };

        let fixcol = with_r_main_state(|rm| rm.fixedcolormap);
        let dc_colormap = if ss.segtextured && fixcol.is_null() {
            let index = (rw_scale >> LIGHTSCALESHIFT) as usize;
            let index = index.min(MAXLIGHTSCALE - 1);
            unsafe { ss.walllights.add(index).read() }
        } else if !fixcol.is_null() {
            fixcol
        } else {
            crate::rendering::r_draw::with_r_draw_state(|rd| rd.dc_colormap)
        };

            crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                rd.dc_colormap = dc_colormap;
                rd.dc_x = rw_x;
                rd.dc_iscale = if rw_scale != 0 {
                    0xffff_ffffu32 / (rw_scale as u32)
                } else {
                    0
                };
            });

        if ss.midtexture != 0 {
            crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                rd.dc_yl = yl;
                rd.dc_yh = yh;
                rd.dc_texturemid = ss.rw_midtexturemid;
                rd.dc_source = r_get_column(ss.midtexture, texturecolumn);
            });
            colfunc();
            ceilingclip[rw_x_usize] = viewheight as i16;
            floorclip[rw_x_usize] = -1;
        } else {
            if ss.toptexture != 0 {
                let mut mid = (pixhigh >> HEIGHTBITS) as i32;
                pixhigh += ss.pixhighstep;

                if mid >= floorclip[rw_x_usize] as i32 {
                    mid = floorclip[rw_x_usize] as i32 - 1;
                }
                if mid >= yl {
                    crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                        rd.dc_yl = yl;
                        rd.dc_yh = mid;
                        rd.dc_texturemid = ss.rw_topexturemid;
                        rd.dc_source = r_get_column(ss.toptexture, texturecolumn);
                    });
                    colfunc();
                    ceilingclip[rw_x_usize] = mid as i16;
                } else {
                    ceilingclip[rw_x_usize] = (yl - 1) as i16;
                }
            } else if ss.markceiling {
                ceilingclip[rw_x_usize] = (yl - 1) as i16;
            }

            if ss.bottomtexture != 0 {
                let mut mid = ((pixlow + HEIGHTUNIT - 1) >> HEIGHTBITS) as i32;
                pixlow += ss.pixlowstep;

                if mid <= ceilingclip[rw_x_usize] as i32 {
                    mid = ceilingclip[rw_x_usize] as i32 + 1;
                }
                if mid <= yh {
                    crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                        rd.dc_yl = mid;
                        rd.dc_yh = yh;
                        rd.dc_texturemid = ss.rw_bottomtexturemid;
                        rd.dc_source = r_get_column(ss.bottomtexture, texturecolumn);
                    });
                    colfunc();
                    floorclip[rw_x_usize] = mid as i16;
                } else {
                    floorclip[rw_x_usize] = (yh + 1) as i16;
                }
            } else if ss.markfloor {
                floorclip[rw_x_usize] = (yh + 1) as i16;
            }

            if ss.maskedtexture {
                if !ss.maskedtexturecol.is_null() {
                    unsafe {
                        *ss.maskedtexturecol.add(rw_x_usize) = texturecolumn as i16;
                    }
                }
            }
        }

        rw_scale += ss.rw_scalestep;
        topfrac += ss.topstep;
        bottomfrac += ss.bottomstep;
        rw_x += 1;
    }

    ss.pixhigh = pixhigh;
    ss.pixlow = pixlow;
    }
}

/// Store wall range and render. Populates drawsegs, draws walls, marks floor/ceiling.
/// Lock order: plane first, then seg.
pub fn r_store_wall_range(start: i32, stop: i32) {
    with_plane_state(|ps| {
        with_seg_state(|ss| {
            r_store_wall_range_inner(start, stop, ps, ss);
        });
    });
}

fn r_store_wall_range_inner(
    start: i32,
    stop: i32,
    ps: &mut crate::rendering::r_plane::PlaneState,
    ss: &mut SegState,
) {
    let ds_p = state::with_state(|s| s.ds_p);
    if ds_p.is_null() {
        return;
    }
    if ds_p as *const _ >= unsafe { state::with_state(|s| s.drawsegs.as_ptr().add(MAXDRAWSEGS)) } {
        return;
    }

    let viewwidth = state::with_state(|s| s.viewwidth);
    let viewheight = state::with_state(|s| s.viewheight);
    let viewz = state::with_state(|s| s.viewz);
    if start >= viewwidth || start > stop {
        return;
    }

    let (curline_idx, curline_ptr, frontsector_idx, backsector_idx, sidedef_idx, linedef_idx, frontsector, backsector, v1x, v1y, v2x, v2y, curline_offset, rw_normalangle) = state::with_state(|s| {
        let curline_idx = match s.curline_idx {
            Some(idx) => idx,
            None => return (None, std::ptr::null_mut(), None, None, None, None, None, None, 0, 0, 0, 0, 0, 0u32),
        };
        let curline = match s.segs.get(curline_idx) {
            Some(seg) => seg,
            None => return (None, std::ptr::null_mut(), None, None, None, None, None, None, 0, 0, 0, 0, 0, 0u32),
        };
        let curline_ptr = curline as *const _ as *mut Seg;
        let v1 = s.vertexes.get(curline.v1_idx).copied().unwrap_or(crate::rendering::defs::Vertex { x: 0, y: 0 });
        let v2 = s.vertexes.get(curline.v2_idx).copied().unwrap_or(crate::rendering::defs::Vertex { x: 0, y: 0 });
        let frontsector = s.frontsector_idx.and_then(|i| s.sectors.get(i).cloned());
        let backsector = s.backsector_idx.and_then(|i| s.sectors.get(i).cloned());
        let sidedef = s.sides.get(curline.sidedef_idx).cloned();
        let linedef = s.lines.get(curline.linedef_idx).cloned();
        let rw_normalangle = curline.angle + ANG90;
        (
            Some(curline_idx),
            curline_ptr,
            s.frontsector_idx,
            s.backsector_idx,
            Some(curline.sidedef_idx),
            Some(curline.linedef_idx),
            frontsector,
            backsector,
            v1.x,
            v1.y,
            v2.x,
            v2.y,
            curline.offset,
            rw_normalangle,
        )
    });

    if curline_idx.is_none() || frontsector.is_none() {
        return;
    }

    let frontsector = frontsector.as_ref().unwrap();
    let backsector = backsector.as_ref();
    let sidedef = state::with_state(|s| sidedef_idx.and_then(|i| s.sides.get(i).cloned()));
    let linedef = state::with_state(|s| linedef_idx.and_then(|i| s.lines.get(i).cloned()));

    if let (Some(ref ld), Some(idx)) = (linedef.as_ref(), linedef_idx) {
        state::with_state_mut(|s| {
            if let Some(l) = s.lines.get_mut(idx) {
                l.flags |= crate::rendering::defs::ML_MAPPED;
            }
        });
    }
    state::with_state_mut(|s| {
        s.sidedef_idx = sidedef_idx;
        s.linedef_idx = linedef_idx;
    });

    let rw_angle1 = state::with_state(|s| s.rw_angle1) as u32;
    state::with_state_mut(|s| s.rw_normalangle = rw_normalangle);
    let mut offsetangle = rw_normalangle.wrapping_sub(rw_angle1);
    if offsetangle > ANG90 {
        offsetangle = ANG90;
    }
    let distangle = ANG90 - offsetangle;
    let hyp = r_point_to_dist(v1x, v1y);
    let sineval = finesine((distangle >> ANGLETOFINESHIFT) as usize);
    state::with_state_mut(|s| s.rw_distance = fixed_mul(hyp, sineval));

    ss.rw_x = start;
    ss.rw_stopx = stop + 1;

    let viewangle = state::with_state(|s| s.viewangle);
    let scale1 = r_scale_from_global_angle(
        viewangle + state::with_state(|s| s.xtoviewangle[start as usize]),
    );
    let scale2 = if stop > start {
        r_scale_from_global_angle(viewangle + state::with_state(|s| s.xtoviewangle[stop as usize]))
    } else {
        scale1
    };
    let scalestep = if stop > start {
        (scale2 - scale1) / (stop - start)
    } else {
        0
    };

    let viewz = state::with_state(|s| s.viewz);
    unsafe {
        (*ds_p).x1 = start;
        (*ds_p).x2 = stop;
        (*ds_p).curline = curline_ptr;
        (*ds_p).scale1 = scale1;
        (*ds_p).scale2 = scale2;
        (*ds_p).scalestep = scalestep;
    }

    ss.rw_scale = scale1;
    ss.rw_scalestep = scalestep;

    let textureheight = state::with_state(|s| s.textureheight);
    let texturetranslation = state::with_state(|s| s.texturetranslation);
    let skyflatnum = r_sky::with_r_sky_state(|rs| rs.skyflatnum);

    ss.worldtop = frontsector.ceilingheight - viewz;
    ss.worldbottom = frontsector.floorheight - viewz;

    ss.midtexture = 0;
    ss.toptexture = 0;
    ss.bottomtexture = 0;
    ss.maskedtexture = false;
    unsafe { (*ds_p).maskedtexturecol = ptr::null_mut() };

    if backsector.is_none() {
        let sidedef = match &sidedef {
            Some(sd) => sd,
            None => return,
        };
        let texnum = if texturetranslation.is_null() {
            sidedef.midtexture as i32
        } else {
            unsafe { *texturetranslation.add(sidedef.midtexture as usize) }
        };
        ss.midtexture = texnum;
        ss.markfloor = true;
        ss.markceiling = true;

        if linedef.as_ref().map(|ld| (ld.flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0).unwrap_or(false)
        {
            let th = if textureheight.is_null() {
                128 << crate::m_fixed::FRACBITS
            } else {
                unsafe { *textureheight.add(texnum as usize) }
            };
            ss.rw_midtexturemid = frontsector.floorheight + th - viewz + sidedef.rowoffset;
        } else {
            ss.rw_midtexturemid = ss.worldtop + sidedef.rowoffset;
        }

        crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
            unsafe {
                (*ds_p).silhouette = SIL_BOTH;
                (*ds_p).sprtopclip = rd.screenheightarray.as_mut_ptr();
                (*ds_p).sprbottomclip = rd.negonearray.as_mut_ptr();
                (*ds_p).bsilheight = i32::MAX;
                (*ds_p).tsilheight = i32::MIN;
            }
        });
    } else {
        let back = match backsector {
            Some(b) => b,
            None => return,
        };
        let sidedef = match &sidedef {
            Some(sd) => sd,
            None => return,
        };
        unsafe {
            (*ds_p).sprtopclip = ptr::null_mut();
            (*ds_p).sprbottomclip = ptr::null_mut();
            (*ds_p).silhouette = 0;
        }

        if frontsector.floorheight > back.floorheight {
            unsafe {
                (*ds_p).silhouette = SIL_BOTTOM;
                (*ds_p).bsilheight = frontsector.floorheight;
            }
        } else if back.floorheight > viewz {
            unsafe {
                (*ds_p).silhouette = SIL_BOTTOM;
                (*ds_p).bsilheight = i32::MAX;
            }
        }

        if frontsector.ceilingheight < back.ceilingheight {
            unsafe {
                (*ds_p).silhouette |= SIL_TOP;
                (*ds_p).tsilheight = frontsector.ceilingheight;
            }
        } else if back.ceilingheight < viewz {
            unsafe {
                (*ds_p).silhouette |= SIL_TOP;
                (*ds_p).tsilheight = i32::MIN;
            }
        }

        if back.ceilingheight <= frontsector.floorheight {
            crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                unsafe {
                    (*ds_p).sprbottomclip = rd.negonearray.as_mut_ptr();
                    (*ds_p).bsilheight = i32::MAX;
                    (*ds_p).silhouette |= SIL_BOTTOM;
                }
            });
        }

        if back.floorheight >= frontsector.ceilingheight {
            crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                unsafe {
                    (*ds_p).sprtopclip = rd.screenheightarray.as_mut_ptr();
                    (*ds_p).tsilheight = i32::MIN;
                    (*ds_p).silhouette |= SIL_TOP;
                }
            });
        }

        ss.worldhigh = back.ceilingheight - viewz;
        ss.worldlow = back.floorheight - viewz;

        if frontsector.ceilingpic as i32 == skyflatnum && back.ceilingpic as i32 == skyflatnum {
            ss.worldtop = ss.worldhigh;
        }

        ss.markfloor = ss.worldlow != ss.worldbottom
            || back.floorpic != frontsector.floorpic
            || back.lightlevel != frontsector.lightlevel;

        ss.markceiling = ss.worldhigh != ss.worldtop
            || back.ceilingpic != frontsector.ceilingpic
            || back.lightlevel != frontsector.lightlevel;

        if back.ceilingheight <= frontsector.floorheight
            || back.floorheight >= frontsector.ceilingheight
        {
            ss.markceiling = true;
            ss.markfloor = true;
        }

        if ss.worldhigh < ss.worldtop {
            let texnum = if texturetranslation.is_null() {
                sidedef.toptexture as i32
            } else {
                unsafe { *texturetranslation.add(sidedef.toptexture as usize) }
            };
            ss.toptexture = texnum;
            let th = if textureheight.is_null() {
                128 << crate::m_fixed::FRACBITS
            } else {
                unsafe { *textureheight.add(texnum as usize) }
            };
            if linedef.as_ref().map(|ld| (ld.flags & crate::rendering::defs::ML_DONTPEGTOP) != 0).unwrap_or(false)
            {
                ss.rw_topexturemid = ss.worldtop + sidedef.rowoffset;
            } else {
                ss.rw_topexturemid = back.ceilingheight + th - viewz + sidedef.rowoffset;
            }
        }

        if ss.worldlow > ss.worldbottom {
            let texnum = if texturetranslation.is_null() {
                sidedef.bottomtexture as i32
            } else {
                unsafe { *texturetranslation.add(sidedef.bottomtexture as usize) }
            };
            ss.bottomtexture = texnum;
            if linedef.as_ref().map(|ld| (ld.flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0).unwrap_or(false)
            {
                ss.rw_bottomtexturemid = ss.worldtop + sidedef.rowoffset;
            } else {
                ss.rw_bottomtexturemid = ss.worldlow + sidedef.rowoffset;
            }
        }

        if sidedef.midtexture != 0 {
            ss.maskedtexture = true;
            let openings_ptr = ps.openings.as_mut_ptr();
            let base = unsafe { openings_ptr.add(ps.lastopening) };
            unsafe { (*ds_p).maskedtexturecol = base.sub(start as usize) };
            ss.maskedtexturecol = unsafe { (*ds_p).maskedtexturecol };
            ps.lastopening += (stop - start + 1) as usize;
        }
    }

    ss.segtextured = ss.midtexture != 0
        || ss.toptexture != 0
        || ss.bottomtexture != 0
        || ss.maskedtexture;

    if ss.segtextured {
        let mut offang = rw_normalangle.wrapping_sub(rw_angle1);
        if offang > ANG180 {
            offang = offang.wrapping_neg();
        }
        if offang > ANG90 {
            offang = ANG90;
        }
        let sineval = finesine((offang >> ANGLETOFINESHIFT) as usize);
        let mut rw_off = fixed_mul(hyp, sineval);
        if rw_normalangle.wrapping_sub(rw_angle1) < ANG180 {
            rw_off = -rw_off;
        }
        let (textureoffset, _) = sidedef.as_ref().map(|s| (s.textureoffset, s.rowoffset)).unwrap_or((0, 0));
        ss.rw_offset = rw_off + textureoffset + curline_offset;
        ss.rw_centerangle = ANG90 + viewangle - rw_normalangle;

        let fixcol = with_r_main_state(|rm| rm.fixedcolormap);
        if fixcol.is_null() {
            let extralight = with_r_main_state(|rm| rm.extralight);
            let mut lightnum = (frontsector.lightlevel as i32 >> LIGHTSEGSHIFT) + extralight;
            if v1y == v2y {
                lightnum -= 1;
            } else if v1x == v2x {
                lightnum += 1;
            }
            ss.walllights = with_r_main_state(|rm| {
                let idx = if lightnum < 0 {
                    0
                } else if lightnum >= LIGHTLEVELS as i32 {
                    LIGHTLEVELS - 1
                } else {
                    lightnum as usize
                };
                rm.scalelight[idx].as_ptr() as *mut *mut u8
            });
        }
    }

    if frontsector.floorheight >= viewz {
        ss.markfloor = false;
    }
    if frontsector.ceilingheight <= viewz && frontsector.ceilingpic as i32 != skyflatnum {
        ss.markceiling = false;
    }

    let centeryfrac = with_r_main_state(|rm| rm.centeryfrac);
    let worldtop = ss.worldtop >> 4;
    let worldbottom = ss.worldbottom >> 4;

    ss.topstep = -fixed_mul(ss.rw_scalestep, worldtop);
    ss.topfrac = (centeryfrac >> 4) - fixed_mul(worldtop, ss.rw_scale);
    ss.bottomstep = -fixed_mul(ss.rw_scalestep, worldbottom);
    ss.bottomfrac = (centeryfrac >> 4) - fixed_mul(worldbottom, ss.rw_scale);

    if backsector.is_some() {
        let worldhigh = ss.worldhigh >> 4;
        let worldlow = ss.worldlow >> 4;

        if worldhigh < worldtop {
            ss.pixhigh = (centeryfrac >> 4) - fixed_mul(worldhigh, ss.rw_scale);
            ss.pixhighstep = -fixed_mul(ss.rw_scalestep, worldhigh);
        }
        if worldlow > worldbottom {
            ss.pixlow = (centeryfrac >> 4) - fixed_mul(worldlow, ss.rw_scale);
            ss.pixlowstep = -fixed_mul(ss.rw_scalestep, worldlow);
        }
    }

    if ss.markceiling {
        state::with_state_mut(|s| {
            s.ceilingplane = r_check_plane(s.ceilingplane, start, stop, ps);
        });
    }
    if ss.markfloor {
        state::with_state_mut(|s| {
            s.floorplane = r_check_plane(s.floorplane, start, stop, ps);
        });
    }

    r_render_seg_loop(ps, ss);

    let rw_stopx = ss.rw_stopx;
    if ((unsafe { (*ds_p).silhouette } & SIL_TOP) != 0 || ss.maskedtexture)
        && unsafe { (*ds_p).sprtopclip.is_null() }
    {
        let len = (rw_stopx - start) as usize;
        unsafe {
            let src = ps.ceilingclip.as_ptr().add(start as usize);
            let dst = ps.openings.as_mut_ptr().add(ps.lastopening);
            ptr::copy_nonoverlapping(src, dst, len);
            (*ds_p).sprtopclip = ps.openings.as_mut_ptr().add(ps.lastopening);
        }
        ps.lastopening += len;
    }

    if ((unsafe { (*ds_p).silhouette } & SIL_BOTTOM) != 0 || ss.maskedtexture)
        && unsafe { (*ds_p).sprbottomclip.is_null() }
    {
        let len = (rw_stopx - start) as usize;
        unsafe {
            let src = ps.floorclip.as_ptr().add(start as usize);
            let dst = ps.openings.as_mut_ptr().add(ps.lastopening);
            ptr::copy_nonoverlapping(src, dst, len);
            (*ds_p).sprbottomclip = ps.openings.as_mut_ptr().add(ps.lastopening);
        }
        ps.lastopening += len;
    }

    if ss.maskedtexture && (unsafe { (*ds_p).silhouette } & SIL_TOP) == 0 {
        unsafe {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = i32::MIN;
        }
    }
    if ss.maskedtexture && (unsafe { (*ds_p).silhouette } & SIL_BOTTOM) == 0 {
        unsafe {
            (*ds_p).silhouette |= SIL_BOTTOM;
            (*ds_p).bsilheight = i32::MAX;
        }
    }

    state::with_state_mut(|s| s.ds_p = unsafe { ds_p.add(1) });
}

/// Render masked mid textures for a drawseg range.
pub fn r_render_masked_seg_range(ds: *mut crate::rendering::defs::DrawSeg, x1: i32, x2: i32) {
    use crate::rendering::r_draw::{colfunc, with_r_draw_state, with_r_draw_state_mut};
    use crate::rendering::r_main::LIGHTSCALESHIFT;

    unsafe {
        if ds.is_null() || (*ds).maskedtexturecol.is_null() {
            return;
        }

        let curline_ptr = (*ds).curline;
        if curline_ptr.is_null() {
            return;
        }
        let seg = unsafe { &*curline_ptr };

        let (sidedef, linedef, frontsector, backsector, v1, v2) = state::with_state(|s| {
            let sd = s.sides.get(seg.sidedef_idx).cloned();
            let ld = s.lines.get(seg.linedef_idx).cloned();
            let fs = s.frontsector_idx.and_then(|i| s.sectors.get(i).cloned());
            let bs = s.backsector_idx.and_then(|i| s.sectors.get(i).cloned());
            let v1 = s.vertexes.get(seg.v1_idx).copied();
            let v2 = s.vertexes.get(seg.v2_idx).copied();
            (sd, ld, fs, bs, v1, v2)
        });

        let sidedef = match sidedef {
            Some(sd) => sd,
            None => return,
        };
        let frontsector = match frontsector {
            Some(fs) => fs,
            None => return,
        };

        let texnum = if state::with_state(|s| s.texturetranslation.is_null()) {
            sidedef.midtexture as i32
        } else {
            unsafe { *state::with_state(|s| s.texturetranslation).add(sidedef.midtexture as usize) }
        };

        let viewz = state::with_state(|s| s.viewz);
        let textureheight = state::with_state(|s| s.textureheight);
        let th = if textureheight.is_null() {
            128 << crate::m_fixed::FRACBITS
        } else {
            unsafe { *textureheight.add(texnum as usize) }
        };

        let back = backsector.as_ref();

        let dc_texturemid = if linedef.as_ref().map(|ld| (ld.flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0).unwrap_or(false)
        {
            let fh = match back {
                None => frontsector.floorheight,
                Some(b) => frontsector.floorheight.max(b.floorheight),
            };
            fh + th - viewz + sidedef.rowoffset
        } else {
            let ch = match back {
                None => frontsector.ceilingheight,
                Some(b) => frontsector.ceilingheight.min(b.ceilingheight),
            };
            ch - viewz + sidedef.rowoffset
        };

        let (fixedcolormap, extralight, centeryfrac) = with_r_main_state(|rm| {
            (rm.fixedcolormap, rm.extralight, rm.centeryfrac)
        });

        with_r_draw_state_mut(|rd| rd.dc_texturemid = dc_texturemid);

        if !fixedcolormap.is_null() {
            with_r_draw_state_mut(|rd| rd.dc_colormap = fixedcolormap);
        }

        let walllights = if fixedcolormap.is_null() {
            let lightnum = (frontsector.lightlevel as i32 >> LIGHTSEGSHIFT) + extralight;
            let (v1x, v1y, v2x, v2y) = v1.and_then(|v1| v2.map(|v2| (v1.x, v1.y, v2.x, v2.y))).unwrap_or((0, 0, 0, 0));
            let mut ln = lightnum;
            if v1y == v2y {
                ln -= 1;
            } else if v1x == v2x {
                ln += 1;
            }
            let ln = ln.max(0).min(crate::rendering::r_main::LIGHTLEVELS as i32 - 1);
            with_r_main_state(|rm| rm.scalelight[ln as usize].as_ptr())
        } else {
            ptr::null()
        };

        let mut spryscale = (*ds).scale1 + fixed_mul((x1 - (*ds).x1) as i32, (*ds).scalestep);
        let mfloorclip = (*ds).sprbottomclip;
        let mceilingclip = (*ds).sprtopclip;

        for dc_x in x1..=x2 {
            let col_idx = dc_x - (*ds).x1;
            let texcol = *(*ds).maskedtexturecol.add(col_idx as usize);
            if texcol == SHRT_MAX {
                spryscale += (*ds).scalestep;
                continue;
            }

            if fixedcolormap.is_null() && !walllights.is_null() {
                let index = (spryscale >> LIGHTSCALESHIFT) as usize;
                let index = index.min(crate::rendering::r_main::MAXLIGHTSCALE - 1);
                with_r_draw_state_mut(|rd| {
                    rd.dc_colormap = ptr::read(walllights.add(index));
                });
            }

            let sprtopscreen = centeryfrac - fixed_mul(dc_texturemid, spryscale);
            with_r_draw_state_mut(|rd| {
                rd.dc_iscale = if spryscale != 0 {
                    0xffff_ffffu32 / (spryscale as u32)
                } else {
                    0
                };
                rd.dc_x = dc_x;
                rd.dc_source = r_get_column(texnum, texcol as i32);
            });

            let mut dc_yl = (sprtopscreen >> FRACBITS) as i32;
            let mut dc_yh = ((sprtopscreen + fixed_mul(th, spryscale)) >> FRACBITS) as i32;

            if !mfloorclip.is_null() && dc_x >= (*ds).x1 {
                let clip = *mfloorclip.add(col_idx as usize);
                if dc_yh >= clip as i32 {
                    dc_yh = clip as i32 - 1;
                }
            }
            if !mceilingclip.is_null() && dc_x >= (*ds).x1 {
                let clip = *mceilingclip.add(col_idx as usize);
                if dc_yl <= clip as i32 {
                    dc_yl = clip as i32 + 1;
                }
            }

            if dc_yl <= dc_yh {
                with_r_draw_state_mut(|rd| {
                    rd.dc_yl = dc_yl;
                    rd.dc_yh = dc_yh;
                });
                colfunc();
            }

            *(*ds).maskedtexturecol.add(col_idx as usize) = SHRT_MAX;
            spryscale += (*ds).scalestep;
        }
    }
}
