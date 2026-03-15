//! Rust translation of doomgeneric/hu_lib.h
//! Heads-up widget library.

use crate::doomtype::*;
use crate::v_patch::*;

pub const HU_CHARERASE: i32 = 8; // KEY_BACKSPACE
pub const HU_MAXLINES: usize = 4;
pub const HU_MAXLINELENGTH: usize = 80;

/// hu_textline_t
#[repr(C)]
pub struct HuTextlineT {
    pub x: i32,
    pub y: i32,
    pub f: *mut *mut PatchT,
    pub sc: i32,
    pub l: [i8; HU_MAXLINELENGTH + 1],
    pub len: i32,
    pub needsupdate: i32,
}

/// hu_stext_t
#[repr(C)]
pub struct HuStextT {
    pub l: [HuTextlineT; HU_MAXLINES],
    pub h: i32,
    pub cl: i32,
    pub on: *mut boolean,
    pub laston: boolean,
}

/// hu_itext_t
#[repr(C)]
pub struct HuItextT {
    pub l: HuTextlineT,
    pub lm: i32,
    pub on: *mut boolean,
    pub laston: boolean,
}

pub fn hu_lib_init() {
    todo!("original: HUlib_init")
}

pub fn hu_lib_clear_text_line(t: *mut HuTextlineT) {
    todo!("original: HUlib_clearTextLine")
}

pub fn hu_lib_init_text_line(t: *mut HuTextlineT, x: i32, y: i32, f: *mut *mut PatchT, sc: i32) {
    todo!("original: HUlib_initTextLine")
}

pub fn hu_lib_add_char_to_text_line(t: *mut HuTextlineT, ch: i8) -> boolean {
    todo!("original: HUlib_addCharToTextLine")
}

pub fn hu_lib_del_char_from_text_line(t: *mut HuTextlineT) -> boolean {
    todo!("original: HUlib_delCharFromTextLine")
}

pub fn hu_lib_draw_text_line(l: *mut HuTextlineT, drawcursor: boolean) {
    todo!("original: HUlib_drawTextLine")
}

pub fn hu_lib_erase_text_line(l: *mut HuTextlineT) {
    todo!("original: HUlib_eraseTextLine")
}

pub fn hu_lib_init_stext(
    s: *mut HuStextT,
    x: i32,
    y: i32,
    h: i32,
    font: *mut *mut PatchT,
    startchar: i32,
    on: *mut boolean,
) {
    todo!("original: HUlib_initSText")
}

pub fn hu_lib_add_line_to_stext(s: *mut HuStextT) {
    todo!("original: HUlib_addLineToSText")
}

pub fn hu_lib_add_message_to_stext(s: *mut HuStextT, prefix: *mut i8, msg: *mut i8) {
    todo!("original: HUlib_addMessageToSText")
}

pub fn hu_lib_draw_stext(s: *mut HuStextT) {
    todo!("original: HUlib_drawSText")
}

pub fn hu_lib_erase_stext(s: *mut HuStextT) {
    todo!("original: HUlib_eraseSText")
}

pub fn hu_lib_init_itext(
    it: *mut HuItextT,
    x: i32,
    y: i32,
    font: *mut *mut PatchT,
    startchar: i32,
    on: *mut boolean,
) {
    todo!("original: HUlib_initIText")
}

pub fn hu_lib_del_char_from_itext(it: *mut HuItextT) {
    todo!("original: HUlib_delCharFromIText")
}

pub fn hu_lib_erase_line_from_itext(it: *mut HuItextT) {
    todo!("original: HUlib_eraseLineFromIText")
}

pub fn hu_lib_reset_itext(it: *mut HuItextT) {
    todo!("original: HUlib_resetIText")
}

pub fn hu_lib_add_prefix_to_itext(it: *mut HuItextT, str: *mut i8) {
    todo!("original: HUlib_addPrefixToIText")
}

pub fn hu_lib_key_in_itext(it: *mut HuItextT, ch: u8) -> boolean {
    todo!("original: HUlib_keyInIText")
}

pub fn hu_lib_draw_itext(it: *mut HuItextT) {
    todo!("original: HUlib_drawIText")
}

pub fn hu_lib_erase_itext(it: *mut HuItextT) {
    todo!("original: HUlib_eraseIText")
}
