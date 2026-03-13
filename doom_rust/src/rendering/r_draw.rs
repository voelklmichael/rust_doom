//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Low-level column and span blitting. Stub implementation.
//
// Original: r_draw.h + r_draw.c

use crate::m_fixed::Fixed;
use std::ptr;

// =============================================================================
// Draw state - ds_* for spans, dc_* for columns
// =============================================================================

pub static mut DS_XFRAC: Fixed = 0;
pub static mut DS_YFRAC: Fixed = 0;
pub static mut DS_COLORMAP: *mut u8 = ptr::null_mut();
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
// Public API - draw functions (stubs)
// =============================================================================

/// Draw a horizontal span. Stub: no-op.
pub fn spanfunc() {}

/// Draw a column. Stub: no-op.
pub fn colfunc() {}
