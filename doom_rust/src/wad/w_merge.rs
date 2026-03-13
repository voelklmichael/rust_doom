//
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
// Handles merging of PWADs, similar to deutex's -merge option
//
// Original: w_merge.h (public) - no .c in codebase, stub implementations that panic

/// Original: W_NWT_MERGE_SPRITES
pub const W_NWT_MERGE_SPRITES: i32 = 0x1;

/// Original: W_NWT_MERGE_FLATS
pub const W_NWT_MERGE_FLATS: i32 = 0x2;

/// Add a new WAD and merge it into the main directory.
/// Stub: panics if called (FEATURE_WAD_MERGE is off).
pub fn w_merge_file(_filename: &str) {
    panic!("W_MergeFile: WAD merge not supported (FEATURE_WAD_MERGE is off)");
}

/// NWT-style merging.
/// Stub: panics if called.
pub fn w_nwt_merge_file(_filename: &str, _flags: i32) {
    panic!("W_NWTMergeFile: WAD merge not supported (FEATURE_WAD_MERGE is off)");
}

/// Acts the same as NWT's "-merge" option.
/// Stub: panics if called.
pub fn w_nwt_dash_merge(_filename: &str) {
    panic!("W_NWTDashMerge: WAD merge not supported (FEATURE_WAD_MERGE is off)");
}

/// Debug function that prints the WAD directory.
pub fn w_print_directory() {
    panic!("W_PrintDirectory: not implemented");
}
