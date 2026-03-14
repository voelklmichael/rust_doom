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
use crate::rendering::r_draw::{colfunc, SCREENHEIGHTARRAY, NEGONEARRAY};
use crate::rendering::r_main::{
    r_point_to_dist, r_scale_from_global_angle, CENTERYFRAC, EXTRALIGHT, FIXEDCOLORMAP,
    LIGHTLEVELS, LIGHTSCALESHIFT, MAXLIGHTSCALE, SCALELIGHT,
};
use crate::rendering::r_plane::{r_check_plane, CEILINGCLIP, FLOORCLIP, LASTOPENING};
use crate::rendering::r_sky;
use crate::rendering::state;
use std::ptr;

const LIGHTSEGSHIFT: i32 = 4;
const HEIGHTBITS: i32 = 12;
const HEIGHTUNIT: i32 = 1 << HEIGHTBITS;

/// SHRT_MAX - used as sentinel for masked texture column.
const SHRT_MAX: i16 = 0x7fff;

// =============================================================================
// State (from r_segs.c)
// =============================================================================

static mut SEGTEXTURED: bool = false;
static mut MARKFLOOR: bool = false;
static mut MARKCEILING: bool = false;
static mut MASKEDTEXTURE: bool = false;
static mut TOPTEXTURE: i32 = 0;
static mut BOTTOMTEXTURE: i32 = 0;
static mut MIDTEXTURE: i32 = 0;

static mut RW_X: i32 = 0;
static mut RW_STOPX: i32 = 0;
static mut RW_CENTERANGLE: u32 = 0;
static mut RW_OFFSET: Fixed = 0;
static mut RW_SCALE: Fixed = 0;
static mut RW_SCALESTEP: Fixed = 0;
static mut RW_MIDTEXTUREMID: Fixed = 0;
static mut RW_TOPEXTUREMID: Fixed = 0;
static mut RW_BOTTOMTEXTUREMID: Fixed = 0;

static mut WORLDTOP: Fixed = 0;
static mut WORLDBOTTOM: Fixed = 0;
static mut WORLDHIGH: Fixed = 0;
static mut WORLDLOW: Fixed = 0;

static mut PIXHIGH: Fixed = 0;
static mut PIXLOW: Fixed = 0;
static mut PIXHIGHSTEP: Fixed = 0;
static mut PIXLOWSTEP: Fixed = 0;

static mut TOPFRAC: Fixed = 0;
static mut TOPSTEP: Fixed = 0;
static mut BOTTOMFRAC: Fixed = 0;
static mut BOTTOMSTEP: Fixed = 0;

static mut WALLLIGHTS: *mut *mut u8 = ptr::null_mut();
static mut MASKEDTEXTURECOL: *mut i16 = ptr::null_mut();

// =============================================================================
// R_RenderSegLoop - core loop, draws walls and marks floor/ceiling
// =============================================================================

