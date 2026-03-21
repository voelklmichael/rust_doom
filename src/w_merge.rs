// w_merge.h - WAD merging
// No dependencies (leaf module)

use std::cell::RefCell;

// Original: #define W_NWT_MERGE_SPRITES 0x1
pub const W_NWT_MERGE_SPRITES: i32 = 0x1;

// Original: #define W_NWT_MERGE_FLATS 0x2
pub const W_NWT_MERGE_FLATS: i32 = 0x2;

#[allow(non_camel_case_types)]
pub struct W_MergeState {
    _placeholder: RefCell<()>,
}

impl W_MergeState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: W_MergeFile
    pub fn w_merge_file(&self, _filename: *mut std::ffi::c_char) {
        todo!("W_MergeFile")
    }

    // Original: W_NWTMergeFile
    pub fn w_nwt_merge_file(&self, _filename: *mut std::ffi::c_char, _flags: i32) {
        todo!("W_NWTMergeFile")
    }

    // Original: W_NWTDashMerge
    pub fn w_nwt_dash_merge(&self, _filename: *mut std::ffi::c_char) {
        todo!("W_NWTDashMerge")
    }

    // Original: W_PrintDirectory
    pub fn w_print_directory(&self) {
        todo!("W_PrintDirectory")
    }
}
