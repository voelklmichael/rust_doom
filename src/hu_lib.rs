// doomgeneric/hu_lib.h

pub use crate::doomdef::*;
pub use crate::doomkeys::*;
pub use crate::doomtype::*;
pub use crate::i_swap::*;
pub use crate::r_defs::*;
pub use crate::r_draw::*;
pub use crate::r_local::*;
pub use crate::v_video::*;

use std::cell::RefCell;
use std::ffi::c_char;

// Original: #define HU_CHARERASE KEY_BACKSPACE
pub const HU_CHARERASE: i32 = KEY_BACKSPACE;
pub const HU_MAXLINES: i32 = 4;
pub const HU_MAXLINELENGTH: usize = 80;

/// Original: hu_textline_t
#[repr(C)]
pub struct HuTextlineT {
    // Original: x
    pub x: i32,
    // Original: y
    pub y: i32,
    // Original: f
    pub f: *mut *mut PatchT,
    // Original: sc
    pub sc: i32,
    // Original: l
    pub l: [c_char; HU_MAXLINELENGTH + 1],
    // Original: len
    pub len: i32,
    // Original: needsupdate
    pub needsupdate: i32,
}

/// Original: hu_stext_t
#[repr(C)]
pub struct HuStextT {
    // Original: l
    pub l: [HuTextlineT; HU_MAXLINES as usize],
    // Original: h
    pub h: i32,
    // Original: cl
    pub cl: i32,
    // Original: on
    pub on: *mut Boolean,
    // Original: laston
    pub laston: Boolean,
}

/// Original: hu_itext_t
#[repr(C)]
pub struct HuItextT {
    // Original: l
    pub l: HuTextlineT,
    // Original: lm
    pub lm: i32,
    // Original: on
    pub on: *mut Boolean,
    // Original: laston
    pub laston: Boolean,
}

#[allow(non_camel_case_types)]
pub struct Hu_LibState {
    pub _placeholder: RefCell<()>,
}

impl Hu_LibState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: HUlib_init
    pub fn hu_lib_init(&self) {
        todo!("HUlib_init");
    }

    // Original: HUlib_clearTextLine
    pub fn hu_lib_clear_text_line(&self, _t: *mut HuTextlineT) {
        todo!("HUlib_clearTextLine");
    }

    // Original: HUlib_initTextLine
    pub fn hu_lib_init_text_line(
        &self,
        _t: *mut HuTextlineT,
        _x: i32,
        _y: i32,
        _f: *mut *mut PatchT,
        _sc: i32,
    ) {
        todo!("HUlib_initTextLine");
    }

    // Original: HUlib_addCharToTextLine
    pub fn hu_lib_add_char_to_text_line(&self, _t: *mut HuTextlineT, _ch: c_char) -> Boolean {
        todo!("HUlib_addCharToTextLine");
    }

    // Original: HUlib_delCharFromTextLine
    pub fn hu_lib_del_char_from_text_line(&self, _t: *mut HuTextlineT) -> Boolean {
        todo!("HUlib_delCharFromTextLine");
    }

    // Original: HUlib_drawTextLine
    pub fn hu_lib_draw_text_line(&self, _l: *mut HuTextlineT, _drawcursor: Boolean) {
        todo!("HUlib_drawTextLine");
    }

    // Original: HUlib_eraseTextLine
    pub fn hu_lib_erase_text_line(&self, _l: *mut HuTextlineT) {
        todo!("HUlib_eraseTextLine");
    }

    // Original: HUlib_initSText
    pub fn hu_lib_init_s_text(
        &self,
        _s: *mut HuStextT,
        _x: i32,
        _y: i32,
        _h: i32,
        _font: *mut *mut PatchT,
        _startchar: i32,
        _on: *mut Boolean,
    ) {
        todo!("HUlib_initSText");
    }

    // Original: HUlib_addLineToSText
    pub fn hu_lib_add_line_to_s_text(&self, _s: *mut HuStextT) {
        todo!("HUlib_addLineToSText");
    }

    // Original: HUlib_addMessageToSText
    pub fn hu_lib_add_message_to_s_text(&self, _s: *mut HuStextT, _prefix: *mut c_char, _msg: *mut c_char) {
        todo!("HUlib_addMessageToSText");
    }

    // Original: HUlib_drawSText
    pub fn hu_lib_draw_s_text(&self, _s: *mut HuStextT) {
        todo!("HUlib_drawSText");
    }

    // Original: HUlib_eraseSText
    pub fn hu_lib_erase_s_text(&self, _s: *mut HuStextT) {
        todo!("HUlib_eraseSText");
    }

    // Original: HUlib_initIText
    pub fn hu_lib_init_i_text(
        &self,
        _it: *mut HuItextT,
        _x: i32,
        _y: i32,
        _font: *mut *mut PatchT,
        _startchar: i32,
        _on: *mut Boolean,
    ) {
        todo!("HUlib_initIText");
    }

    // Original: HUlib_delCharFromIText
    pub fn hu_lib_del_char_from_i_text(&self, _it: *mut HuItextT) {
        todo!("HUlib_delCharFromIText");
    }

    // Original: HUlib_eraseLineFromIText
    pub fn hu_lib_erase_line_from_i_text(&self, _it: *mut HuItextT) {
        todo!("HUlib_eraseLineFromIText");
    }

    // Original: HUlib_resetIText
    pub fn hu_lib_reset_i_text(&self, _it: *mut HuItextT) {
        todo!("HUlib_resetIText");
    }

    // Original: HUlib_addPrefixToIText
    pub fn hu_lib_add_prefix_to_i_text(&self, _it: *mut HuItextT, _str: *mut c_char) {
        todo!("HUlib_addPrefixToIText");
    }

    // Original: HUlib_keyInIText
    pub fn hu_lib_key_in_i_text(&self, _it: *mut HuItextT, _ch: u8) -> Boolean {
        todo!("HUlib_keyInIText");
    }

    // Original: HUlib_drawIText
    pub fn hu_lib_draw_i_text(&self, _it: *mut HuItextT) {
        todo!("HUlib_drawIText");
    }

    // Original: HUlib_eraseIText
    pub fn hu_lib_erase_i_text(&self, _it: *mut HuItextT) {
        todo!("HUlib_eraseIText");
    }
}
