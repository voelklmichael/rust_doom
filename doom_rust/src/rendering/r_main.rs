//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Rendering main loop and setup functions, utility functions (BSP, geometry).
//
// Original: r_main.h + r_main.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::geometry::{
    finecosine, finesine, finetangent, slope_div, tantoangle, Angle, ANG90, ANG180, ANG270,
    ANGLETOFINESHIFT, DBITS, FINEANGLES,
};
use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS, FRACUNIT};
use crate::rendering::defs::{Node, Seg, Subsector};
use crate::rendering::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::rendering::state;
use std::ptr;

// =============================================================================
// Constants
// =============================================================================

const FIELDOFVIEW: i32 = 2048;

pub const LIGHTLEVELS: usize = 16;
pub const LIGHTSEGSHIFT: usize = 4;
pub const MAXLIGHTSCALE: usize = 48;
pub const LIGHTSCALESHIFT: i32 = 12;
pub const MAXLIGHTZ: usize = 128;
pub const LIGHTZSHIFT: i32 = 20;
pub const NUMCOLORMAPS: i32 = 32;

/// BSP child index: high bit indicates subsector.
pub const NF_SUBSECTOR: u16 = 0x8000;

/// Sign bit mask for fixed_t (i32).
const SIGN_MASK: i32 = (1u32 << 31) as i32;

// =============================================================================
// State (from r_main.c)
// =============================================================================

pub static mut VALIDCOUNT: i32 = 1;
pub static mut FIXEDCOLORMAP: *mut u8 = ptr::null_mut();

pub static mut CENTERX: i32 = 0;
pub static mut CENTERY: i32 = 0;
pub static mut CENTERXFRAC: Fixed = 0;
pub static mut CENTERYFRAC: Fixed = 0;
pub static mut PROJECTION: Fixed = 0;

pub static mut FRAMECOUNT: i32 = 0;
pub static mut LINECOUNT: i32 = 0;
pub static mut LOOPCOUNT: i32 = 0;

pub static mut VIEWCOS: Fixed = 0;
pub static mut VIEWSIN: Fixed = 0;
pub static mut VIEWWINDOWX: i32 = 0;
pub static mut VIEWWINDOWY: i32 = 0;

pub static mut EXTRALIGHT: i32 = 0;

pub static mut DETAILSHIFT: i32 = 0;

