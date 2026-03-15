//! Rust translation of doomgeneric/r_main.h

use crate::d_player::*;
use crate::doomtype::*;
use crate::i_video::*;
use crate::m_fixed::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::tables::*;

pub static mut viewcos: FixedT = 0;
pub static mut viewsin: FixedT = 0;
pub static mut viewwindowx: i32 = 0;
pub static mut viewwindowy: i32 = 0;
pub static mut centerx: i32 = 0;
pub static mut centery: i32 = 0;
pub static mut centerxfrac: FixedT = 0;
pub static mut centeryfrac: FixedT = 0;
pub static mut projection: FixedT = 0;
pub static mut validcount: i32 = 0;
pub static mut linecount: i32 = 0;
pub static mut loopcount: i32 = 0;

/// C #define: LIGHTLEVELS
pub const LIGHTLEVELS: usize = 16;
/// C #define: LIGHTSEGSHIFT
pub const LIGHTSEGSHIFT: i32 = 4;
/// C #define: MAXLIGHTSCALE
pub const MAXLIGHTSCALE: usize = 48;
/// C #define: LIGHTSCALESHIFT
pub const LIGHTSCALESHIFT: i32 = 12;
/// C #define: MAXLIGHTZ
pub const MAXLIGHTZ: usize = 128;
/// C #define: LIGHTZSHIFT
pub const LIGHTZSHIFT: i32 = 20;

pub static mut scalelight: [[*mut LighttableT; MAXLIGHTSCALE]; LIGHTLEVELS] =
    [[std::ptr::null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS];
pub static mut scalelightfixed: [*mut LighttableT; MAXLIGHTSCALE] =
    [std::ptr::null_mut(); MAXLIGHTSCALE];
pub static mut zlight: [[*mut LighttableT; MAXLIGHTZ]; LIGHTLEVELS] =
    [[std::ptr::null_mut(); MAXLIGHTZ]; LIGHTLEVELS];

pub static mut extralight: i32 = 0;
pub static mut fixedcolormap: *mut LighttableT = std::ptr::null_mut();
/// C #define: NUMCOLORMAPS
pub const NUMCOLORMAPS: i32 = 32;

pub static mut detailshift: i32 = 0;

pub static mut colfunc: Option<fn()> = None;
pub static mut transcolfunc: Option<fn()> = None;
pub static mut basecolfunc: Option<fn()> = None;
pub static mut fuzzcolfunc: Option<fn()> = None;
pub static mut spanfunc: Option<fn()> = None;

/// C function: R_PointOnSide
pub fn r_point_on_side(x: FixedT, y: FixedT, node: *mut NodeT) -> i32 {
    todo!("original: R_PointOnSide")
}

/// C function: R_PointOnSegSide
pub fn r_point_on_seg_side(x: FixedT, y: FixedT, line: *mut SegT) -> i32 {
    todo!("original: R_PointOnSegSide")
}

/// C function: R_PointToAngle
pub fn r_point_to_angle(x: FixedT, y: FixedT) -> AngleT {
    todo!("original: R_PointToAngle")
}

/// C function: R_PointToAngle2
pub fn r_point_to_angle2(x1: FixedT, y1: FixedT, x2: FixedT, y2: FixedT) -> AngleT {
    todo!("original: R_PointToAngle2")
}

/// C function: R_PointToDist
pub fn r_point_to_dist(x: FixedT, y: FixedT) -> FixedT {
    todo!("original: R_PointToDist")
}

/// C function: R_ScaleFromGlobalAngle
pub fn r_scale_from_global_angle(visangle: AngleT) -> FixedT {
    todo!("original: R_ScaleFromGlobalAngle")
}

/// C function: R_PointInSubsector
pub fn r_point_in_subsector(x: FixedT, y: FixedT) -> *mut SubsectorT {
    todo!("original: R_PointInSubsector")
}

/// C function: R_AddPointToBox
pub fn r_add_point_to_box(x: i32, y: i32, box_: *mut FixedT) {
    todo!("original: R_AddPointToBox")
}

/// C function: R_RenderPlayerView
pub fn r_render_player_view(player: *mut PlayerT) {
    todo!("original: R_RenderPlayerView")
}

/// C function: R_Init
pub fn r_init() {
    todo!("original: R_Init")
}

/// C function: R_SetViewSize
pub fn r_set_view_size(blocks: i32, detail: i32) {
    todo!("original: R_SetViewSize")
}
