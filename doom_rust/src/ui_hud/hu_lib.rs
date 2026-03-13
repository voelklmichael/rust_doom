//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Heads-up text and input widget library.
// Original: hu_lib.h + hu_lib.c

use crate::doomtype::Boolean;
use crate::rendering::patch_t;

// =============================================================================
// Public API (from hu_lib.h)
// =============================================================================

pub const HU_CHARERASE: i32 = crate::doomkeys::KEY_BACKSPACE;
pub const HU_MAXLINES: usize = 4;
pub const HU_MAXLINELENGTH: usize = 80;

/// Text line widget.
#[repr(C)]
#[derive(Debug)]
pub struct HuTextline {
    pub x: i32,
    pub y: i32,
    pub f: *mut *mut patch_t,
    pub sc: i32,
    pub l: [u8; HU_MAXLINELENGTH + 1],
    pub len: i32,
    pub needsupdate: i32,
}

impl Default for HuTextline {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            f: std::ptr::null_mut(),
            sc: 0,
            l: [0; HU_MAXLINELENGTH + 1],
            len: 0,
            needsupdate: 0,
        }
    }
}

/// Scrolling text window widget.
#[repr(C)]
#[derive(Debug)]
pub struct HuStext {
    pub l: [HuTextline; HU_MAXLINES],
    pub h: i32,
    pub cl: i32,
    pub on: *mut Boolean,
    pub laston: Boolean,
}

/// Input text line widget.
#[repr(C)]
#[derive(Debug)]
pub struct HuItext {
    pub l: HuTextline,
    pub lm: i32,
    pub on: *mut Boolean,
    pub laston: Boolean,
}

// =============================================================================
// Implementation (from hu_lib.c) — stubs
// =============================================================================

pub fn hulib_init() {
    // Stub
}

pub fn hulib_clear_text_line(_t: &mut HuTextline) {
    // Stub: t.len = 0; t.l[0] = 0; t.needsupdate = true;
}

pub fn hulib_init_text_line(
    t: &mut HuTextline,
    x: i32,
    y: i32,
    _f: *mut *mut patch_t,
    sc: i32,
) {
    t.x = x;
    t.y = y;
    t.sc = sc;
    t.len = 0;
    t.l[0] = 0;
    t.needsupdate = 1;
}

pub fn hulib_add_char_to_text_line(_t: &mut HuTextline, _ch: u8) -> bool {
    // Stub
    false
}

pub fn hulib_del_char_from_text_line(_t: &mut HuTextline) -> bool {
    // Stub
    false
}

pub fn hulib_draw_text_line(_l: &HuTextline, _drawcursor: bool) {
    // Stub
}

pub fn hulib_erase_text_line(_l: &HuTextline) {
    // Stub
}

pub fn hulib_init_stext(
    _s: &mut HuStext,
    _x: i32,
    _y: i32,
    _h: i32,
    _font: *mut *mut patch_t,
    _startchar: i32,
    _on: *mut Boolean,
) {
    // Stub
}

pub fn hulib_add_line_to_stext(_s: &mut HuStext) {
    // Stub
}

pub fn hulib_add_message_to_stext(_s: &mut HuStext, _prefix: &str, _msg: &str) {
    // Stub
}

pub fn hulib_draw_stext(_s: &HuStext) {
    // Stub
}

pub fn hulib_erase_stext(_s: &mut HuStext) {
    // Stub
}

pub fn hulib_init_itext(
    _it: &mut HuItext,
    _x: i32,
    _y: i32,
    _font: *mut *mut patch_t,
    _startchar: i32,
    _on: *mut Boolean,
) {
    // Stub
}

pub fn hulib_del_char_from_itext(_it: &mut HuItext) {
    // Stub
}

pub fn hulib_erase_line_from_itext(_it: &mut HuItext) {
    // Stub
}

pub fn hulib_reset_itext(_it: &mut HuItext) {
    // Stub
}

pub fn hulib_add_prefix_to_itext(_it: &mut HuItext, _str: &str) {
    // Stub
}

pub fn hulib_key_in_itext(_it: &mut HuItext, _ch: u8) -> bool {
    // Stub
    false
}

pub fn hulib_draw_itext(_it: &HuItext) {
    // Stub
}

pub fn hulib_erase_itext(_it: &mut HuItext) {
    // Stub
}
