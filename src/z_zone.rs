// z_zone.h / z_zone.c

pub use crate::doomtype::*;

use std::cell::RefCell;
use std::io::Write;
use std::os::raw::c_char;
use std::os::raw::c_void;

// Original: enum PU_*
pub const PU_STATIC: i32 = 1;
pub const PU_SOUND: i32 = 2;
pub const PU_MUSIC: i32 = 3;
pub const PU_FREE: i32 = 4;
pub const PU_LEVEL: i32 = 5;
pub const PU_LEVSPEC: i32 = 6;
pub const PU_PURGELEVEL: i32 = 7;
pub const PU_CACHE: i32 = 8;
pub const PU_NUM_TAGS: i32 = 9;

#[allow(non_camel_case_types)]
pub struct Z_ZoneState {
    // Original: memzone_t* mainzone
    pub mainzone: RefCell<*mut c_void>,
}

impl Z_ZoneState {
    pub fn new() -> Self {
        Self {
            mainzone: RefCell::new(std::ptr::null_mut()),
        }
    }

    // Original: Z_Init
    pub fn z_init(&self) {
        todo!("Z_Init")
    }

    // Original: Z_Malloc
    pub fn z_malloc(&self, _size: i32, _tag: i32, _ptr: *mut c_void) -> *mut c_void {
        todo!("Z_Malloc")
    }

    // Original: Z_Free
    pub fn z_free(&self, _ptr: *mut c_void) {
        todo!("Z_Free")
    }

    // Original: Z_FreeTags
    pub fn z_free_tags(&self, _lowtag: i32, _hightag: i32) {
        todo!("Z_FreeTags")
    }

    // Original: Z_DumpHeap
    pub fn z_dump_heap(&self, _lowtag: i32, _hightag: i32) {
        todo!("Z_DumpHeap")
    }

    // Original: Z_FileDumpHeap
    pub fn z_file_dump_heap(&self, _f: &mut dyn Write) {
        todo!("Z_FileDumpHeap")
    }

    // Original: Z_CheckHeap
    pub fn z_check_heap(&self) {
        todo!("Z_CheckHeap")
    }

    // Original: Z_ChangeTag2
    pub fn z_change_tag2(&self, _ptr: *mut c_void, _tag: i32, _file: *const c_char, _line: i32) {
        todo!("Z_ChangeTag2")
    }

    // Original: Z_ChangeUser
    pub fn z_change_user(&self, _ptr: *mut c_void, _user: *mut *mut c_void) {
        todo!("Z_ChangeUser")
    }

    // Original: Z_FreeMemory
    pub fn z_free_memory(&self) -> i32 {
        todo!("Z_FreeMemory")
    }

    // Original: Z_ZoneSize
    pub fn z_zone_size(&self) -> u32 {
        todo!("Z_ZoneSize")
    }
}
