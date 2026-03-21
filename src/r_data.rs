// doomgeneric/r_data.h

pub use crate::doomtype::*;
pub use crate::r_defs::*;
pub use crate::r_state::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_DataState {
    pub _placeholder: RefCell<()>,
}

impl R_DataState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: R_GetColumn
    pub fn r_get_column(&self, _tex: i32, _col: i32) -> *mut Byte {
        todo!("R_GetColumn");
    }

    // Original: R_InitData
    pub fn r_init_data(&self) {
        todo!("R_InitData");
    }

    // Original: R_PrecacheLevel
    pub fn r_precache_level(&self) {
        todo!("R_PrecacheLevel");
    }

    // Original: R_FlatNumForName
    pub fn r_flat_num_for_name(&self, _name: *mut std::ffi::c_char) -> i32 {
        todo!("R_FlatNumForName");
    }

    // Original: R_TextureNumForName
    pub fn r_texture_num_for_name(&self, _name: *mut std::ffi::c_char) -> i32 {
        todo!("R_TextureNumForName");
    }

    // Original: R_CheckTextureNumForName
    pub fn r_check_texture_num_for_name(&self, _name: *mut std::ffi::c_char) -> i32 {
        todo!("R_CheckTextureNumForName");
    }
}
