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
use crate::rendering::r_main;
use crate::rendering::state;
use crate::rendering::v_video;
use std::ptr;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// RDrawState - thread-safe via OnceLock + Mutex
// =============================================================================

static R_DRAW_STATE: OnceLock<Mutex<RDrawState>> = OnceLock::new();

/// Safety: Raw pointers in RDrawState point to zone-allocated data that outlives the state.
unsafe impl Send for RDrawState {}

pub struct RDrawState {
    pub fuzzpos: usize,
    pub dc_translation: *const u8,
    pub ds_xfrac: Fixed,
    pub ds_yfrac: Fixed,
    pub ds_colormap: *mut u8,
    pub ds_source: *const u8,
    pub ds_y: i32,
    pub ds_x1: i32,
    pub ds_x2: i32,
    pub ds_xstep: Fixed,
    pub ds_ystep: Fixed,
    pub dc_yl: i32,
    pub dc_yh: i32,
    pub dc_x: i32,
    pub dc_source: *const u8,
    pub dc_colormap: *mut u8,
    pub dc_iscale: u32,
    pub dc_texturemid: Fixed,
    pub pspriteiscale: u32,
    pub pspritescale: Fixed,
    pub yslope: [Fixed; 200],
    pub distscale: [Fixed; 320],
    pub screenheightarray: [i16; 320],
    pub negonearray: [i16; 320],
}

impl Default for RDrawState {
    fn default() -> Self {
        Self {
            fuzzpos: 0,
            dc_translation: ptr::null(),
            ds_xfrac: 0,
            ds_yfrac: 0,
            ds_colormap: ptr::null_mut(),
            ds_source: ptr::null(),
            ds_y: 0,
            ds_x1: 0,
            ds_x2: 0,
            ds_xstep: 0,
            ds_ystep: 0,
            dc_yl: 0,
            dc_yh: 0,
            dc_x: 0,
            dc_source: ptr::null(),
            dc_colormap: ptr::null_mut(),
            dc_iscale: 0,
            dc_texturemid: 0,
            pspriteiscale: 0,
            pspritescale: 0,
            yslope: [0; 200],
            distscale: [0; 320],
            screenheightarray: [0; 320],
            negonearray: [-1; 320],
        }
    }
}

fn get_r_draw_state() -> &'static Mutex<RDrawState> {
    R_DRAW_STATE.get_or_init(|| Mutex::new(RDrawState::default()))
}

/// Access RDrawState.
pub fn with_r_draw_state<F, R>(f: F) -> R
where
    F: FnOnce(&RDrawState) -> R,
{
    let guard = get_r_draw_state().lock().unwrap();
    f(&guard)
}

/// Mutably access RDrawState.
pub fn with_r_draw_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut RDrawState) -> R,
{
    let mut guard = get_r_draw_state().lock().unwrap();
    f(&mut guard)
}

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

// =============================================================================
// Public API - draw functions
// =============================================================================

