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
use crate::rendering::r_draw::{colfunc, spanfunc, DC_COLORMAP, DC_ISCALE, DC_SOURCE, DC_TEXTUREMID, DC_X, DC_YH, DC_YL, DISTSCALE, DS_COLORMAP, DS_SOURCE, DS_X1, DS_X2, DS_XFRAC, DS_XSTEP, DS_Y, DS_YFRAC, DS_YSTEP, PSPRITEISCALE, YSLOPE};
use crate::rendering::r_main::{
    CENTERXFRAC, CENTERY, DETAILSHIFT, EXTRALIGHT, FIXEDCOLORMAP, LIGHTLEVELS, LIGHTSEGSHIFT,
    LIGHTZSHIFT, MAXLIGHTZ, PROJECTION, ZLIGHT,
};
use crate::rendering::r_sky::{ANGLETOSKYSHIFT, SKYFLATNUM, SKYTEXTURE, SKYTEXTUREMID};
use crate::rendering::state;
use crate::wad::{w_cache_lump_num, w_release_lump_num};
use crate::z_zone::PU_STATIC;
use std::ptr;

const MAXVISPLANES: usize = 128;
const MAXOPENINGS: usize = 320 * 64; // SCREENWIDTH * 64

/// Sentinel for unused visplane top/bottom.
const PLANE_SENTINEL: u8 = 0xff;

// =============================================================================
// State (from r_plane.c)
// =============================================================================

static mut VISPLANES: [Visplane; MAXVISPLANES] = unsafe { std::mem::zeroed() };
static mut LASTVISPLANE: *mut Visplane = ptr::null_mut();
/// Floor clip per column - used by r_segs during wall rendering.
pub static mut FLOORCLIP: [i16; 320] = [0; 320];
/// Ceiling clip per column - used by r_segs during wall rendering.
pub static mut CEILINGCLIP: [i16; 320] = [0; 320];
static mut SPANSTART: [i32; 200] = [0; 200];
static mut SPANSTOP: [i32; 200] = [0; 200];
static mut OPENINGS: [i16; MAXOPENINGS] = [0; MAXOPENINGS];
/// Current position in openings array - used by r_segs for masked texture allocation.
pub static mut LASTOPENING: *mut i16 = ptr::null_mut();

static mut PLANEHEIGHT: Fixed = 0;
static mut PLANEZLIGHT: *mut *mut u8 = ptr::null_mut();
static mut BASEXSCALE: Fixed = 0;
static mut BASEYSCALE: Fixed = 0;

static mut CACHEDHEIGHT: [Fixed; 200] = [0; 200];
static mut CACHEDDISTANCE: [Fixed; 200] = [0; 200];
static mut CACHEDXSTEP: [Fixed; 200] = [0; 200];
static mut CACHEDYSTEP: [Fixed; 200] = [0; 200];

// =============================================================================
// Public API
// =============================================================================

/// Find or create visplane for given height, picnum, lightlevel.
pub fn r_find_plane(height: Fixed, picnum: i32, lightlevel: i32) -> *mut Visplane {
    let skyflatnum = unsafe { SKYFLATNUM };
    let (height, picnum, lightlevel) = if picnum == skyflatnum {
        (0, picnum, 0)
    } else {
        (height, picnum, lightlevel)
    };

    unsafe {
        let mut check = VISPLANES.as_mut_ptr();
        let last = LASTVISPLANE;
        while check < last {
            if (*check).height == height && (*check).picnum == picnum && (*check).lightlevel == lightlevel
            {
                return check;
            }
            check = check.add(1);
        }

        if check < last {
            return check;
        }

        if last.offset_from(VISPLANES.as_ptr()) as usize >= MAXVISPLANES {
            crate::i_system::i_error("R_FindPlane: no more visplanes");
        }

        LASTVISPLANE = check.add(1);

        (*check).height = height;
        (*check).picnum = picnum;
        (*check).lightlevel = lightlevel;
        (*check).minx = SCREENWIDTH;
        (*check).maxx = -1;

        for t in (*check).top.iter_mut() {
            *t = PLANE_SENTINEL;
        }

        check
    }
}

