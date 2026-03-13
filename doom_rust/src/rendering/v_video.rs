//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Gamma correction LUT, patch drawing, screen blitting.
//
// Original: v_video.h / v_video.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::rendering::v_patch::patch_t;

// =============================================================================
// Public API (from .h)
// =============================================================================

/// Center Y (SCREENHEIGHT/2).
pub const CENTERY: i32 = SCREENHEIGHT / 2;

/// Dirty region box [minx, miny, maxx, maxy].
pub static mut DIRTYBOX: [i32; 4] = [0, 0, 0, 0];

/// Tint table for translucency (Heretic/Hexen).
pub static mut TINTTABLE: *mut u8 = std::ptr::null_mut();

/// Patch clipping callback type (Strife).
pub type VpatchClipFunc = fn(*const patch_t, i32, i32) -> bool;

/// Set patch clip callback. Stub: no-op.
pub fn v_set_patch_clip_callback(_func: Option<VpatchClipFunc>) {}

/// Allocate buffer screens. Call before R_Init. Stub: no-op.
pub fn v_init() {}

/// Copy rect from source to dest. Stub: no-op.
pub fn v_copy_rect(
    _srcx: i32,
    _srcy: i32,
    _source: *const u8,
    _width: i32,
    _height: i32,
    _destx: i32,
    _desty: i32,
) {
}

/// Draw patch at (x, y). Stub: no-op.
pub fn v_draw_patch(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw patch flipped. Stub: no-op.
pub fn v_draw_patch_flipped(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw translucency patch. Stub: no-op.
pub fn v_draw_tl_patch(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw alt TL patch. Stub: no-op.
pub fn v_draw_alt_tl_patch(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw shadowed patch. Stub: no-op.
pub fn v_draw_shadowed_patch(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw XLA patch (Strife). Stub: no-op.
pub fn v_draw_xla_patch(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw patch direct. Stub: no-op.
pub fn v_draw_patch_direct(_x: i32, _y: i32, _patch: *const patch_t) {}

/// Draw block of pixels. Stub: no-op.
pub fn v_draw_block(_x: i32, _y: i32, _width: i32, _height: i32, _src: *const u8) {}

/// Mark rect as dirty. Stub: no-op.
pub fn v_mark_rect(_x: i32, _y: i32, _width: i32, _height: i32) {}

/// Draw filled box. Stub: no-op.
pub fn v_draw_filled_box(_x: i32, _y: i32, _w: i32, _h: i32, _c: u8) {}

/// Draw horizontal line. Stub: no-op.
pub fn v_draw_horiz_line(_x: i32, _y: i32, _w: i32, _c: u8) {}

/// Draw vertical line. Stub: no-op.
pub fn v_draw_vert_line(_x: i32, _y: i32, _h: i32, _c: u8) {}

/// Draw box outline. Stub: no-op.
pub fn v_draw_box(_x: i32, _y: i32, _w: i32, _h: i32, _c: u8) {}

/// Draw raw screen lump. Stub: no-op.
pub fn v_draw_raw_screen(_raw: *const u8) {}

/// Switch to alternate buffer. Stub: no-op.
pub fn v_use_buffer(_buffer: *mut u8) {}

/// Restore normal buffer. Stub: no-op.
pub fn v_restore_buffer() {}

/// Save screenshot. Stub: no-op.
pub fn v_screen_shot(_format: &str) {}

/// Load TINTTAB lump. Stub: no-op.
pub fn v_load_tint_table() {}

/// Load XLATAB lump (Strife). Stub: no-op.
pub fn v_load_xla_table() {}

/// Draw mouse speed box. Stub: no-op.
pub fn v_draw_mouse_speed_box(_speed: i32) {}
