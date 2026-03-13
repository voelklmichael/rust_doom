//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Intermission screen.
// Original: wi_stuff.h + wi_stuff.c

use crate::deh::deh_string;
use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::doomstat::{DEATHMATCH, GAMEMODE, LEVELTIME, NETGAME, TOTALITEMS, TOTALKILLS, TOTALSECRET, WMINFO};
use crate::game::d_mode::GameMode;
use crate::rendering::v_draw_patch;
use crate::sound::{s_start_sound, SfxEnum};
use crate::wad::w_cache_lump_name;
use crate::wad::w_release_lump_name;
use crate::z_zone::PU_STATIC;
use std::ptr;

// =============================================================================
// Public API (from wi_stuff.h)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WiStateEnum {
    NoState = -1,
    StatCount,
    ShowNextLoc,
}

// Layout constants (from wi_stuff.c)
const WI_TITLEY: i32 = 2;
const SP_STATSX: i32 = 50;
const SP_STATSY: i32 = 50;
const SP_TIMEX: i32 = 16;
const SP_TIMEY: i32 = SCREENHEIGHT - 32;
const SHOWNEXTLOCDELAY: i32 = 4; // seconds
const TICRATE: i32 = 35;

// =============================================================================
// Internal state (from wi_stuff.c)
// =============================================================================

static mut WI_STATE: WiStateEnum = WiStateEnum::NoState;
static mut WI_BCNT: i32 = 0;
static mut WI_CNT: i32 = 0;
static mut WI_BACKGROUND: *mut crate::rendering::patch_t = ptr::null_mut();

// Single-player stat count animation (sp_state 1–10)
static mut WI_SP_STATE: i32 = 1;
static mut WI_CNT_KILLS: i32 = -1;
static mut WI_CNT_ITEMS: i32 = -1;
static mut WI_CNT_SECRET: i32 = -1;
static mut WI_CNT_TIME: i32 = -1;
static mut WI_CNT_PAR: i32 = -1;
static mut WI_CNT_PAUSE: i32 = 0;
static mut WI_ACCELERATE: bool = false;
static mut WI_BACKGROUND_NAME: [u8; 9] = [0; 9];

// Stats patches
static mut WI_NUM: [*mut crate::rendering::patch_t; 10] = [ptr::null_mut(); 10];
static mut WI_MINUS: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_PERCENT: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_FINISHED: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_ENTERING: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_KILLS: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_ITEMS: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_SP_SECRET: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_TIMEPATCH: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_PAR: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_COLON: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_SUCKS: *mut crate::rendering::patch_t = ptr::null_mut();

// Level name patches (WILVxx or CWILVxx)
const NUMMAPS: usize = 9;
const NUMCMAPS: usize = 34;
static mut WI_LNAMES: [*mut crate::rendering::patch_t; NUMCMAPS] = [ptr::null_mut(); NUMCMAPS];
static mut WI_NUM_LNAMES: usize = NUMMAPS;

// =============================================================================
// Implementation (from wi_stuff.c)
// =============================================================================

