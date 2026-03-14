//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Low-level column and span blitting.
//
// Original: r_draw.h + r_draw.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::m_fixed::{Fixed, FRACBITS};
use crate::rendering::r_main::{self, DETAILSHIFT};
use crate::rendering::state;
use crate::rendering::v_video::{COLUMNOFS, CENTERY, VIEWIMAGE, YLOOKUP};
use std::ptr;

/// Status bar height at bottom of screen.
const SBARHEIGHT: i32 = 32;

/// Fuzz table size for spectre/invisibility effect.
const FUZZTABLE: usize = 50;
const FUZZOFF: i32 = SCREENWIDTH;

/// Fuzz offset table - samples adjacent pixels for shadow effect.
static FUZZOFFSET: [i32; FUZZTABLE] = [
    FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF,
    FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF,
    FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
];

static mut FUZZPOS: usize = 0;

/// Translation table for player colors (green→gray/brown/red). Set by r_init_translation_tables.
pub static mut DC_TRANSLATION: *const u8 = ptr::null();

// =============================================================================
// Draw state - ds_* for spans, dc_* for columns
// =============================================================================

pub static mut DS_XFRAC: Fixed = 0;
pub static mut DS_YFRAC: Fixed = 0;
pub static mut DS_COLORMAP: *mut u8 = ptr::null_mut();
pub static mut DS_SOURCE: *const u8 = ptr::null();
pub static mut DS_Y: i32 = 0;
pub static mut DS_X1: i32 = 0;
pub static mut DS_X2: i32 = 0;
pub static mut DS_XSTEP: Fixed = 0;
pub static mut DS_YSTEP: Fixed = 0;

pub static mut DC_YL: i32 = 0;
pub static mut DC_YH: i32 = 0;
pub static mut DC_X: i32 = 0;
pub static mut DC_SOURCE: *const u8 = ptr::null();
pub static mut DC_COLORMAP: *mut u8 = ptr::null_mut();
pub static mut DC_ISCALE: u32 = 0;
pub static mut DC_TEXTUREMID: Fixed = 0;

/// pspriteiscale - used for sky column scaling. Set by r_main when view size changes.
pub static mut PSPRITEISCALE: u32 = 0;
/// pspritescale - weapon sprite scale. Set by r_main when view size changes.
pub static mut PSPRITESCALE: Fixed = 0;

// =============================================================================
// Plane rendering helpers - yslope, distscale (from r_draw / r_main)
// =============================================================================

pub static mut YSLOPE: [Fixed; 200] = [0; 200]; // SCREENHEIGHT
pub static mut DISTSCALE: [Fixed; 320] = [0; 320]; // SCREENWIDTH

// =============================================================================
// Clipping arrays for single-sided walls (sprtopclip/sprbottomclip)
// =============================================================================

/// Array of viewheight - used when ceiling fills entire column.
pub static mut SCREENHEIGHTARRAY: [i16; 320] = [0; 320];
/// Array of -1 - used when floor fills entire column.
pub static mut NEGONEARRAY: [i16; 320] = [-1; 320];

// =============================================================================
// Public API - draw functions
// =============================================================================

