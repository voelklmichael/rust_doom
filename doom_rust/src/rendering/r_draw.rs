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
use crate::rendering::v_video::{COLUMNOFS, CENTERY, YLOOKUP};
use std::ptr;

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

/// pspriteiscale - used for sky column scaling. Set by r_things when view size changes.
pub static mut PSPRITEISCALE: u32 = 0;

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

/// Draw a wall column. Uses dc_yl, dc_yh, dc_x, dc_source, dc_colormap, dc_iscale, dc_texturemid.
pub fn colfunc() {
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

/// Draw a horizontal flat span. Uses ds_y, ds_x1, ds_x2, ds_xfrac, ds_yfrac, ds_xstep, ds_ystep, ds_colormap, ds_source.
pub fn spanfunc() {
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