/// Check if plane can be extended, or create new one.
pub fn r_check_plane(pl: *mut Visplane, start: i32, stop: i32) -> *mut Visplane {
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

        let last = LASTVISPLANE;
        (*last).height = (*pl).height;
        (*last).picnum = (*pl).picnum;
        (*last).lightlevel = (*pl).lightlevel;

        if last.offset_from(VISPLANES.as_ptr()) as usize >= MAXVISPLANES {
            crate::i_system::i_error("R_CheckPlane: no more visplanes");
        }

        let new_pl = last;
        LASTVISPLANE = last.add(1);

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
    unsafe {
        let viewwidth = state::VIEWWIDTH as usize;
        let viewheight = state::VIEWHEIGHT as usize;

        for i in 0..viewwidth {
            FLOORCLIP[i] = SCREENHEIGHT as i16;
            CEILINGCLIP[i] = -1;
        }

        // Initialize screenheightarray for single-sided wall clipping.
        let viewheight_i16 = viewheight as i16;
        for i in 0..viewwidth {
            crate::rendering::r_draw::SCREENHEIGHTARRAY[i] = viewheight_i16;
        }

        for i in 0..viewheight {
            SPANSTART[i] = 0;
        }

        LASTVISPLANE = VISPLANES.as_mut_ptr();
        LASTOPENING = OPENINGS.as_mut_ptr();

        let viewangle = state::VIEWANGLE;
        let angle = (viewangle >> ANGLETOFINESHIFT) as usize;
        let finecos = finecosine(angle);
        let finesin = finesine(angle);

        BASEXSCALE = fixed_div(finecos, CENTERXFRAC);
        BASEYSCALE = fixed_div(-finesin, CENTERXFRAC);

        let projection = PROJECTION;
        for x in 0..viewwidth {
            let x_angle = state::XTOVIEWANGLE[x] as i32;
            let xtoview = (x_angle >> ANGLETOFINESHIFT).abs() as usize;
            let dist = finecosine(xtoview % (5 * crate::geometry::FINEANGLES / 4));
            let abs_dist = dist.abs().max(1);
            DISTSCALE[x] = fixed_div(projection, abs_dist);
        }

        let centery = CENTERY;
        for i in 0..viewheight {
            let dy = (centery - i as i32).abs().max(1);
            YSLOPE[i] = fixed_div(projection, (dy as Fixed) << FRACBITS);
        }
    }
}

// =============================================================================
// R_MapPlane - texture mapping for one span
// =============================================================================

fn r_map_plane(y: i32, x1: i32, x2: i32) {
    use crate::geometry::finecosine;
    use crate::geometry::finesine;

    unsafe {
        let viewwidth = state::VIEWWIDTH;
        let viewheight = state::VIEWHEIGHT;
        if x2 < x1 || x1 < 0 || x2 >= viewwidth || y > viewheight {
            return;
        }

        let y = y as usize;
        if PLANEHEIGHT != CACHEDHEIGHT[y] {
            CACHEDHEIGHT[y] = PLANEHEIGHT;
            let distance = fixed_mul(PLANEHEIGHT, YSLOPE[y]);
            CACHEDDISTANCE[y] = distance;
            DS_XSTEP = fixed_mul(distance, BASEXSCALE);
            DS_YSTEP = fixed_mul(distance, BASEYSCALE);
            CACHEDXSTEP[y] = DS_XSTEP;
            CACHEDYSTEP[y] = DS_YSTEP;
        } else {
            DS_XSTEP = CACHEDXSTEP[y];
            DS_YSTEP = CACHEDYSTEP[y];
        }

        let distance = CACHEDDISTANCE[y];
        let length = fixed_mul(distance, DISTSCALE[x1 as usize]);
        let viewangle = state::VIEWANGLE;
        let viewx = state::VIEWX;
        let viewy = state::VIEWY;
        let angle_idx = ((viewangle.wrapping_add(state::XTOVIEWANGLE[x1 as usize]))
            >> ANGLETOFINESHIFT) as usize;
        DS_XFRAC = viewx + fixed_mul(finecosine(angle_idx), length);
        DS_YFRAC = -viewy - fixed_mul(finesine(angle_idx), length);

        if !FIXEDCOLORMAP.is_null() {
            DS_COLORMAP = FIXEDCOLORMAP;
        } else {
            let index = (distance >> LIGHTZSHIFT) as usize;
            let index = index.min(MAXLIGHTZ - 1);
            DS_COLORMAP = PLANEZLIGHT.add(index).read();
        }

        DS_Y = y as i32;
        DS_X1 = x1;
        DS_X2 = x2;

        spanfunc();
    }
}

