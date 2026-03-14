//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Heads-up display.
// Original: hu_stuff.h + hu_stuff.c

use crate::deh::deh_string;
use crate::doomdef::{MAXPLAYERS, TICRATE};
use crate::doomstat::{logical_gamemission, GAMEMAP, GAMEEPISODE, CONSOLEPLAYER, NETGAME, PLAYERS};
use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::game::d_mode::GameMission;
use crate::ui_hud::hu_lib::{hulib_add_char_to_text_line, hulib_draw_itext, hulib_draw_stext, hulib_draw_text_line, hulib_erase_itext, hulib_erase_stext, hulib_erase_text_line, hulib_init_itext, hulib_init_stext, hulib_init_text_line, hulib_add_message_to_stext, hulib_reset_itext, HuItext, HuStext, HuTextline, HU_MAXLINELENGTH};
use crate::wad::w_cache_lump_name;
use crate::z_zone::PU_STATIC;

// =============================================================================
// Public API (from hu_stuff.h)
// =============================================================================

pub const HU_FONTSTART: u8 = b'!';
pub const HU_FONTEND: u8 = b'_';
pub const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize; // 63

pub const HU_BROADCAST: i32 = 5;

pub const HU_MSGX: i32 = 0;
pub const HU_MSGY: i32 = 0;
pub const HU_MSGWIDTH: i32 = 64;
pub const HU_MSGHEIGHT: i32 = 1;

pub const HU_MSGTIMEOUT: i32 = 4 * TICRATE;

pub static mut CHAT_MACROS: [&'static str; 10] = [
    "", "", "", "", "", "", "", "", "", "",
];

// =============================================================================
// Private state (from hu_stuff.c)
// =============================================================================

static mut HU_FONT: [*mut crate::rendering::patch_t; HU_FONTSIZE] =
    [std::ptr::null_mut(); HU_FONTSIZE];
static mut W_TITLE: HuTextline = HuTextline {
    x: 0,
    y: 0,
    f: std::ptr::null_mut(),
    sc: 0,
    l: [0; HU_MAXLINELENGTH + 1],
    len: 0,
    needsupdate: 0,
};
static mut MESSAGE_ON: Boolean = false;
static mut MESSAGE_DONTFUCKWITHME: Boolean = false;
static mut MESSAGE_NOTTOBEFUCKEDWITH: Boolean = false;
static mut W_MESSAGE: HuStext = HuStext {
    l: [
        HuTextline { x: 0, y: 0, f: std::ptr::null_mut(), sc: 0, l: [0; HU_MAXLINELENGTH + 1], len: 0, needsupdate: 0 },
        HuTextline { x: 0, y: 0, f: std::ptr::null_mut(), sc: 0, l: [0; HU_MAXLINELENGTH + 1], len: 0, needsupdate: 0 },
        HuTextline { x: 0, y: 0, f: std::ptr::null_mut(), sc: 0, l: [0; HU_MAXLINELENGTH + 1], len: 0, needsupdate: 0 },
        HuTextline { x: 0, y: 0, f: std::ptr::null_mut(), sc: 0, l: [0; HU_MAXLINELENGTH + 1], len: 0, needsupdate: 0 },
    ],
    h: 0,
    cl: 0,
    on: std::ptr::null_mut(),
    laston: false,
};
static mut MESSAGE_COUNTER: i32 = 0;
static mut CHAT_ON: Boolean = false;
static mut W_CHAT: HuItext = HuItext {
    l: HuTextline { x: 0, y: 0, f: std::ptr::null_mut(), sc: 0, l: [0; HU_MAXLINELENGTH + 1], len: 0, needsupdate: 0 },
    lm: 0,
    on: std::ptr::null_mut(),
    laston: false,
};
static mut ALWAYS_OFF: Boolean = false;
static mut HEADSUPACTIVE: Boolean = false;

// Save string input (menu Save to empty slot)
static mut W_SAVESTRING: HuItext = HuItext {
    l: HuTextline { x: 0, y: 0, f: std::ptr::null_mut(), sc: 0, l: [0; HU_MAXLINELENGTH + 1], len: 0, needsupdate: 0 },
    lm: 0,
    on: std::ptr::null_mut(),
    laston: false,
};
static mut SAVE_STRING_ACTIVE: bool = false;
static mut SAVE_STRING_SLOT: usize = 0;
static mut SAVE_STRING_ON: Boolean = false;

pub static mut MESSAGE_DONTFUCKWITHME_EXTERN: Boolean = false;

