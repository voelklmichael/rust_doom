//! Video buffer, patch drawing (v_video.h, v_video.c)
//! Original: v_video.h, v_video.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Byte;
use crate::v_patch::PatchT;

pub const CENTERY: i32 = 100; // SCREENHEIGHT/2

pub struct V_VideoState {
    pub dirtybox: Arc<Mutex<[i32; 4]>>,
    pub tinttable: Arc<Mutex<Option<Vec<Byte>>>>,
}

impl V_VideoState {
    /// Original: void V_Init(void)
    pub fn v_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_CopyRect(int srcx, int srcy, byte *source, int width, int height, int destx, int desty)
    pub fn v_copy_rect(&self, _srcx: i32, _srcy: i32, _source: &[u8], _width: i32, _height: i32, _destx: i32, _desty: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_DrawPatch(int x, int y, patch_t *patch)
    pub fn v_draw_patch(&self, _x: i32, _y: i32, _patch: &PatchT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_DrawPatchFlipped(int x, int y, patch_t *patch)
    pub fn v_draw_patch_flipped(&self, _x: i32, _y: i32, _patch: &PatchT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_DrawTLPatch(int x, int y, patch_t *patch)
    pub fn v_draw_tl_patch(&self, _x: i32, _y: i32, _patch: &PatchT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_DrawPatchDirect(int x, int y, patch_t *patch)
    pub fn v_draw_patch_direct(&self, _x: i32, _y: i32, _patch: &PatchT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_DrawBlock(int x, int y, int width, int height, byte *src)
    pub fn v_draw_block(&self, _x: i32, _y: i32, _width: i32, _height: i32, _src: &[u8]) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_MarkRect(int x, int y, int width, int height)
    pub fn v_mark_rect(&self, _x: i32, _y: i32, _width: i32, _height: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_UseBuffer(byte *buffer)
    pub fn v_use_buffer(&self, _buffer: &[u8]) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_RestoreBuffer(void)
    pub fn v_restore_buffer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void V_LoadTintTable(void)
    pub fn v_load_tint_table(&self) {
        todo!("Basic stage-0 stub")
    }
}
