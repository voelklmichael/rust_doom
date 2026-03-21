// v_video.h - video/graphics

pub use crate::doomtype::*;
pub use crate::v_patch::*;

// From i_video: SCREENHEIGHT, SCREENWIDTH
// Original: #define CENTERY (SCREENHEIGHT/2)
pub const SCREENWIDTH: i32 = 320;
pub const SCREENHEIGHT: i32 = 200;
pub const CENTERY: i32 = SCREENHEIGHT / 2;

// Original: extern int dirtybox[4]
pub fn dirtybox() -> [i32; 4] {
    todo!("dirtybox: extern variable")
}

// Original: extern byte *tinttable
pub fn tinttable() -> *mut Byte {
    todo!("tinttable: extern variable")
}

/// Original: vpatchclipfunc_t
pub type VpatchclipfuncT = fn(*mut PatchT, i32, i32) -> bool;

// Original: V_SetPatchClipCallback
pub fn v_set_patch_clip_callback(_func: VpatchclipfuncT) {
    todo!("V_SetPatchClipCallback")
}

// Original: V_Init
pub fn v_init() {
    todo!("V_Init")
}

// Original: V_CopyRect
pub fn v_copy_rect(
    _srcx: i32,
    _srcy: i32,
    _source: *mut Byte,
    _width: i32,
    _height: i32,
    _destx: i32,
    _desty: i32,
) {
    todo!("V_CopyRect")
}

// Original: V_DrawPatch
pub fn v_draw_patch(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawPatch")
}

// Original: V_DrawPatchFlipped
pub fn v_draw_patch_flipped(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawPatchFlipped")
}

// Original: V_DrawTLPatch
pub fn v_draw_tl_patch(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawTLPatch")
}

// Original: V_DrawAltTLPatch
pub fn v_draw_alt_tl_patch(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawAltTLPatch")
}

// Original: V_DrawShadowedPatch
pub fn v_draw_shadowed_patch(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawShadowedPatch")
}

// Original: V_DrawXlaPatch
pub fn v_draw_xla_patch(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawXlaPatch")
}

// Original: V_DrawPatchDirect
pub fn v_draw_patch_direct(_x: i32, _y: i32, _patch: *mut PatchT) {
    todo!("V_DrawPatchDirect")
}

// Original: V_DrawBlock
pub fn v_draw_block(_x: i32, _y: i32, _width: i32, _height: i32, _src: *mut Byte) {
    todo!("V_DrawBlock")
}

// Original: V_MarkRect
pub fn v_mark_rect(_x: i32, _y: i32, _width: i32, _height: i32) {
    todo!("V_MarkRect")
}

// Original: V_DrawFilledBox
pub fn v_draw_filled_box(_x: i32, _y: i32, _w: i32, _h: i32, _c: i32) {
    todo!("V_DrawFilledBox")
}

// Original: V_DrawHorizLine
pub fn v_draw_horiz_line(_x: i32, _y: i32, _w: i32, _c: i32) {
    todo!("V_DrawHorizLine")
}

// Original: V_DrawVertLine
pub fn v_draw_vert_line(_x: i32, _y: i32, _h: i32, _c: i32) {
    todo!("V_DrawVertLine")
}

// Original: V_DrawBox
pub fn v_draw_box(_x: i32, _y: i32, _w: i32, _h: i32, _c: i32) {
    todo!("V_DrawBox")
}

// Original: V_DrawRawScreen
pub fn v_draw_raw_screen(_raw: *mut Byte) {
    todo!("V_DrawRawScreen")
}

// Original: V_UseBuffer
pub fn v_use_buffer(_buffer: *mut Byte) {
    todo!("V_UseBuffer")
}

// Original: V_RestoreBuffer
pub fn v_restore_buffer() {
    todo!("V_RestoreBuffer")
}

// Original: V_ScreenShot
pub fn v_screen_shot(_format: *mut i8) {
    todo!("V_ScreenShot")
}

// Original: V_LoadTintTable
pub fn v_load_tint_table() {
    todo!("V_LoadTintTable")
}

// Original: V_LoadXlaTable
pub fn v_load_xla_table() {
    todo!("V_LoadXlaTable")
}

// Original: V_DrawMouseSpeedBox
pub fn v_draw_mouse_speed_box(_speed: i32) {
    todo!("V_DrawMouseSpeedBox")
}
