// doomgeneric/r_main.h

pub use crate::d_player::*;
pub use crate::m_fixed::*;
pub use crate::r_data::*;
pub use crate::r_defs::*;
pub use crate::tables::AngleT;

use std::cell::RefCell;

// Original: #define LIGHTLEVELS 16
pub const LIGHTLEVELS: usize = 16;
// Original: #define MAXLIGHTSCALE 48
pub const MAXLIGHTSCALE: usize = 48;
// Original: #define MAXLIGHTZ 128
pub const MAXLIGHTZ: usize = 128;
// Original: #define NUMCOLORMAPS 32
pub const NUMCOLORMAPS: i32 = 32;

#[allow(non_camel_case_types)]
pub struct R_MainState {
    // Original: viewcos
    pub viewcos: RefCell<FixedT>,
    // Original: viewsin
    pub viewsin: RefCell<FixedT>,
    // Original: centerx
    pub centerx: RefCell<i32>,
    // Original: centery
    pub centery: RefCell<i32>,
    // Original: validcount
    pub validcount: RefCell<i32>,
    // Original: extralight
    pub extralight: RefCell<i32>,
    // Original: detailshift
    pub detailshift: RefCell<i32>,
}

impl R_MainState {
    pub fn new() -> Self {
        Self {
            viewcos: RefCell::new(0),
            viewsin: RefCell::new(0),
            centerx: RefCell::new(0),
            centery: RefCell::new(0),
            validcount: RefCell::new(0),
            extralight: RefCell::new(0),
            detailshift: RefCell::new(0),
        }
    }

    // Original: R_PointOnSide
    pub fn r_point_on_side(&self, _x: FixedT, _y: FixedT, _node: *mut NodeT) -> i32 {
        todo!("R_PointOnSide");
    }

    // Original: R_PointOnSegSide
    pub fn r_point_on_seg_side(&self, _x: FixedT, _y: FixedT, _line: *mut SegT) -> i32 {
        todo!("R_PointOnSegSide");
    }

    // Original: R_PointToAngle
    pub fn r_point_to_angle(&self, _x: FixedT, _y: FixedT) -> AngleT {
        todo!("R_PointToAngle");
    }

    // Original: R_PointToAngle2
    pub fn r_point_to_angle2(&self, _x1: FixedT, _y1: FixedT, _x2: FixedT, _y2: FixedT) -> AngleT {
        todo!("R_PointToAngle2");
    }

    // Original: R_PointToDist
    pub fn r_point_to_dist(&self, _x: FixedT, _y: FixedT) -> FixedT {
        todo!("R_PointToDist");
    }

    // Original: R_ScaleFromGlobalAngle
    pub fn r_scale_from_global_angle(&self, _visangle: AngleT) -> FixedT {
        todo!("R_ScaleFromGlobalAngle");
    }

    // Original: R_PointInSubsector
    pub fn r_point_in_subsector(&self, _x: FixedT, _y: FixedT) -> *mut SubsectorT {
        todo!("R_PointInSubsector");
    }

    // Original: R_AddPointToBox
    pub fn r_add_point_to_box(&self, _x: i32, _y: i32, _box_: *mut FixedT) {
        todo!("R_AddPointToBox");
    }

    // Original: R_RenderPlayerView
    pub fn r_render_player_view(&self, _player: *mut PlayerT) {
        todo!("R_RenderPlayerView");
    }

    // Original: R_Init
    pub fn r_init(&self) {
        todo!("R_Init");
    }

    // Original: R_SetViewSize
    pub fn r_set_view_size(&self, _blocks: i32, _detail: i32) {
        todo!("R_SetViewSize");
    }
}
