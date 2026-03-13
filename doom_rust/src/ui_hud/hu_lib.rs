//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Heads-up text and input widget library.
// Original: hu_lib.h + hu_lib.c

use crate::doomdef::SCREENWIDTH;
use crate::doomstat::AUTOMAPACTIVE;
use crate::doomtype::Boolean;
use crate::rendering::{VIEWWINDOWX, VIEWWINDOWY};
use crate::rendering::state::{VIEWHEIGHT, VIEWWIDTH};
use crate::rendering::{patch_t, r_video_erase, v_draw_patch_direct};

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

pub fn hulib_draw_text_line(l: &HuTextline, drawcursor: bool) {
    unsafe {
        if l.f.is_null() {
            return;
        }
        let sc = l.sc;
        let mut x = l.x;
        for i in 0..l.len as usize {
            let c = l.l[i];
            let c_upper = if c >= b'a' && c <= b'z' { c - 32 } else { c };
            if c_upper == b' ' {
                x += 4;
                if x >= SCREENWIDTH {
                    break;
                }
            } else if (c_upper as i32) >= sc && c_upper <= b'_' {
                let idx = (c_upper as i32 - sc) as usize;
                let patch = *l.f.add(idx);
                if !patch.is_null() {
                    let w = (*patch).width as i32;
                    if x + w > SCREENWIDTH {
                        break;
                    }
                    v_draw_patch_direct(x, l.y, patch);
                    x += w;
                } else {
                    x += 4;
                    if x >= SCREENWIDTH {
                        break;
                    }
                }
            } else {
                x += 4;
                if x >= SCREENWIDTH {
                    break;
                }
            }
        }
        if drawcursor {
            let underscore_idx = (b'_' as i32 - sc) as usize;
            let patch = *l.f.add(underscore_idx);
            if !patch.is_null() && x + (*patch).width as i32 <= SCREENWIDTH {
                v_draw_patch_direct(x, l.y, patch);
            }
        }
    }
}

pub fn hulib_erase_text_line(l: &mut HuTextline) {
    unsafe {
        if !AUTOMAPACTIVE && VIEWWINDOWX != 0 && l.needsupdate != 0 {
            if !l.f.is_null() {
                let lh = (*(*l.f.add(0))).height as i32 + 1;
                let screen_width = SCREENWIDTH as usize;
                for y in l.y..(l.y + lh) {
                    let yoffset = (y as usize) * screen_width;
                    if y < VIEWWINDOWY || y >= VIEWWINDOWY + VIEWHEIGHT {
                        r_video_erase(yoffset, SCREENWIDTH);
                    } else {
                        r_video_erase(yoffset, VIEWWINDOWX);
                        r_video_erase(
                            yoffset + (VIEWWINDOWX + VIEWWIDTH) as usize,
                            VIEWWINDOWX,
                        );
                    }
                }
            }
        }
        if l.needsupdate != 0 {
            l.needsupdate -= 1;
        }
    }
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

pub fn hulib_draw_stext(s: &HuStext) {
    unsafe {
        if s.on.is_null() || !(*s.on) {
            return;
        }
        for i in 0..s.h as usize {
            let mut idx = s.cl as i32 - i as i32;
            if idx < 0 {
                idx += s.h;
            }
            let l = &s.l[idx as usize];
            hulib_draw_text_line(l, false);
        }
    }
}

pub fn hulib_erase_stext(s: &mut HuStext) {
    unsafe {
        let on_val = if s.on.is_null() { false } else { *s.on };
        for i in 0..s.h as usize {
            if s.laston && !on_val {
                s.l[i].needsupdate = 4;
            }
            hulib_erase_text_line(&mut s.l[i]);
        }
        s.laston = on_val;
    }
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

pub fn hulib_draw_itext(it: &HuItext) {
    hulib_draw_text_line(&it.l, true);
}

pub fn hulib_erase_itext(it: &mut HuItext) {
    hulib_erase_text_line(&mut it.l);
}
