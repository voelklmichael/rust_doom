//! Rust translation of doomgeneric/w_wad.h
//! WAD I/O functions.

use crate::doomtype::*;
use crate::d_mode::*;
use crate::w_file::*;

/// lumpinfo_t
#[repr(C)]
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

pub fn w_add_file(filename: *mut i8) -> *mut WadFileT {
    todo!("original: W_AddFile")
}

pub fn w_check_num_for_name(name: *mut i8) -> i32 {
    todo!("original: W_CheckNumForName")
}

pub fn w_get_num_for_name(name: *mut i8) -> i32 {
    todo!("original: W_GetNumForName")
}

pub fn w_lump_length(lump: u32) -> i32 {
    todo!("original: W_LumpLength")
}

pub fn w_read_lump(lump: u32, dest: *mut std::ffi::c_void) {
    todo!("original: W_ReadLump")
}

pub fn w_cache_lump_num(lump: i32, tag: i32) -> *mut std::ffi::c_void {
    todo!("original: W_CacheLumpNum")
}

pub fn w_cache_lump_name(name: *mut i8, tag: i32) -> *mut std::ffi::c_void {
    todo!("original: W_CacheLumpName")
}

pub fn w_generate_hash_table() {
    todo!("original: W_GenerateHashTable")
}

pub fn w_lump_name_hash(s: *const i8) -> u32 {
    todo!("original: W_LumpNameHash")
}

pub fn w_release_lump_num(lump: i32) {
    todo!("original: W_ReleaseLumpNum")
}

pub fn w_release_lump_name(name: *mut i8) {
    todo!("original: W_ReleaseLumpName")
}

pub fn w_check_correct_iwad(mission: GameMissionT) {
    todo!("original: W_CheckCorrectIWAD")
}
