//! Rust translation of doomgeneric/z_zone.h
//! Zone Memory Allocation.

use std::sync::{Arc, Mutex};

/// C enum: PU_STATIC
pub const PU_STATIC: i32 = 1;
/// C enum: PU_SOUND
pub const PU_SOUND: i32 = 2;
/// C enum: PU_MUSIC
pub const PU_MUSIC: i32 = 3;
/// C enum: PU_FREE
pub const PU_FREE: i32 = 4;
/// C enum: PU_LEVEL
pub const PU_LEVEL: i32 = 5;
/// C enum: PU_LEVSPEC
pub const PU_LEVSPEC: i32 = 6;
/// C enum: PU_PURGELEVEL
pub const PU_PURGELEVEL: i32 = 7;
/// C enum: PU_CACHE
pub const PU_CACHE: i32 = 8;
/// C enum: PU_NUM_TAGS
pub const PU_NUM_TAGS: i32 = 9;

/// C function: Z_Init
pub fn z_init() {
    todo!("original: Z_Init")
}

/// C function: Z_Malloc
pub fn z_malloc(size: i32, tag: i32, ptr: Option<&mut [u8]>) -> Arc<Mutex<Vec<u8>>> {
    todo!("original: Z_Malloc")
}

/// C function: Z_Free
pub fn z_free(ptr: &mut [u8]) {
    todo!("original: Z_Free")
}

/// C function: Z_FreeTags
pub fn z_free_tags(lowtag: i32, hightag: i32) {
    todo!("original: Z_FreeTags")
}

/// C function: Z_DumpHeap
pub fn z_dump_heap(lowtag: i32, hightag: i32) {
    todo!("original: Z_DumpHeap")
}

/// C function: Z_FileDumpHeap
pub fn z_file_dump_heap(f: &mut std::ffi::c_void) {
    todo!("original: Z_FileDumpHeap")
}

/// C function: Z_CheckHeap
pub fn z_check_heap() {
    todo!("original: Z_CheckHeap")
}

/// C function: Z_ChangeTag2
pub fn z_change_tag2(ptr: &mut [u8], tag: i32, file: &str, line: i32) {
    todo!("original: Z_ChangeTag2")
}

/// C function: Z_ChangeUser
pub fn z_change_user(ptr: &mut [u8], user: &mut Arc<Mutex<Vec<u8>>>) {
    todo!("original: Z_ChangeUser")
}

/// C function: Z_FreeMemory
pub fn z_free_memory() -> i32 {
    todo!("original: Z_FreeMemory")
}

/// C function: Z_ZoneSize
pub fn z_zone_size() -> u32 {
    todo!("original: Z_ZoneSize")
}
