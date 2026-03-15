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
use crate::rendering::defs::{Line, SideDef, SIL_BOTH, SIL_BOTTOM, SIL_TOP, MAXDRAWSEGS};
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

    let curline = state::with_state(|s| s.curline);
    let frontsector = state::with_state(|s| s.frontsector);
    let backsector = state::with_state(|s| s.backsector);

    if curline.is_null() || frontsector.is_null() {
        return;
    }

    let sidedef = unsafe { (*curline).sidedef };
    let linedef = unsafe { (*curline).linedef };

    if !linedef.is_null() {
        unsafe { (*linedef).flags |= crate::rendering::defs::ML_MAPPED };
    }
    state::with_state_mut(|s| {
        s.sidedef = sidedef;
        s.linedef = linedef;
    });

    let rw_angle1 = state::with_state(|s| s.rw_angle1) as u32;
    let rw_normalangle = unsafe { (*curline).angle + ANG90 };
    state::with_state_mut(|s| s.rw_normalangle = rw_normalangle);
    let mut offsetangle = rw_normalangle.wrapping_sub(rw_angle1);
    if offsetangle > ANG90 {
        offsetangle = ANG90;
    }
    let distangle = ANG90 - offsetangle;
    let hyp = r_point_to_dist(unsafe { (*(*curline).v1).x }, unsafe { (*(*curline).v1).y });
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

    unsafe {
        (*ds_p).x1 = start;
        (*ds_p).x2 = stop;
        (*ds_p).curline = curline;
        (*ds_p).scale1 = scale1;
        (*ds_p).scale2 = scale2;
        (*ds_p).scalestep = scalestep;
    }

    ss.rw_scale = scale1;
    ss.rw_scalestep = scalestep;

    let textureheight = state::with_state(|s| s.textureheight);
    let texturetranslation = state::with_state(|s| s.texturetranslation);
    let skyflatnum = r_sky::with_r_sky_state(|rs| rs.skyflatnum);

    ss.worldtop = unsafe { (*frontsector).ceilingheight - viewz };
    ss.worldbottom = unsafe { (*frontsector).floorheight - viewz };

    ss.midtexture = 0;
    ss.toptexture = 0;
    ss.bottomtexture = 0;
    ss.maskedtexture = false;
    unsafe { (*ds_p).maskedtexturecol = ptr::null_mut() };

    if backsector.is_null() {
        let texnum = if texturetranslation.is_null() {
            unsafe { (*sidedef).midtexture as i32 }
        } else {
            unsafe { *texturetranslation.add((*sidedef).midtexture as usize) }
        };
        ss.midtexture = texnum;
        ss.markfloor = true;
        ss.markceiling = true;

        if !linedef.is_null()
            && unsafe { ((*linedef).flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0 }
        {
            let th = if textureheight.is_null() {
                128 << crate::m_fixed::FRACBITS
            } else {
                unsafe { *textureheight.add(texnum as usize) }
            };
            ss.rw_midtexturemid = unsafe {
                (*frontsector).floorheight + th - viewz + (*sidedef).rowoffset
            };
        } else {
            ss.rw_midtexturemid = ss.worldtop + unsafe { (*sidedef).rowoffset };
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
        unsafe {
            (*ds_p).sprtopclip = ptr::null_mut();
            (*ds_p).sprbottomclip = ptr::null_mut();
            (*ds_p).silhouette = 0;
        }

        if unsafe { (*frontsector).floorheight > (*backsector).floorheight } {
            unsafe {
                (*ds_p).silhouette = SIL_BOTTOM;
                (*ds_p).bsilheight = (*frontsector).floorheight;
            }
        } else if unsafe { (*backsector).floorheight > viewz } {
            unsafe {
                (*ds_p).silhouette = SIL_BOTTOM;
                (*ds_p).bsilheight = i32::MAX;
            }
        }

        if unsafe { (*frontsector).ceilingheight < (*backsector).ceilingheight } {
            unsafe {
                (*ds_p).silhouette |= SIL_TOP;
                (*ds_p).tsilheight = (*frontsector).ceilingheight;
            }
        } else if unsafe { (*backsector).ceilingheight < viewz } {
            unsafe {
                (*ds_p).silhouette |= SIL_TOP;
                (*ds_p).tsilheight = i32::MIN;
            }
        }

        if unsafe { (*backsector).ceilingheight <= (*frontsector).floorheight } {
            crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                unsafe {
                    (*ds_p).sprbottomclip = rd.negonearray.as_mut_ptr();
                    (*ds_p).bsilheight = i32::MAX;
                    (*ds_p).silhouette |= SIL_BOTTOM;
                }
            });
        }

        if unsafe { (*backsector).floorheight >= (*frontsector).ceilingheight } {
            crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                unsafe {
                    (*ds_p).sprtopclip = rd.screenheightarray.as_mut_ptr();
                    (*ds_p).tsilheight = i32::MIN;
                    (*ds_p).silhouette |= SIL_TOP;
                }
            });
        }

        ss.worldhigh = unsafe { (*backsector).ceilingheight - viewz };
        ss.worldlow = unsafe { (*backsector).floorheight - viewz };

        if unsafe { (*frontsector).ceilingpic as i32 == skyflatnum }
            && unsafe { (*backsector).ceilingpic as i32 == skyflatnum }
        {
            ss.worldtop = ss.worldhigh;
        }

        ss.markfloor = ss.worldlow != ss.worldbottom
            || unsafe { (*backsector).floorpic != (*frontsector).floorpic }
            || unsafe { (*backsector).lightlevel != (*frontsector).lightlevel };

        ss.markceiling = ss.worldhigh != ss.worldtop
            || unsafe { (*backsector).ceilingpic != (*frontsector).ceilingpic }
            || unsafe { (*backsector).lightlevel != (*frontsector).lightlevel };

        if unsafe { (*backsector).ceilingheight <= (*frontsector).floorheight }
            || unsafe { (*backsector).floorheight >= (*frontsector).ceilingheight }
        {
            ss.markceiling = true;
            ss.markfloor = true;
        }

        if ss.worldhigh < ss.worldtop {
            let texnum = if texturetranslation.is_null() {
                unsafe { (*sidedef).toptexture as i32 }
            } else {
                unsafe { *texturetranslation.add((*sidedef).toptexture as usize) }
            };
            ss.toptexture = texnum;
            let th = if textureheight.is_null() {
                128 << crate::m_fixed::FRACBITS
            } else {
                unsafe { *textureheight.add(texnum as usize) }
            };
            if !linedef.is_null()
                && unsafe { ((*linedef).flags & crate::rendering::defs::ML_DONTPEGTOP) != 0 }
            {
                ss.rw_topexturemid = ss.worldtop + unsafe { (*sidedef).rowoffset };
            } else {
                ss.rw_topexturemid = unsafe {
                    (*backsector).ceilingheight + th - viewz + (*sidedef).rowoffset
                };
            }
        }

        if ss.worldlow > ss.worldbottom {
            let texnum = if texturetranslation.is_null() {
                unsafe { (*sidedef).bottomtexture as i32 }
            } else {
                unsafe { *texturetranslation.add((*sidedef).bottomtexture as usize) }
            };
            ss.bottomtexture = texnum;
            if !linedef.is_null()
                && unsafe { ((*linedef).flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0 }
            {
                ss.rw_bottomtexturemid = ss.worldtop + unsafe { (*sidedef).rowoffset };
            } else {
                ss.rw_bottomtexturemid = ss.worldlow + unsafe { (*sidedef).rowoffset };
            }
        }

        if unsafe { (*sidedef).midtexture != 0 } {
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
        ss.rw_offset = rw_off + unsafe { (*sidedef).textureoffset + (*curline).offset };
        ss.rw_centerangle = ANG90 + viewangle - rw_normalangle;

        let fixcol = with_r_main_state(|rm| rm.fixedcolormap);
        if fixcol.is_null() {
            let extralight = with_r_main_state(|rm| rm.extralight);
            let mut lightnum =
                unsafe { (*frontsector).lightlevel as i32 >> LIGHTSEGSHIFT } + extralight;
            let v1 = unsafe { *(*curline).v1 };
            let v2 = unsafe { *(*curline).v2 };
            if v1.y == v2.y {
                lightnum -= 1;
            } else if v1.x == v2.x {
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

    if unsafe { (*frontsector).floorheight >= viewz } {
        ss.markfloor = false;
    }
    if unsafe { (*frontsector).ceilingheight <= viewz }
        && unsafe { (*frontsector).ceilingpic as i32 != skyflatnum }
    {
        ss.markceiling = false;
    }

    let centeryfrac = with_r_main_state(|rm| rm.centeryfrac);
    let worldtop = ss.worldtop >> 4;
    let worldbottom = ss.worldbottom >> 4;

    ss.topstep = -fixed_mul(ss.rw_scalestep, worldtop);
    ss.topfrac = (centeryfrac >> 4) - fixed_mul(worldtop, ss.rw_scale);
    ss.bottomstep = -fixed_mul(ss.rw_scalestep, worldbottom);
    ss.bottomfrac = (centeryfrac >> 4) - fixed_mul(worldbottom, ss.rw_scale);

    if !backsector.is_null() {
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

        let curline = (*ds).curline;
        let frontsector = state::with_state(|s| s.frontsector);
        let backsector = state::with_state(|s| s.backsector);
        if curline.is_null() || frontsector.is_null() {
            return;
        }

        let sidedef = (*curline).sidedef;
        let linedef = (*curline).linedef;
        if sidedef.is_null() {
            return;
        }

        let texnum = if state::with_state(|s| s.texturetranslation.is_null()) {
            (*sidedef).midtexture as i32
        } else {
            *state::with_state(|s| s.texturetranslation).add((*sidedef).midtexture as usize)
        };

        let viewz = state::with_state(|s| s.viewz);
        let textureheight = state::with_state(|s| s.textureheight);
        let th = if textureheight.is_null() {
            128 << crate::m_fixed::FRACBITS
        } else {
            *textureheight.add(texnum as usize)
        };

        let dc_texturemid = if !linedef.is_null()
            && ((*linedef).flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0
        {
            let fh = if backsector.is_null() {
                (*frontsector).floorheight
            } else if (*frontsector).floorheight > (*backsector).floorheight {
                (*frontsector).floorheight
            } else {
                (*backsector).floorheight
            };
            fh + th - viewz + (*sidedef).rowoffset
        } else {
            let ch = if backsector.is_null() {
                (*frontsector).ceilingheight
            } else if (*frontsector).ceilingheight < (*backsector).ceilingheight {
                (*frontsector).ceilingheight
            } else {
                (*backsector).ceilingheight
            };
            ch - viewz + (*sidedef).rowoffset
        };

        let (fixedcolormap, extralight, centeryfrac) = with_r_main_state(|rm| {
            (rm.fixedcolormap, rm.extralight, rm.centeryfrac)
        });

        with_r_draw_state_mut(|rd| rd.dc_texturemid = dc_texturemid);

        if !fixedcolormap.is_null() {
            with_r_draw_state_mut(|rd| rd.dc_colormap = fixedcolormap);
        }

        let walllights = if fixedcolormap.is_null() {
            let lightnum = ((*frontsector).lightlevel as i32 >> LIGHTSEGSHIFT) + extralight;
            let v1 = *(*curline).v1;
            let v2 = *(*curline).v2;
            let mut ln = lightnum;
            if v1.y == v2.y {
                ln -= 1;
            } else if v1.x == v2.x {
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
