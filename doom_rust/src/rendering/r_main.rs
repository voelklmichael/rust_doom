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
use crate::rendering::defs::{Node, Seg, Subsector, Vertex};
use crate::rendering::m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::rendering::r_draw;
use crate::rendering::state;
use std::ptr;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// RMainState - thread-safe via OnceLock + Mutex
// =============================================================================

static R_MAIN_STATE: OnceLock<Mutex<RMainState>> = OnceLock::new();

pub struct RMainState {
    pub setsizeneeded: bool,
    pub setblocks: i32,
    pub setdetail: i32,
    pub linecount: i32,
    pub loopcount: i32,

    pub validcount: i32,
    pub fixedcolormap: *mut u8,
    pub centerx: i32,
    pub centery: i32,
    pub centerxfrac: Fixed,
    pub centeryfrac: Fixed,
    pub projection: Fixed,
    pub framecount: i32,
    pub viewcos: Fixed,
    pub viewsin: Fixed,
    pub viewwindowx: i32,
    pub viewwindowy: i32,
    pub extralight: i32,
    pub detailshift: i32,
    pub scalelight: [[*mut u8; MAXLIGHTSCALE]; LIGHTLEVELS],
    pub scalelightfixed: [*mut u8; MAXLIGHTSCALE],
    pub zlight: [[*mut u8; MAXLIGHTZ]; LIGHTLEVELS],
}

impl Default for RMainState {
    fn default() -> Self {
        Self {
            setsizeneeded: false,
            setblocks: 10,
            setdetail: 0,
            linecount: 0,
            loopcount: 0,
            validcount: 1,
            fixedcolormap: ptr::null_mut(),
            centerx: 0,
            centery: 0,
            centerxfrac: 0,
            centeryfrac: 0,
            projection: 0,
            framecount: 0,
            viewcos: 0,
            viewsin: 0,
            viewwindowx: 0,
            viewwindowy: 0,
            extralight: 0,
            detailshift: 0,
            scalelight: [[ptr::null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS],
            scalelightfixed: [ptr::null_mut(); MAXLIGHTSCALE],
            zlight: [[ptr::null_mut(); MAXLIGHTZ]; LIGHTLEVELS],
        }
    }
}

fn get_r_main_state() -> &'static Mutex<RMainState> {
    R_MAIN_STATE.get_or_init(|| Mutex::new(RMainState::default()))
}

/// Access RMainState.
pub fn with_r_main_state<F, R>(f: F) -> R
where
    F: FnOnce(&RMainState) -> R,
{
    let guard = get_r_main_state().lock().unwrap();
    f(&guard)
}

