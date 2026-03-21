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
    // Original: x
    pub x: i32,
    // Original: y
    pub y: i32,
    // Original: width
    pub width: i32,
    // Original: oldnum
    pub oldnum: i32,
    // Original: num
    pub num: *mut i32,
    // Original: on
    pub on: *mut Boolean,
    // Original: p
    pub p: *mut *mut PatchT,
    // Original: data
    pub data: i32,
}

/// Original: st_percent_t
#[repr(C)]
pub struct StPercentT {
    // Original: n
    pub n: StNumberT,
    // Original: p
    pub p: *mut PatchT,
}

/// Original: st_multicon_t
#[repr(C)]
pub struct StMulticonT {
    // Original: x
    pub x: i32,
    // Original: y
    pub y: i32,
    // Original: oldinum
    pub oldinum: i32,
    // Original: inum
    pub inum: *mut i32,
    // Original: on
    pub on: *mut Boolean,
    // Original: p
    pub p: *mut *mut PatchT,
    // Original: data
    pub data: i32,
}

/// Original: st_binicon_t
#[repr(C)]
pub struct StBiniconT {
    // Original: x
    pub x: i32,
    // Original: y
    pub y: i32,
    // Original: oldval
    pub oldval: Boolean,
    // Original: val
    pub val: *mut Boolean,
    // Original: on
    pub on: *mut Boolean,
    // Original: p
    pub p: *mut PatchT,
    // Original: data
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

    // Original: STlib_init
    pub fn st_lib_init(&self) {
        todo!("STlib_init");
    }

    // Original: STlib_initNum
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

    // Original: STlib_updateNum
    pub fn st_lib_update_num(&self, _n: *mut StNumberT, _refresh: Boolean) {
        todo!("STlib_updateNum");
    }

    // Original: STlib_initPercent
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

    // Original: STlib_updatePercent
    pub fn st_lib_update_percent(&self, _per: *mut StPercentT, _refresh: i32) {
        todo!("STlib_updatePercent");
    }

    // Original: STlib_initMultIcon
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

    // Original: STlib_updateMultIcon
    pub fn st_lib_update_mult_icon(&self, _mi: *mut StMulticonT, _refresh: Boolean) {
        todo!("STlib_updateMultIcon");
    }

    // Original: STlib_initBinIcon
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

    // Original: STlib_updateBinIcon
    pub fn st_lib_update_bin_icon(&self, _bi: *mut StBiniconT, _refresh: Boolean) {
        todo!("STlib_updateBinIcon");
    }
}
