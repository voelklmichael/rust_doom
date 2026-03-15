// TODO(UNSAFE_ELIMINATION): Remove when migrated to Vec + indices
#[allow(unsafe_code)]
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
use std::sync::{Mutex, OnceLock};

// =============================================================================
// VVideoState - thread-safe via OnceLock + Mutex
// =============================================================================

const SCREEN_SIZE: usize = (SCREENWIDTH * SCREENHEIGHT) as usize;

static V_VIDEO_STATE: OnceLock<Mutex<VVideoState>> = OnceLock::new();

/// Safety: Raw pointers in VVideoState point to leaked/zone-allocated data.
unsafe impl Send for VVideoState {}

pub struct VVideoState {
    /// Main screen buffer - 8-bit palette indices, row-major. Leaked at init.
    pub screens: &'static mut [u8; SCREEN_SIZE],
    /// Saved VIEWIMAGE for V_RestoreBuffer (when drawing to alternate buffer).
    pub saved_viewimage: *mut u8,
    /// Current drawing target. Points into screens or alternate buffer.
    pub viewimage: *mut u8,
    /// Row offset table: ylookup[y] = viewimage + y * SCREENWIDTH.
    pub ylookup: [*mut u8; 200],
    /// Column offset table for subwindows (columnofs[x] = x for fullscreen).
    pub columnofs: [i32; 320],
    /// Dirty region box [minx, miny, maxx, maxy].
    pub dirtybox: [i32; 4],
    /// Tint table for translucency (Heretic/Hexen).
    pub tinttable: *mut u8,
}

impl Default for VVideoState {
    fn default() -> Self {
        Self {
            screens: Box::leak(Box::new([0u8; SCREEN_SIZE])),
            saved_viewimage: std::ptr::null_mut(),
            viewimage: std::ptr::null_mut(),
            ylookup: [std::ptr::null_mut(); 200],
            columnofs: [0; 320],
            dirtybox: [0, 0, 0, 0],
            tinttable: std::ptr::null_mut(),
        }
    }
}

fn get_v_video_state() -> &'static Mutex<VVideoState> {
    V_VIDEO_STATE.get_or_init(|| Mutex::new(VVideoState::default()))
}

/// Access VVideoState.
pub fn with_v_video_state<F, R>(f: F) -> R
where
    F: FnOnce(&VVideoState) -> R,
{
    let guard = get_v_video_state().lock().unwrap();
    f(&guard)
}

/// Mutably access VVideoState.
pub fn with_v_video_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut VVideoState) -> R,
{
    let mut guard = get_v_video_state().lock().unwrap();
    f(&mut guard)
}

// =============================================================================
// Public API (from .h)
// =============================================================================

/// Center Y (SCREENHEIGHT/2). For view center use r_main::with_r_main_state(|s| s.centery).
pub const CENTERY: i32 = SCREENHEIGHT / 2;

/// Patch clipping callback type (Strife).
pub type VpatchClipFunc = fn(*const patch_t, i32, i32) -> bool;

/// Set patch clip callback. Stub: no-op.
pub fn v_set_patch_clip_callback(_func: Option<VpatchClipFunc>) {}

/// Allocate buffer screens. Call before R_Init.
pub fn v_init() {
    with_v_video_state_mut(|s| {
        s.viewimage = s.screens.as_mut_ptr();
        for y in 0..SCREENHEIGHT {
            s.ylookup[y as usize] =
                unsafe { s.screens.as_mut_ptr().add((y as usize) * SCREENWIDTH as usize) };
        }
        for x in 0..SCREENWIDTH {
            s.columnofs[x as usize] = x;
        }
    });
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
    with_v_video_state(|s| {
        unsafe {
            if source.is_null() || s.viewimage.is_null() {
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
            let mut dest = s.viewimage.add((desty as usize) * SCREENWIDTH as usize + destx as usize);
            for _ in 0..h {
                std::ptr::copy_nonoverlapping(src, dest, w as usize);
                src = src.add(SCREENWIDTH as usize);
                dest = dest.add(SCREENWIDTH as usize);
            }
        }
    });
}

/// Draw patch at (x, y). Patch is column-based masked format.
pub fn v_draw_patch(x: i32, y: i32, patch: *const patch_t) {
    with_v_video_state(|s| {
        unsafe {
            if patch.is_null() || s.viewimage.is_null() {
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
                let dest_col = s.viewimage.add((y as usize) * SCREENWIDTH as usize + (x + col) as usize);
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
    });
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
    with_v_video_state(|s| {
        unsafe {
            if s.viewimage.is_null() || src.is_null() {
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
                let dest = s.ylookup[(y + row) as usize];
                if !dest.is_null() {
                    std::ptr::copy_nonoverlapping(
                        src.add((row * width) as usize),
                        dest.add(x as usize),
                        w as usize,
                    );
                }
            }
        }
    });
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

/// Copy current screen buffer to dest. For wipe/finale. Original: I_ReadScreen
pub fn v_read_screen(dest: *mut u8) {
    with_v_video_state(|s| {
        unsafe {
            if !s.viewimage.is_null() && !dest.is_null() {
                std::ptr::copy_nonoverlapping(s.viewimage, dest, SCREEN_SIZE);
            }
        }
    });
}

/// Draw raw screen lump. Stub: no-op.
pub fn v_draw_raw_screen(_raw: *const u8) {}

/// Switch to alternate buffer (e.g. status bar backing screen).
pub fn v_use_buffer(buffer: *mut u8) {
    with_v_video_state_mut(|s| {
        s.saved_viewimage = s.viewimage;
        s.viewimage = buffer;
        if !buffer.is_null() {
            for y in 0..SCREENHEIGHT {
                s.ylookup[y as usize] =
                    unsafe { buffer.add((y as usize) * SCREENWIDTH as usize) };
            }
        }
    });
}

/// Restore normal buffer after v_use_buffer.
pub fn v_restore_buffer() {
    with_v_video_state_mut(|s| {
        if !s.saved_viewimage.is_null() {
            s.viewimage = s.saved_viewimage;
            for y in 0..SCREENHEIGHT {
                s.ylookup[y as usize] =
                    unsafe { s.saved_viewimage.add((y as usize) * SCREENWIDTH as usize) };
            }
            s.saved_viewimage = std::ptr::null_mut();
        }
    });
}

/// Save screenshot. Stub: no-op.
pub fn v_screen_shot(_format: &str) {}

/// Load TINTTAB lump. Stub: no-op.
pub fn v_load_tint_table() {}

/// Load XLATAB lump (Strife). Stub: no-op.
pub fn v_load_xla_table() {}

/// Draw mouse speed box. Stub: no-op.
pub fn v_draw_mouse_speed_box(_speed: i32) {}
