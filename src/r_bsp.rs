//! BSP renderer (r_bsp.h, r_bsp.c)
//! Original: r_bsp.h, r_bsp.c

use crate::doomtype::Boolean;

pub const MAXDRAWSEGS: usize = 256;

pub struct R_BspState;

impl R_BspState {
    /// Original: void R_ClearClipSegs(void)
    pub fn r_clear_clip_segs(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_ClearDrawSegs(void)
    pub fn r_clear_draw_segs(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_RenderBSPNode(int bspnum)
    pub fn r_render_bsp_node(&self, _bspnum: i32) {
        todo!("Basic stage-0 stub")
    }
}
