//! Rust translation of doomgeneric/w_wad.h
//! WAD I/O functions.

use crate::doomtype::*;
use crate::d_mode::*;
use crate::w_file::*;
use std::sync::{Arc, Mutex};

/// lumpinfo_t
#[repr(C)]
/// C typedef: lumpinfo_t
pub struct LumpinfoT {
    pub name: [i8; 8],
    pub wad_file: *mut WadFileT,
    pub position: i32,
    pub size: i32,
    pub cache: *mut std::ffi::c_void,
    pub next: *mut LumpinfoT,
}

pub static mut lumpinfo: *mut LumpinfoT = std::ptr::null_mut();
pub static mut numlumps: u32 = 0;

/// C function: W_AddFile
pub fn w_add_file(filename: &str) -> *mut WadFileT {
    todo!("original: W_AddFile")
}

/// C function: W_CheckNumForName
pub fn w_check_num_for_name(name: &str) -> i32 {
    todo!("original: W_CheckNumForName")
}

/// C function: W_GetNumForName
pub fn w_get_num_for_name(name: &str) -> i32 {
    todo!("original: W_GetNumForName")
}

/// C function: W_LumpLength
pub fn w_lump_length(lump: u32) -> i32 {
    todo!("original: W_LumpLength")
}

/// C function: W_ReadLump
pub fn w_read_lump(lump: u32, dest: &mut [u8]) {
    todo!("original: W_ReadLump")
}

/// C function: W_CacheLumpNum
pub fn w_cache_lump_num(lump: i32, tag: i32) -> Arc<Mutex<Vec<u8>>> {
    todo!("original: W_CacheLumpNum")
}

/// C function: W_CacheLumpName
pub fn w_cache_lump_name(name: &str, tag: i32) -> Arc<Mutex<Vec<u8>>> {
    todo!("original: W_CacheLumpName")
}

/// C function: W_GenerateHashTable
pub fn w_generate_hash_table() {
    todo!("original: W_GenerateHashTable")
}

/// C function: W_LumpNameHash
pub fn w_lump_name_hash(s: *const i8) -> u32 {
    todo!("original: W_LumpNameHash")
}

/// C function: W_ReleaseLumpNum
pub fn w_release_lump_num(lump: i32) {
    todo!("original: W_ReleaseLumpNum")
}

/// C function: W_ReleaseLumpName
pub fn w_release_lump_name(name: &str) {
    todo!("original: W_ReleaseLumpName")
}

/// C function: W_CheckCorrectIWAD
pub fn w_check_correct_iwad(mission: GameMissionT) {
    todo!("original: W_CheckCorrectIWAD")
}