// =============================================================================
// R_MakeSpans
// =============================================================================

fn r_make_spans(x: i32, t1: i32, b1: i32, t2: i32, b2: i32) {
    unsafe {
        let mut t1 = t1;
        let mut b1 = b1;
        let mut t2 = t2;
        let mut b2 = b2;

        while t1 < t2 && t1 <= b1 {
            r_map_plane(t1, SPANSTART[t1 as usize], x - 1);
            t1 += 1;
        }
        while b1 > b2 && b1 >= t1 {
            r_map_plane(b1, SPANSTART[b1 as usize], x - 1);
            b1 -= 1;
        }

        while t2 < t1 && t2 <= b2 {
            SPANSTART[t2 as usize] = x;
            t2 += 1;
        }
        while b2 > b1 && b2 >= t2 {
            SPANSTART[b2 as usize] = x;
            b2 -= 1;
        }
    }
}

// =============================================================================
// R_DrawPlanes - at end of each frame
// =============================================================================

pub fn r_draw_planes() {
    unsafe {
        let skyflatnum = SKYFLATNUM;
        let colormaps = state::COLORMAPS;
        let firstflat = state::FIRSTFLAT;
        let flattranslation = state::FLATTRANSLATION;
        let viewz = state::VIEWZ;

        let detailshift = DETAILSHIFT;
        let extralight = EXTRALIGHT;

        let mut pl = VISPLANES.as_mut_ptr();
        let last = LASTVISPLANE;

        while pl < last {
            if (*pl).minx > (*pl).maxx {
                pl = pl.add(1);
                continue;
            }

            if (*pl).picnum == skyflatnum {
                let pspriteiscale = if PSPRITEISCALE != 0 {
                    PSPRITEISCALE
                } else {
                    0x8000
                };
                DC_ISCALE = pspriteiscale >> detailshift;
                DC_COLORMAP = colormaps;
                DC_TEXTUREMID = SKYTEXTUREMID;

                let skytexture = SKYTEXTURE;
                let minx = (*pl).minx;
                let maxx = (*pl).maxx;

                for x in minx..=maxx {
                    DC_YL = (*pl).top[x as usize] as i32;
                    DC_YH = (*pl).bottom[x as usize] as i32;

                    if DC_YL <= DC_YH {
                        let angle = (state::VIEWANGLE
                            .wrapping_add(state::XTOVIEWANGLE[x as usize]))
                            >> ANGLETOSKYSHIFT;
                        DC_X = x;
                        DC_SOURCE = r_get_column(skytexture, angle as i32);
                        colfunc();
                    }
                }
                pl = pl.add(1);
                continue;
            }

            let lumpnum = if flattranslation.is_null() {
                firstflat + (*pl).picnum
            } else {
                firstflat + *flattranslation.add((*pl).picnum as usize)
            };

            let lump = w_cache_lump_num(lumpnum, PU_STATIC);
            DS_SOURCE = lump;
            PLANEHEIGHT = ((*pl).height - viewz).abs();

            let mut light = ((*pl).lightlevel >> LIGHTSEGSHIFT) + extralight;
            if light >= LIGHTLEVELS as i32 {
                light = LIGHTLEVELS as i32 - 1;
            }
            if light < 0 {
                light = 0;
            }

            PLANEZLIGHT = ZLIGHT[light as usize].as_mut_ptr();

            let maxx = (*pl).maxx as usize;
            let minx = (*pl).minx as usize;
            (*pl).top[maxx + 1] = PLANE_SENTINEL;
            if minx > 0 {
                (*pl).top[minx - 1] = PLANE_SENTINEL;
            }

            let stop = maxx + 1;
            for x in minx..=stop {
                r_make_spans(
                    x as i32,
                    (*pl).top[x.wrapping_sub(1)] as i32,
                    (*pl).bottom[x.wrapping_sub(1)] as i32,
                    (*pl).top[x] as i32,
                    (*pl).bottom[x] as i32,
                );
            }

            w_release_lump_num(lumpnum);

            pl = pl.add(1);
        }
    }
}
