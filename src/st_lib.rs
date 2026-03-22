//! Status bar widget library (st_lib.h, st_lib.c)
//! Original: st_lib.h, st_lib.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct StNumberT {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub oldnum: i32,
    pub data: i32,
}

pub struct StPercentT {
    pub n: StNumberT,
}

pub struct StMulticonT {
    pub x: i32,
    pub y: i32,
    pub oldinum: i32,
    pub data: i32,
}

pub struct StBiniconT {
    pub x: i32,
    pub y: i32,
    pub oldval: Boolean,
    pub data: i32,
}

pub struct St_LibState;

impl St_LibState {
    /// Original: void STlib_init(void)
    pub fn st_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_initNum(st_number_t *n, ...)
    pub fn st_init_num(&self, _n: &mut StNumberT, _x: i32, _y: i32, _p: &(), _num: &mut i32, _on: &bool) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_updateNum(st_number_t *n, ...)
    pub fn st_update_num(&self, _n: &mut StNumberT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_initPercent(st_percent_t *p, ...)
    pub fn st_init_percent(&self, _p: &mut StPercentT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_updatePercent(st_percent_t *per, ...)
    pub fn st_update_percent(&self, _per: &mut StPercentT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_initMultIcon(st_multicon_t *mi, ...)
    pub fn st_init_mult_icon(&self, _mi: &mut StMulticonT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_updateMultIcon(st_multicon_t *mi, ...)
    pub fn st_update_mult_icon(&self, _mi: &mut StMulticonT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_initBinIcon(st_binicon_t *b, ...)
    pub fn st_init_bin_icon(&self, _b: &mut StBiniconT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void STlib_updateBinIcon(st_binicon_t *bi, ...)
    pub fn st_update_bin_icon(&self, _bi: &mut StBiniconT) {
        todo!("Basic stage-0 stub")
    }
}