fn r_render_seg_loop() {
    unsafe {
        let viewheight = state::with_state(|s| s.viewheight);
        let floorplane = state::with_state(|s| s.floorplane);
        let ceilingplane = state::with_state(|s| s.ceilingplane);

        let mut rw_x = RW_X;
        let stopx = RW_STOPX;
        let mut rw_scale = RW_SCALE;
        let mut topfrac = TOPFRAC;
        let mut bottomfrac = BOTTOMFRAC;
        let mut pixhigh = PIXHIGH;
        let mut pixlow = PIXLOW;

        while rw_x < stopx {
            let yl = (topfrac >> HEIGHTBITS) as i32;
            let mut yl = yl;
            if yl < CEILINGCLIP[rw_x as usize] as i32 + 1 {
                yl = CEILINGCLIP[rw_x as usize] as i32 + 1;
            }

            if MARKCEILING {
                let top = CEILINGCLIP[rw_x as usize] as i32 + 1;
                let mut bottom = yl - 1;
                if bottom >= FLOORCLIP[rw_x as usize] as i32 {
                    bottom = FLOORCLIP[rw_x as usize] as i32 - 1;
                }
                if top <= bottom && !ceilingplane.is_null() {
                    (*ceilingplane).top[rw_x as usize] = top as u8;
                    (*ceilingplane).bottom[rw_x as usize] = bottom as u8;
                }
            }

            let mut yh = (bottomfrac >> HEIGHTBITS) as i32;
            if yh >= FLOORCLIP[rw_x as usize] as i32 {
                yh = FLOORCLIP[rw_x as usize] as i32 - 1;
            }

            if MARKFLOOR {
                let top = yh + 1;
                let mut bottom = FLOORCLIP[rw_x as usize] as i32 - 1;
                if top <= CEILINGCLIP[rw_x as usize] as i32 {
                    bottom = CEILINGCLIP[rw_x as usize] as i32 + 1;
                }
                if top <= bottom && !floorplane.is_null() {
                    (*floorplane).top[rw_x as usize] = top as u8;
                    (*floorplane).bottom[rw_x as usize] = bottom as u8;
                }
            }

            let texturecolumn: i32 = if SEGTEXTURED {
                let angle = (RW_CENTERANGLE.wrapping_add(state::with_state(|s| s.xtoviewangle[rw_x as usize])))
                    >> ANGLETOFINESHIFT;
                let tanval = finetangent(angle as usize);
                (RW_OFFSET - fixed_mul(tanval, state::with_state(|s| s.rw_distance))) >> FRACBITS
            } else {
                0
            };

            let dc_colormap = if SEGTEXTURED && FIXEDCOLORMAP.is_null() {
                let index = (rw_scale >> LIGHTSCALESHIFT) as usize;
                let index = index.min(MAXLIGHTSCALE - 1);
                WALLLIGHTS.add(index).read()
            } else if !FIXEDCOLORMAP.is_null() {
                FIXEDCOLORMAP
            } else {
                crate::rendering::r_draw::DC_COLORMAP
            };

            crate::rendering::r_draw::DC_COLORMAP = dc_colormap;
            crate::rendering::r_draw::DC_X = rw_x;
            crate::rendering::r_draw::DC_ISCALE = if rw_scale != 0 {
                0xffff_ffffu32 / (rw_scale as u32)
            } else {
                0
            };

            if MIDTEXTURE != 0 {
                crate::rendering::r_draw::DC_YL = yl;
                crate::rendering::r_draw::DC_YH = yh;
                crate::rendering::r_draw::DC_TEXTUREMID = RW_MIDTEXTUREMID;
                crate::rendering::r_draw::DC_SOURCE =
                    r_get_column(MIDTEXTURE, texturecolumn);
                colfunc();
                CEILINGCLIP[rw_x as usize] = viewheight as i16;
                FLOORCLIP[rw_x as usize] = -1;
            } else {
                if TOPTEXTURE != 0 {
                    let mut mid = (pixhigh >> HEIGHTBITS) as i32;
                    pixhigh += PIXHIGHSTEP;

                    if mid >= FLOORCLIP[rw_x as usize] as i32 {
                        mid = FLOORCLIP[rw_x as usize] as i32 - 1;
                    }
                    if mid >= yl {
                        crate::rendering::r_draw::DC_YL = yl;
                        crate::rendering::r_draw::DC_YH = mid;
                        crate::rendering::r_draw::DC_TEXTUREMID = RW_TOPEXTUREMID;
                        crate::rendering::r_draw::DC_SOURCE =
                            r_get_column(TOPTEXTURE, texturecolumn);
                        colfunc();
                        CEILINGCLIP[rw_x as usize] = mid as i16;
                    } else {
                        CEILINGCLIP[rw_x as usize] = (yl - 1) as i16;
                    }
                } else if MARKCEILING {
                    CEILINGCLIP[rw_x as usize] = (yl - 1) as i16;
                }

                if BOTTOMTEXTURE != 0 {
                    let mut mid = ((pixlow + HEIGHTUNIT - 1) >> HEIGHTBITS) as i32;
                    pixlow += PIXLOWSTEP;

                    if mid <= CEILINGCLIP[rw_x as usize] as i32 {
                        mid = CEILINGCLIP[rw_x as usize] as i32 + 1;
                    }
                    if mid <= yh {
                        crate::rendering::r_draw::DC_YL = mid;
                        crate::rendering::r_draw::DC_YH = yh;
                        crate::rendering::r_draw::DC_TEXTUREMID = RW_BOTTOMTEXTUREMID;
                        crate::rendering::r_draw::DC_SOURCE =
                            r_get_column(BOTTOMTEXTURE, texturecolumn);
                        colfunc();
                        FLOORCLIP[rw_x as usize] = mid as i16;
                    } else {
                        FLOORCLIP[rw_x as usize] = (yh + 1) as i16;
                    }
                } else if MARKFLOOR {
                    FLOORCLIP[rw_x as usize] = (yh + 1) as i16;
                }

                if MASKEDTEXTURE {
                    if !MASKEDTEXTURECOL.is_null() {
                        *MASKEDTEXTURECOL.add(rw_x as usize) = texturecolumn as i16;
                    }
                }
            }

            rw_scale += RW_SCALESTEP;
            topfrac += TOPSTEP;
            bottomfrac += BOTTOMSTEP;
            rw_x += 1;
        }

        PIXHIGH = pixhigh;
        PIXLOW = pixlow;
    }
}

