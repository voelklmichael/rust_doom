//! Main renderer (r_main.h, r_main.c)
//! Original: r_main.h, r_main.c

use crate::m_fixed::FixedT;

pub const LIGHTLEVELS: usize = 16;
pub const LIGHTSEGSHIFT: i32 = 4;
pub const MAXLIGHTSCALE: usize = 48;
pub const LIGHTSCALESHIFT: i32 = 12;
pub const MAXLIGHTZ: usize = 128;
pub const LIGHTZSHIFT: i32 = 20;
pub const NUMCOLORMAPS: i32 = 32;

pub struct R_MainState;

impl R_MainState {
    /// Original: int R_PointOnSide(fixed_t x, fixed_t y, node_t *node)
    pub fn r_point_on_side(&self, _x: FixedT, _y: FixedT, _node: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int R_PointOnSegSide(fixed_t x, fixed_t y, seg_t *line)
    pub fn r_point_on_seg_side(&self, _x: FixedT, _y: FixedT, _line: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: angle_t R_PointToAngle(fixed_t x, fixed_t y)
    pub fn r_point_to_angle(&self, _x: FixedT, _y: FixedT) -> u32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_InitPointToAngle(void)
    pub fn r_init_point_to_angle(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t R_ScaleFromGlobalAngle(angle_t visangle)
    pub fn r_scale_from_global_angle(&self, _visangle: u32) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_InitTables(void)
    pub fn r_init_tables(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_InitTextureMapping(void)
    pub fn r_init_texture_mapping(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_InitLightTables(void)
    pub fn r_init_light_tables(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_ExecuteSetViewSize(void)
    pub fn r_execute_set_view_size(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_Init(void)
    pub fn r_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_SetupFrame(player_t *player)
    pub fn r_setup_frame(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_RenderPlayerView(player_t *player)
    pub fn r_render_player_view(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }
}
