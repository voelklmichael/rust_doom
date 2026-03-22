//! Planes renderer (r_plane.h, r_plane.c)
//! Original: r_plane.h, r_plane.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::m_fixed::FixedT;

pub struct R_PlaneState;

impl R_PlaneState {
    /// Original: void R_InitPlanes(void)
    pub fn r_init_planes(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_ClearPlanes(void)
    pub fn r_clear_planes(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_MapPlane(int y, int x1, int x2)
    pub fn r_map_plane(&self, _y: i32, _x1: i32, _x2: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_DrawPlanes(void)
    pub fn r_draw_planes(&self) {
        todo!("Basic stage-0 stub")
    }
}