// Scalelight/zlight - allocated in R_InitLightTables / R_ExecuteSetViewSize
pub static mut SCALELIGHT: [[*mut u8; MAXLIGHTSCALE]; LIGHTLEVELS] =
    [[ptr::null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS];
pub static mut SCALELIGHTFIXED: [*mut u8; MAXLIGHTSCALE] = [ptr::null_mut(); MAXLIGHTSCALE];
pub static mut ZLIGHT: [[*mut u8; MAXLIGHTZ]; LIGHTLEVELS] =
    [[ptr::null_mut(); MAXLIGHTZ]; LIGHTLEVELS];

// View size change deferred
static mut SETSIZENEEDED: bool = false;
static mut SETBLOCKS: i32 = 10;
static mut SETDETAIL: i32 = 0;

// =============================================================================
// Public API - Utility functions
// =============================================================================

/// Expand bbox to enclose point (x, y).
pub fn r_add_point_to_box(x: i32, y: i32, box_: &mut [Fixed; 4]) {
    if x < box_[BOXLEFT as usize] {
        box_[BOXLEFT as usize] = x;
    }
    if x > box_[BOXRIGHT as usize] {
        box_[BOXRIGHT as usize] = x;
    }
    if y < box_[BOXBOTTOM as usize] {
        box_[BOXBOTTOM as usize] = y;
    }
    if y > box_[BOXTOP as usize] {
        box_[BOXTOP as usize] = y;
    }
}

/// BSP point-on-side test. Returns 0 (front) or 1 (back).
pub fn r_point_on_side(x: Fixed, y: Fixed, node: *const Node) -> i32 {
    unsafe {
        let dx = x - (*node).x;
        let dy = y - (*node).y;

        if (*node).dx == 0 {
            return if x <= (*node).x {
                if (*node).dy > 0 {
                    1
                } else {
                    0
                }
            } else {
                if (*node).dy < 0 {
                    1
                } else {
                    0
                }
            };
        }
        if (*node).dy == 0 {
            return if y <= (*node).y {
                if (*node).dx < 0 {
                    1
                } else {
                    0
                }
            } else {
                if (*node).dx > 0 {
                    1
                } else {
                    0
                }
            };
        }

        if ((*node).dy ^ (*node).dx ^ dx ^ dy) & SIGN_MASK != 0 {
            if ((*node).dy ^ dx) & SIGN_MASK != 0 {
                return 1;
            }
            return 0;
        }

        let left = fixed_mul((*node).dy >> FRACBITS, dx);
        let right = fixed_mul(dy, (*node).dx >> FRACBITS);

        if right < left {
            0
        } else {
            1
        }
    }
}

/// Point-on-seg-side test. Returns 0 (front) or 1 (back).
pub fn r_point_on_seg_side(x: Fixed, y: Fixed, line: *const Seg) -> i32 {
    unsafe {
        let lx = (*(*line).v1).x;
        let ly = (*(*line).v1).y;
        let ldx = (*(*line).v2).x - lx;
        let ldy = (*(*line).v2).y - ly;

        if ldx == 0 {
            return if x <= lx {
                if ldy > 0 {
                    1
                } else {
                    0
                }
            } else {
                if ldy < 0 {
                    1
                } else {
                    0
                }
            };
        }
        if ldy == 0 {
            return if y <= ly {
                if ldx < 0 {
                    1
                } else {
                    0
                }
            } else {
                if ldx > 0 {
                    1
                } else {
                    0
                }
            };
        }

        let dx = x - lx;
        let dy = y - ly;

        if (ldy ^ ldx ^ dx ^ dy) & SIGN_MASK != 0 {
            if (ldy ^ dx) & SIGN_MASK != 0 {
                return 1;
            }
            return 0;
        }

        let left = fixed_mul(ldy >> FRACBITS, dx);
        let right = fixed_mul(dy, ldx >> FRACBITS);

        if right < left {
            0
        } else {
            1
        }
    }
}

/// Global angle from cartesian coords (relative to view).
pub fn r_point_to_angle(x: Fixed, y: Fixed) -> Angle {
    let viewx = unsafe { state::VIEWX };
    let viewy = unsafe { state::VIEWY };

    let mut x = x - viewx;
    let mut y = y - viewy;

    if x == 0 && y == 0 {
        return 0;
    }

    if x >= 0 {
        if y >= 0 {
            if x > y {
                tantoangle(slope_div(y as u32, x as u32) as usize)
            } else {
                ANG90 - 1 - tantoangle(slope_div(x as u32, y as u32) as usize)
            }
        } else {
            y = -y;
            if x > y {
                0u32.wrapping_sub(tantoangle(slope_div(y as u32, x as u32) as usize))
            } else {
                ANG270 + tantoangle(slope_div(x as u32, y as u32) as usize)
            }
        }
    } else {
        x = -x;
        if y >= 0 {
            if x > y {
                ANG180 - 1 - tantoangle(slope_div(y as u32, x as u32) as usize)
            } else {
                ANG90 + tantoangle(slope_div(x as u32, y as u32) as usize)
            }
        } else {
            y = -y;
            if x > y {
                ANG180 + tantoangle(slope_div(y as u32, x as u32) as usize)
            } else {
                ANG270 - 1 - tantoangle(slope_div(x as u32, y as u32) as usize)
            }
        }
    }
}

/// Angle from (x1,y1) to (x2,y2). Temporarily sets viewx/viewy.
pub fn r_point_to_angle2(x1: Fixed, y1: Fixed, x2: Fixed, y2: Fixed) -> Angle {
    unsafe {
        state::VIEWX = x1;
        state::VIEWY = y1;
    }
    r_point_to_angle(x2, y2)
}

/// Distance from view to (x,y).
pub fn r_point_to_dist(x: Fixed, y: Fixed) -> Fixed {
    let viewx = unsafe { state::VIEWX };
    let viewy = unsafe { state::VIEWY };

    let mut dx = (x - viewx).abs();
    let mut dy = (y - viewy).abs();

    if dy > dx {
        std::mem::swap(&mut dx, &mut dy);
    }

    let frac = if dx != 0 {
        fixed_div(dy, dx)
    } else {
        0
    };

    let angle = (tantoangle((frac >> DBITS) as usize) + ANG90) >> ANGLETOFINESHIFT;
    fixed_div(dx, finesine(angle as usize))
}

/// Texture mapping scale for current line at given angle. rw_distance must be set.
pub fn r_scale_from_global_angle(visangle: Angle) -> Fixed {
    let viewangle = unsafe { state::VIEWANGLE };
    let rw_distance = unsafe { state::RW_DISTANCE };
    let rw_normalangle = unsafe { state::RW_NORMALANGLE };
    let projection = unsafe { PROJECTION };
    let detailshift = unsafe { DETAILSHIFT };

    let anglea = ANG90 + (visangle - viewangle);
    let angleb = ANG90 + (visangle - rw_normalangle);

    let sinea = finesine((anglea >> ANGLETOFINESHIFT) as usize);
    let sineb = finesine((angleb >> ANGLETOFINESHIFT) as usize);
    let num = fixed_mul(projection, sineb) << detailshift;
    let den = fixed_mul(rw_distance, sinea);

    if den > num >> 16 {
        let mut scale = fixed_div(num, den);
        if scale > 64 * FRACUNIT {
            scale = 64 * FRACUNIT;
        } else if scale < 256 {
            scale = 256;
        }
        scale
    } else {
        64 * FRACUNIT
    }
}

/// Find subsector containing point (x,y).
pub fn r_point_in_subsector(x: Fixed, y: Fixed) -> *mut Subsector {
    let numnodes = unsafe { state::NUMNODES };
    let subsectors = unsafe { state::SUBSECTORS };
    let nodes = unsafe { state::NODES };

    if numnodes == 0 {
        return subsectors;
    }

    let mut nodenum = (numnodes - 1) as usize;

    while nodenum & (NF_SUBSECTOR as usize) == 0 {
        let node = unsafe { nodes.add(nodenum) };
        let side = r_point_on_side(x, y, node);
        nodenum = unsafe { (*node).children[side as usize] as usize };
    }

    unsafe { subsectors.add(nodenum & !(NF_SUBSECTOR as usize)) }
}

// =============================================================================
// R_SetViewSize - deferred, called by M_Responder
// =============================================================================

/// Request view size change. Takes effect next refresh.
pub fn r_set_view_size(blocks: i32, detail: i32) {
    unsafe {
        SETSIZENEEDED = true;
        SETBLOCKS = blocks;
        SETDETAIL = detail;
    }
}

// =============================================================================
// R_Init
// =============================================================================

/// Stub for translation tables (not in doomgeneric).
fn r_init_translation_tables() {
    // R_InitTranslationTables - player color translation etc.
}

/// Initialize lighting LUTs (zlight only; scalelight changes with view size).
fn r_init_light_tables() {
    let colormaps = unsafe { state::COLORMAPS };
    if colormaps.is_null() {
        return;
    }

    const DISTMAP: i32 = 2;

    for i in 0..LIGHTLEVELS {
        let startmap = ((LIGHTLEVELS - 1 - i) * 2) * (NUMCOLORMAPS as usize) / LIGHTLEVELS;
        for j in 0..MAXLIGHTZ {
            let scale = fixed_div(
                (SCREENWIDTH / 2 * FRACUNIT) as Fixed,
                ((j + 1) << LIGHTZSHIFT) as Fixed,
            );
            let scale = scale >> LIGHTSCALESHIFT;
            let mut level = startmap as i32 - scale / DISTMAP;

            level = level.max(0);
            level = level.min(NUMCOLORMAPS - 1);

            unsafe {
                ZLIGHT[i][j] = colormaps.add((level as usize) * 256);
            }
        }
    }
}

/// Initialize texture mapping (viewangletox, xtoviewangle).
fn r_init_texture_mapping() {
    let viewwidth = unsafe { state::VIEWWIDTH };
    let centerxfrac = unsafe { CENTERXFRAC };
    let _centerx = unsafe { CENTERX };

    let focallength = fixed_div(
        centerxfrac,
        finetangent((FINEANGLES / 4 + FIELDOFVIEW as usize / 2) % (FINEANGLES / 2)),
    );

    for i in 0..(FINEANGLES / 2) {
        let ft = finetangent(i);
        let t = if ft > FRACUNIT * 2 {
            -1
        } else if ft < -FRACUNIT * 2 {
            (viewwidth + 1) as i32
        } else {
            let t = fixed_mul(ft, focallength);
            let t = (centerxfrac - t + FRACUNIT - 1) >> FRACBITS;
            if t < -1 {
                -1
            } else if t > viewwidth + 1 {
                (viewwidth + 1) as i32
            } else {
                t
            }
        };
        unsafe {
            state::VIEWANGLETOX[i] = t;
        }
    }

    for x in 0..=(viewwidth as usize) {
        let mut i = 0;
        while i < FINEANGLES / 2 && unsafe { state::VIEWANGLETOX[i] } > x as i32 {
            i += 1;
        }
        unsafe {
            state::XTOVIEWANGLE[x] = ((i << ANGLETOFINESHIFT) as u32).wrapping_sub(ANG90);
        }
    }

    for i in 0..(FINEANGLES / 2) {
        let vat = unsafe { state::VIEWANGLETOX[i] };
        if vat == -1 {
            unsafe { state::VIEWANGLETOX[i] = 0 };
        } else if vat == viewwidth + 1 {
            unsafe { state::VIEWANGLETOX[i] = viewwidth };
        }
    }

    unsafe {
        state::CLIPANGLE = state::XTOVIEWANGLE[0];
    }
}

/// Execute deferred view size change.
fn r_execute_set_view_size() {
    unsafe {
        SETSIZENEEDED = false;

        let setblocks = SETBLOCKS;
        let setdetail = SETDETAIL;

        if setblocks == 11 {
            state::SCALEDVIEWWIDTH = SCREENWIDTH;
            state::VIEWHEIGHT = SCREENHEIGHT;
        } else {
            state::SCALEDVIEWWIDTH = setblocks * 32;
            state::VIEWHEIGHT = (setblocks * 168 / 10) & !7;
        }

        DETAILSHIFT = setdetail;
        state::VIEWWIDTH = state::SCALEDVIEWWIDTH >> setdetail;

        CENTERY = state::VIEWHEIGHT / 2;
        CENTERX = state::VIEWWIDTH / 2;
        CENTERXFRAC = (CENTERX << FRACBITS) as Fixed;
        CENTERYFRAC = (CENTERY << FRACBITS) as Fixed;
        PROJECTION = CENTERXFRAC;

        // colfunc, basecolfunc, fuzzcolfunc, transcolfunc, spanfunc - set by r_draw
        // R_InitBuffer - from r_draw, stub for now
        // pspritescale, pspriteiscale - from r_things
        // screenheightarray, yslope, distscale - from r_draw
    }

    r_init_texture_mapping();

    // Scalelight - depends on colormaps
    let colormaps = unsafe { state::COLORMAPS };
    if !colormaps.is_null() {
        const DISTMAP: i32 = 2;
        let viewwidth = unsafe { state::VIEWWIDTH };
        let detailshift = unsafe { DETAILSHIFT };

        for i in 0..LIGHTLEVELS {
            let startmap =
                ((LIGHTLEVELS - 1 - i) * 2) * (NUMCOLORMAPS as usize) / LIGHTLEVELS;
            for j in 0..MAXLIGHTSCALE {
                let level = startmap as i32
                    - (j as i32 * SCREENWIDTH / (viewwidth << detailshift)) / DISTMAP;
                let level = level.max(0).min(NUMCOLORMAPS - 1);

                unsafe {
                    SCALELIGHT[i][j] = colormaps.add((level as usize) * 256);
                }
            }
        }
    }
}

/// Called by startup code.
pub fn r_init() {

    crate::rendering::r_data::r_init_data();
    crate::rendering::r_sky::r_init_sky_map();
    r_init_light_tables();
    r_init_translation_tables();

    crate::rendering::r_plane::r_init_planes();
    r_set_view_size(10, 0); // default: screenblocks=10, detail=0
    r_execute_set_view_size();

    unsafe {
        FRAMECOUNT = 0;
    }
}

// =============================================================================
// R_SetupFrame - set up POV for player
// =============================================================================

/// Stub player type for view setup.
pub struct ViewPlayerStub {
    pub mo_x: Fixed,
    pub mo_y: Fixed,
    pub mo_angle: Angle,
    pub viewz: Fixed,
    pub extralight: i32,
    pub fixedcolormap: i32,
}

/// Set up frame for rendering. Called before R_RenderPlayerView.
pub fn r_setup_frame(player: &ViewPlayerStub) {
    unsafe {
        state::VIEWX = player.mo_x;
        state::VIEWY = player.mo_y;
        state::VIEWANGLE = player.mo_angle;
        state::VIEWZ = player.viewz;
        EXTRALIGHT = player.extralight;

        let viewangle_idx = (state::VIEWANGLE >> ANGLETOFINESHIFT) as usize;
        VIEWSIN = finesine(viewangle_idx);
        VIEWCOS = finecosine(viewangle_idx);

        state::SSCOUNT = 0;

        if player.fixedcolormap != 0 {
            let colormaps = state::COLORMAPS;
            if !colormaps.is_null() {
                FIXEDCOLORMAP = colormaps.add(player.fixedcolormap as usize * 256);
                for i in 0..MAXLIGHTSCALE {
                    SCALELIGHTFIXED[i] = FIXEDCOLORMAP;
                }
            }
        } else {
            FIXEDCOLORMAP = ptr::null_mut();
        }

        FRAMECOUNT += 1;
        VALIDCOUNT += 1;
    }
}

// =============================================================================
// R_RenderPlayerView - main entry, called by G_Drawer
// =============================================================================

/// Render player view.
pub fn r_render_player_view(player: &ViewPlayerStub) {
    use crate::rendering::r_bsp;
    use crate::rendering::r_plane;
    use crate::rendering::r_things;

    r_setup_frame(player);

    r_bsp::r_clear_clip_segs();
    r_bsp::r_clear_draw_segs();
    r_plane::r_clear_planes();
    r_things::r_clear_sprites();

    let numnodes = unsafe { state::NUMNODES };
    if numnodes > 0 {
        r_bsp::r_render_bsp_node((numnodes - 1) as i32);
    }

    r_plane::r_draw_planes();
    // R_DrawMasked - r_things (stub)
}
