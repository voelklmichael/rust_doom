//! Rust translation of doomgeneric/z_zone.h
//! Zone Memory Allocation.

pub const PU_STATIC: i32 = 1;
pub const PU_SOUND: i32 = 2;
pub const PU_MUSIC: i32 = 3;
pub const PU_FREE: i32 = 4;
pub const PU_LEVEL: i32 = 5;
pub const PU_LEVSPEC: i32 = 6;
pub const PU_PURGELEVEL: i32 = 7;
pub const PU_CACHE: i32 = 8;
pub const PU_NUM_TAGS: i32 = 9;

pub fn z_init() {
    todo!("original: Z_Init")
}

pub fn z_malloc(size: i32, tag: i32, ptr: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    todo!("original: Z_Malloc")
}

pub fn z_free(ptr: *mut std::ffi::c_void) {
    todo!("original: Z_Free")
}

pub fn z_free_tags(lowtag: i32, hightag: i32) {
    todo!("original: Z_FreeTags")
}

pub fn z_dump_heap(lowtag: i32, hightag: i32) {
    todo!("original: Z_DumpHeap")
}

pub fn z_file_dump_heap(f: *mut std::ffi::c_void) {
    todo!("original: Z_FileDumpHeap")
}

pub fn z_check_heap() {
    todo!("original: Z_CheckHeap")
}

pub fn z_change_tag2(ptr: *mut std::ffi::c_void, tag: i32, file: *mut i8, line: i32) {
    todo!("original: Z_ChangeTag2")
}

pub fn z_change_user(ptr: *mut std::ffi::c_void, user: *mut *mut std::ffi::c_void) {
    todo!("original: Z_ChangeUser")
}

pub fn z_free_memory() -> i32 {
    todo!("original: Z_FreeMemory")
}

pub fn z_zone_size() -> u32 {
    todo!("original: Z_ZoneSize")
}