/// Draw a wall column (high detail). Uses dc_yl, dc_yh, dc_x, dc_source, dc_colormap, dc_iscale, dc_texturemid.
fn colfunc_high(rd: &RDrawState, centery: i32, ylookup: &[*mut u8; 200], columnofs: &[i32; 320]) {
    unsafe {
        let count = rd.dc_yh - rd.dc_yl;
        if count < 0 {
            return;
        }
        if rd.dc_source.is_null() || rd.dc_colormap.is_null() {
            return;
        }
        let dest = ylookup[rd.dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs = columnofs[rd.dc_x as usize] as usize;
        let mut dest_ptr = dest.add(colofs);

        let fracstep = rd.dc_iscale as i32;
        let mut frac = rd.dc_texturemid + (rd.dc_yl - centery) * fracstep;

        let source = rd.dc_source;
        let colormap = rd.dc_colormap;

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
fn colfunc_low(rd: &RDrawState, centery: i32, ylookup: &[*mut u8; 200], columnofs: &[i32; 320]) {
    unsafe {
        let count = rd.dc_yh - rd.dc_yl;
        if count < 0 {
            return;
        }
        if rd.dc_source.is_null() || rd.dc_colormap.is_null() {
            return;
        }
        let x = rd.dc_x << 1;
        if x + 1 >= SCREENWIDTH {
            return;
        }
        let dest = ylookup[rd.dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs0 = columnofs[x as usize] as usize;
        let colofs1 = columnofs[(x + 1) as usize] as usize;
        let mut dest_ptr = dest.add(colofs0);
        let mut dest2_ptr = dest.add(colofs1);

        let fracstep = rd.dc_iscale as i32;
        let mut frac = rd.dc_texturemid + (rd.dc_yl - centery) * fracstep;

        let source = rd.dc_source;
        let colormap = rd.dc_colormap;

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
    let (rd, centery, ylookup, columnofs) = with_colfunc_params();
    if r_main::with_r_main_state(|s| s.detailshift) == 0 {
        colfunc_high(&rd, centery, &ylookup, &columnofs);
    } else {
        colfunc_low(&rd, centery, &ylookup, &columnofs);
    }
}

fn with_colfunc_params() -> (RDrawState, i32, [*mut u8; 200], [i32; 320]) {
    let rd = with_r_draw_state(|r| RDrawState {
        fuzzpos: r.fuzzpos,
        dc_translation: r.dc_translation,
        ds_xfrac: r.ds_xfrac,
        ds_yfrac: r.ds_yfrac,
        ds_colormap: r.ds_colormap,
        ds_source: r.ds_source,
        ds_y: r.ds_y,
        ds_x1: r.ds_x1,
        ds_x2: r.ds_x2,
        ds_xstep: r.ds_xstep,
        ds_ystep: r.ds_ystep,
        dc_yl: r.dc_yl,
        dc_yh: r.dc_yh,
        dc_x: r.dc_x,
        dc_source: r.dc_source,
        dc_colormap: r.dc_colormap,
        dc_iscale: r.dc_iscale,
        dc_texturemid: r.dc_texturemid,
        pspriteiscale: r.pspriteiscale,
        pspritescale: r.pspritescale,
        yslope: r.yslope,
        distscale: r.distscale,
        screenheightarray: r.screenheightarray,
        negonearray: r.negonearray,
    });
    let centery = r_main::with_r_main_state(|s| s.centery);
    let (ylookup, columnofs) = v_video::with_v_video_state(|v| (v.ylookup, v.columnofs));
    (rd, centery, ylookup, columnofs)
}

/// Draw fuzz column (spectre/invisibility effect). High detail.
fn fuzzcolfunc_high(
    mut dc_yl: i32,
    mut dc_yh: i32,
    dc_x: i32,
    dc_source: *const u8,
    dc_colormap: *mut u8,
    dc_iscale: u32,
    dc_texturemid: Fixed,
    centery: i32,
    ylookup: &[*mut u8; 200],
    columnofs: &[i32; 320],
    fuzzpos: &mut usize,
) {
    unsafe {
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
        if dc_source.is_null() || dc_colormap.is_null() {
            return;
        }
        let colormaps = state::with_state(|s| s.colormaps);
        if colormaps.is_null() {
            return;
        }
        let fuzz_cmap = colormaps.add(6 * 256);
        let dest = ylookup[dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs = columnofs[dc_x as usize] as usize;
        let mut dest_ptr = dest.add(colofs);

        let fracstep = dc_iscale as i32;
        let mut _frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut n = count;
        while n >= 0 {
            let fuzz_off = FUZZOFFSET[*fuzzpos];
            *fuzzpos += 1;
            if *fuzzpos >= FUZZTABLE {
                *fuzzpos = 0;
            }
            let src_idx = dest_ptr.offset(fuzz_off as isize).read() as usize;
            *dest_ptr = *fuzz_cmap.add(src_idx);

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            _frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw fuzz column. Low detail.
fn fuzzcolfunc_low(
    mut dc_yl: i32,
    mut dc_yh: i32,
    dc_x: i32,
    dc_source: *const u8,
    dc_colormap: *mut u8,
    dc_iscale: u32,
    dc_texturemid: Fixed,
    centery: i32,
    ylookup: &[*mut u8; 200],
    columnofs: &[i32; 320],
    fuzzpos: &mut usize,
) {
    unsafe {
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
        let x = dc_x << 1;
        if x + 1 >= SCREENWIDTH {
            return;
        }
        if dc_source.is_null() || dc_colormap.is_null() {
            return;
        }
        let colormaps = state::with_state(|s| s.colormaps);
        if colormaps.is_null() {
            return;
        }
        let fuzz_cmap = colormaps.add(6 * 256);
        let dest = ylookup[dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs0 = columnofs[x as usize] as usize;
        let colofs1 = columnofs[(x + 1) as usize] as usize;
        let mut dest_ptr = dest.add(colofs0);
        let mut dest2_ptr = dest.add(colofs1);

        let fracstep = dc_iscale as i32;
        let mut _frac = dc_texturemid + (dc_yl - centery) * fracstep;

        let mut n = count;
        while n >= 0 {
            let fuzz_off = FUZZOFFSET[*fuzzpos];
            *fuzzpos += 1;
            if *fuzzpos >= FUZZTABLE {
                *fuzzpos = 0;
            }
            let src_idx = dest_ptr.offset(fuzz_off as isize).read() as usize;
            let pixel = *fuzz_cmap.add(src_idx);
            *dest_ptr = pixel;
            let src_idx2 = dest2_ptr.offset(fuzz_off as isize).read() as usize;
            *dest2_ptr = *fuzz_cmap.add(src_idx2);

            dest_ptr = dest_ptr.add(SCREENWIDTH as usize);
            dest2_ptr = dest2_ptr.add(SCREENWIDTH as usize);
            _frac += fracstep;
            n -= 1;
        }
    }
}

/// Draw fuzz column - dispatches by detailshift.
pub fn fuzzcolfunc() {
    v_video::with_v_video_state(|vv| {
        r_main::with_r_main_state(|rm| {
            with_r_draw_state_mut(|rd| {
                let centery = rm.centery;
                let dc_yl = rd.dc_yl;
                let dc_yh = rd.dc_yh;
                let dc_x = rd.dc_x;
                let dc_source = rd.dc_source;
                let dc_colormap = rd.dc_colormap;
                let dc_iscale = rd.dc_iscale;
                let dc_texturemid = rd.dc_texturemid;
                if rm.detailshift == 0 {
                    fuzzcolfunc_high(
                        dc_yl, dc_yh, dc_x, dc_source, dc_colormap, dc_iscale, dc_texturemid,
                        centery, &vv.ylookup, &vv.columnofs, &mut rd.fuzzpos,
                    );
                } else {
                    fuzzcolfunc_low(
                        dc_yl, dc_yh, dc_x, dc_source, dc_colormap, dc_iscale, dc_texturemid,
                        centery, &vv.ylookup, &vv.columnofs, &mut rd.fuzzpos,
                    );
                }
            })
        })
    })
}

/// Draw translated column (player colors). High detail.
fn transcolfunc_high(rd: &RDrawState, centery: i32, ylookup: &[*mut u8; 200], columnofs: &[i32; 320]) {
    unsafe {
        let count = rd.dc_yh - rd.dc_yl;
        if count < 0 {
            return;
        }
        if rd.dc_source.is_null() || rd.dc_colormap.is_null() || rd.dc_translation.is_null() {
            return;
        }
        let dest = ylookup[rd.dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs = columnofs[rd.dc_x as usize] as usize;
        let mut dest_ptr = dest.add(colofs);

        let fracstep = rd.dc_iscale as i32;
        let mut frac = rd.dc_texturemid + (rd.dc_yl - centery) * fracstep;

        let source = rd.dc_source;
        let colormap = rd.dc_colormap;
        let translation = rd.dc_translation;

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
fn transcolfunc_low(rd: &RDrawState, centery: i32, ylookup: &[*mut u8; 200], columnofs: &[i32; 320]) {
    unsafe {
        let count = rd.dc_yh - rd.dc_yl;
        if count < 0 {
            return;
        }
        if rd.dc_source.is_null() || rd.dc_colormap.is_null() || rd.dc_translation.is_null() {
            return;
        }
        let x = rd.dc_x << 1;
        if x + 1 >= SCREENWIDTH {
            return;
        }
        let dest = ylookup[rd.dc_yl as usize];
        if dest.is_null() {
            return;
        }
        let colofs0 = columnofs[x as usize] as usize;
        let colofs1 = columnofs[(x + 1) as usize] as usize;
        let mut dest_ptr = dest.add(colofs0);
        let mut dest2_ptr = dest.add(colofs1);

        let fracstep = rd.dc_iscale as i32;
        let mut frac = rd.dc_texturemid + (rd.dc_yl - centery) * fracstep;

        let source = rd.dc_source;
        let colormap = rd.dc_colormap;
        let translation = rd.dc_translation;

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
    v_video::with_v_video_state(|vv| {
        r_main::with_r_main_state(|rm| {
            with_r_draw_state(|rd| {
                let centery = rm.centery;
                if rm.detailshift == 0 {
                    transcolfunc_high(rd, centery, &vv.ylookup, &vv.columnofs);
                } else {
                    transcolfunc_low(rd, centery, &vv.ylookup, &vv.columnofs);
                }
            })
        })
    })
}

/// Erase a horizontal run of pixels (for HUD text when view is reduced).
/// offset = byte offset into screen buffer; count = number of bytes.
/// Without background_buffer we clear to 0 (black).
pub fn r_video_erase(offset: usize, count: i32) {
    v_video::with_v_video_state(|vv| {
        unsafe {
            if vv.viewimage.is_null() || count <= 0 {
                return;
            }
            let screen_size = (SCREENWIDTH * SCREENHEIGHT) as usize;
            if offset >= screen_size {
                return;
            }
            let count = count.min(screen_size as i32 - offset as i32) as usize;
            if count > 0 {
                std::ptr::write_bytes(vv.viewimage.add(offset), 0, count);
            }
        }
    });
}

/// Draw a horizontal flat span (high detail). Uses ds_*.
fn spanfunc_high(rd: &RDrawState, ylookup: &[*mut u8; 200], columnofs: &[i32; 320]) {
    unsafe {
        if rd.ds_source.is_null() || rd.ds_colormap.is_null() {
            return;
        }
        let dest = ylookup[rd.ds_y as usize];
        if dest.is_null() {
            return;
        }

        // Pack position: x in top 16 bits, y in bottom 16 (Doom's optimization).
        let mut position: u32 = ((rd.ds_xfrac as u32) << 10) & 0xffff_0000
            | ((rd.ds_yfrac as u32) >> 6) & 0x0000_ffff;
        let step: u32 = ((rd.ds_xstep as u32) << 10) & 0xffff_0000
            | ((rd.ds_ystep as u32) >> 6) & 0x0000_ffff;

        let mut count = rd.ds_x2 - rd.ds_x1;
        let mut dest_ptr = dest.add(columnofs[rd.ds_x1 as usize] as usize);

        while count >= 0 {
            let ytemp = (position >> 4) & 0x0fc0;
            let xtemp = position >> 26;
            let spot = ((xtemp | ytemp) & 0xfff) as usize; // 64x64 flat = 4096 bytes

            let idx = *rd.ds_source.add(spot) as usize;
            *dest_ptr = *rd.ds_colormap.add(idx);

            position = position.wrapping_add(step);
            dest_ptr = dest_ptr.add(1);
            count -= 1;
        }
    }
}

/// Draw a horizontal flat span (low detail, blocky 2x). Draws each pixel twice.
fn spanfunc_low(rd: &RDrawState, ylookup: &[*mut u8; 200], columnofs: &[i32; 320]) {
    unsafe {
        if rd.ds_source.is_null() || rd.ds_colormap.is_null() {
            return;
        }
        let dest = ylookup[rd.ds_y as usize];
        if dest.is_null() {
            return;
        }

        let mut position: u32 = ((rd.ds_xfrac as u32) << 10) & 0xffff_0000
            | ((rd.ds_yfrac as u32) >> 6) & 0x0000_ffff;
        let step: u32 = ((rd.ds_xstep as u32) << 10) & 0xffff_0000
            | ((rd.ds_ystep as u32) >> 6) & 0x0000_ffff;

        let mut count = rd.ds_x2 - rd.ds_x1;
        let x1 = rd.ds_x1 << 1;

        let mut dest_ptr = dest.add(columnofs[x1 as usize] as usize);

        while count >= 0 {
            let ytemp = (position >> 4) & 0x0fc0;
            let xtemp = position >> 26;
            let spot = ((xtemp | ytemp) & 0xfff) as usize;

            let idx = *rd.ds_source.add(spot) as usize;
            let pixel = *rd.ds_colormap.add(idx);
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
    v_video::with_v_video_state(|vv| {
        r_main::with_r_main_state(|rm| {
            with_r_draw_state(|rd| {
                if rm.detailshift == 0 {
                    spanfunc_high(rd, &vv.ylookup, &vv.columnofs);
                } else {
                    spanfunc_low(rd, &vv.ylookup, &vv.columnofs);
                }
            })
        })
    })
}

// =============================================================================
// R_InitBuffer - view window setup (columnofs, ylookup, viewwindowx/y)
// =============================================================================

/// Initialize view buffer lookup tables for given view dimensions.
/// Called from r_execute_set_view_size when view size changes.
pub fn r_init_buffer(width: i32, height: i32) {
    let viewwindowx = (SCREENWIDTH - width) >> 1;
    let viewwindowy = if width == SCREENWIDTH {
        0
    } else {
        (SCREENHEIGHT - SBARHEIGHT - height) >> 1
    };

    r_main::with_r_main_state_mut(|rm| {
        rm.viewwindowx = viewwindowx;
        rm.viewwindowy = viewwindowy;
    });

    v_video::with_v_video_state_mut(|vv| {
        for i in 0..width {
            vv.columnofs[i as usize] = viewwindowx + i;
        }
        let base = vv.viewimage;
        if !base.is_null() {
            for i in 0..height {
                vv.ylookup[i as usize] =
                    unsafe { base.add((i as usize + viewwindowy as usize) * SCREENWIDTH as usize) };
            }
        }
    });
}
