// doomgeneric/st_lib.h

pub use crate::deh_main::*;
pub use crate::doomdef::*;
pub use crate::doomtype::*;
pub use crate::i_swap::*;
pub use crate::i_system::*;
pub use crate::r_defs::*;
pub use crate::r_local::*;
// st_stuff omitted: circular dependency (st_stuff re-exports st_lib)
pub use crate::v_video::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

/// Original: st_number_t
#[repr(C)]
pub struct StNumberT {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub oldnum: i32,
    pub num: *mut i32,
    pub on: *mut Boolean,
    pub p: *mut *mut PatchT,
    pub data: i32,
}

/// Original: st_percent_t
#[repr(C)]
pub struct StPercentT {
    pub n: StNumberT,
    pub p: *mut PatchT,
}

/// Original: st_multicon_t
#[repr(C)]
pub struct StMulticonT {
    pub x: i32,
    pub y: i32,
    pub oldinum: i32,
    pub inum: *mut i32,
    pub on: *mut Boolean,
    pub p: *mut *mut PatchT,
    pub data: i32,
}

/// Original: st_binicon_t
#[repr(C)]
pub struct StBiniconT {
    pub x: i32,
    pub y: i32,
    pub oldval: Boolean,
    pub val: *mut Boolean,
    pub on: *mut Boolean,
    pub p: *mut PatchT,
    pub data: i32,
}

#[allow(non_camel_case_types)]
pub struct St_LibState {
    pub _placeholder: RefCell<()>,
}

impl St_LibState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn st_lib_init(&self) {
        todo!("STlib_init");
    }

    pub fn st_lib_init_num(
        &self,
        _n: *mut StNumberT,
        _x: i32,
        _y: i32,
        _pl: *mut *mut PatchT,
        _num: *mut i32,
        _on: *mut Boolean,
        _width: i32,
    ) {
        todo!("STlib_initNum");
    }

    pub fn st_lib_update_num(&self, _n: *mut StNumberT, _refresh: Boolean) {
        todo!("STlib_updateNum");
    }

    pub fn st_lib_init_percent(
        &self,
        _p: *mut StPercentT,
        _x: i32,
        _y: i32,
        _pl: *mut *mut PatchT,
        _num: *mut i32,
        _on: *mut Boolean,
        _percent: *mut PatchT,
    ) {
        todo!("STlib_initPercent");
    }

    pub fn st_lib_update_percent(&self, _per: *mut StPercentT, _refresh: i32) {
        todo!("STlib_updatePercent");
    }

    pub fn st_lib_init_mult_icon(
        &self,
        _mi: *mut StMulticonT,
        _x: i32,
        _y: i32,
        _il: *mut *mut PatchT,
        _inum: *mut i32,
        _on: *mut Boolean,
    ) {
        todo!("STlib_initMultIcon");
    }

    pub fn st_lib_update_mult_icon(&self, _mi: *mut StMulticonT, _refresh: Boolean) {
        todo!("STlib_updateMultIcon");
    }

    pub fn st_lib_init_bin_icon(
        &self,
        _b: *mut StBiniconT,
        _x: i32,
        _y: i32,
        _i: *mut PatchT,
        _val: *mut Boolean,
        _on: *mut Boolean,
    ) {
        todo!("STlib_initBinIcon");
    }

    pub fn st_lib_update_bin_icon(&self, _bi: *mut StBiniconT, _refresh: Boolean) {
        todo!("STlib_updateBinIcon");
    }
}
