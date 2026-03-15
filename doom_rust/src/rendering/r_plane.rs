//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Visplane stuff (floor, ceilings).
//
// Original: r_plane.h + r_plane.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::geometry::{finecosine, finesine, ANGLETOFINESHIFT};
use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS};
use crate::rendering::defs::Visplane;
use crate::rendering::r_data::r_get_column;
use crate::rendering::r_draw::{colfunc, spanfunc, with_r_draw_state_mut};
use crate::rendering::r_main::{
    with_r_main_state, LIGHTLEVELS, LIGHTSEGSHIFT, LIGHTZSHIFT, MAXLIGHTZ,
};
use crate::rendering::r_sky::{with_r_sky_state, ANGLETOSKYSHIFT};
use crate::rendering::state;
use crate::wad::{w_cache_lump_num, w_release_lump_num};
use crate::z_zone::PU_STATIC;
use std::ptr;
use std::sync::{Mutex, OnceLock};

const MAXVISPLANES: usize = 128;
const MAXOPENINGS: usize = 320 * 64; // SCREENWIDTH * 64

/// Sentinel for unused visplane top/bottom.
const PLANE_SENTINEL: u8 = 0xff;

// =============================================================================
// State (from r_plane.c) - thread-safe via OnceLock + Mutex
// =============================================================================

pub struct PlaneState {
    pub visplanes: [Visplane; MAXVISPLANES],
    pub lastvisplane: usize,
    pub floorclip: [i16; 320],
    pub ceilingclip: [i16; 320],
    pub spanstart: [i32; 200],
    pub spanstop: [i32; 200],
    pub openings: [i16; MAXOPENINGS],
    pub lastopening: usize,
    planeheight: Fixed,
    planezlight: *mut *mut u8,
    basexscale: Fixed,
    baseyscale: Fixed,
    cachedheight: [Fixed; 200],
    cacheddistance: [Fixed; 200],
    cachedxstep: [Fixed; 200],
    cachedystep: [Fixed; 200],
}

impl Default for PlaneState {
    fn default() -> Self {
        Self {
            visplanes: unsafe { std::mem::zeroed() },
            lastvisplane: 0,
            floorclip: [0; 320],
            ceilingclip: [0; 320],
            spanstart: [0; 200],
            spanstop: [0; 200],
            openings: [0; MAXOPENINGS],
            lastopening: 0,
            planeheight: 0,
            planezlight: ptr::null_mut(),
            basexscale: 0,
            baseyscale: 0,
            cachedheight: [0; 200],
            cacheddistance: [0; 200],
            cachedxstep: [0; 200],
            cachedystep: [0; 200],
        }
    }
}

unsafe impl Send for PlaneState {}

static PLANE_STATE: OnceLock<Mutex<PlaneState>> = OnceLock::new();

pub fn with_plane_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut PlaneState) -> R,
{
    let mut guard = PLANE_STATE
        .get_or_init(|| Mutex::new(PlaneState::default()))
        .lock()
        .unwrap();
    f(&mut guard)
}

// =============================================================================
// Public API
// =============================================================================

/// Find or create visplane for given height, picnum, lightlevel.
pub fn r_find_plane(height: Fixed, picnum: i32, lightlevel: i32) -> *mut Visplane {
    let skyflatnum = with_r_sky_state(|rs| rs.skyflatnum);
    let (height, picnum, lightlevel) = if picnum == skyflatnum {
        (0, picnum, 0)
    } else {
        (height, picnum, lightlevel)
    };

    with_plane_state(|ps| {
        let visplanes = ps.visplanes.as_mut_ptr();
        let mut check_idx = 0;
        let last_idx = ps.lastvisplane;

        while check_idx < last_idx {
            let check = unsafe { visplanes.add(check_idx) };
            if unsafe {
                (*check).height == height
                    && (*check).picnum == picnum
                    && (*check).lightlevel == lightlevel
            } {
                return check;
            }
            check_idx += 1;
        }

        if check_idx < last_idx {
            return unsafe { visplanes.add(check_idx) };
        }

        if last_idx >= MAXVISPLANES {
            crate::i_system::i_error("R_FindPlane: no more visplanes");
        }

        let check = unsafe { visplanes.add(last_idx) };
        ps.lastvisplane = last_idx + 1;

        unsafe {
            (*check).height = height;
            (*check).picnum = picnum;
            (*check).lightlevel = lightlevel;
            (*check).minx = SCREENWIDTH;
            (*check).maxx = -1;

            for t in (*check).top.iter_mut() {
                *t = PLANE_SENTINEL;
            }
        }

        check
    })
}

