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

pub fn hulib_clear_text_line(t: &mut HuTextline) {
    t.len = 0;
    t.l[0] = 0;
    t.needsupdate = 1;
}

pub fn hulib_init_text_line(
    t: &mut HuTextline,
    x: i32,
    y: i32,
    f: *mut *mut patch_t,
    sc: i32,
) {
    t.x = x;
    t.y = y;
    t.f = f;
    t.sc = sc;
    hulib_clear_text_line(t);
}

pub fn hulib_add_char_to_text_line(t: &mut HuTextline, ch: u8) -> bool {
    if t.len as usize >= HU_MAXLINELENGTH {
        false
    } else {
        t.l[t.len as usize] = ch;
        t.len += 1;
        t.l[t.len as usize] = 0;
        t.needsupdate = 4;
        true
    }
}

pub fn hulib_del_char_from_text_line(t: &mut HuTextline) -> bool {
    if t.len == 0 {
        false
    } else {
        t.len -= 1;
        t.l[t.len as usize] = 0;
        t.needsupdate = 4;
        true
    }
}

pub fn hulib_draw_text_line(_l: &HuTextline, _drawcursor: bool) {
    // Stub
}

pub fn hulib_erase_text_line(_l: &HuTextline) {
    // Stub
}

pub fn hulib_init_stext(
    s: &mut HuStext,
    x: i32,
    y: i32,
    h: i32,
    font: *mut *mut patch_t,
    startchar: i32,
    on: *mut Boolean,
) {
    s.h = h;
    s.on = on;
    s.laston = true;
    s.cl = 0;
    unsafe {
        if !font.is_null() {
            let f0 = *font;
            let line_height = if !f0.is_null() { (*f0).height as i32 + 1 } else { 8 };
            for i in 0..h {
                hulib_init_text_line(
                    &mut s.l[i as usize],
                    x,
                    y - i * line_height,
                    font,
                    startchar,
                );
            }
        }
    }
}

pub fn hulib_add_line_to_stext(s: &mut HuStext) {
    s.cl += 1;
    if s.cl >= s.h {
        s.cl = 0;
    }
    hulib_clear_text_line(&mut s.l[s.cl as usize]);
    for i in 0..s.h as usize {
        s.l[i].needsupdate = 4;
    }
}

pub fn hulib_add_message_to_stext(s: &mut HuStext, prefix: &str, msg: &str) {
    hulib_add_line_to_stext(s);
    let cl = s.cl as usize;
    for b in prefix.bytes() {
        hulib_add_char_to_text_line(&mut s.l[cl], b);
    }
    for b in msg.bytes() {
        hulib_add_char_to_text_line(&mut s.l[cl], b);
    }
}

pub fn hulib_draw_stext(_s: &HuStext) {
    // Stub
}

pub fn hulib_erase_stext(_s: &mut HuStext) {
    // Stub
}

pub fn hulib_init_itext(
    it: &mut HuItext,
    x: i32,
    y: i32,
    font: *mut *mut patch_t,
    startchar: i32,
    on: *mut Boolean,
) {
    it.lm = 0;
    it.on = on;
    it.laston = true;
    hulib_init_text_line(&mut it.l, x, y, font, startchar);
}

pub fn hulib_del_char_from_itext(it: &mut HuItext) {
    if it.l.len != it.lm {
        hulib_del_char_from_text_line(&mut it.l);
    }
}

pub fn hulib_erase_line_from_itext(it: &mut HuItext) {
    while it.lm != it.l.len {
        hulib_del_char_from_text_line(&mut it.l);
    }
}

pub fn hulib_reset_itext(it: &mut HuItext) {
    it.lm = 0;
    hulib_clear_text_line(&mut it.l);
}

pub fn hulib_add_prefix_to_itext(it: &mut HuItext, s: &str) {
    for b in s.bytes() {
        hulib_add_char_to_text_line(&mut it.l, b);
    }
    it.lm = it.l.len;
}

pub fn hulib_key_in_itext(it: &mut HuItext, ch: u8) -> bool {
    let ch = if ch >= b'a' && ch <= b'z' { ch - 32 } else { ch };
    if ch >= b' ' && ch <= b'_' {
        hulib_add_char_to_text_line(&mut it.l, ch);
    } else if ch == crate::doomkeys::KEY_BACKSPACE as u8 {
        hulib_del_char_from_itext(it);
    } else if ch != crate::doomkeys::KEY_ENTER as u8 {
        return false;
    }
    true
}

pub fn hulib_draw_itext(_it: &HuItext) {
    // Stub
}

pub fn hulib_erase_itext(_it: &mut HuItext) {
    // Stub
}