/// Mutably access RMainState.
pub fn with_r_main_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut RMainState) -> R,
{
    let mut guard = get_r_main_state().lock().unwrap();
    f(&mut guard)
}

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
pub fn r_point_on_side(x: Fixed, y: Fixed, node: &Node) -> i32 {
    let dx = x - node.x;
    let dy = y - node.y;

    if node.dx == 0 {
        return if x <= node.x {
            if node.dy > 0 {
                1
            } else {
                0
            }
        } else {
            if node.dy < 0 {
                1
            } else {
                0
            }
        };
    }
    if node.dy == 0 {
        return if y <= node.y {
            if node.dx < 0 {
                1
            } else {
                0
            }
        } else {
            if node.dx > 0 {
                1
            } else {
                0
            }
        };
    }

    if (node.dy ^ node.dx ^ dx ^ dy) & SIGN_MASK != 0 {
        if (node.dy ^ dx) & SIGN_MASK != 0 {
            return 1;
        }
        return 0;
    }

    let left = fixed_mul(node.dy >> FRACBITS, dx);
    let right = fixed_mul(dy, node.dx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

/// Point-on-seg-side test. Returns 0 (front) or 1 (back).
pub fn r_point_on_seg_side(x: Fixed, y: Fixed, line: &Seg) -> i32 {
    let (lx, ly, ldx, ldy) = state::with_state(|s| {
        let v1 = s
            .vertexes
            .get(line.v1_idx)
            .copied()
            .unwrap_or(Vertex { x: 0, y: 0 });
        let v2 = s
            .vertexes
            .get(line.v2_idx)
            .copied()
            .unwrap_or(Vertex { x: 0, y: 0 });
        (v1.x, v1.y, v2.x - v1.x, v2.y - v1.y)
    });

    if ldx == 0 {
        return if x <= lx {
            if ldy > 0 {
                1
            } else {
                0
            }
        } else if ldy < 0 {
            1
        } else {
            0
        };
    }
    if ldy == 0 {
        return if y <= ly {
            if ldx < 0 {
                1
            } else {
                0
            }
        } else if ldx > 0 {
            1
        } else {
            0
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
    let (projection, detailshift) = with_r_main_state(|s| (s.projection, s.detailshift));

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

/// Find subsector containing point (x,y). Returns subsector index.
pub fn r_point_in_subsector(x: Fixed, y: Fixed) -> usize {
    state::with_state(|s| {
        let numnodes = s.numnodes;
        let nodes = &s.nodes;
        let subsectors = &s.subsectors;

        if numnodes == 0 || subsectors.is_empty() {
            return 0;
        }

        let mut nodenum = (numnodes - 1) as usize;

        while nodenum & (NF_SUBSECTOR as usize) == 0 {
            let node = &nodes[nodenum];
            let side = r_point_on_side(x, y, node);
            nodenum = node.children[side as usize] as usize;
        }

        nodenum & !(NF_SUBSECTOR as usize)
    })
}

// =============================================================================
// R_SetViewSize - deferred, called by M_Responder
// =============================================================================

/// Request view size change. Takes effect next refresh.
pub fn r_set_view_size(blocks: i32, detail: i32) {
    with_r_main_state_mut(|s| {
        s.setsizeneeded = true;
        s.setblocks = blocks;
        s.setdetail = detail;
    });
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

            with_r_main_state_mut(|s| {
                s.zlight[i][j] = unsafe { colormaps.add((level as usize) * 256) };
            });
        }
    }
}

/// Initialize texture mapping (viewangletox, xtoviewangle).
fn r_init_texture_mapping() {
    let viewwidth = state::with_state(|s| s.viewwidth);
    let (centerxfrac, _centerx) = with_r_main_state(|s| (s.centerxfrac, s.centerx));

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
    let (setblocks, setdetail) = with_r_main_state_mut(|s| {
        s.setsizeneeded = false;
        (s.setblocks, s.setdetail)
    });

    unsafe {
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
        with_r_main_state_mut(|s| {
            s.centery = viewheight / 2;
            s.centerx = viewwidth / 2;
            s.centerxfrac = (s.centerx << FRACBITS) as Fixed;
            s.centeryfrac = (s.centery << FRACBITS) as Fixed;
            s.projection = s.centerxfrac;
        });

        r_draw::with_r_draw_state_mut(|rd| {
            // Thing clipping - screenheightarray (r_clear_planes also sets per-frame)
            for i in 0..(viewwidth as usize) {
                rd.screenheightarray[i] = viewheight as i16;
            }

            // Plane slopes - yslope (C formula)
            for i in 0..(viewheight as usize) {
                let dy = ((i as i32 - viewheight / 2) << FRACBITS) + FRACUNIT / 2;
                let dy = dy.abs().max(1);
                rd.yslope[i] = fixed_div(
                    ((viewwidth << setdetail) / 2 * FRACUNIT) as Fixed,
                    dy as Fixed,
                );
            }

            // distscale (C formula: FRACUNIT/cos)
            for i in 0..(viewwidth as usize) {
                let cosadj = finecosine(
                    (state::with_state(|s| s.xtoviewangle[i]) >> ANGLETOFINESHIFT) as usize,
                )
                .abs();
                rd.distscale[i] = fixed_div(FRACUNIT, cosadj.max(1));
            }

            // Psprite scales
            rd.pspritescale = (FRACUNIT * viewwidth / SCREENWIDTH) as Fixed;
            rd.pspriteiscale = if viewwidth != 0 {
                (FRACUNIT as u64 * SCREENWIDTH as u64 / viewwidth as u64) as u32
            } else {
                0
            };
        });

        r_draw::r_init_buffer(state::with_state(|s| s.scaledviewwidth), viewheight);
    }

    r_init_texture_mapping();

    // Scalelight - depends on colormaps
    let colormaps = state::with_state(|s| s.colormaps);
    if !colormaps.is_null() {
        const DISTMAP: i32 = 2;
        let viewwidth = state::with_state(|s| s.viewwidth);
        let detailshift = with_r_main_state(|s| s.detailshift);

        with_r_main_state_mut(|s| {
            for i in 0..LIGHTLEVELS {
                let startmap = ((LIGHTLEVELS - 1 - i) * 2) * (NUMCOLORMAPS as usize) / LIGHTLEVELS;
                for j in 0..MAXLIGHTSCALE {
                    let level = startmap as i32
                        - (j as i32 * SCREENWIDTH / (viewwidth << detailshift)) / DISTMAP;
                    let level = level.max(0).min(NUMCOLORMAPS - 1);
                    s.scalelight[i][j] = unsafe { colormaps.add((level as usize) * 256) };
                }
            }
        });
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

    with_r_main_state_mut(|s| s.framecount = 0);
}

// =============================================================================
// View player from console - for D_Display / R_RenderPlayerView
// =============================================================================

/// Build ViewPlayerStub from the spawned console player. Returns None if no valid player.
pub fn view_player_from_console() -> Option<ViewPlayerStub> {
    use crate::doomstat::with_doomstat_state;

    with_doomstat_state(|st| {
        let idx = st.consoleplayer as usize;
        if idx >= MAXPLAYERS {
            return None;
        }
        let p = &st.players[idx];
        let mo_idx = p.mo?;
        crate::player::mobjs::with_mobj_ref(mo_idx, |mo| ViewPlayerStub {
            mo_x: mo.x,
            mo_y: mo.y,
            mo_angle: mo.angle,
            viewz: p.viewz,
            extralight: p.extralight,
            fixedcolormap: p.fixedcolormap,
        })
    })
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
    state::with_state_mut(|s| {
        s.viewx = player.mo_x;
        s.viewy = player.mo_y;
        s.viewangle = player.mo_angle.wrapping_add(s.viewangleoffset);
        s.viewz = player.viewz;
    });

    with_r_main_state_mut(|s| {
        s.extralight = player.extralight;

        let viewangle_idx = (state::with_state(|st| st.viewangle) >> ANGLETOFINESHIFT) as usize;
        s.viewsin = finesine(viewangle_idx);
        s.viewcos = finecosine(viewangle_idx);

        if player.fixedcolormap != 0 {
            let colormaps = state::with_state(|st| st.colormaps);
            if !colormaps.is_null() {
                s.fixedcolormap = unsafe { colormaps.add(player.fixedcolormap as usize * 256) };
                for i in 0..MAXLIGHTSCALE {
                    s.scalelightfixed[i] = s.fixedcolormap;
                }
            }
        } else {
            s.fixedcolormap = ptr::null_mut();
        }

        s.framecount += 1;
        s.validcount += 1;
    });
    state::with_state_mut(|s| s.sscount = 0);
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
