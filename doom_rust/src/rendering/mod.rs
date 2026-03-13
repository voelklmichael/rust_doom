//! Rendering subsystem - BSP, visplanes, sprites, column drawing.
//!
//! Original: r_*, v_* modules from Doom.

mod defs;
mod m_bbox;
mod r_data;
mod r_sky;
mod state;
mod v_patch;
mod v_video;

pub use defs::{
    Angle, DrawSeg, DegenMobj, Line, LightTable, Node, Sector, Seg, SideDef, SlopeType, Spritedef,
    SpriteFrame, Subsector, Thinker, Vertex, Visplane, Vissprite, MAXDRAWSEGS, SIL_BOTH, SIL_BOTTOM,
    SIL_NONE, SIL_TOP,
};
pub use m_bbox::{m_add_to_box, m_clear_box, Bbox, BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
pub use state::{
    CEILINGPLANE, COLORMAPS, FIRSTFLAT, FIRSTSPRITELUMP, FLOORPLANE, FLATTRANSLATION, LASTSPRITELUMP,
    LINES, NODES, NUMSPRITELUMPS, NUMSECTORS, NUMSIDES, NUMLINES, NUMNODES, NUMSEGS, NUMSPRITES,
    NUMSUBSECTORS, NUMVERTEXES, RW_ANGLE1, RW_DISTANCE, RW_NORMALANGLE, SECTORS,
    SEGS, SIDES, SPRITES, SPRITEOFFSET, SPRITETOPOFFSET, SPRITEWIDTH, SSCOUNT, SCALEDVIEWWIDTH,
    TEXTUREHEIGHT, TEXTURETRANSLATION, VERTEXES, VIEWANGLETOX, VIEWANGLE, VIEWHEIGHT, VIEWWIDTH,
    VIEWX, VIEWY, VIEWZ, XTOVIEWANGLE,
};
pub use v_patch::{column_t, patch_t, post_t};
pub use r_data::{
    r_check_texture_num_for_name, r_flat_num_for_name, r_get_column, r_init_data,
    r_precache_level, r_texture_num_for_name,
};
pub use r_sky::{r_init_sky_map, ANGLETOSKYSHIFT, SKYFLATNAME, SKYTEXTURE, SKYTEXTUREMID};
pub use v_video::{
    v_copy_rect, v_draw_alt_tl_patch, v_draw_block, v_draw_box, v_draw_filled_box,
    v_draw_horiz_line, v_draw_mouse_speed_box, v_draw_patch, v_draw_patch_direct,
    v_draw_patch_flipped, v_draw_raw_screen, v_draw_shadowed_patch, v_draw_tl_patch,
    v_draw_vert_line, v_draw_xla_patch, v_init, v_load_tint_table, v_load_xla_table,
    v_mark_rect, v_restore_buffer, v_screen_shot, v_set_patch_clip_callback, v_use_buffer,
    CENTERY, DIRTYBOX, TINTTABLE,
};
