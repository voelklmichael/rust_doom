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
// Screen buffer (for r_draw colfunc/spanfunc)
// =============================================================================

const SCREEN_SIZE: usize = (SCREENWIDTH * SCREENHEIGHT) as usize;

/// Main screen buffer - 8-bit palette indices, row-major.
static mut SCREENS: [u8; SCREEN_SIZE] = [0; SCREEN_SIZE];

/// Current drawing target. Points into SCREENS.
pub static mut VIEWIMAGE: *mut u8 = std::ptr::null_mut();

/// Row offset table: ylookup[y] = VIEWIMAGE + y * SCREENWIDTH.
pub static mut YLOOKUP: [*mut u8; 200] = [std::ptr::null_mut(); 200];

/// Saved VIEWIMAGE for V_RestoreBuffer (when drawing to alternate buffer).
static mut SAVED_VIEWIMAGE: *mut u8 = std::ptr::null_mut();

/// Column offset table for subwindows (columnofs[x] = x for fullscreen).
pub static mut COLUMNOFS: [i32; 320] = [0; 320];

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

/// Allocate buffer screens. Call before R_Init.
pub fn v_init() {
    unsafe {
        VIEWIMAGE = SCREENS.as_mut_ptr();
        for y in 0..SCREENHEIGHT {
            YLOOKUP[y as usize] = SCREENS.as_mut_ptr().add((y as usize) * SCREENWIDTH as usize);
        }
        for x in 0..SCREENWIDTH {
            COLUMNOFS[x as usize] = x;
        }
    }
}

/// Copy rect from source to dest.
/// Source and dest are row-major buffers with SCREENWIDTH bytes per row.
pub fn v_copy_rect(
    srcx: i32,
    srcy: i32,
    source: *const u8,
    width: i32,
    height: i32,
    destx: i32,
    desty: i32,
) {
    unsafe {
        if source.is_null() || VIEWIMAGE.is_null() {
            return;
        }
        let srcx = srcx.max(0).min(SCREENWIDTH - 1);
        let srcy = srcy.max(0).min(SCREENHEIGHT - 1);
        let destx = destx.max(0).min(SCREENWIDTH - 1);
        let desty = desty.max(0).min(SCREENHEIGHT - 1);
        let w = width.min(SCREENWIDTH - srcx).min(SCREENWIDTH - destx);
        let h = height.min(SCREENHEIGHT - srcy).min(SCREENHEIGHT - desty);
        if w <= 0 || h <= 0 {
            return;
        }
        let mut src = source.add((srcy as usize) * SCREENWIDTH as usize + srcx as usize);
        let mut dest = VIEWIMAGE.add((desty as usize) * SCREENWIDTH as usize + destx as usize);
        for _ in 0..h {
            std::ptr::copy_nonoverlapping(src, dest, w as usize);
            src = src.add(SCREENWIDTH as usize);
            dest = dest.add(SCREENWIDTH as usize);
        }
    }
}

/// Draw patch at (x, y). Patch is column-based masked format.
pub fn v_draw_patch(x: i32, y: i32, patch: *const patch_t) {
    unsafe {
        if patch.is_null() || VIEWIMAGE.is_null() {
            return;
        }
        let w = (*patch).width as i32;
        let _h = (*patch).height as i32;
        let left = (*patch).leftoffset as i32;
        let top = (*patch).topoffset as i32;
        let x = x - left;
        let y = y - top;
        if x + w > SCREENWIDTH || y + _h > SCREENHEIGHT || x < 0 || y < 0 {
            return;
        }
        let patch_bytes = patch as *const u8;
        for col in 0..w {
            let ofs = i32::from_le_bytes(std::ptr::read_unaligned(patch_bytes.add(8 + col as usize * 4) as *const [u8; 4]));
            let column = patch_bytes.add(ofs as usize);
            let mut col_ptr = column;
            let dest_col = VIEWIMAGE.add((y as usize) * SCREENWIDTH as usize + (x + col) as usize);
            loop {
                let topdelta = *col_ptr;
                if topdelta == 0xff {
                    break;
                }
                let length = *col_ptr.add(1) as usize;
                let source = col_ptr.add(3);
                let dest = dest_col.add(topdelta as usize * SCREENWIDTH as usize);
                for row in 0..length {
                    *dest.add(row * SCREENWIDTH as usize) = *source.add(row);
                }
                col_ptr = col_ptr.add(4 + length);
            }
        }
    }
}

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

/// Draw patch direct. Same as v_draw_patch (C: V_DrawPatchDirect calls V_DrawPatch).
pub fn v_draw_patch_direct(x: i32, y: i32, patch: *const patch_t) {
    v_draw_patch(x, y, patch);
}

/// Draw block of pixels.
pub fn v_draw_block(x: i32, y: i32, width: i32, height: i32, src: *const u8) {
    unsafe {
        if VIEWIMAGE.is_null() || src.is_null() {
            return;
        }
        let x = x.max(0).min(SCREENWIDTH - 1);
        let y = y.max(0).min(SCREENHEIGHT - 1);
        let w = width.min(SCREENWIDTH - x);
        let h = height.min(SCREENHEIGHT - y);
        if w <= 0 || h <= 0 {
            return;
        }
        for row in 0..h {
            let dest = YLOOKUP[(y + row) as usize];
            if !dest.is_null() {
                std::ptr::copy_nonoverlapping(
                    src.add((row * width) as usize),
                    dest.add(x as usize),
                    w as usize,
                );
            }
        }
    }
}

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

/// Switch to alternate buffer (e.g. status bar backing screen).
pub fn v_use_buffer(buffer: *mut u8) {
    unsafe {
        SAVED_VIEWIMAGE = VIEWIMAGE;
        VIEWIMAGE = buffer;
        if !buffer.is_null() {
            for y in 0..SCREENHEIGHT {
                YLOOKUP[y as usize] = buffer.add((y as usize) * SCREENWIDTH as usize);
            }
        }
    }
}

/// Restore normal buffer after v_use_buffer.
pub fn v_restore_buffer() {
    unsafe {
        if !SAVED_VIEWIMAGE.is_null() {
            VIEWIMAGE = SAVED_VIEWIMAGE;
            for y in 0..SCREENHEIGHT {
                YLOOKUP[y as usize] = SAVED_VIEWIMAGE.add((y as usize) * SCREENWIDTH as usize);
            }
            SAVED_VIEWIMAGE = std::ptr::null_mut();
        }
    }
}

/// Save screenshot. Stub: no-op.
pub fn v_screen_shot(_format: &str) {}

/// Load TINTTAB lump. Stub: no-op.
pub fn v_load_tint_table() {}

/// Load XLATAB lump (Strife). Stub: no-op.
pub fn v_load_xla_table() {}

/// Draw mouse speed box. Stub: no-op.
pub fn v_draw_mouse_speed_box(_speed: i32) {}
