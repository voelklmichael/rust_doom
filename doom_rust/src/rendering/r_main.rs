//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Rendering main loop and setup functions, utility functions (BSP, geometry).
//
// Original: r_main.h + r_main.c

use crate::doomdef::{MAXPLAYERS, SCREENHEIGHT, SCREENWIDTH};
use crate::geometry::{
    finecosine, finesine, finetangent, slope_div, tantoangle, Angle, ANG180, ANG270, ANG90,
    ANGLETOFINESHIFT, DBITS, FINEANGLES,
};
use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS, FRACUNIT};
use crate::rendering::defs::{Node, Seg, Subsector};
use crate::rendering::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::rendering::r_draw;
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
    let viewx = state::with_state(|s| s.viewx);
    let viewy = state::with_state(|s| s.viewy);

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
        state::with_state_mut(|s| {
            s.viewx = x1;
            s.viewy = y1;
        });
    }
    r_point_to_angle(x2, y2)
}

/// Distance from view to (x,y).
pub fn r_point_to_dist(x: Fixed, y: Fixed) -> Fixed {
    let viewx = state::with_state(|s| s.viewx);
    let viewy = state::with_state(|s| s.viewy);

    let mut dx = (x - viewx).abs();
    let mut dy = (y - viewy).abs();

    if dy > dx {
        std::mem::swap(&mut dx, &mut dy);
    }

    let frac = if dx != 0 { fixed_div(dy, dx) } else { 0 };

    let angle = (tantoangle((frac >> DBITS) as usize) + ANG90) >> ANGLETOFINESHIFT;
    fixed_div(dx, finesine(angle as usize))
}

/// Texture mapping scale for current line at given angle. rw_distance must be set.
pub fn r_scale_from_global_angle(visangle: Angle) -> Fixed {
    let viewangle = state::with_state(|s| s.viewangle);
    let rw_distance = state::with_state(|s| s.rw_distance);
    let rw_normalangle = state::with_state(|s| s.rw_normalangle);
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
    let numnodes = state::with_state(|s| s.numnodes);
    let subsectors = state::with_state(|s| s.subsectors);
    let nodes = state::with_state(|s| s.nodes);

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
    let colormaps = state::with_state(|s| s.colormaps);
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
    let viewwidth = state::with_state(|s| s.viewwidth);
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
        state::with_state_mut(|s| s.viewangletox[i] = t);
    }

    for x in 0..=(viewwidth as usize) {
        let mut i = 0;
        while i < FINEANGLES / 2 && state::with_state(|s| s.viewangletox[i]) > x as i32 {
            i += 1;
        }
        state::with_state_mut(|s| {
            s.xtoviewangle[x] = ((i << ANGLETOFINESHIFT) as u32).wrapping_sub(ANG90)
        });
    }

    for i in 0..(FINEANGLES / 2) {
        let vat = state::with_state(|s| s.viewangletox[i]);
        if vat == -1 {
            state::with_state_mut(|s| s.viewangletox[i] = 0);
        } else if vat == viewwidth + 1 {
            state::with_state_mut(|s| s.viewangletox[i] = viewwidth);
        }
    }

    state::with_state_mut(|s| s.clipangle = s.xtoviewangle[0]);
}