/// Check if plane can be extended, or create new one.
/// Must be called within with_plane_state; pass the PlaneState reference.
pub fn r_check_plane(pl: *mut Visplane, start: i32, stop: i32, ps: &mut PlaneState) -> *mut Visplane {
    unsafe {
        if pl.is_null() {
            return pl;
        }

        let intrl: i32;
        let intrh: i32;
        let unionl: i32;
        let unionh: i32;

        if start < (*pl).minx {
            intrl = (*pl).minx;
            unionl = start;
        } else {
            unionl = (*pl).minx;
            intrl = start;
        }

        if stop > (*pl).maxx {
            intrh = (*pl).maxx;
            unionh = stop;
        } else {
            unionh = (*pl).maxx;
            intrh = stop;
        }

        let mut x = intrl;
        while x <= intrh {
            if (*pl).top[x as usize] != PLANE_SENTINEL {
                break;
            }
            x += 1;
        }

        if x > intrh {
            (*pl).minx = unionl;
            (*pl).maxx = unionh;
            return pl;
        }

        let last_idx = ps.lastvisplane;
        if last_idx >= MAXVISPLANES {
            crate::i_system::i_error("R_CheckPlane: no more visplanes");
        }

        let visplanes = ps.visplanes.as_mut_ptr();
        let last = visplanes.add(last_idx);
        (*last).height = (*pl).height;
        (*last).picnum = (*pl).picnum;
        (*last).lightlevel = (*pl).lightlevel;

        ps.lastvisplane = last_idx + 1;

        let new_pl = last;
        (*new_pl).minx = start;
        (*new_pl).maxx = stop;

        for t in (*new_pl).top.iter_mut() {
            *t = PLANE_SENTINEL;
        }

        new_pl
    }
}

/// Initialize planes. Only at game startup.
pub fn r_init_planes() {}

/// Clear planes at beginning of frame.
pub fn r_clear_planes() {
    with_plane_state(|ps| {
        let viewwidth = state::with_state(|s| s.viewwidth) as usize;
        let viewheight = state::with_state(|s| s.viewheight) as usize;

        for i in 0..viewwidth {
            ps.floorclip[i] = SCREENHEIGHT as i16;
            ps.ceilingclip[i] = -1;
        }

        let (centerxfrac, projection, centery) =
            crate::rendering::r_main::with_r_main_state(|rm| (rm.centerxfrac, rm.projection, rm.centery));

        crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
            // Initialize screenheightarray for single-sided wall clipping.
            let viewheight_i16 = viewheight as i16;
            for i in 0..viewwidth {
                rd.screenheightarray[i] = viewheight_i16;
            }
        });

        for i in 0..viewheight {
            ps.spanstart[i] = 0;
        }

        ps.lastvisplane = 0;
        ps.lastopening = 0;

        let viewangle = state::with_state(|s| s.viewangle);
        let angle = (viewangle >> ANGLETOFINESHIFT) as usize;
        let finecos = finecosine(angle);
        let finesin = finesine(angle);

        ps.basexscale = fixed_div(finecos, centerxfrac);
        ps.baseyscale = fixed_div(-finesin, centerxfrac);

        crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
            for x in 0..viewwidth {
                let x_angle = state::with_state(|s| s.xtoviewangle[x]) as i32;
                let xtoview = (x_angle >> ANGLETOFINESHIFT).abs() as usize;
                let dist = finecosine(xtoview % (5 * crate::geometry::FINEANGLES / 4));
                let abs_dist = dist.abs().max(1);
                rd.distscale[x] = fixed_div(projection, abs_dist);
            }

            for i in 0..viewheight {
                let dy = (centery - i as i32).abs().max(1);
                rd.yslope[i] = fixed_div(projection, (dy as Fixed) << FRACBITS);
            }
        });
    });
}

