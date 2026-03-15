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
use std::sync::{Mutex, OnceLock};

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

// =============================================================================
// HuStuffState - thread-safe via OnceLock + Mutex
// =============================================================================

static HU_STUFF_STATE: OnceLock<Mutex<HuStuffState>> = OnceLock::new();

/// Safety: Raw pointers in HuStuffState are only used while holding the Mutex lock.
/// They point to zone-allocated data that outlives the state.
unsafe impl Send for HuStuffState {}

pub struct HuStuffState {
    pub hu_font: [*mut crate::rendering::patch_t; HU_FONTSIZE],
    pub w_title: HuTextline,
    pub message_on: Boolean,
    pub message_dontfuckwithme: Boolean,
    pub message_nottobefuckedwith: Boolean,
    pub w_message: HuStext,
    pub message_counter: i32,
    pub chat_on: Boolean,
    pub w_chat: HuItext,
    pub always_off: Boolean,
    pub headsupactive: Boolean,
    pub w_savestring: HuItext,
    pub save_string_active: bool,
    pub save_string_slot: usize,
    pub save_string_on: Boolean,
    pub chat_macros: [&'static str; 10],
    pub message_dontfuckwithme_extern: Boolean,
}

fn default_hu_textline() -> HuTextline {
    HuTextline {
        x: 0,
        y: 0,
        f: std::ptr::null_mut(),
        sc: 0,
        l: [0; HU_MAXLINELENGTH + 1],
        len: 0,
        needsupdate: 0,
    }
}

fn default_hu_itext() -> HuItext {
    HuItext {
        l: default_hu_textline(),
        lm: 0,
        on: std::ptr::null_mut(),
        laston: false,
    }
}

impl Default for HuStuffState {
    fn default() -> Self {
        Self {
            hu_font: [std::ptr::null_mut(); HU_FONTSIZE],
            w_title: default_hu_textline(),
            message_on: false,
            message_dontfuckwithme: false,
            message_nottobefuckedwith: false,
            w_message: HuStext {
                l: [
                    default_hu_textline(),
                    default_hu_textline(),
                    default_hu_textline(),
                    default_hu_textline(),
                ],
                h: 0,
                cl: 0,
                on: std::ptr::null_mut(),
                laston: false,
            },
            message_counter: 0,
            chat_on: false,
            w_chat: default_hu_itext(),
            always_off: false,
            headsupactive: false,
            w_savestring: default_hu_itext(),
            save_string_active: false,
            save_string_slot: 0,
            save_string_on: false,
            chat_macros: ["", "", "", "", "", "", "", "", "", ""],
            message_dontfuckwithme_extern: false,
        }
    }
}

fn get_hu_stuff_state() -> &'static Mutex<HuStuffState> {
    HU_STUFF_STATE.get_or_init(|| Mutex::new(HuStuffState::default()))
}

/// Access HuStuffState.
pub fn with_hu_stuff_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut HuStuffState) -> R,
{
    let mut guard = get_hu_stuff_state().lock().unwrap();
    f(&mut guard)
}

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
    with_hu_stuff_state(|st| {
        if st.hu_font[0].is_null() {
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
            let patch = st.hu_font[idx as usize];
            if patch.is_null() {
                cx += 4;
                continue;
            }
            let w = unsafe { (*patch).width as i32 };
            if cx + w > crate::doomdef::SCREENWIDTH {
                break;
            }
            crate::rendering::v_draw_patch_direct(cx, cy, patch);
            cx += w;
        }
    });
}

/// Draw a single character at (x,y). Returns width in pixels, or 4 for non-drawable chars.
/// Used by finale text scroll. Original: used in F_TextWrite.
pub fn hu_draw_char(x: i32, y: i32, c: u8) -> i32 {
    with_hu_stuff_state(|st| {
        if st.hu_font[0].is_null() {
            return 4;
        }
        let c_upper = if c >= b'a' && c <= b'z' { c - 32 } else { c };
        let idx = (c_upper as i32) - (HU_FONTSTART as i32);
        if idx < 0 || idx >= HU_FONTSIZE as i32 {
            return 4;
        }
        let patch = st.hu_font[idx as usize];
        if patch.is_null() {
            return 4;
        }
        let w = unsafe { (*patch).width as i32 };
        crate::rendering::v_draw_patch_direct(x, y, patch);
        w
    })
}

/// String width in pixels using HU_FONT.
pub fn hu_string_width(s: &str) -> i32 {
    with_hu_stuff_state(|st| {
        if st.hu_font[0].is_null() {
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
                let patch = st.hu_font[idx as usize];
                if patch.is_null() {
                    w += 4;
                } else {
                    w += unsafe { (*patch).width as i32 };
                }
            }
        }
        w
    })
}

pub fn hu_init() {
    let mut j = HU_FONTSTART;
    with_hu_stuff_state(|st| {
        for i in 0..HU_FONTSIZE {
            let name = format!("STCFN{:03}", j);
            let lump = w_cache_lump_name(deh_string(&name), PU_STATIC);
            st.hu_font[i] = lump.as_ptr_mut() as *mut crate::rendering::patch_t;
            j += 1;
        }
    });
}

