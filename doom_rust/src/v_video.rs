//! Rust translation of doomgeneric/v_video.h

use crate::doomtype::*;
use crate::i_video::SCREENHEIGHT;
use crate::v_patch::*;

/// C #define: CENTERY
pub const CENTERY: i32 = SCREENHEIGHT / 2;

/// C extern: dirtybox
pub static mut dirtybox: [i32; 4] = [0; 4];

/// C extern: tinttable
pub static mut tinttable: *mut byte = std::ptr::null_mut();

/// C typedef: vpatchclipfunc_t
pub type VpatchclipfuncT = extern "C" fn(*mut PatchT, i32, i32) -> boolean;

/// C function: V_SetPatchClipCallback
pub fn v_set_patch_clip_callback(func: VpatchclipfuncT) {
    todo!("original: V_SetPatchClipCallback")
}

/// C function: V_Init
pub fn v_init() {
    todo!("original: V_Init")
}

/// C function: V_CopyRect
pub fn v_copy_rect(
    srcx: i32,
    srcy: i32,
    source: &mut [byte],
    width: i32,
    height: i32,
    destx: i32,
    desty: i32,
) {
    todo!("original: V_CopyRect")
}

/// C function: V_DrawPatch
pub fn v_draw_patch(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawPatch")
}

/// C function: V_DrawPatchFlipped
pub fn v_draw_patch_flipped(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawPatchFlipped")
}

/// C function: V_DrawTLPatch
pub fn v_draw_tl_patch(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawTLPatch")
}

/// C function: V_DrawAltTLPatch
pub fn v_draw_alt_tl_patch(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawAltTLPatch")
}

/// C function: V_DrawShadowedPatch
pub fn v_draw_shadowed_patch(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawShadowedPatch")
}

/// C function: V_DrawXlaPatch
pub fn v_draw_xla_patch(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawXlaPatch")
}

/// C function: V_DrawPatchDirect
pub fn v_draw_patch_direct(x: i32, y: i32, patch: &mut PatchT) {
    todo!("original: V_DrawPatchDirect")
}

/// C function: V_DrawBlock
pub fn v_draw_block(x: i32, y: i32, width: i32, height: i32, src: &mut [byte]) {
    todo!("original: V_DrawBlock")
}

/// C function: V_MarkRect
pub fn v_mark_rect(x: i32, y: i32, width: i32, height: i32) {
    todo!("original: V_MarkRect")
}

/// C function: V_DrawFilledBox
pub fn v_draw_filled_box(x: i32, y: i32, w: i32, h: i32, c: i32) {
    todo!("original: V_DrawFilledBox")
}

/// C function: V_DrawHorizLine
pub fn v_draw_horiz_line(x: i32, y: i32, w: i32, c: i32) {
    todo!("original: V_DrawHorizLine")
}

/// C function: V_DrawVertLine
pub fn v_draw_vert_line(x: i32, y: i32, h: i32, c: i32) {
    todo!("original: V_DrawVertLine")
}

/// C function: V_DrawBox
pub fn v_draw_box(x: i32, y: i32, w: i32, h: i32, c: i32) {
    todo!("original: V_DrawBox")
}

/// C function: V_DrawRawScreen
pub fn v_draw_raw_screen(raw: &mut [byte]) {
    todo!("original: V_DrawRawScreen")
}

/// C function: V_UseBuffer
pub fn v_use_buffer(buffer: &mut [byte]) {
    todo!("original: V_UseBuffer")
}

/// C function: V_RestoreBuffer
pub fn v_restore_buffer() {
    todo!("original: V_RestoreBuffer")
}

/// C function: V_ScreenShot
pub fn v_screen_shot(format: &str) {
    todo!("original: V_ScreenShot")
}

/// C function: V_LoadTintTable
pub fn v_load_tint_table() {
    todo!("original: V_LoadTintTable")
}

/// C function: V_LoadXlaTable
pub fn v_load_xla_table() {
    todo!("original: V_LoadXlaTable")
}

/// C function: V_DrawMouseSpeedBox
pub fn v_draw_mouse_speed_box(speed: i32) {
    todo!("original: V_DrawMouseSpeedBox")
}