// =============================================================================
// R_MapPlane - texture mapping for one span
// =============================================================================

fn r_map_plane(ps: &mut PlaneState, y: i32, x1: i32, x2: i32) {
    use crate::geometry::finecosine;
    use crate::geometry::finesine;

    let viewwidth = state::with_state(|s| s.viewwidth);
    let viewheight = state::with_state(|s| s.viewheight);
    if x2 < x1 || x1 < 0 || x2 >= viewwidth || y > viewheight {
        return;
    }

    let y = y as usize;
    let (_d, ds_xstep, ds_ystep, _l, ds_xfrac, ds_yfrac, ds_colormap) =
        crate::rendering::r_draw::with_r_draw_state(|rd| {
            let (distance, ds_xstep, ds_ystep) = if ps.planeheight != ps.cachedheight[y] {
                let distance = fixed_mul(ps.planeheight, rd.yslope[y]);
                ps.cachedheight[y] = ps.planeheight;
                ps.cacheddistance[y] = distance;
                let ds_xstep = fixed_mul(distance, ps.basexscale);
                let ds_ystep = fixed_mul(distance, ps.baseyscale);
                ps.cachedxstep[y] = ds_xstep;
                ps.cachedystep[y] = ds_ystep;
                (distance, ds_xstep, ds_ystep)
            } else {
                (
                    ps.cacheddistance[y],
                    ps.cachedxstep[y],
                    ps.cachedystep[y],
                )
            };
            let length = fixed_mul(distance, rd.distscale[x1 as usize]);
            let viewangle = state::with_state(|s| s.viewangle);
            let viewx = state::with_state(|s| s.viewx);
            let viewy = state::with_state(|s| s.viewy);
            let angle_idx = ((viewangle
                .wrapping_add(state::with_state(|s| s.xtoviewangle[x1 as usize])))
                >> ANGLETOFINESHIFT) as usize;
            let ds_xfrac = viewx + fixed_mul(finecosine(angle_idx), length);
            let ds_yfrac = -viewy - fixed_mul(finesine(angle_idx), length);
            let fixcol = crate::rendering::r_main::with_r_main_state(|rm| rm.fixedcolormap);
            let ds_colormap = if !fixcol.is_null() {
                fixcol
            } else {
                let index = (distance >> LIGHTZSHIFT) as usize;
                let index = index.min(MAXLIGHTZ - 1);
                unsafe { ps.planezlight.add(index).read() }
            };
            (distance, ds_xstep, ds_ystep, length, ds_xfrac, ds_yfrac, ds_colormap)
        });

    crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
        rd.ds_xstep = ds_xstep;
        rd.ds_ystep = ds_ystep;
        rd.ds_xfrac = ds_xfrac;
        rd.ds_yfrac = ds_yfrac;
        rd.ds_colormap = ds_colormap;
        rd.ds_y = y as i32;
        rd.ds_x1 = x1;
        rd.ds_x2 = x2;
    });

    spanfunc();
}

// =============================================================================
// R_MakeSpans
// =============================================================================

fn r_make_spans(ps: &mut PlaneState, x: i32, t1: i32, b1: i32, t2: i32, b2: i32) {
    let mut t1 = t1;
    let mut b1 = b1;
    let mut t2 = t2;
    let mut b2 = b2;

    while t1 < t2 && t1 <= b1 {
        r_map_plane(ps, t1, ps.spanstart[t1 as usize], x - 1);
        t1 += 1;
    }
    while b1 > b2 && b1 >= t1 {
        r_map_plane(ps, b1, ps.spanstart[b1 as usize], x - 1);
        b1 -= 1;
    }

    while t2 < t1 && t2 <= b2 {
        ps.spanstart[t2 as usize] = x;
        t2 += 1;
    }
    while b2 > b1 && b2 >= t2 {
        ps.spanstart[b2 as usize] = x;
        b2 -= 1;
    }
}

// =============================================================================
// R_DrawPlanes - at end of each frame
// =============================================================================

