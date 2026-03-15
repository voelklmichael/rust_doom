//! Rendering subsystem - BSP, visplanes, sprites, column drawing.
//!
//! Original: r_*, v_* modules from Doom.

pub(crate) mod defs;
mod m_bbox;
mod r_bsp;
pub(crate) mod r_data;
mod r_draw;
mod r_main;
mod r_plane;
mod r_segs;
pub(crate) mod r_sky;
mod r_things;
pub(crate) mod state;
mod v_patch;
mod v_video;

pub use defs::{
    Angle, DegenMobj, DrawSeg, LightTable, Line, Node, Sector, Seg, SideDef, SlopeType,
    SpriteFrame, Spritedef, Subsector, Thinker, Vertex, Visplane, Vissprite, MAXDRAWSEGS, SIL_BOTH,
    SIL_BOTTOM, SIL_NONE, SIL_TOP,
};
pub use m_bbox::{m_add_to_box, m_clear_box, slope_type_from_dx_dy, Bbox, BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
pub use r_bsp::{r_clear_clip_segs, r_clear_draw_segs, r_render_bsp_node};
pub use r_data::{
    r_check_flat_num_for_name,
    r_check_texture_num_for_name,
    r_flat_num_for_name,
    r_get_column,
    r_init_data,
    // r_precache_level,
    r_texture_num_for_name,
};
pub use r_draw::r_video_erase;
pub use r_main::{
    r_add_point_to_box,
    r_init,
    r_point_in_subsector,
    r_point_on_seg_side,
    r_point_on_side,
    r_point_to_angle,
    r_point_to_angle2,
    r_point_to_dist,
    r_render_player_view,
    r_scale_from_global_angle,
    r_set_view_size,
    r_setup_frame,
    view_player_from_console,
    with_r_main_state,
    with_r_main_state_mut,
    ViewPlayerStub,
    LIGHTLEVELS,
    MAXLIGHTSCALE,
    MAXLIGHTZ,
    NF_SUBSECTOR,
    NUMCOLORMAPS,
};
pub use r_plane::{r_check_plane, r_clear_planes, r_draw_planes, r_find_plane, r_init_planes};
pub use r_segs::{r_render_masked_seg_range, r_store_wall_range};
pub use r_sky::{
    r_init_sky_map, with_r_sky_state, with_r_sky_state_mut, ANGLETOSKYSHIFT, SKYFLATNAME,
};
pub use r_things::{r_init_sprites, r_sprite_num_for_name};
pub use state::{with_state, with_state_mut, RenderState};
pub use v_patch::{patch_t, post_t, ColumnT};
pub use v_video::{
    v_copy_rect, v_draw_alt_tl_patch, v_draw_block, v_draw_box, v_draw_filled_box,
    v_draw_horiz_line, v_draw_mouse_speed_box, v_draw_patch, v_draw_patch_direct,
    v_draw_patch_flipped, v_draw_raw_screen, v_draw_shadowed_patch, v_draw_tl_patch,
    v_draw_vert_line, v_draw_xla_patch, v_init, v_load_tint_table, v_load_xla_table, v_mark_rect,
    v_read_screen, v_restore_buffer, v_screen_shot, v_set_patch_clip_callback, v_use_buffer,
    with_v_video_state, with_v_video_state_mut, CENTERY,
};
