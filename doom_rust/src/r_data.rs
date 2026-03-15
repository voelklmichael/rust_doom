//! Rust translation of doomgeneric/r_data.h
//! Refresh module, data I/O, caching, retrieval of graphics by name.

use crate::doomtype::*;

/// R_GetColumn - retrieve column data for span blitting
pub fn r_get_column(tex: i32, col: i32) -> *mut byte {
    todo!("original: R_GetColumn")
}

pub fn r_init_data() {
    todo!("original: R_InitData")
}

pub fn r_precache_level() {
    todo!("original: R_PrecacheLevel")
}

pub fn r_flat_num_for_name(name: *mut i8) -> i32 {
    todo!("original: R_FlatNumForName")
}

pub fn r_texture_num_for_name(name: *mut i8) -> i32 {
    todo!("original: R_TextureNumForName")
}

pub fn r_check_texture_num_for_name(name: *mut i8) -> i32 {
    todo!("original: R_CheckTextureNumForName")
}
