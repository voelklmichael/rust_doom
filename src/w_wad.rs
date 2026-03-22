//! WAD I/O (w_wad.h, w_wad.c)
//! Original: w_wad.h, w_wad.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_mode::GameMissionT;

#[repr(C)]
pub struct LumpinfoT {
    pub name: [u8; 8],
    pub position: i32,
    pub size: i32,
}

pub struct W_WadState {
    pub numlumps: Arc<Mutex<u32>>,
}

impl W_WadState {
    /// Original: wad_file_t *W_AddFile(char *filename)
    pub fn w_add_file(&self, _filename: &str) -> Option<()> {
        todo!("Basic stage-0 stub")
    }

    /// Original: int W_CheckNumForName(char *name)
    pub fn w_check_num_for_name(&self, _name: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int W_GetNumForName(char *name)
    pub fn w_get_num_for_name(&self, _name: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int W_LumpLength(unsigned int lump)
    pub fn w_lump_length(&self, _lump: u32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_ReadLump(unsigned int lump, void *dest)
    pub fn w_read_lump(&self, _lump: u32, _dest: &mut [u8]) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void *W_CacheLumpNum(int lump, int tag)
    pub fn w_cache_lump_num(&self, _lump: i32, _tag: i32) -> Option<Vec<u8>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void *W_CacheLumpName(char *name, int tag)
    pub fn w_cache_lump_name(&self, _name: &str, _tag: i32) -> Option<Vec<u8>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_ReleaseLumpNum(int lumpnum)
    pub fn w_release_lump_num(&self, _lumpnum: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_ReleaseLumpName(char *name)
    pub fn w_release_lump_name(&self, _name: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_GenerateHashTable(void)
    pub fn w_generate_hash_table(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_CheckCorrectIWAD(GameMission_t mission)
    pub fn w_check_correct_iwad(&self, _mission: GameMissionT) {
        todo!("Basic stage-0 stub")
    }
}