pub fn r_draw_planes() {
    with_plane_state(|ps| {
        let (skyflatnum, detailshift, extralight) =
            crate::rendering::r_main::with_r_main_state(|rm| {
                (crate::rendering::r_sky::with_r_sky_state(|rs| rs.skyflatnum), rm.detailshift, rm.extralight)
            });
        let colormaps = state::with_state(|s| s.colormaps);
        let firstflat = state::with_state(|s| s.firstflat);
        let flattranslation = state::with_state(|s| s.flattranslation);
        let viewz = state::with_state(|s| s.viewz);

        let visplanes = ps.visplanes.as_mut_ptr();
        let mut pl_idx = 0;
        let last_idx = ps.lastvisplane;

        while pl_idx < last_idx {
            let pl = unsafe { visplanes.add(pl_idx) };

            if unsafe { (*pl).minx > (*pl).maxx } {
                pl_idx += 1;
                continue;
            }

            if unsafe { (*pl).picnum == skyflatnum } {
                let (pspriteiscale, skytexturemid, skytexture) = (
                    crate::rendering::r_draw::with_r_draw_state(|rd| {
                        if rd.pspriteiscale != 0 { rd.pspriteiscale } else { 0x8000 }
                    }),
                    crate::rendering::r_sky::with_r_sky_state(|rs| rs.skytexturemid),
                    crate::rendering::r_sky::with_r_sky_state(|rs| rs.skytexture),
                );
                crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                    rd.dc_iscale = pspriteiscale >> detailshift;
                    rd.dc_colormap = colormaps;
                    rd.dc_texturemid = skytexturemid;
                });
                let minx = unsafe { (*pl).minx };
                let maxx = unsafe { (*pl).maxx };

                for x in minx..=maxx {
                    let dc_yl = unsafe { (*pl).top[x as usize] as i32 };
                    let dc_yh = unsafe { (*pl).bottom[x as usize] as i32 };

                    if dc_yl <= dc_yh {
                        let angle = (state::with_state(|s| s.viewangle)
                            .wrapping_add(state::with_state(|s| s.xtoviewangle[x as usize])))
                            >> ANGLETOSKYSHIFT;
                        crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
                            rd.dc_yl = dc_yl;
                            rd.dc_yh = dc_yh;
                            rd.dc_x = x;
                            rd.dc_source = r_get_column(skytexture, angle as i32);
                        });
                        colfunc();
                    }
                }
                pl_idx += 1;
                continue;
            }

            let lumpnum = if flattranslation.is_null() {
                firstflat + unsafe { (*pl).picnum }
            } else {
                firstflat + unsafe { *flattranslation.add((*pl).picnum as usize) }
            };

            let lump = w_cache_lump_num(lumpnum, PU_STATIC);
            crate::rendering::r_draw::with_r_draw_state_mut(|rd| rd.ds_source = lump.as_ptr());
            ps.planeheight = unsafe { ((*pl).height - viewz).abs() };

            let mut light =
                unsafe { (*pl).lightlevel >> LIGHTSEGSHIFT } as i32 + extralight;
            if light >= LIGHTLEVELS as i32 {
                light = LIGHTLEVELS as i32 - 1;
            }
            if light < 0 {
                light = 0;
            }

            ps.planezlight =
                crate::rendering::r_main::with_r_main_state(|rm| rm.zlight[light as usize].as_ptr() as *mut *mut u8);

            let maxx = unsafe { (*pl).maxx as usize };
            let minx = unsafe { (*pl).minx as usize };
            unsafe {
                (*pl).top[maxx + 1] = PLANE_SENTINEL;
                if minx > 0 {
                    (*pl).top[minx - 1] = PLANE_SENTINEL;
                }
            }

            let stop = maxx + 1;
            for x in minx..=stop {
                r_make_spans(
                    ps,
                    x as i32,
                    unsafe { (*pl).top[x.wrapping_sub(1)] as i32 },
                    unsafe { (*pl).bottom[x.wrapping_sub(1)] as i32 },
                    unsafe { (*pl).top[x] as i32 },
                    unsafe { (*pl).bottom[x] as i32 },
                );
            }

            w_release_lump_num(lumpnum);

            pl_idx += 1;
        }
    });
}