fn map_title() -> &'static str {
    unsafe {
        match logical_gamemission() {
            GameMission::Doom => {
                let ep = GAMEEPISODE.max(1).min(4);
                let map = GAMEMAP.max(1).min(9);
                match (ep, map) {
                    (1, 1) => "Hangar",
                    (1, 2) => "Nuclear Plant",
                    (1, 3) => "Toxin Refinery",
                    (1, 4) => "Command Control",
                    (1, 5) => "Phobos Lab",
                    (1, 6) => "Central Processing",
                    (1, 7) => "Computer Station",
                    (1, 8) => "Phobos Anomaly",
                    (1, 9) => "Military Base",
                    _ => "Unknown",
                }
            }
            GameMission::Doom2 => {
                let map = GAMEMAP.max(1).min(32);
                match map {
                    1 => "Entryway",
                    2 => "Underhalls",
                    3 => "The Gantlet",
                    _ => "MAP",
                }
            }
            _ => "Unknown",
        }
    }
}

// =============================================================================
// Implementation (from hu_stuff.c)
// =============================================================================

/// Draw string at x,y using HU_FONT. Used by menu Load/Save.
pub fn hu_write_text(x: i32, y: i32, s: &str) {
    unsafe {
        if HU_FONT[0].is_null() {
            return;
        }
        let sc = HU_FONTSTART as i32;
        let mut cx = x;
        let cy = y;
        for b in s.bytes() {
            let c = if b == b'\n' {
                continue;
            } else if b >= b'a' && b <= b'z' {
                b - 32
            } else {
                b
            };
            let idx = (c as i32) - sc;
            if idx < 0 || idx >= HU_FONTSIZE as i32 {
                cx += 4;
                continue;
            }
            let patch = HU_FONT[idx as usize];
            if patch.is_null() {
                cx += 4;
                continue;
            }
            let w = (*patch).width as i32;
            if cx + w > crate::doomdef::SCREENWIDTH {
                break;
            }
            crate::rendering::v_draw_patch_direct(cx, cy, patch);
            cx += w;
        }
    }
}

/// Draw a single character at (x,y). Returns width in pixels, or 4 for non-drawable chars.
/// Used by finale text scroll. Original: used in F_TextWrite.
pub fn hu_draw_char(x: i32, y: i32, c: u8) -> i32 {
    unsafe {
        if HU_FONT[0].is_null() {
            return 4;
        }
        let c_upper = if c >= b'a' && c <= b'z' { c - 32 } else { c };
        let idx = (c_upper as i32) - (HU_FONTSTART as i32);
        if idx < 0 || idx >= HU_FONTSIZE as i32 {
            return 4;
        }
        let patch = HU_FONT[idx as usize];
        if patch.is_null() {
            return 4;
        }
        let w = (*patch).width as i32;
        crate::rendering::v_draw_patch_direct(x, y, patch);
        w
    }
}

/// String width in pixels using HU_FONT.
pub fn hu_string_width(s: &str) -> i32 {
    unsafe {
        if HU_FONT[0].is_null() {
            return (s.len() as i32) * 4;
        }
        let sc = HU_FONTSTART as i32;
        let mut w = 0i32;
        for b in s.bytes() {
            let c = if b >= b'a' && b <= b'z' { b - 32 } else { b };
            let idx = (c as i32) - sc;
            if idx < 0 || idx >= HU_FONTSIZE as i32 {
                w += 4;
            } else {
                let patch = HU_FONT[idx as usize];
                if patch.is_null() {
                    w += 4;
                } else {
                    w += (*patch).width as i32;
                }
            }
        }
        w
    }
}

pub fn hu_init() {
    let mut j = HU_FONTSTART;
    for i in 0..HU_FONTSIZE {
        let name = format!("STCFN{:03}", j);
        let lump = w_cache_lump_name(deh_string(&name), PU_STATIC);
        unsafe {
            HU_FONT[i] = lump as *mut crate::rendering::patch_t;
        }
        j += 1;
    }
}

fn hu_stop() {
    unsafe {
        HEADSUPACTIVE = false;
    }
}

pub fn hu_start() {
    unsafe {
        if HEADSUPACTIVE {
            hu_stop();
        }
        let _plr_idx = CONSOLEPLAYER as usize;
        MESSAGE_ON = false;
        MESSAGE_DONTFUCKWITHME = false;
        MESSAGE_NOTTOBEFUCKEDWITH = false;
        CHAT_ON = false;

        let font_ptr = HU_FONT.as_mut_ptr();
        hulib_init_stext(
            &mut W_MESSAGE,
            HU_MSGX,
            HU_MSGY,
            HU_MSGHEIGHT,
            font_ptr,
            HU_FONTSTART as i32,
            &mut MESSAGE_ON,
        );

        let hu_title_y = 167
            - if !HU_FONT[0].is_null() {
                unsafe { (*HU_FONT[0]).height as i32 }
            } else {
                8
            };
        hulib_init_text_line(&mut W_TITLE, 0, hu_title_y, font_ptr, HU_FONTSTART as i32);

        let title = deh_string(map_title());
        for b in title.bytes() {
            hulib_add_char_to_text_line(&mut W_TITLE, b);
        }

        let hu_input_y = HU_MSGY + HU_MSGHEIGHT * (8 + 1);
        hulib_init_itext(
            &mut W_CHAT,
            0,
            hu_input_y,
            font_ptr,
            HU_FONTSTART as i32,
            &mut CHAT_ON,
        );

        HEADSUPACTIVE = true;
    }
}