/// Store wall range and render. Populates drawsegs, draws walls, marks floor/ceiling.
pub fn r_store_wall_range(start: i32, stop: i32) {
    unsafe {
        let ds_p = state::with_state(|s| s.ds_p);
        if ds_p.is_null() {
            return;
        }
        if ds_p as *const _ >= state::with_state(|s| s.drawsegs.as_ptr().add(MAXDRAWSEGS)) {
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

        let sidedef = (*curline).sidedef;
        let linedef = (*curline).linedef;

        if !linedef.is_null() {
            (*linedef).flags |= crate::rendering::defs::ML_MAPPED;
        }
        state::with_state_mut(|s| {
            s.sidedef = sidedef;
            s.linedef = linedef;
        });

        let rw_angle1 = state::with_state(|s| s.rw_angle1) as u32;
        let rw_normalangle = (*curline).angle + ANG90;
        state::with_state_mut(|s| s.rw_normalangle = rw_normalangle);
        let mut offsetangle = rw_normalangle.wrapping_sub(rw_angle1);
        if offsetangle > ANG90 {
            offsetangle = ANG90;
        }
        let distangle = ANG90 - offsetangle;
        let hyp = r_point_to_dist((*(*curline).v1).x, (*(*curline).v1).y);
        let sineval = finesine((distangle >> ANGLETOFINESHIFT) as usize);
        state::with_state_mut(|s| s.rw_distance = fixed_mul(hyp, sineval));

        RW_X = start;
        RW_STOPX = stop + 1;

        let viewangle = state::with_state(|s| s.viewangle);
        let scale1 = r_scale_from_global_angle(viewangle + state::with_state(|s| s.xtoviewangle[start as usize]));
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

        (*ds_p).x1 = start;
        (*ds_p).x2 = stop;
        (*ds_p).curline = curline;
        (*ds_p).scale1 = scale1;
        (*ds_p).scale2 = scale2;
        (*ds_p).scalestep = scalestep;

        RW_SCALE = scale1;
        RW_SCALESTEP = scalestep;

        let textureheight = state::with_state(|s| s.textureheight);
        let texturetranslation = state::with_state(|s| s.texturetranslation);
        let skyflatnum = r_sky::SKYFLATNUM;

        WORLDTOP = (*frontsector).ceilingheight - viewz;
        WORLDBOTTOM = (*frontsector).floorheight - viewz;

        MIDTEXTURE = 0;
        TOPTEXTURE = 0;
        BOTTOMTEXTURE = 0;
        MASKEDTEXTURE = false;
        (*ds_p).maskedtexturecol = ptr::null_mut();

        if backsector.is_null() {
            let texnum = if texturetranslation.is_null() {
                (*sidedef).midtexture as i32
            } else {
                *texturetranslation.add((*sidedef).midtexture as usize)
            };
            MIDTEXTURE = texnum;
            MARKFLOOR = true;
            MARKCEILING = true;

            if !linedef.is_null() && ((*linedef).flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0
            {
                let th = if textureheight.is_null() {
                    128 << crate::m_fixed::FRACBITS
                } else {
                    *textureheight.add(texnum as usize)
                };
                RW_MIDTEXTUREMID =
                    (*frontsector).floorheight + th - viewz + (*sidedef).rowoffset;
            } else {
                RW_MIDTEXTUREMID = WORLDTOP + (*sidedef).rowoffset;
            }

            (*ds_p).silhouette = SIL_BOTH;
            (*ds_p).sprtopclip = SCREENHEIGHTARRAY.as_mut_ptr();
            (*ds_p).sprbottomclip = NEGONEARRAY.as_mut_ptr();
            (*ds_p).bsilheight = i32::MAX;
            (*ds_p).tsilheight = i32::MIN;
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
                (*ds_p).sprbottomclip = NEGONEARRAY.as_mut_ptr();
                (*ds_p).bsilheight = i32::MAX;
                (*ds_p).silhouette |= SIL_BOTTOM;
            }

            if (*backsector).floorheight >= (*frontsector).ceilingheight {
                (*ds_p).sprtopclip = SCREENHEIGHTARRAY.as_mut_ptr();
                (*ds_p).tsilheight = i32::MIN;
                (*ds_p).silhouette |= SIL_TOP;
            }

            WORLDHIGH = (*backsector).ceilingheight - viewz;
            WORLDLOW = (*backsector).floorheight - viewz;

            if (*frontsector).ceilingpic as i32 == skyflatnum
                && (*backsector).ceilingpic as i32 == skyflatnum
            {
                WORLDTOP = WORLDHIGH;
            }

            MARKFLOOR = WORLDLOW != WORLDBOTTOM
                || (*backsector).floorpic != (*frontsector).floorpic
                || (*backsector).lightlevel != (*frontsector).lightlevel;

            MARKCEILING = WORLDHIGH != WORLDTOP
                || (*backsector).ceilingpic != (*frontsector).ceilingpic
                || (*backsector).lightlevel != (*frontsector).lightlevel;

            if (*backsector).ceilingheight <= (*frontsector).floorheight
                || (*backsector).floorheight >= (*frontsector).ceilingheight
            {
                MARKCEILING = true;
                MARKFLOOR = true;
            }

            if WORLDHIGH < WORLDTOP {
                let texnum = if texturetranslation.is_null() {
                    (*sidedef).toptexture as i32
                } else {
                    *texturetranslation.add((*sidedef).toptexture as usize)
                };
                TOPTEXTURE = texnum;
                let th = if textureheight.is_null() {
                    128 << crate::m_fixed::FRACBITS
                } else {
                    *textureheight.add(texnum as usize)
                };
                if !linedef.is_null() && ((*linedef).flags & crate::rendering::defs::ML_DONTPEGTOP) != 0
                {
                    RW_TOPEXTUREMID = WORLDTOP + (*sidedef).rowoffset;
                } else {
                    RW_TOPEXTUREMID =
                        (*backsector).ceilingheight + th - viewz + (*sidedef).rowoffset;
                }
            }

            if WORLDLOW > WORLDBOTTOM {
                let texnum = if texturetranslation.is_null() {
                    (*sidedef).bottomtexture as i32
                } else {
                    *texturetranslation.add((*sidedef).bottomtexture as usize)
                };
                BOTTOMTEXTURE = texnum;
                if !linedef.is_null()
                    && ((*linedef).flags & crate::rendering::defs::ML_DONTPEGBOTTOM) != 0
                {
                    RW_BOTTOMTEXTUREMID = WORLDTOP + (*sidedef).rowoffset;
                } else {
                    RW_BOTTOMTEXTUREMID = WORLDLOW + (*sidedef).rowoffset;
                }
            }

            if (*sidedef).midtexture != 0 {
                MASKEDTEXTURE = true;
                let lastop = LASTOPENING;
                (*ds_p).maskedtexturecol = lastop.sub(start as usize);
                MASKEDTEXTURECOL = (*ds_p).maskedtexturecol;
                LASTOPENING = lastop.add((stop - start + 1) as usize);
            }
        }

        SEGTEXTURED = MIDTEXTURE != 0 || TOPTEXTURE != 0 || BOTTOMTEXTURE != 0 || MASKEDTEXTURE;

        if SEGTEXTURED {
            let mut offang = rw_normalangle.wrapping_sub(rw_angle1);
            if offang > ANG180 {
                offang = offang.wrapping_neg();
            }
            if offang > ANG90 {
                offang = ANG90;
            }
            let sineval = finesine((offang >> ANGLETOFINESHIFT) as usize);
            let mut rw_off = fixed_mul(hyp, sineval);
            if (rw_normalangle.wrapping_sub(rw_angle1)) < ANG180 {
                rw_off = -rw_off;
            }
            RW_OFFSET = rw_off + (*sidedef).textureoffset + (*curline).offset;
            RW_CENTERANGLE = ANG90 + viewangle - rw_normalangle;

            if FIXEDCOLORMAP.is_null() {
                let mut lightnum =
                    ((*frontsector).lightlevel as i32 >> LIGHTSEGSHIFT) + EXTRALIGHT;
                let v1 = *(*curline).v1;
                let v2 = *(*curline).v2;
                if v1.y == v2.y {
                    lightnum -= 1;
                } else if v1.x == v2.x {
                    lightnum += 1;
                }
                if lightnum < 0 {
                    WALLLIGHTS = SCALELIGHT[0].as_mut_ptr();
                } else if lightnum >= LIGHTLEVELS as i32 {
                    WALLLIGHTS = SCALELIGHT[LIGHTLEVELS - 1].as_mut_ptr();
                } else {
                    WALLLIGHTS = SCALELIGHT[lightnum as usize].as_mut_ptr();
                }
            }
        }

        if (*frontsector).floorheight >= viewz {
            MARKFLOOR = false;
        }
        if (*frontsector).ceilingheight <= viewz
            && (*frontsector).ceilingpic as i32 != skyflatnum
        {
            MARKCEILING = false;
        }

        WORLDTOP >>= 4;
        WORLDBOTTOM >>= 4;

        TOPSTEP = -fixed_mul(RW_SCALESTEP, WORLDTOP);
        TOPFRAC = (CENTERYFRAC >> 4) - fixed_mul(WORLDTOP, RW_SCALE);
        BOTTOMSTEP = -fixed_mul(RW_SCALESTEP, WORLDBOTTOM);
        BOTTOMFRAC = (CENTERYFRAC >> 4) - fixed_mul(WORLDBOTTOM, RW_SCALE);

        if !backsector.is_null() {
            WORLDHIGH >>= 4;
            WORLDLOW >>= 4;

            if WORLDHIGH < WORLDTOP {
                PIXHIGH = (CENTERYFRAC >> 4) - fixed_mul(WORLDHIGH, RW_SCALE);
                PIXHIGHSTEP = -fixed_mul(RW_SCALESTEP, WORLDHIGH);
            }
            if WORLDLOW > WORLDBOTTOM {
                PIXLOW = (CENTERYFRAC >> 4) - fixed_mul(WORLDLOW, RW_SCALE);
                PIXLOWSTEP = -fixed_mul(RW_SCALESTEP, WORLDLOW);
            }
        }

        if MARKCEILING {
            state::with_state_mut(|s| s.ceilingplane = r_check_plane(s.ceilingplane, start, stop));
        }
        if MARKFLOOR {
            state::with_state_mut(|s| s.floorplane = r_check_plane(s.floorplane, start, stop));
        }

        r_render_seg_loop();

        if (((*ds_p).silhouette & SIL_TOP) != 0 || MASKEDTEXTURE) && (*ds_p).sprtopclip.is_null()
        {
            let len = (RW_STOPX - start) as usize;
            let src = CEILINGCLIP.as_ptr().add(start as usize);
            let dst = LASTOPENING;
            ptr::copy_nonoverlapping(src, dst, len);
            (*ds_p).sprtopclip = LASTOPENING;
            LASTOPENING = LASTOPENING.add(len);
        }

        if (((*ds_p).silhouette & SIL_BOTTOM) != 0 || MASKEDTEXTURE)
            && (*ds_p).sprbottomclip.is_null()
        {
            let len = (RW_STOPX - start) as usize;
            let src = FLOORCLIP.as_ptr().add(start as usize);
            let dst = LASTOPENING;
            ptr::copy_nonoverlapping(src, dst, len);
            (*ds_p).sprbottomclip = LASTOPENING;
            LASTOPENING = LASTOPENING.add(len);
        }

        if MASKEDTEXTURE && ((*ds_p).silhouette & SIL_TOP) == 0 {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = i32::MIN;
        }
        if MASKEDTEXTURE && ((*ds_p).silhouette & SIL_BOTTOM) == 0 {
            (*ds_p).silhouette |= SIL_BOTTOM;
            (*ds_p).bsilheight = i32::MAX;
        }

        state::with_state_mut(|s| s.ds_p = ds_p.add(1));
    }
}

/// Render masked mid textures for a drawseg range.
pub fn r_render_masked_seg_range(ds: *mut crate::rendering::defs::DrawSeg, x1: i32, x2: i32) {
    use crate::rendering::r_draw::{colfunc, DC_COLORMAP, DC_ISCALE, DC_SOURCE, DC_TEXTUREMID, DC_X, DC_YH, DC_YL};
    use crate::rendering::r_main::{FIXEDCOLORMAP, LIGHTSCALESHIFT, SCALELIGHT};

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

        DC_TEXTUREMID = dc_texturemid;

        if !FIXEDCOLORMAP.is_null() {
            DC_COLORMAP = FIXEDCOLORMAP;
        }

        let walllights = if FIXEDCOLORMAP.is_null() {
            let lightnum = ((*frontsector).lightlevel as i32 >> LIGHTSEGSHIFT)
                + crate::rendering::r_main::EXTRALIGHT;
            let v1 = *(*curline).v1;
            let v2 = *(*curline).v2;
            let mut ln = lightnum;
            if v1.y == v2.y {
                ln -= 1;
            } else if v1.x == v2.x {
                ln += 1;
            }
            let ln = ln.max(0).min(crate::rendering::r_main::LIGHTLEVELS as i32 - 1);
            SCALELIGHT[ln as usize].as_ptr()
        } else {
            std::ptr::null()
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

            if FIXEDCOLORMAP.is_null() && !walllights.is_null() {
                let index = (spryscale >> LIGHTSCALESHIFT) as usize;
                let index = index.min(crate::rendering::r_main::MAXLIGHTSCALE - 1);
                DC_COLORMAP = walllights.add(index).read();
            }

            let sprtopscreen =
                crate::rendering::r_main::CENTERYFRAC - fixed_mul(DC_TEXTUREMID, spryscale);
            DC_ISCALE = if spryscale != 0 {
                0xffff_ffffu32 / (spryscale as u32)
            } else {
                0
            };

            DC_X = dc_x;
            DC_SOURCE = r_get_column(texnum, texcol as i32);

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

            DC_YL = dc_yl;
            DC_YH = dc_yh;

            if DC_YL <= DC_YH {
                colfunc();
            }

            *(*ds).maskedtexturecol.add(col_idx as usize) = SHRT_MAX;
            spryscale += (*ds).scalestep;
        }
    }
}
