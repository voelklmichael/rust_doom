// w_wad.h / w_wad.c

pub use crate::d_mode::*;
pub use crate::doomtype::*;
pub use crate::w_file::*;

use std::cell::RefCell;
use std::ffi::c_char;
use std::os::raw::c_void;

// Original: typedef struct lumpinfo_s
#[allow(non_camel_case_types)]
pub struct LumpinfoT {
    // Original: name
    pub name: [u8; 8],
    // Original: wad_file
    pub wad_file: *mut WadFileT,
    // Original: position
    pub position: i32,
    // Original: size
    pub size: i32,
    // Original: cache
    pub cache: *mut c_void,
    // Original: next
    pub next: *mut LumpinfoT,
}

#[allow(non_camel_case_types)]
pub struct W_WadState {
    // Original: lumpinfo_t *lumpinfo
    pub lumpinfo: RefCell<*mut LumpinfoT>,
    // Original: unsigned int numlumps
    pub numlumps: RefCell<u32>,
}

impl W_WadState {
    pub fn new() -> Self {
        Self {
            lumpinfo: RefCell::new(std::ptr::null_mut()),
            numlumps: RefCell::new(0),
        }
    }

    // Original: W_LumpNameHash
    pub fn w_lump_name_hash(&self, s: &[u8]) -> u32 {
        let mut result: u32 = 5381;
        for i in 0..8 {
            let c = s.get(i).copied().unwrap_or(0);
            if c == 0 {
                break;
            }
            let u = c.to_ascii_uppercase() as u32;
            result = ((result << 5) ^ result) ^ u;
        }
        result
    }

    // Original: W_AddFile
    pub fn w_add_file(&self, _filename: *mut c_char) -> *mut WadFileT {
        todo!("W_AddFile")
    }

    // Original: W_CheckNumForName
    pub fn w_check_num_for_name(&self, _name: *mut c_char) -> i32 {
        todo!("W_CheckNumForName")
    }

    // Original: W_GetNumForName
    pub fn w_get_num_for_name(&self, _name: *mut c_char) -> i32 {
        todo!("W_GetNumForName")
    }

    // Original: W_LumpLength
    pub fn w_lump_length(&self, _lump: u32) -> i32 {
        todo!("W_LumpLength")
    }

    // Original: W_ReadLump
    pub fn w_read_lump(&self, _lump: u32, _dest: *mut c_void) {
        todo!("W_ReadLump")
    }

    // Original: W_CacheLumpNum
    pub fn w_cache_lump_num(&self, _lump: i32, _tag: i32) -> *mut c_void {
        todo!("W_CacheLumpNum")
    }

    // Original: W_CacheLumpName
    pub fn w_cache_lump_name(&self, _name: *mut c_char, _tag: i32) -> *mut c_void {
        todo!("W_CacheLumpName")
    }

    // Original: W_GenerateHashTable
    pub fn w_generate_hash_table(&self) {
        todo!("W_GenerateHashTable")
    }

    // Original: W_ReleaseLumpNum
    pub fn w_release_lump_num(&self, _lump: i32) {
        todo!("W_ReleaseLumpNum")
    }

    // Original: W_ReleaseLumpName
    pub fn w_release_lump_name(&self, _name: *mut c_char) {
        todo!("W_ReleaseLumpName")
    }

    // Original: W_CheckCorrectIWAD
    pub fn w_check_correct_iwad(&self, _mission: GameMissionT) {
        todo!("W_CheckCorrectIWAD")
    }
}
