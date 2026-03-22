//! WAD merging (w_merge.h)
//! Original: w_merge.h
//! Plan §4: FEATURE_WAD_MERGE unused - declarations only.

// #define W_NWT_MERGE_SPRITES 0x1
pub const W_NWT_MERGE_SPRITES: i32 = 0x1;
// #define W_NWT_MERGE_FLATS 0x2
pub const W_NWT_MERGE_FLATS: i32 = 0x2;

pub struct W_MergeState;

impl W_MergeState {
    /// Original: void W_MergeFile(char *filename)
    pub fn w_merge_file(&self, _filename: &str) {
        // C body: (in w_merge.c when FEATURE_WAD_MERGE - not migrated)
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_NWTMergeFile(char *filename, int flags)
    pub fn w_nwt_merge_file(&self, _filename: &str, _flags: i32) {
        // C body: (in w_merge.c when FEATURE_WAD_MERGE - not migrated)
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_NWTDashMerge(char *filename)
    pub fn w_nwt_dash_merge(&self, _filename: &str) {
        // C body: (in w_merge.c when FEATURE_WAD_MERGE - not migrated)
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_PrintDirectory(void)
    pub fn w_print_directory(&self) {
        // C body: (in w_merge.c when FEATURE_WAD_MERGE - not migrated)
        todo!("Basic stage-0 stub")
    }
}
