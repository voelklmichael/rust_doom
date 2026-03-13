//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Status bar widget code.
// Original: st_lib.h + st_lib.c

use crate::doomtype::Boolean;
use crate::rendering::patch_t;

// =============================================================================
// Public API (from st_lib.h)
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

// =============================================================================
// Implementation (from st_lib.c) — stubs
// =============================================================================

pub fn stlib_init() {
    // Stub: would load STTMINUS lump via W_CacheLumpName
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

pub fn stlib_update_num(_n: &mut StNumber, _refresh: bool) {
    // Stub: would draw number via V_CopyRect, V_DrawPatch
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

pub fn stlib_update_percent(_per: &mut StPercent, _refresh: bool) {
    // Stub: would update percent display
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

pub fn stlib_update_mult_icon(_mi: &mut StMultIcon, _refresh: bool) {
    // Stub: would draw icon
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

pub fn stlib_update_bin_icon(_bi: &mut StBinIcon, _refresh: bool) {
    // Stub: would draw binary icon
}