pub fn hu_start() {
    with_hu_stuff_state(|st| {
        if st.headsupactive {
            st.headsupactive = false;
        }
        let _plr_idx = CONSOLEPLAYER as usize;
        st.message_on = false;
        st.message_dontfuckwithme = false;
        st.message_nottobefuckedwith = false;
        st.chat_on = false;

        let font_ptr = st.hu_font.as_mut_ptr();
        hulib_init_stext(
            &mut st.w_message,
            HU_MSGX,
            HU_MSGY,
            HU_MSGHEIGHT,
            font_ptr,
            HU_FONTSTART as i32,
            &mut st.message_on,
        );

        let hu_title_y = 167
            - if !st.hu_font[0].is_null() {
                unsafe { (*st.hu_font[0]).height as i32 }
            } else {
                8
            };
        hulib_init_text_line(&mut st.w_title, 0, hu_title_y, font_ptr, HU_FONTSTART as i32);

        let title = deh_string(map_title());
        for b in title.bytes() {
            hulib_add_char_to_text_line(&mut st.w_title, b);
        }

        let hu_input_y = HU_MSGY + HU_MSGHEIGHT * (8 + 1);
        hulib_init_itext(
            &mut st.w_chat,
            0,
            hu_input_y,
            font_ptr,
            HU_FONTSTART as i32,
            &mut st.chat_on,
        );

        st.headsupactive = true;
    });
}

pub fn hu_responder(_ev: &Event) -> Boolean {
    false
}

pub fn hu_ticker() {
    with_hu_stuff_state(|st| {
        if st.message_counter != 0 {
            st.message_counter -= 1;
            if st.message_counter == 0 {
                st.message_on = false;
                st.message_nottobefuckedwith = false;
            }
        }
    });
}

/// Set a message to display on the HUD (e.g. from pickup, etc.).
pub fn hu_set_message(msg: &str) {
    with_hu_stuff_state(|st| {
        hulib_add_message_to_stext(&mut st.w_message, "", msg);
        st.message_on = true;
        st.message_counter = HU_MSGTIMEOUT;
    });
}

pub fn hu_drawer() {
    with_hu_stuff_state(|st| {
        hulib_draw_stext(&st.w_message);
        hulib_draw_itext(&st.w_chat);
        if crate::doomstat::AUTOMAPACTIVE {
            hulib_draw_text_line(&st.w_title, false);
        }
    });
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
    with_hu_stuff_state(|st| {
        st.save_string_active = true;
        st.save_string_slot = slot;
        st.save_string_on = true;
        let font_ptr = st.hu_font.as_mut_ptr();
        hulib_init_itext(
            &mut st.w_savestring,
            x,
            y,
            font_ptr,
            HU_FONTSTART as i32,
            &mut st.save_string_on,
        );
        hulib_reset_itext(&mut st.w_savestring);
    });
}

/// Handle key for save string input. Returns Some(result) when in save string mode.
pub fn hu_save_string_key(ch: u8) -> Option<SaveStringKeyResult> {
    use crate::doomkeys;
    use crate::ui_hud::hu_lib::hulib_key_in_itext;
    with_hu_stuff_state(|st| {
        if !st.save_string_active {
            return None;
        }
        if ch == doomkeys::KEY_ESCAPE as u8 {
            st.save_string_active = false;
            return Some(SaveStringKeyResult::Cancel);
        }
        if ch == doomkeys::KEY_ENTER as u8 {
            let slot = st.save_string_slot;
            let desc = hu_save_string_desc_from_state(st);
            st.save_string_active = false;
            return Some(SaveStringKeyResult::Commit { slot, desc });
        }
        if hulib_key_in_itext(&mut st.w_savestring, ch) {
            return Some(SaveStringKeyResult::Consumed);
        }
        Some(SaveStringKeyResult::Consumed)
    })
}

pub fn hu_save_string_active() -> bool {
    with_hu_stuff_state(|st| st.save_string_active)
}

pub fn hu_save_string_slot() -> usize {
    with_hu_stuff_state(|st| st.save_string_slot)
}

fn hu_save_string_desc_from_state(st: &HuStuffState) -> String {
    use crate::ui_hud::hu_lib::hulib_itext_to_string;
    let s = hulib_itext_to_string(&st.w_savestring);
    s.chars()
        .take(crate::player::p_saveg::SAVESTRINGSIZE)
        .collect::<String>()
}

/// Get current save string text. Truncate to SAVESTRINGSIZE (24).
pub fn hu_save_string_desc() -> String {
    with_hu_stuff_state(|st| {
        if !st.save_string_active {
            return String::new();
        }
        hu_save_string_desc_from_state(st)
    })
}

pub fn hu_draw_save_string() {
    with_hu_stuff_state(|st| {
        if st.save_string_active {
            hulib_draw_itext(&st.w_savestring);
        }
    });
}

pub fn hu_cancel_save_string() {
    with_hu_stuff_state(|st| st.save_string_active = false);
}

pub fn hu_erase() {
    with_hu_stuff_state(|st| {
        hulib_erase_stext(&mut st.w_message);
        hulib_erase_itext(&mut st.w_chat);
        hulib_erase_text_line(&mut st.w_title);
    });
}
