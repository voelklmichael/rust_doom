//! Rust translation of doomgeneric/hu_lib.h
//! Heads-up widget library.

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::*;
use crate::v_patch::*;

/// C #define: HU_CHARERASE
pub const HU_CHARERASE: i32 = 8; // KEY_BACKSPACE
/// C #define: HU_MAXLINES
pub const HU_MAXLINES: usize = 4;
/// C #define: HU_MAXLINELENGTH
pub const HU_MAXLINELENGTH: usize = 80;

/// hu_textline_t
#[repr(C)]
/// C typedef: hu_textline_t
pub struct HuTextlineT {
    pub x: i32,
    pub y: i32,
    pub f: Option<Vec<Arc<Mutex<PatchT>>>>,
    pub sc: i32,
    pub l: [i8; HU_MAXLINELENGTH + 1],
    pub len: i32,
    pub needsupdate: i32,
}

/// hu_stext_t
#[repr(C)]
/// C typedef: hu_stext_t
pub struct HuStextT {
    pub l: [HuTextlineT; HU_MAXLINES],
    pub h: i32,
    pub cl: i32,
    pub on: *mut boolean,
    pub laston: boolean,
}

/// hu_itext_t
#[repr(C)]
/// C typedef: hu_itext_t
pub struct HuItextT {
    pub l: HuTextlineT,
    pub lm: i32,
    pub on: *mut boolean,
    pub laston: boolean,
}

/// C function: HUlib_init
pub fn hu_lib_init() {
    todo!("original: HUlib_init")
}

/// C function: HUlib_clearTextLine
pub fn hu_lib_clear_text_line(t: &mut HuTextlineT) {
    todo!("original: HUlib_clearTextLine")
}

/// C function: HUlib_initTextLine
pub fn hu_lib_init_text_line(
    t: &mut HuTextlineT,
    x: i32,
    y: i32,
    f: &mut Vec<Arc<Mutex<PatchT>>>,
    sc: i32,
) {
    todo!("original: HUlib_initTextLine")
}

/// C function: HUlib_addCharToTextLine
pub fn hu_lib_add_char_to_text_line(t: &mut HuTextlineT, ch: i8) -> boolean {
    todo!("original: HUlib_addCharToTextLine")
}

/// C function: HUlib_delCharFromTextLine
pub fn hu_lib_del_char_from_text_line(t: &mut HuTextlineT) -> boolean {
    todo!("original: HUlib_delCharFromTextLine")
}

/// C function: HUlib_drawTextLine
pub fn hu_lib_draw_text_line(l: &mut HuTextlineT, drawcursor: boolean) {
    todo!("original: HUlib_drawTextLine")
}

/// C function: HUlib_eraseTextLine
pub fn hu_lib_erase_text_line(l: &mut HuTextlineT) {
    todo!("original: HUlib_eraseTextLine")
}

/// C function: HUlib_initSText
pub fn hu_lib_init_stext(
    s: &mut HuStextT,
    x: i32,
    y: i32,
    h: i32,
    font: &mut Vec<Arc<Mutex<PatchT>>>,
    startchar: i32,
    on: &mut boolean,
) {
    todo!("original: HUlib_initSText")
}

/// C function: HUlib_addLineToSText
pub fn hu_lib_add_line_to_stext(s: &mut HuStextT) {
    todo!("original: HUlib_addLineToSText")
}

/// C function: HUlib_addMessageToSText
pub fn hu_lib_add_message_to_stext(s: &mut HuStextT, prefix: &str, msg: &str) {
    todo!("original: HUlib_addMessageToSText")
}

/// C function: HUlib_drawSText
pub fn hu_lib_draw_stext(s: &mut HuStextT) {
    todo!("original: HUlib_drawSText")
}

/// C function: HUlib_eraseSText
pub fn hu_lib_erase_stext(s: &mut HuStextT) {
    todo!("original: HUlib_eraseSText")
}

/// C function: HUlib_initIText
pub fn hu_lib_init_itext(
    it: &mut HuItextT,
    x: i32,
    y: i32,
    font: &mut Vec<Arc<Mutex<PatchT>>>,
    startchar: i32,
    on: &mut boolean,
) {
    todo!("original: HUlib_initIText")
}

/// C function: HUlib_delCharFromIText
pub fn hu_lib_del_char_from_itext(it: &mut HuItextT) {
    todo!("original: HUlib_delCharFromIText")
}

/// C function: HUlib_eraseLineFromIText
pub fn hu_lib_erase_line_from_itext(it: &mut HuItextT) {
    todo!("original: HUlib_eraseLineFromIText")
}

/// C function: HUlib_resetIText
pub fn hu_lib_reset_itext(it: &mut HuItextT) {
    todo!("original: HUlib_resetIText")
}

/// C function: HUlib_addPrefixToIText
pub fn hu_lib_add_prefix_to_itext(it: &mut HuItextT, str: &str) {
    todo!("original: HUlib_addPrefixToIText")
}

/// C function: HUlib_keyInIText
pub fn hu_lib_key_in_itext(it: &mut HuItextT, ch: u8) -> boolean {
    todo!("original: HUlib_keyInIText")
}

/// C function: HUlib_drawIText
pub fn hu_lib_draw_itext(it: &mut HuItextT) {
    todo!("original: HUlib_drawIText")
}

/// C function: HUlib_eraseIText
pub fn hu_lib_erase_itext(it: &mut HuItextT) {
    todo!("original: HUlib_eraseIText")
}