/// Draw a wall column (high detail). Uses dc_yl, dc_yh, dc_x, dc_source, dc_colormap, dc_iscale, dc_texturemid.
fn colfunc_high() {
    unsafe {
        let count = DC_YH - DC_YL;
        if count < 0 {
            return;
        }
        if DC_SOURCE.is_null() || DC_COLORMAP.is_null() {
            return;
        }
        let dest = YLOOKUP[DC_YL as usize];
        if dest.is_null() {
            return;
        }
        let colofs = COLUMNOFS[DC_X as usize] as usize;
        let mut dest_ptr = dest.add(colofs);

        let centery = CENTERY;
        let fracstep = DC_ISCALE as i32;
        let mut frac = DC_TEXTUREMID + (DC_YL - centery) * fracstep;

        let source = DC_SOURCE;
        let colormap = DC_COLORMAP;

        let mut n = count;
        while n >= 0 {
            let row = (frac >> FRACBITS).max(0).min(127) as usize;
            let idx = *source.add(row) as usize;
            *dest_ptr = *colormap.add(idx);

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw a wall column (low detail, blocky 2x). Draws to dc_x*2 and dc_x*2+1.
fn colfunc_low() {
    unsafe {
        let count = DC_YH - DC_YL;
        if count < 0 {
            return;
        }
        if DC_SOURCE.is_null() || DC_COLORMAP.is_null() {
            return;
        }
        let x = DC_X << 1;
        if x + 1 >= SCREENWIDTH {
            return;
        }
        let dest = YLOOKUP[DC_YL as usize];
        if dest.is_null() {
            return;
        }
        let colofs0 = COLUMNOFS[x as usize] as usize;
        let colofs1 = COLUMNOFS[(x + 1) as usize] as usize;
        let mut dest_ptr = dest.add(colofs0);
        let mut dest2_ptr = dest.add(colofs1);

        let centery = CENTERY;
        let fracstep = DC_ISCALE as i32;
        let mut frac = DC_TEXTUREMID + (DC_YL - centery) * fracstep;

        let source = DC_SOURCE;
        let colormap = DC_COLORMAP;

        let mut n = count;
        while n >= 0 {
            let row = (frac >> FRACBITS).max(0).min(127) as usize;
            let idx = *source.add(row) as usize;
            let pixel = *colormap.add(idx);
            *dest_ptr = pixel;
            *dest2_ptr = pixel;

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            dest2_ptr = dest2_ptr.add(SCREENWIDTH as usize);
            frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw wall column - dispatches to high or low based on detailshift.
pub fn colfunc() {
    if unsafe { DETAILSHIFT } == 0 {
        colfunc_high();
    } else {
        colfunc_low();
    }
}

/// Draw fuzz column (spectre/invisibility effect). High detail.
fn fuzzcolfunc_high() {
    unsafe {
        let mut dc_yl = DC_YL;
        let mut dc_yh = DC_YH;
        if dc_yl == 0 {
            dc_yl = 1;
        }
        let viewheight = state::with_state(|s| s.viewheight);
        if dc_yh == viewheight - 1 {
            dc_yh = viewheight - 2;
        }
        let count = dc_yh - dc_yl;
        if count < 0 {
            return;
        }
        if DC_SOURCE.is_null() || DC_COLORMAP.is_null() || VIEWIMAGE.is_null() {
            return;
        }
        let colormaps = state::with_state(|s| s.colormaps);
        if colormaps.is_null() {
            return;
        }
        let fuzz_cmap = colormaps.add(6 * 256);
        let dest = YLOOKUP[dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs = COLUMNOFS[DC_X as usize] as usize;
        let mut dest_ptr = dest.add(colofs);

        let centery = CENTERY;
        let fracstep = DC_ISCALE as i32;
        let mut frac = DC_TEXTUREMID + (dc_yl - centery) * fracstep;

        let mut n = count;
        while n >= 0 {
            let fuzz_off = FUZZOFFSET[FUZZPOS];
            FUZZPOS += 1;
            if FUZZPOS >= FUZZTABLE {
                FUZZPOS = 0;
            }
            let src_idx = dest_ptr.offset(fuzz_off as isize).read() as usize;
            *dest_ptr = *fuzz_cmap.add(src_idx);

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw fuzz column. Low detail.
fn fuzzcolfunc_low() {
    unsafe {
        let mut dc_yl = DC_YL;
        let mut dc_yh = DC_YH;
        if dc_yl == 0 {
            dc_yl = 1;
        }
        let viewheight = state::with_state(|s| s.viewheight);
        if dc_yh == viewheight - 1 {
            dc_yh = viewheight - 2;
        }
        let count = dc_yh - dc_yl;
        if count < 0 {
            return;
        }
        let x = DC_X << 1;
        if x + 1 >= SCREENWIDTH {
            return;
        }
        if DC_SOURCE.is_null() || DC_COLORMAP.is_null() || VIEWIMAGE.is_null() {
            return;
        }
        let colormaps = state::with_state(|s| s.colormaps);
        if colormaps.is_null() {
            return;
        }
        let fuzz_cmap = colormaps.add(6 * 256);
        let dest = YLOOKUP[dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs0 = COLUMNOFS[x as usize] as usize;
        let colofs1 = COLUMNOFS[(x + 1) as usize] as usize;
        let mut dest_ptr = dest.add(colofs0);
        let mut dest2_ptr = dest.add(colofs1);

        let centery = CENTERY;
        let fracstep = DC_ISCALE as i32;
        let mut frac = DC_TEXTUREMID + (dc_yl - centery) * fracstep;

        let mut n = count;
        while n >= 0 {
            let fuzz_off = FUZZOFFSET[FUZZPOS];
            FUZZPOS += 1;
            if FUZZPOS >= FUZZTABLE {
                FUZZPOS = 0;
            }
            let src_idx = dest_ptr.offset(fuzz_off as isize).read() as usize;
            let pixel = *fuzz_cmap.add(src_idx);
            *dest_ptr = pixel;
            let src_idx2 = dest2_ptr.offset(fuzz_off as isize).read() as usize;
            *dest2_ptr = *fuzz_cmap.add(src_idx2);

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            dest2_ptr = dest2_ptr.add(SCREENWIDTH as usize);
            frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw fuzz column - dispatches by detailshift.
pub fn fuzzcolfunc() {
    if unsafe { DETAILSHIFT } == 0 {
        fuzzcolfunc_high();
    } else {
        fuzzcolfunc_low();
    }
}

/// Draw translated column (player colors). High detail.
fn transcolfunc_high() {
    unsafe {
        let count = DC_YH - DC_YL;
        if count < 0 {
            return;
        }
        if DC_SOURCE.is_null() || DC_COLORMAP.is_null() || DC_TRANSLATION.is_null() {
            return;
        }
        let dest = YLOOKUP[DC_YL as usize];
        if dest.is_null() {
            return;
        }
        let colofs = COLUMNOFS[DC_X as usize] as usize;
        let mut dest_ptr = dest.add(colofs);

        let centery = CENTERY;
        let fracstep = DC_ISCALE as i32;
        let mut frac = DC_TEXTUREMID + (DC_YL - centery) * fracstep;

        let source = DC_SOURCE;
        let colormap = DC_COLORMAP;
        let translation = DC_TRANSLATION;

        let mut n = count;
        while n >= 0 {
            let row = (frac >> FRACBITS).max(0).min(127) as usize;
            let src_idx = *source.add(row) as usize;
            let trans_idx = *translation.add(src_idx) as usize;
            *dest_ptr = *colormap.add(trans_idx);

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw translated column. Low detail.
fn transcolfunc_low() {
    unsafe {
        let count = DC_YH - DC_YL;
        if count < 0 {
            return;
        }
        if DC_SOURCE.is_null() || DC_COLORMAP.is_null() || DC_TRANSLATION.is_null() {
            return;
        }
        let x = DC_X << 1;
        if x + 1 >= SCREENWIDTH {
            return;
        }
        let dest = YLOOKUP[DC_YL as usize];
        if dest.is_null() {
            return;
        }
        let colofs0 = COLUMNOFS[x as usize] as usize;
        let colofs1 = COLUMNOFS[(x + 1) as usize] as usize;
        let mut dest_ptr = dest.add(colofs0);
        let mut dest2_ptr = dest.add(colofs1);

        let centery = CENTERY;
        let fracstep = DC_ISCALE as i32;
        let mut frac = DC_TEXTUREMID + (DC_YL - centery) * fracstep;

        let source = DC_SOURCE;
        let colormap = DC_COLORMAP;
        let translation = DC_TRANSLATION;

        let mut n = count;
        while n >= 0 {
            let row = (frac >> FRACBITS).max(0).min(127) as usize;
            let src_idx = *source.add(row) as usize;
            let trans_idx = *translation.add(src_idx) as usize;
            let pixel = *colormap.add(trans_idx);
            *dest_ptr = pixel;
            *dest2_ptr = pixel;

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            dest2_ptr = dest2_ptr.add(SCREENWIDTH as usize);
            frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw translated column - dispatches by detailshift.
pub fn transcolfunc() {
    if unsafe { DETAILSHIFT } == 0 {
        transcolfunc_high();
    } else {
        transcolfunc_low();
    }
}

/// Erase a horizontal run of pixels (for HUD text when view is reduced).
/// offset = byte offset into screen buffer; count = number of bytes.
/// Without background_buffer we clear to 0 (black).
pub fn r_video_erase(offset: usize, count: i32) {
    use crate::rendering::v_video::VIEWIMAGE;
    unsafe {
        if VIEWIMAGE.is_null() || count <= 0 {
            return;
        }
        let screen_size = (SCREENWIDTH * SCREENHEIGHT) as usize;
        if offset >= screen_size {
            return;
        }
        let count = count.min(screen_size as i32 - offset as i32) as usize;
        if count > 0 {
            std::ptr::write_bytes(VIEWIMAGE.add(offset), 0, count);
        }
    }
}

/// Draw a horizontal flat span (high detail). Uses ds_*.
fn spanfunc_high() {
    unsafe {
        if DS_SOURCE.is_null() || DS_COLORMAP.is_null() {
            return;
        }
        let dest = YLOOKUP[DS_Y as usize];
        if dest.is_null() {
            return;
        }

        // Pack position: x in top 16 bits, y in bottom 16 (Doom's optimization).
        let mut position: u32 = ((DS_XFRAC as u32) << 10) & 0xffff_0000
            | ((DS_YFRAC as u32) >> 6) & 0x0000_ffff;
        let step: u32 = ((DS_XSTEP as u32) << 10) & 0xffff_0000
            | ((DS_YSTEP as u32) >> 6) & 0x0000_ffff;

        let mut dest_ptr = dest.add(COLUMNOFS[DS_X1 as usize] as usize);
        let mut count = DS_X2 - DS_X1;

        while count >= 0 {
            let ytemp = (position >> 4) & 0x0fc0;
            let xtemp = position >> 26;
            let spot = ((xtemp | ytemp) & 0xfff) as usize; // 64x64 flat = 4096 bytes

            let idx = *DS_SOURCE.add(spot) as usize;
            *dest_ptr = *DS_COLORMAP.add(idx);

            position = position.wrapping_add(step);
            dest_ptr = dest_ptr.add(1);
            count -= 1;
        }
    }
}

/// Draw a horizontal flat span (low detail, blocky 2x). Draws each pixel twice.
fn spanfunc_low() {
    unsafe {
        if DS_SOURCE.is_null() || DS_COLORMAP.is_null() {
            return;
        }
        let dest = YLOOKUP[DS_Y as usize];
        if dest.is_null() {
            return;
        }

        let mut position: u32 = ((DS_XFRAC as u32) << 10) & 0xffff_0000
            | ((DS_YFRAC as u32) >> 6) & 0x0000_ffff;
        let step: u32 = ((DS_XSTEP as u32) << 10) & 0xffff_0000
            | ((DS_YSTEP as u32) >> 6) & 0x0000_ffff;

        let mut count = DS_X2 - DS_X1;
        let x1 = DS_X1 << 1;

        let mut dest_ptr = dest.add(COLUMNOFS[x1 as usize] as usize);

        while count >= 0 {
            let ytemp = (position >> 4) & 0x0fc0;
            let xtemp = position >> 26;
            let spot = ((xtemp | ytemp) & 0xfff) as usize;

            let idx = *DS_SOURCE.add(spot) as usize;
            let pixel = *DS_COLORMAP.add(idx);
            *dest_ptr = pixel;
            dest_ptr = dest_ptr.add(1);
            *dest_ptr = pixel;
            dest_ptr = dest_ptr.add(1);

            position = position.wrapping_add(step);
            count -= 1;
        }
    }
}

/// Draw horizontal flat span - dispatches by detailshift.
pub fn spanfunc() {
    if unsafe { DETAILSHIFT } == 0 {
        spanfunc_high();
    } else {
        spanfunc_low();
    }
}

// =============================================================================
// R_InitBuffer - view window setup (columnofs, ylookup, viewwindowx/y)
// =============================================================================

/// Initialize view buffer lookup tables for given view dimensions.
/// Called from r_execute_set_view_size when view size changes.
pub fn r_init_buffer(width: i32, height: i32) {
    unsafe {
        let viewwindowx = (SCREENWIDTH - width) >> 1;
        r_main::VIEWWINDOWX = viewwindowx;
        r_main::VIEWWINDOWY = if width == SCREENWIDTH {
            0
        } else {
            (SCREENHEIGHT - SBARHEIGHT - height) >> 1
        };

        for i in 0..width {
            COLUMNOFS[i as usize] = viewwindowx + i;
        }

        let viewwindowy = r_main::VIEWWINDOWY;
        let base = VIEWIMAGE;
        if !base.is_null() {
            for i in 0..height {
                YLOOKUP[i as usize] = base.add((i as usize + viewwindowy as usize) * SCREENWIDTH as usize);
            }
        }
    }
}
