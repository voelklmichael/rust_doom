//! Rust translation of doomgeneric/w_merge.h
//! Handles merging of PWADs.

/// C #define: W_NWT_MERGE_SPRITES
pub const W_NWT_MERGE_SPRITES: i32 = 0x1;
/// C #define: W_NWT_MERGE_FLATS
pub const W_NWT_MERGE_FLATS: i32 = 0x2;

/// C function: W_MergeFile
pub fn w_merge_file(filename: *mut i8) {
    todo!("original: W_MergeFile")
}

/// C function: W_NWTMergeFile
pub fn w_nwt_merge_file(filename: *mut i8, flags: i32) {
    todo!("original: W_NWTMergeFile")
}

/// C function: W_NWTDashMerge
pub fn w_nwt_dash_merge(filename: *mut i8) {
    todo!("original: W_NWTDashMerge")
}

/// C function: W_PrintDirectory
pub fn w_print_directory() {
    todo!("original: W_PrintDirectory")
}