/// Execute deferred view size change.
fn r_execute_set_view_size() {
    unsafe {
        SETSIZENEEDED = false;

        let setblocks = SETBLOCKS;
        let setdetail = SETDETAIL;

        state::with_state_mut(|s| {
            if setblocks == 11 {
                s.scaledviewwidth = SCREENWIDTH;
                s.viewheight = SCREENHEIGHT;
            } else {
                s.scaledviewwidth = setblocks * 32;
                s.viewheight = (setblocks * 168 / 10) & !7;
            }
            s.viewwidth = s.scaledviewwidth >> setdetail;
        });

        let viewheight = state::with_state(|s| s.viewheight);
        let viewwidth = state::with_state(|s| s.viewwidth);
        CENTERY = viewheight / 2;
        CENTERX = viewwidth / 2;
        CENTERXFRAC = (CENTERX << FRACBITS) as Fixed;
        CENTERYFRAC = (CENTERY << FRACBITS) as Fixed;
        PROJECTION = CENTERXFRAC;

        // Thing clipping - screenheightarray (r_clear_planes also sets per-frame)
        for i in 0..(viewwidth as usize) {
            r_draw::SCREENHEIGHTARRAY[i] = viewheight as i16;
        }

        // Plane slopes - yslope (C formula)
        for i in 0..(viewheight as usize) {
            let dy = ((i as i32 - viewheight / 2) << FRACBITS) + FRACUNIT / 2;
            let dy = dy.abs().max(1);
            r_draw::YSLOPE[i] = fixed_div(
                ((viewwidth << setdetail) / 2 * FRACUNIT) as Fixed,
                dy as Fixed,
            );
        }

        // distscale (C formula: FRACUNIT/cos)
        for i in 0..(viewwidth as usize) {
            let cosadj =
                finecosine((state::with_state(|s| s.xtoviewangle[i]) >> ANGLETOFINESHIFT) as usize)
                    .abs();
            r_draw::DISTSCALE[i] = fixed_div(FRACUNIT, cosadj.max(1));
        }

        // Psprite scales
        r_draw::PSPRITESCALE = (FRACUNIT * viewwidth / SCREENWIDTH) as Fixed;
        r_draw::PSPRITEISCALE = if viewwidth != 0 {
            (FRACUNIT as u64 * SCREENWIDTH as u64 / viewwidth as u64) as u32
        } else {
            0
        };

        r_draw::r_init_buffer(state::with_state(|s| s.scaledviewwidth), viewheight);
    }

    r_init_texture_mapping();

    // Scalelight - depends on colormaps
    let colormaps = state::with_state(|s| s.colormaps);
    if !colormaps.is_null() {
        const DISTMAP: i32 = 2;
        let viewwidth = state::with_state(|s| s.viewwidth);
        let detailshift = unsafe { DETAILSHIFT };

        for i in 0..LIGHTLEVELS {
            let startmap = ((LIGHTLEVELS - 1 - i) * 2) * (NUMCOLORMAPS as usize) / LIGHTLEVELS;
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
    crate::rendering::v_video::v_init();

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
// View player from console - for D_Display / R_RenderPlayerView
// =============================================================================

/// Build ViewPlayerStub from the spawned console player. Returns None if no valid player.
pub fn view_player_from_console() -> Option<ViewPlayerStub> {
    use crate::doomstat::{CONSOLEPLAYER, PLAYERS};

    unsafe {
        let idx = CONSOLEPLAYER as usize;
        if idx >= MAXPLAYERS {
            return None;
        }
        let p = &PLAYERS[idx];
        let mo = p.mo;
        if mo.is_null() {
            return None;
        }
        let mo = mo as *const crate::player::p_mobj::Mobj;
        Some(ViewPlayerStub {
            mo_x: (*mo).x,
            mo_y: (*mo).y,
            mo_angle: (*mo).angle,
            viewz: p.viewz,
            extralight: p.extralight,
            fixedcolormap: p.fixedcolormap,
        })
    }
}

// =============================================================================
// R_SetupFrame - set up POV for player
// =============================================================================

/// Stub player type for view setup.
#[derive(Default)]
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
        state::with_state_mut(|s| {
            s.viewx = player.mo_x;
            s.viewy = player.mo_y;
            s.viewangle = player.mo_angle.wrapping_add(s.viewangleoffset);
            s.viewz = player.viewz;
        });
        EXTRALIGHT = player.extralight;

        let viewangle_idx = (state::with_state(|s| s.viewangle) >> ANGLETOFINESHIFT) as usize;
        VIEWSIN = finesine(viewangle_idx);
        VIEWCOS = finecosine(viewangle_idx);

        state::with_state_mut(|s| s.sscount = 0);

        if player.fixedcolormap != 0 {
            let colormaps = state::with_state(|s| s.colormaps);
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

    let numnodes = state::with_state(|s| s.numnodes);
    if numnodes > 0 {
        r_bsp::r_render_bsp_node((numnodes - 1) as i32);
    }

    r_plane::r_draw_planes();
    r_things::r_draw_masked();
}