fn wi_load_data(wbs: &crate::doomstat::WbStartStruct) {
    unsafe {
        let name = if GAMEMODE == GameMode::Commercial {
            "INTERPIC"
        } else if GAMEMODE == GameMode::Retail && wbs.epsd == 3 {
            "INTERPIC"
        } else {
            match wbs.epsd {
                0 => "WIMAP0",
                1 => "WIMAP1",
                2 => "WIMAP2",
                _ => "INTERPIC",
            }
        };
        let name_bytes = name.as_bytes();
        let n = name_bytes.len().min(8);
        WI_BACKGROUND_NAME[..n].copy_from_slice(&name_bytes[..n]);
        for i in n..8 {
            WI_BACKGROUND_NAME[i] = 0;
        }
        WI_BACKGROUND = w_cache_lump_name(deh_string(name), PU_STATIC) as *mut crate::rendering::patch_t;

        // Load stats patches
        for i in 0..10 {
            let lump = format!("WINUM{}", i);
            WI_NUM[i] = w_cache_lump_name(deh_string(&lump), PU_STATIC) as *mut crate::rendering::patch_t;
        }
        WI_MINUS = w_cache_lump_name(deh_string("WIMINUS"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_PERCENT = w_cache_lump_name(deh_string("WIPCNT"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_FINISHED = w_cache_lump_name(deh_string("WIF"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_ENTERING = w_cache_lump_name(deh_string("WIENTER"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_KILLS = w_cache_lump_name(deh_string("WIOSTK"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_ITEMS = w_cache_lump_name(deh_string("WIOSTI"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_SP_SECRET = w_cache_lump_name(deh_string("WISCRT2"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_TIMEPATCH = w_cache_lump_name(deh_string("WITIME"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_PAR = w_cache_lump_name(deh_string("WIPAR"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_COLON = w_cache_lump_name(deh_string("WICOLON"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_SUCKS = w_cache_lump_name(deh_string("WISUCKS"), PU_STATIC) as *mut crate::rendering::patch_t;

        // Level names
        if GAMEMODE == GameMode::Commercial {
            WI_NUM_LNAMES = NUMCMAPS;
            for i in 0..NUMCMAPS {
                let lump = format!("CWILV{:02}", i);
                WI_LNAMES[i] = w_cache_lump_name(deh_string(&lump), PU_STATIC) as *mut crate::rendering::patch_t;
            }
        } else {
            WI_NUM_LNAMES = NUMMAPS;
            for i in 0..NUMMAPS {
                let lump = format!("WILV{}{}", wbs.epsd, i);
                WI_LNAMES[i] = w_cache_lump_name(deh_string(&lump), PU_STATIC) as *mut crate::rendering::patch_t;
            }
        }
    }
}

fn wi_unload_data() {
    unsafe {
        if !WI_BACKGROUND.is_null() {
            let len = WI_BACKGROUND_NAME.iter().position(|&b| b == 0).unwrap_or(8);
            let name = std::str::from_utf8_unchecked(&WI_BACKGROUND_NAME[..len]);
            let name = if name.is_empty() { "INTERPIC" } else { name };
            w_release_lump_name(deh_string(name));
            WI_BACKGROUND = ptr::null_mut();
        }
        // Note: stats patches not released for simplicity; could add full unload
    }
}

/// Draw number (right-aligned). Returns new x.
fn wi_draw_num(x: i32, y: i32, n: i32, digits: i32) -> i32 {
    unsafe {
        let p0 = WI_NUM[0];
        if p0.is_null() {
            return x;
        }
        let fontwidth = (*p0).width as i32;
        let mut n = n;
        let mut digits = digits;
        if digits < 0 {
            digits = if n == 0 {
                1
            } else {
                let mut d = 0;
                let mut t = n;
                while t != 0 {
                    t /= 10;
                    d += 1;
                }
                d
            };
        }
        let neg = n < 0;
        if neg {
            n = -n;
        }
        let mut x = x;
        while digits > 0 {
            x -= fontwidth;
            let digit = (n % 10) as usize;
            v_draw_patch(x, y, WI_NUM[digit]);
            n /= 10;
            digits -= 1;
        }
        if neg {
            x -= 8;
            v_draw_patch(x, y, WI_MINUS);
        }
        x
    }
}

/// Draw percentage (right-aligned).
fn wi_draw_percent(x: i32, y: i32, p: i32) {
    unsafe {
        if p < 0 || WI_PERCENT.is_null() {
            return;
        }
        v_draw_patch(x, y, WI_PERCENT);
        wi_draw_num(x, y, p, -1);
    }
}

/// Draw time as MM:SS (or "sucks" if overflow).
fn wi_draw_time(mut x: i32, y: i32, t: i32) {
    unsafe {
        if t < 0 {
            return;
        }
        if t <= 61 * 59 {
            let mut div = 1i32;
            let colon_w = if WI_COLON.is_null() {
                8
            } else {
                (*WI_COLON).width as i32
            };
            loop {
                let n = (t / div) % 60;
                x = wi_draw_num(x, y, n, 2) - colon_w;
                div *= 60;
                if div == 60 || t / div != 0 {
                    if !WI_COLON.is_null() {
                        v_draw_patch(x, y, WI_COLON);
                    }
                }
                if t / div == 0 {
                    break;
                }
            }
        } else if !WI_SUCKS.is_null() {
            let w = (*WI_SUCKS).width as i32;
            v_draw_patch(x - w, y, WI_SUCKS);
        }
    }
}

/// Draw "<Levelname> Finished!"
fn wi_draw_lf() {
    unsafe {
        let wbs = &WMINFO;
        let last = wbs.last as usize;
        if last >= WI_NUM_LNAMES {
            return;
        }
        let lname = WI_LNAMES[last];
        if lname.is_null() || WI_FINISHED.is_null() {
            return;
        }
        let mut y = WI_TITLEY;
        let lw = (*lname).width as i32;
        let lh = (*lname).height as i32;
        v_draw_patch((SCREENWIDTH - lw) / 2, y, lname);
        y += (5 * lh) / 4;
        let fw = (*WI_FINISHED).width as i32;
        v_draw_patch((SCREENWIDTH - fw) / 2, y, WI_FINISHED);
    }
}

/// Draw "Entering <LevelName>"
fn wi_draw_el() {
    unsafe {
        let wbs = &WMINFO;
        let next = wbs.next as usize;
        if next >= WI_NUM_LNAMES {
            return;
        }
        let lname = WI_LNAMES[next];
        if WI_ENTERING.is_null() || lname.is_null() {
            return;
        }
        let mut y = WI_TITLEY;
        let ew = (*WI_ENTERING).width as i32;
        v_draw_patch((SCREENWIDTH - ew) / 2, y, WI_ENTERING);
        let lh = (*lname).height as i32;
        y += (5 * lh) / 4;
        let lw = (*lname).width as i32;
        v_draw_patch((SCREENWIDTH - lw) / 2, y, lname);
    }
}

/// Draw stats (kills, items, secret, time) using animated cnt_* values.
fn wi_draw_stats() {
    unsafe {
        let wbs = &WMINFO;
        let p0 = WI_NUM[0];
        if p0.is_null() {
            return;
        }
        let lh = (3 * (*p0).height as i32) / 2;

        wi_slam_background();
        wi_draw_lf();

        if !WI_KILLS.is_null() {
            v_draw_patch(SP_STATSX, SP_STATSY, WI_KILLS);
            wi_draw_percent(SCREENWIDTH - SP_STATSX, SP_STATSY, WI_CNT_KILLS);
        }
        if !WI_ITEMS.is_null() {
            v_draw_patch(SP_STATSX, SP_STATSY + lh, WI_ITEMS);
            wi_draw_percent(SCREENWIDTH - SP_STATSX, SP_STATSY + lh, WI_CNT_ITEMS);
        }
        if !WI_SP_SECRET.is_null() {
            v_draw_patch(SP_STATSX, SP_STATSY + 2 * lh, WI_SP_SECRET);
            wi_draw_percent(SCREENWIDTH - SP_STATSX, SP_STATSY + 2 * lh, WI_CNT_SECRET);
        }
        if !WI_TIMEPATCH.is_null() {
            v_draw_patch(SP_TIMEX, SP_TIMEY, WI_TIMEPATCH);
            wi_draw_time(SCREENWIDTH / 2 - SP_TIMEX, SP_TIMEY, WI_CNT_TIME);
        }
        // Par time (episodes 0-2 only, not Thy Flesh Consumed)
        if wbs.epsd < 3 && !WI_PAR.is_null() {
            v_draw_patch(SCREENWIDTH / 2 + SP_TIMEX, SP_TIMEY, WI_PAR);
            wi_draw_time(SCREENWIDTH - SP_TIMEX, SP_TIMEY, WI_CNT_PAR);
        }
    }
}

fn wi_slam_background() {
    unsafe {
        if !WI_BACKGROUND.is_null() {
            v_draw_patch(0, 0, WI_BACKGROUND);
        }
    }
}

/// Call when player presses attack or use during intermission to skip stat animation.
pub fn wi_set_accelerate() {
    unsafe {
        WI_ACCELERATE = true;
    }
}

fn wi_init_stats() {
    unsafe {
        WI_SP_STATE = 1;
        WI_CNT_KILLS = -1;
        WI_CNT_ITEMS = -1;
        WI_CNT_SECRET = -1;
        WI_CNT_TIME = -1;
        WI_CNT_PAR = -1;
        WI_CNT_PAUSE = TICRATE;
    }
}

fn wi_update_stats() {
    unsafe {
        let wbs = &WMINFO;
        let me = wbs.pnum as usize;
        if me >= crate::doomdef::MAXPLAYERS {
            return;
        }
        let plyr = &wbs.plyr[me];
        let maxk = wbs.maxkills.max(1);
        let maxi = wbs.maxitems.max(1);
        let maxs = wbs.maxsecret.max(1);
        let target_kills = (plyr.kills * 100) / maxk;
        let target_items = (plyr.items * 100) / maxi;
        let target_secret = (plyr.secret * 100) / maxs;
        let target_time = plyr.time / TICRATE;
        let target_par = wbs.partime / TICRATE;

        if WI_ACCELERATE && WI_SP_STATE != 10 {
            WI_ACCELERATE = false;
            WI_CNT_KILLS = target_kills;
            WI_CNT_ITEMS = target_items;
            WI_CNT_SECRET = target_secret;
            WI_CNT_TIME = target_time;
            WI_CNT_PAR = target_par;
            s_start_sound(None, SfxEnum::Barexp as i32, None);
            WI_SP_STATE = 10;
        }

        if WI_SP_STATE == 2 {
            WI_CNT_KILLS += 2;
            if (WI_BCNT & 3) == 0 {
                s_start_sound(None, SfxEnum::Pistol as i32, None);
            }
            if WI_CNT_KILLS >= target_kills {
                WI_CNT_KILLS = target_kills;
                s_start_sound(None, SfxEnum::Barexp as i32, None);
                WI_SP_STATE += 1;
            }
        } else if WI_SP_STATE == 4 {
            WI_CNT_ITEMS += 2;
            if (WI_BCNT & 3) == 0 {
                s_start_sound(None, SfxEnum::Pistol as i32, None);
            }
            if WI_CNT_ITEMS >= target_items {
                WI_CNT_ITEMS = target_items;
                s_start_sound(None, SfxEnum::Barexp as i32, None);
                WI_SP_STATE += 1;
            }
        } else if WI_SP_STATE == 6 {
            WI_CNT_SECRET += 2;
            if (WI_BCNT & 3) == 0 {
                s_start_sound(None, SfxEnum::Pistol as i32, None);
            }
            if WI_CNT_SECRET >= target_secret {
                WI_CNT_SECRET = target_secret;
                s_start_sound(None, SfxEnum::Barexp as i32, None);
                WI_SP_STATE += 1;
            }
        } else if WI_SP_STATE == 8 {
            if (WI_BCNT & 3) == 0 {
                s_start_sound(None, SfxEnum::Pistol as i32, None);
            }
            WI_CNT_TIME += 3;
            if WI_CNT_TIME >= target_time {
                WI_CNT_TIME = target_time;
            }
            WI_CNT_PAR += 3;
            if WI_CNT_PAR >= target_par {
                WI_CNT_PAR = target_par;
                if WI_CNT_TIME >= target_time {
                    s_start_sound(None, SfxEnum::Barexp as i32, None);
                    WI_SP_STATE += 1;
                }
            }
        } else if WI_SP_STATE == 10 {
            if WI_ACCELERATE {
                WI_ACCELERATE = false;
                s_start_sound(None, SfxEnum::Sgcock as i32, None);
                if GAMEMODE == GameMode::Commercial {
                    WI_STATE = WiStateEnum::NoState;
                } else {
                    WI_STATE = WiStateEnum::ShowNextLoc;
                    WI_CNT = SHOWNEXTLOCDELAY * TICRATE;
                }
            }
        } else if (WI_SP_STATE & 1) != 0 {
            WI_CNT_PAUSE -= 1;
            if WI_CNT_PAUSE <= 0 {
                WI_SP_STATE += 1;
                WI_CNT_PAUSE = TICRATE;
            }
        }
    }
}

pub fn wi_start(wbstartstruct: &crate::doomstat::WbStartStruct) {
    unsafe {
        WI_STATE = WiStateEnum::StatCount;
        WI_BCNT = 0;
        WI_CNT = SHOWNEXTLOCDELAY * TICRATE;
        WI_ACCELERATE = false;
        wi_load_data(wbstartstruct);
        let deathmatch = DEATHMATCH != 0;
        let netgame = NETGAME;
        if !deathmatch && !netgame {
            wi_init_stats();
        }
    }
}

pub fn wi_end() {
    wi_unload_data();
    unsafe {
        WI_STATE = WiStateEnum::NoState;
    }
}

pub fn wi_ticker() {
    unsafe {
        WI_BCNT += 1;
        match WI_STATE {
            WiStateEnum::StatCount => {
                let deathmatch = DEATHMATCH != 0;
                let netgame = NETGAME;
                if !deathmatch && !netgame {
                    wi_update_stats();
                } else {
                    // Deathmatch/netgame: simple countdown (not yet implemented)
                    WI_CNT -= 1;
                    if WI_CNT <= 0 {
                        WI_STATE = WiStateEnum::ShowNextLoc;
                        WI_CNT = SHOWNEXTLOCDELAY * TICRATE;
                    }
                }
            }
            WiStateEnum::ShowNextLoc => {
                WI_CNT -= 1;
                if WI_CNT <= 0 {
                    WI_STATE = WiStateEnum::NoState;
                }
            }
            WiStateEnum::NoState => {}
        }
    }
}

pub fn wi_drawer() {
    match unsafe { WI_STATE } {
        WiStateEnum::StatCount => {
            wi_draw_stats();
        }
        WiStateEnum::ShowNextLoc => {
            wi_slam_background();
            wi_draw_el();
        }
        WiStateEnum::NoState => {}
    }
}