pub fn hu_responder(_ev: &Event) -> Boolean {
    false
}

pub fn hu_ticker() {
    unsafe {
        if MESSAGE_COUNTER != 0 {
            MESSAGE_COUNTER -= 1;
            if MESSAGE_COUNTER == 0 {
                MESSAGE_ON = false;
                MESSAGE_NOTTOBEFUCKEDWITH = false;
            }
        }
    }
}

/// Set a message to display on the HUD (e.g. from pickup, etc.).
pub fn hu_set_message(msg: &str) {
    unsafe {
        hulib_add_message_to_stext(&mut W_MESSAGE, "", msg);
        MESSAGE_ON = true;
        MESSAGE_COUNTER = HU_MSGTIMEOUT;
    }
}

pub fn hu_drawer() {
    unsafe {
        hulib_draw_stext(&W_MESSAGE);
        hulib_draw_itext(&W_CHAT);
        if crate::doomstat::AUTOMAPACTIVE {
            hulib_draw_text_line(&W_TITLE, false);
        }
    }
}

pub fn hu_dequeue_chat_char() -> u8 {
    0
}

/// Result of save string key handling.
#[derive(Debug, Clone)]
pub enum SaveStringKeyResult {
    Consumed,
    Commit { slot: usize, desc: String },
    Cancel,
}

/// Start save string input for slot at (x, y). Call hu_init first.
pub fn hu_start_save_string(slot: usize, x: i32, y: i32) {
    unsafe {
        SAVE_STRING_ACTIVE = true;
        SAVE_STRING_SLOT = slot;
        SAVE_STRING_ON = true;
        let font_ptr = HU_FONT.as_mut_ptr();
        hulib_init_itext(
            &mut W_SAVESTRING,
            x,
            y,
            font_ptr,
            HU_FONTSTART as i32,
            &mut SAVE_STRING_ON,
        );
        hulib_reset_itext(&mut W_SAVESTRING);
    }
}

/// Handle key for save string input. Returns Some(result) when in save string mode.
pub fn hu_save_string_key(ch: u8) -> Option<SaveStringKeyResult> {
    use crate::ui_hud::hu_lib::hulib_key_in_itext;
    use crate::doomkeys;
    unsafe {
        if !SAVE_STRING_ACTIVE {
            return None;
        }
        if ch == doomkeys::KEY_ESCAPE as u8 {
            SAVE_STRING_ACTIVE = false;
            return Some(SaveStringKeyResult::Cancel);
        }
        if ch == doomkeys::KEY_ENTER as u8 {
            let slot = SAVE_STRING_SLOT;
            let desc = hu_save_string_desc_inner();
            SAVE_STRING_ACTIVE = false;
            return Some(SaveStringKeyResult::Commit { slot, desc });
        }
        if hulib_key_in_itext(&mut W_SAVESTRING, ch) {
            return Some(SaveStringKeyResult::Consumed);
        }
        Some(SaveStringKeyResult::Consumed)
    }
}

pub fn hu_save_string_active() -> bool {
    unsafe { SAVE_STRING_ACTIVE }
}

pub fn hu_save_string_slot() -> usize {
    unsafe { SAVE_STRING_SLOT }
}

fn hu_save_string_desc_inner() -> String {
    use crate::ui_hud::hu_lib::hulib_itext_to_string;
    unsafe {
        let s = hulib_itext_to_string(&W_SAVESTRING);
        s.chars().take(crate::player::p_saveg::SAVESTRINGSIZE).collect::<String>()
    }
}

/// Get current save string text. Truncate to SAVESTRINGSIZE (24).
pub fn hu_save_string_desc() -> String {
    unsafe {
        if !SAVE_STRING_ACTIVE {
            return String::new();
        }
        hu_save_string_desc_inner()
    }
}

pub fn hu_draw_save_string() {
    unsafe {
        if SAVE_STRING_ACTIVE {
            hulib_draw_itext(&W_SAVESTRING);
        }
    }
}

pub fn hu_cancel_save_string() {
    unsafe {
        SAVE_STRING_ACTIVE = false;
    }
}

pub fn hu_erase() {
    unsafe {
        hulib_erase_stext(&mut W_MESSAGE);
        hulib_erase_itext(&mut W_CHAT);
        hulib_erase_text_line(&mut W_TITLE);
    }
}
