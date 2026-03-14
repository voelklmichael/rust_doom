//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Status bar widget code.
// Original: st_lib.h + st_lib.c

use crate::deh::deh_string;
use crate::doomtype::Boolean;
use crate::rendering::patch_t;
use crate::rendering::{v_copy_rect, v_draw_patch};
use crate::wad::w_cache_lump_name;
use crate::z_zone::PU_STATIC;

// =============================================================================
// Public API (from st_lib.h)
// =============================================================================

/// STTMINUS patch for negative numbers.
static mut STTMINUS: *mut patch_t = std::ptr::null_mut();

// =============================================================================
// Implementation (from st_lib.c)
// =============================================================================

/// Number widget for status bar.
#[repr(C)]
#[derive(Debug)]
pub struct StNumber {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub oldnum: i32,
    pub num: *mut i32,
    pub on: *mut Boolean,
    pub p: *mut *mut patch_t,
    pub data: i32,
}

/// Percent widget (contains number widget).
#[repr(C)]
#[derive(Debug)]
pub struct StPercent {
    pub n: StNumber,
    pub p: *mut patch_t,
}

/// Multiple icon widget.
#[repr(C)]
#[derive(Debug)]
pub struct StMultIcon {
    pub x: i32,
    pub y: i32,
    pub oldinum: i32,
    pub inum: *mut i32,
    pub on: *mut Boolean,
    pub p: *mut *mut patch_t,
    pub data: i32,
}

/// Binary icon widget.
#[repr(C)]
#[derive(Debug)]
pub struct StBinIcon {
    pub x: i32,
    pub y: i32,
    pub oldval: Boolean,
    pub val: *mut Boolean,
    pub on: *mut Boolean,
    pub p: *mut patch_t,
    pub data: i32,
}

pub fn stlib_init() {
    unsafe {
        STTMINUS = w_cache_lump_name(deh_string("STTMINUS"), PU_STATIC).as_ptr_mut() as *mut patch_t;
    }
}

pub fn stlib_init_num(
    n: &mut StNumber,
    x: i32,
    y: i32,
    pl: *mut *mut patch_t,
    num: *mut i32,
    on: *mut Boolean,
    width: i32,
) {
    n.x = x;
    n.y = y;
    n.oldnum = 0;
    n.width = width;
    n.num = num;
    n.on = on;
    n.p = pl;
}

/// Draw number. Requires backing_screen (st_backing_screen) and st_y (ST_Y).
pub fn stlib_update_num(
    n: &mut StNumber,
    refresh: bool,
    backing_screen: *const u8,
    st_y: i32,
) {
    unsafe {
        if *n.on {
            stlib_draw_num(n, refresh, backing_screen, st_y);
        }
    }
}

fn stlib_draw_num(
    n: &mut StNumber,
    _refresh: bool,
    backing_screen: *const u8,
    st_y: i32,
) {
    unsafe {
        let numdigits = n.width;
        let mut num = *n.num;
        let p0 = *n.p;
        if p0.is_null() {
            return;
        }
        let w = (*p0).width as i32;
        let h = (*p0).height as i32;
        n.oldnum = *n.num;
        let neg = num < 0;
        if neg {
            if numdigits == 2 && num < -9 {
                num = -9;
            } else if numdigits == 3 && num < -99 {
                num = -99;
            }
            num = -num;
        }
        let mut x = n.x - numdigits * w;
        if n.y - st_y < 0 {
            return;
        }
        if !backing_screen.is_null() {
            v_copy_rect(x, n.y - st_y, backing_screen, w * numdigits, h, x, n.y);
        }
        if num == 1994 {
            return;
        }
        x = n.x;
        if num == 0 {
            v_draw_patch(x - w, n.y, *n.p.add(0));
        }
        let mut num = num;
        let mut numdigits = numdigits;
        while num != 0 && numdigits > 0 {
            x -= w;
            let digit = (num % 10) as usize;
            v_draw_patch(x, n.y, *n.p.add(digit));
            num /= 10;
            numdigits -= 1;
        }
        if neg {
            v_draw_patch(x - 8, n.y, STTMINUS);
        }
    }
}

pub fn stlib_init_percent(
    p: &mut StPercent,
    x: i32,
    y: i32,
    pl: *mut *mut patch_t,
    num: *mut i32,
    on: *mut Boolean,
    percent: *mut patch_t,
) {
    stlib_init_num(&mut p.n, x, y, pl, num, on, 3);
    p.p = percent;
}

/// Update percent. Requires backing_screen and st_y.
pub fn stlib_update_percent(
    per: &mut StPercent,
    refresh: bool,
    backing_screen: *const u8,
    st_y: i32,
) {
    unsafe {
        if refresh && *per.n.on {
            v_draw_patch(per.n.x, per.n.y, per.p);
        }
        stlib_update_num(&mut per.n, refresh, backing_screen, st_y);
    }
}

pub fn stlib_init_mult_icon(
    mi: &mut StMultIcon,
    x: i32,
    y: i32,
    il: *mut *mut patch_t,
    inum: *mut i32,
    on: *mut Boolean,
) {
    mi.x = x;
    mi.y = y;
    mi.oldinum = -1;
    mi.inum = inum;
    mi.on = on;
    mi.p = il;
}

/// Update mult icon. Requires backing_screen and st_y.
pub fn stlib_update_mult_icon(
    mi: &mut StMultIcon,
    refresh: bool,
    backing_screen: *const u8,
    st_y: i32,
) {
    unsafe {
        if *mi.on && (mi.oldinum != *mi.inum || refresh) && *mi.inum != -1 {
            if mi.oldinum != -1 {
                let p_old = *mi.p.add(mi.oldinum as usize);
                if !p_old.is_null() {
                    let x = mi.x - (*p_old).leftoffset as i32;
                    let y = mi.y - (*p_old).topoffset as i32;
                    let w = (*p_old).width as i32;
                    let h = (*p_old).height as i32;
                    if y - st_y >= 0 && !backing_screen.is_null() {
                        v_copy_rect(x, y - st_y, backing_screen, w, h, x, y);
                    }
                }
            }
            let p_cur = *mi.p.add(*mi.inum as usize);
            if !p_cur.is_null() {
                v_draw_patch(mi.x, mi.y, p_cur);
            }
            mi.oldinum = *mi.inum;
        }
    }
}

pub fn stlib_init_bin_icon(
    b: &mut StBinIcon,
    x: i32,
    y: i32,
    i: *mut patch_t,
    val: *mut Boolean,
    on: *mut Boolean,
) {
    b.x = x;
    b.y = y;
    b.oldval = false;
    b.val = val;
    b.on = on;
    b.p = i;
}

/// Update binary icon. Requires backing_screen and st_y.
pub fn stlib_update_bin_icon(
    bi: &mut StBinIcon,
    refresh: bool,
    backing_screen: *const u8,
    st_y: i32,
) {
    unsafe {
        if *bi.on && (bi.oldval != *bi.val || refresh) {
            let p = bi.p;
            if !p.is_null() {
                let x = bi.x - (*p).leftoffset as i32;
                let y = bi.y - (*p).topoffset as i32;
                let w = (*p).width as i32;
                let h = (*p).height as i32;
                if y - st_y >= 0 {
                    if *bi.val {
                        v_draw_patch(bi.x, bi.y, p);
                    } else if !backing_screen.is_null() {
                        v_copy_rect(x, y - st_y, backing_screen, w, h, x, y);
                    }
                }
            }
            bi.oldval = *bi.val;
        }
    }
}
