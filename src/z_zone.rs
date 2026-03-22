//! Memory zone (z_zone.h, z_zone.c)
//! Original: z_zone.h, z_zone.c

use std::sync::Arc;
use std::sync::Mutex;

pub const PU_STATIC: i32 = 1;
pub const PU_SOUND: i32 = 2;
pub const PU_MUSIC: i32 = 3;
pub const PU_FREE: i32 = 4;
pub const PU_LEVEL: i32 = 5;
pub const PU_LEVSPEC: i32 = 6;
pub const PU_PURGELEVEL: i32 = 7;
pub const PU_CACHE: i32 = 8;
pub const PU_NUM_TAGS: i32 = 9;

pub struct Z_ZoneState;

impl Z_ZoneState {
    /// Original: void Z_Init(void)
    pub fn z_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void *Z_Malloc(int size, int tag, void *ptr)
    pub fn z_malloc(&self, _size: i32, _tag: i32, _ptr: Option<&()>) -> Option<Vec<u8>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void Z_Free(void *ptr)
    pub fn z_free(&self, _ptr: &[u8]) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void Z_FreeTags(int lowtag, int hightag)
    pub fn z_free_tags(&self, _lowtag: i32, _hightag: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void Z_DumpHeap(int lowtag, int hightag)
    pub fn z_dump_heap(&self, _lowtag: i32, _hightag: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void Z_CheckHeap(void)
    pub fn z_check_heap(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void Z_ChangeTag2(void *ptr, int tag, char *file, int line)
    pub fn z_change_tag(&self, _ptr: &mut [u8], _tag: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int Z_FreeMemory(void)
    pub fn z_free_memory(&self) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: unsigned int Z_ZoneSize(void)
    pub fn z_zone_size(&self) -> u32 {
        todo!("Basic stage-0 stub")
    }
}
