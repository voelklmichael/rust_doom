//! Rust translation of doomgeneric/st_lib.h
//! The status bar widget code.

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::*;
use crate::v_patch::*;

/// st_number_t
#[repr(C)]
/// C typedef: st_number_t
pub struct StNumberT {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub oldnum: i32,
    pub num: *mut i32,
    pub on: *mut boolean,
    pub p: *mut *mut PatchT,
    pub data: i32,
}

/// st_percent_t
#[repr(C)]
/// C typedef: st_percent_t
pub struct StPercentT {
    pub n: StNumberT,
    pub p: *mut PatchT,
}

/// st_multicon_t
#[repr(C)]
/// C typedef: st_multicon_t
pub struct StMulticonT {
    pub x: i32,
    pub y: i32,
    pub oldinum: i32,
    pub inum: *mut i32,
    pub on: *mut boolean,
    pub p: *mut *mut PatchT,
    pub data: i32,
}

/// st_binicon_t
#[repr(C)]
/// C typedef: st_binicon_t
pub struct StBiniconT {
    pub x: i32,
    pub y: i32,
    pub oldval: boolean,
    pub val: *mut boolean,
    pub on: *mut boolean,
    pub p: *mut PatchT,
    pub data: i32,
}

/// C function: STlib_init
pub fn stlib_init() {
    todo!("original: STlib_init")
}

/// C function: STlib_initNum
pub fn stlib_init_num(
    n: &mut StNumberT,
    x: i32,
    y: i32,
    pl: &mut [Arc<Mutex<PatchT>>],
    num: &mut i32,
    on: &mut boolean,
    width: i32,
) {
    todo!("original: STlib_initNum")
}

/// C function: STlib_updateNum
pub fn stlib_update_num(n: &mut StNumberT, refresh: boolean) {
    todo!("original: STlib_updateNum")
}

/// C function: STlib_initPercent
pub fn stlib_init_percent(
    p: &mut StPercentT,
    x: i32,
    y: i32,
    pl: &mut [Arc<Mutex<PatchT>>],
    num: &mut i32,
    on: &mut boolean,
    percent: &mut PatchT,
) {
    todo!("original: STlib_initPercent")
}

/// C function: STlib_updatePercent
pub fn stlib_update_percent(per: &mut StPercentT, refresh: i32) {
    todo!("original: STlib_updatePercent")
}

/// C function: STlib_initMultIcon
pub fn stlib_init_mult_icon(
    mi: &mut StMulticonT,
    x: i32,
    y: i32,
    il: &mut [Arc<Mutex<PatchT>>],
    inum: &mut i32,
    on: &mut boolean,
) {
    todo!("original: STlib_initMultIcon")
}

/// C function: STlib_updateMultIcon
pub fn stlib_update_mult_icon(mi: &mut StMulticonT, refresh: boolean) {
    todo!("original: STlib_updateMultIcon")
}

/// C function: STlib_initBinIcon
pub fn stlib_init_bin_icon(
    b: &mut StBiniconT,
    x: i32,
    y: i32,
    i: &mut PatchT,
    val: &mut boolean,
    on: &mut boolean,
) {
    todo!("original: STlib_initBinIcon")
}

/// C function: STlib_updateBinIcon
pub fn stlib_update_bin_icon(bi: &mut StBiniconT, refresh: boolean) {
    todo!("original: STlib_updateBinIcon")
}
