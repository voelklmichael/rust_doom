//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Intermission screen.
// Original: wi_stuff.h + wi_stuff.c

use crate::deh::deh_string;
use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::doomstat::{DEATHMATCH, GAMEMODE, NETGAME, WMINFO};
use crate::game::d_mode::GameMode;
use crate::m_random::m_random;
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

// Deathmatch/netgame patches
static mut WI_KILLERS: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_VICTIMS: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_TOTAL: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_FRAGS: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_STAR: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_BSTAR: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_P: [*mut crate::rendering::patch_t; crate::doomdef::MAXPLAYERS] =
    [ptr::null_mut(); crate::doomdef::MAXPLAYERS];

// Deathmatch state
const DM_MATRIXX: i32 = 42;
const DM_MATRIXY: i32 = 68;
const DM_SPACINGX: i32 = 40;
const DM_TOTALSX: i32 = 269;
const DM_KILLERSX: i32 = 10;
const DM_KILLERSY: i32 = 100;
const DM_VICTIMSX: i32 = 5;
const DM_VICTIMSY: i32 = 50;
const WI_SPACINGY: i32 = 33;

static mut WI_DM_STATE: i32 = 1;
static mut WI_DM_FRAGS: [[i32; crate::doomdef::MAXPLAYERS]; crate::doomdef::MAXPLAYERS] =
    [[0; crate::doomdef::MAXPLAYERS]; crate::doomdef::MAXPLAYERS];
static mut WI_DM_TOTALS: [i32; crate::doomdef::MAXPLAYERS] = [0; crate::doomdef::MAXPLAYERS];

// Netgame state
const NG_STATSX: i32 = 32;
const NG_STATSY: i32 = 50;
const NG_SPACINGX: i32 = 64;
const NG_SPACINGY: i32 = 33;

static mut WI_NG_STATE: i32 = 1;
static mut WI_NG_CNT_KILLS: [i32; crate::doomdef::MAXPLAYERS] = [-1; crate::doomdef::MAXPLAYERS];
static mut WI_NG_CNT_ITEMS: [i32; crate::doomdef::MAXPLAYERS] = [-1; crate::doomdef::MAXPLAYERS];
static mut WI_NG_CNT_SECRET: [i32; crate::doomdef::MAXPLAYERS] = [-1; crate::doomdef::MAXPLAYERS];
static mut WI_NG_CNT_FRAGS: [i32; crate::doomdef::MAXPLAYERS] = [-1; crate::doomdef::MAXPLAYERS];

// Level name patches (WILVxx or CWILVxx)
const NUMMAPS: usize = 9;
const NUMCMAPS: usize = 34;
const NUMEPISODES: usize = 3;
static mut WI_LNAMES: [*mut crate::rendering::patch_t; NUMCMAPS] = [ptr::null_mut(); NUMCMAPS];
static mut WI_NUM_LNAMES: usize = NUMMAPS;

// Animated background (episodes 0-2 only)
#[derive(Clone, Copy)]
#[repr(i32)]
enum AnimType {
    Always = 0,
    Random = 1,
    Level = 2,
}

#[derive(Clone, Copy)]
struct WiAnim {
    anim_type: AnimType,
    period: i32,
    nanims: i32,
    loc_x: i32,
    loc_y: i32,
    data1: i32,
    data2: i32,
    p: [*mut crate::rendering::patch_t; 3],
    nexttic: i32,
    ctr: i32,
}

const NUMANIMS: [usize; NUMEPISODES] = [10, 9, 6];
static mut WI_ANIMS_0: [WiAnim; 10] = [WiAnim {
    anim_type: AnimType::Always,
    period: 0,
    nanims: 0,
    loc_x: 0,
    loc_y: 0,
    data1: 0,
    data2: 0,
    p: [ptr::null_mut(); 3],
    nexttic: 0,
    ctr: 0,
}; 10];
static mut WI_ANIMS_1: [WiAnim; 9] = [WiAnim {
    anim_type: AnimType::Always,
    period: 0,
    nanims: 0,
    loc_x: 0,
    loc_y: 0,
    data1: 0,
    data2: 0,
    p: [ptr::null_mut(); 3],
    nexttic: 0,
    ctr: 0,
}; 9];
static mut WI_ANIMS_2: [WiAnim; 6] = [WiAnim {
    anim_type: AnimType::Always,
    period: 0,
    nanims: 0,
    loc_x: 0,
    loc_y: 0,
    data1: 0,
    data2: 0,
    p: [ptr::null_mut(); 3],
    nexttic: 0,
    ctr: 0,
}; 6];

fn wi_anim_epsd0() {
    const P: i32 = TICRATE / 3;
    let a = [
        (AnimType::Always, P, 3, 224, 104),
        (AnimType::Always, P, 3, 184, 160),
        (AnimType::Always, P, 3, 112, 136),
        (AnimType::Always, P, 3, 72, 112),
        (AnimType::Always, P, 3, 88, 96),
        (AnimType::Always, P, 3, 64, 48),
        (AnimType::Always, P, 3, 192, 40),
        (AnimType::Always, P, 3, 136, 16),
        (AnimType::Always, P, 3, 80, 16),
        (AnimType::Always, P, 3, 64, 24),
    ];
    unsafe {
        for (i, (t, period, nanims, x, y)) in a.iter().enumerate() {
            WI_ANIMS_0[i] = WiAnim {
                anim_type: *t,
                period: *period,
                nanims: *nanims,
                loc_x: *x,
                loc_y: *y,
                data1: 0,
                data2: 0,
                p: [ptr::null_mut(); 3],
                nexttic: 0,
                ctr: 0,
            };
        }
    }
}

fn wi_anim_epsd1() {
    const P: i32 = TICRATE / 3;
    let a = [
        (AnimType::Level, P, 1, 128, 136, 1),
        (AnimType::Level, P, 1, 128, 136, 2),
        (AnimType::Level, P, 1, 128, 136, 3),
        (AnimType::Level, P, 1, 128, 136, 4),
        (AnimType::Level, P, 1, 128, 136, 5),
        (AnimType::Level, P, 1, 128, 136, 6),
        (AnimType::Level, P, 1, 128, 136, 7),
        (AnimType::Level, P, 3, 192, 144, 8),
        (AnimType::Level, P, 1, 128, 136, 8),
    ];
    unsafe {
        for (i, (t, period, nanims, x, y, d1)) in a.iter().enumerate() {
            WI_ANIMS_1[i] = WiAnim {
                anim_type: *t,
                period: *period,
                nanims: *nanims,
                loc_x: *x,
                loc_y: *y,
                data1: *d1,
                data2: 0,
                p: [ptr::null_mut(); 3],
                nexttic: 0,
                ctr: 0,
            };
        }
    }
}

fn wi_anim_epsd2() {
    const P: i32 = TICRATE / 3;
    let a = [
        (AnimType::Always, P, 3, 104, 168),
        (AnimType::Always, P, 3, 40, 136),
        (AnimType::Always, P, 3, 160, 96),
        (AnimType::Always, P, 3, 104, 80),
        (AnimType::Always, P, 3, 120, 32),
        (AnimType::Always, TICRATE / 4, 3, 40, 0),
    ];
    unsafe {
        for (i, (t, period, nanims, x, y)) in a.iter().enumerate() {
            WI_ANIMS_2[i] = WiAnim {
                anim_type: *t,
                period: *period,
                nanims: *nanims,
                loc_x: *x,
                loc_y: *y,
                data1: 0,
                data2: 0,
                p: [ptr::null_mut(); 3],
                nexttic: 0,
                ctr: 0,
            };
        }
    }
}

fn wi_load_anim_patches(epsd: i32) {
    unsafe {
        let num_anims = NUMANIMS[epsd as usize];
        let anims: *mut WiAnim = match epsd {
            0 => WI_ANIMS_0.as_mut_ptr(),
            1 => WI_ANIMS_1.as_mut_ptr(),
            2 => WI_ANIMS_2.as_mut_ptr(),
            _ => return,
        };
        for j in 0..num_anims {
            let a = &mut *anims.add(j);
            let nanims = a.nanims as usize;
            for i in 0..nanims {
                if epsd == 1 && j == 8 {
                    a.p[i] = (*anims.add(4)).p[i];
                } else {
                    let lump = format!("WIA{}{:02}{:02}", epsd, j, i);
                    a.p[i] = w_cache_lump_name(deh_string(&lump), PU_STATIC) as *mut crate::rendering::patch_t;
                }
            }
        }
    }
}

fn wi_init_animated_back() {
    unsafe {
        if GAMEMODE == GameMode::Commercial {
            return;
        }
        let wbs = &WMINFO;
        if wbs.epsd > 2 {
            return;
        }
        let bcnt = WI_BCNT;
        let num_anims = NUMANIMS[wbs.epsd as usize];
        let anims: *mut WiAnim = match wbs.epsd {
            0 => WI_ANIMS_0.as_mut_ptr(),
            1 => WI_ANIMS_1.as_mut_ptr(),
            2 => WI_ANIMS_2.as_mut_ptr(),
            _ => return,
        };
        for i in 0..num_anims {
            let a = &mut *anims.add(i);
            a.ctr = -1;
            a.nexttic = match a.anim_type {
                AnimType::Always => bcnt + 1 + (m_random() % a.period.max(1)),
                AnimType::Random => bcnt + 1 + a.data2 + (m_random() % a.data1.max(1)),
                AnimType::Level => bcnt + 1,
            };
        }
    }
}

fn wi_update_animated_back() {
    unsafe {
        if GAMEMODE == GameMode::Commercial {
            return;
        }
        let wbs = &WMINFO;
        if wbs.epsd > 2 {
            return;
        }
        let bcnt = WI_BCNT;
        let state = WI_STATE;
        let num_anims = NUMANIMS[wbs.epsd as usize];
        let anims: *mut WiAnim = match wbs.epsd {
            0 => WI_ANIMS_0.as_mut_ptr(),
            1 => WI_ANIMS_1.as_mut_ptr(),
            2 => WI_ANIMS_2.as_mut_ptr(),
            _ => return,
        };
        for i in 0..num_anims {
            let a = &mut *anims.add(i);
            if bcnt == a.nexttic {
                match a.anim_type {
                    AnimType::Always => {
                        a.ctr += 1;
                        if a.ctr >= a.nanims {
                            a.ctr = 0;
                        }
                        a.nexttic = bcnt + a.period;
                    }
                    AnimType::Random => {
                        a.ctr += 1;
                        if a.ctr == a.nanims {
                            a.ctr = -1;
                            a.nexttic = bcnt + a.data2 + (m_random() % a.data1);
                        } else {
                            a.nexttic = bcnt + a.period;
                        }
                    }
                    AnimType::Level => {
                        if !(state == WiStateEnum::StatCount && i == 7) && wbs.next == a.data1 {
                            a.ctr += 1;
                            if a.ctr >= a.nanims {
                                a.ctr = a.nanims - 1;
                            }
                            a.nexttic = bcnt + a.period;
                        }
                    }
                }
            }
        }
    }
}

fn wi_draw_animated_back() {
    unsafe {
        if GAMEMODE == GameMode::Commercial {
            return;
        }
        let wbs = &WMINFO;
        if wbs.epsd > 2 {
            return;
        }
        let num_anims = NUMANIMS[wbs.epsd as usize];
        let anims: *mut WiAnim = match wbs.epsd {
            0 => WI_ANIMS_0.as_mut_ptr(),
            1 => WI_ANIMS_1.as_mut_ptr(),
            2 => WI_ANIMS_2.as_mut_ptr(),
            _ => return,
        };
        for i in 0..num_anims {
            let a = &*anims.add(i);
            if a.ctr >= 0 && !a.p[a.ctr as usize].is_null() {
                v_draw_patch(a.loc_x, a.loc_y, a.p[a.ctr as usize]);
            }
        }
    }
}

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

        // Deathmatch/netgame patches
        WI_KILLERS = w_cache_lump_name(deh_string("WIKILRS"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_VICTIMS = w_cache_lump_name(deh_string("WIVCTMS"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_TOTAL = w_cache_lump_name(deh_string("WIMSTT"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_FRAGS = w_cache_lump_name(deh_string("WIFRGS"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_STAR = w_cache_lump_name(deh_string("STFST01"), PU_STATIC) as *mut crate::rendering::patch_t;
        WI_BSTAR = w_cache_lump_name(deh_string("STFDEAD0"), PU_STATIC) as *mut crate::rendering::patch_t;
        for i in 0..crate::doomdef::MAXPLAYERS {
            let lump = format!("WIP{}", i);
            WI_P[i] = w_cache_lump_name(deh_string(&lump), PU_STATIC) as *mut crate::rendering::patch_t;
        }

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
        if GAMEMODE != GameMode::Commercial && wbs.epsd < 3 {
            wi_anim_epsd0();
            wi_anim_epsd1();
            wi_anim_epsd2();
            wi_load_anim_patches(wbs.epsd);
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
        wi_draw_animated_back();
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
    wi_init_animated_back();
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

fn wi_frag_sum(playernum: usize) -> i32 {
    unsafe {
        let wbs = &WMINFO;
        let mut sum = 0i32;
        if playernum < crate::doomdef::MAXPLAYERS {
            for j in 0..crate::doomdef::MAXPLAYERS {
                sum += wbs.plyr[playernum].frags[j];
            }
        }
        sum
    }
}

fn wi_init_deathmatch_stats() {
    unsafe {
        let wbs = &WMINFO;
        WI_DM_STATE = 1;
        WI_CNT_PAUSE = TICRATE;
        for i in 0..crate::doomdef::MAXPLAYERS {
            for j in 0..crate::doomdef::MAXPLAYERS {
                WI_DM_FRAGS[i][j] = 0;
            }
            WI_DM_TOTALS[i] = 0;
        }
    }
}

fn wi_update_deathmatch_stats() {
    wi_update_animated_back();
    unsafe {
        let wbs = &WMINFO;
        let me = wbs.pnum as usize;
        if WI_ACCELERATE && WI_DM_STATE != 4 {
            WI_ACCELERATE = false;
            for i in 0..crate::doomdef::MAXPLAYERS {
                if wbs.plyr[i].in_game != 0 {
                    for j in 0..crate::doomdef::MAXPLAYERS {
                        WI_DM_FRAGS[i][j] = wbs.plyr[i].frags[j];
                    }
                    WI_DM_TOTALS[i] = wi_frag_sum(i);
                }
            }
            s_start_sound(None, SfxEnum::Barexp as i32, None);
            WI_DM_STATE = 4;
        }
        if WI_DM_STATE == 2 {
            let mut still_ticking = false;
            for i in 0..crate::doomdef::MAXPLAYERS {
                if wbs.plyr[i].in_game == 0 {
                    continue;
                }
                let mut tot = 0i32;
                for j in 0..crate::doomdef::MAXPLAYERS {
                    let target = wbs.plyr[i].frags[j];
                    if WI_DM_FRAGS[i][j] < target {
                        WI_DM_FRAGS[i][j] = (WI_DM_FRAGS[i][j] + 1).min(target);
                        still_ticking = true;
                    }
                    WI_DM_FRAGS[i][j] = WI_DM_FRAGS[i][j].clamp(-99, 99);
                    tot += WI_DM_FRAGS[i][j];
                }
                WI_DM_TOTALS[i] = tot.clamp(-99, 99);
            }
            if !still_ticking {
                s_start_sound(None, SfxEnum::Barexp as i32, None);
                WI_DM_STATE += 1;
            }
        } else if WI_DM_STATE == 4 {
            if WI_ACCELERATE {
                WI_ACCELERATE = false;
                s_start_sound(None, SfxEnum::Slop as i32, None);
                if GAMEMODE == GameMode::Commercial {
                    WI_STATE = WiStateEnum::NoState;
                } else {
                    WI_STATE = WiStateEnum::ShowNextLoc;
                    WI_CNT = SHOWNEXTLOCDELAY * TICRATE;
                    wi_init_animated_back();
                }
            }
        } else if (WI_DM_STATE & 1) != 0 {
            WI_CNT_PAUSE -= 1;
            if WI_CNT_PAUSE <= 0 {
                WI_DM_STATE += 1;
                WI_CNT_PAUSE = TICRATE;
            }
        }
    }
}

fn wi_draw_deathmatch_stats() {
    unsafe {
        let wbs = &WMINFO;
        let me = wbs.pnum as usize;
        let p0 = WI_NUM[0];
        if p0.is_null() {
            return;
        }

        wi_slam_background();
        wi_draw_animated_back();
        wi_draw_lf();

        if !WI_TOTAL.is_null() {
            let tw = (*WI_TOTAL).width as i32;
            v_draw_patch(DM_TOTALSX - tw / 2, DM_MATRIXY - WI_SPACINGY + 10, WI_TOTAL);
        }
        if !WI_KILLERS.is_null() {
            v_draw_patch(DM_KILLERSX, DM_KILLERSY, WI_KILLERS);
        }
        if !WI_VICTIMS.is_null() {
            v_draw_patch(DM_VICTIMSX, DM_VICTIMSY, WI_VICTIMS);
        }

        let mut x = DM_MATRIXX + DM_SPACINGX;
        let mut y = DM_MATRIXY;
        for i in 0..crate::doomdef::MAXPLAYERS {
            if wbs.plyr[i].in_game == 0 {
                x += DM_SPACINGX;
                y += WI_SPACINGY;
                continue;
            }
            let pi = WI_P[i];
            if !pi.is_null() {
                let pw = (*pi).width as i32;
                v_draw_patch(x - pw / 2, DM_MATRIXY - WI_SPACINGY, pi);
                v_draw_patch(DM_MATRIXX - pw / 2, y, pi);
                if i == me {
                    if !WI_BSTAR.is_null() {
                        let bw = (*WI_BSTAR).width as i32;
                        v_draw_patch(x - bw / 2, DM_MATRIXY - WI_SPACINGY, WI_BSTAR);
                    }
                    if !WI_STAR.is_null() {
                        let sw = (*WI_STAR).width as i32;
                        v_draw_patch(DM_MATRIXX - sw / 2, y, WI_STAR);
                    }
                }
            }
            x += DM_SPACINGX;
            y += WI_SPACINGY;
        }

        y = DM_MATRIXY + 10;
        for i in 0..crate::doomdef::MAXPLAYERS {
            if wbs.plyr[i].in_game == 0 {
                continue;
            }
            x = DM_MATRIXX + DM_SPACINGX;
            for j in 0..crate::doomdef::MAXPLAYERS {
                if wbs.plyr[j].in_game == 0 {
                    x += DM_SPACINGX;
                    continue;
                }
                wi_draw_num(x, y, WI_DM_FRAGS[i][j], -1);
                x += DM_SPACINGX;
            }
            wi_draw_num(DM_TOTALSX, y, WI_DM_TOTALS[i], -1);
            y += WI_SPACINGY;
        }
    }
}

fn wi_init_netgame_stats() {
    wi_init_animated_back();
    unsafe {
        let wbs = &WMINFO;
        WI_NG_STATE = 1;
        WI_CNT_PAUSE = TICRATE;
        for i in 0..crate::doomdef::MAXPLAYERS {
            WI_NG_CNT_KILLS[i] = -1;
            WI_NG_CNT_ITEMS[i] = -1;
            WI_NG_CNT_SECRET[i] = -1;
            WI_NG_CNT_FRAGS[i] = -1;
        }
        for i in 0..crate::doomdef::MAXPLAYERS {
            if wbs.plyr[i].in_game != 0 {
                let maxk = wbs.maxkills.max(1);
                let maxi = wbs.maxitems.max(1);
                let maxs = wbs.maxsecret.max(1);
                WI_NG_CNT_KILLS[i] = (wbs.plyr[i].kills * 100) / maxk;
                WI_NG_CNT_ITEMS[i] = (wbs.plyr[i].items * 100) / maxi;
                WI_NG_CNT_SECRET[i] = (wbs.plyr[i].secret * 100) / maxs;
                WI_NG_CNT_FRAGS[i] = wi_frag_sum(i);
            }
        }
    }
}

fn wi_update_netgame_stats() {
    wi_update_animated_back();
    unsafe {
        let wbs = &WMINFO;
        if WI_ACCELERATE && WI_NG_STATE != 10 {
            WI_ACCELERATE = false;
            for i in 0..crate::doomdef::MAXPLAYERS {
                if wbs.plyr[i].in_game != 0 {
                    let maxk = wbs.maxkills.max(1);
                    let maxi = wbs.maxitems.max(1);
                    let maxs = wbs.maxsecret.max(1);
                    WI_NG_CNT_KILLS[i] = (wbs.plyr[i].kills * 100) / maxk;
                    WI_NG_CNT_ITEMS[i] = (wbs.plyr[i].items * 100) / maxi;
                    WI_NG_CNT_SECRET[i] = (wbs.plyr[i].secret * 100) / maxs;
                    WI_NG_CNT_FRAGS[i] = wi_frag_sum(i);
                }
            }
            s_start_sound(None, SfxEnum::Barexp as i32, None);
            WI_NG_STATE = 10;
        }
        if WI_NG_STATE == 2 {
            let mut still_ticking = false;
            for i in 0..crate::doomdef::MAXPLAYERS {
                if wbs.plyr[i].in_game == 0 {
                    continue;
                }
                let maxk = wbs.maxkills.max(1);
                let maxi = wbs.maxitems.max(1);
                let maxs = wbs.maxsecret.max(1);
                let tk = (wbs.plyr[i].kills * 100) / maxk;
                let ti = (wbs.plyr[i].items * 100) / maxi;
                let ts = (wbs.plyr[i].secret * 100) / maxs;
                if WI_NG_CNT_KILLS[i] < tk {
                    WI_NG_CNT_KILLS[i] = (WI_NG_CNT_KILLS[i] + 2).min(tk);
                    still_ticking = true;
                }
                if WI_NG_CNT_ITEMS[i] < ti {
                    WI_NG_CNT_ITEMS[i] = (WI_NG_CNT_ITEMS[i] + 2).min(ti);
                    still_ticking = true;
                }
                if WI_NG_CNT_SECRET[i] < ts {
                    WI_NG_CNT_SECRET[i] = (WI_NG_CNT_SECRET[i] + 2).min(ts);
                    still_ticking = true;
                }
                WI_NG_CNT_FRAGS[i] = wi_frag_sum(i);
            }
            if !still_ticking {
                s_start_sound(None, SfxEnum::Barexp as i32, None);
                WI_NG_STATE += 1;
            }
        } else if WI_NG_STATE == 10 {
            if WI_ACCELERATE {
                WI_ACCELERATE = false;
                s_start_sound(None, SfxEnum::Sgcock as i32, None);
                if GAMEMODE == GameMode::Commercial {
                    WI_STATE = WiStateEnum::NoState;
                } else {
                    WI_STATE = WiStateEnum::ShowNextLoc;
                    WI_CNT = SHOWNEXTLOCDELAY * TICRATE;
                    wi_init_animated_back();
                }
            }
        } else if (WI_NG_STATE & 1) != 0 {
            WI_CNT_PAUSE -= 1;
            if WI_CNT_PAUSE <= 0 {
                WI_NG_STATE += 1;
                WI_CNT_PAUSE = TICRATE;
            }
        }
    }
}

fn wi_draw_netgame_stats() {
    unsafe {
        let wbs = &WMINFO;
        let me = wbs.pnum as usize;
        let p0 = WI_NUM[0];
        if p0.is_null() || WI_PERCENT.is_null() {
            return;
        }
        let lh = (3 * (*p0).height as i32) / 2;
        let pwidth = (*WI_PERCENT).width as i32;

        wi_slam_background();
        wi_draw_animated_back();
        wi_draw_lf();

        if !WI_KILLS.is_null() {
            v_draw_patch(NG_STATSX + NG_SPACINGX - (*WI_KILLS).width as i32, NG_STATSY, WI_KILLS);
        }
        if !WI_ITEMS.is_null() {
            v_draw_patch(NG_STATSX + 2 * NG_SPACINGX - (*WI_ITEMS).width as i32, NG_STATSY, WI_ITEMS);
        }
        if !WI_SP_SECRET.is_null() {
            v_draw_patch(NG_STATSX + 3 * NG_SPACINGX - (*WI_SP_SECRET).width as i32, NG_STATSY, WI_SP_SECRET);
        }
        if !WI_FRAGS.is_null() {
            v_draw_patch(NG_STATSX + 4 * NG_SPACINGX - (*WI_FRAGS).width as i32, NG_STATSY, WI_FRAGS);
        }

        let mut y = NG_STATSY + lh;
        for i in 0..crate::doomdef::MAXPLAYERS {
            if wbs.plyr[i].in_game == 0 {
                continue;
            }
            let mut x = NG_STATSX;
            let pi = WI_P[i];
            if !pi.is_null() {
                v_draw_patch(x, y, pi);
                if i == me && !WI_STAR.is_null() {
                    v_draw_patch(x - (*WI_STAR).width as i32, y, WI_STAR);
                }
                x += (*pi).width as i32;
            }
            x += NG_SPACINGX - (if pi.is_null() { 0 } else { (*pi).width as i32 });
            if WI_NG_CNT_KILLS[i] >= 0 {
                wi_draw_percent(x - pwidth, y + 10, WI_NG_CNT_KILLS[i]);
            }
            x += NG_SPACINGX;
            if WI_NG_CNT_ITEMS[i] >= 0 {
                wi_draw_percent(x - pwidth, y + 10, WI_NG_CNT_ITEMS[i]);
            }
            x += NG_SPACINGX;
            if WI_NG_CNT_SECRET[i] >= 0 {
                wi_draw_percent(x - pwidth, y + 10, WI_NG_CNT_SECRET[i]);
            }
            x += NG_SPACINGX;
            if WI_NG_CNT_FRAGS[i] >= 0 {
                wi_draw_num(x, y + 10, WI_NG_CNT_FRAGS[i], -1);
            }
            y += NG_SPACINGY;
        }
    }
}

fn wi_update_stats() {
    wi_update_animated_back();
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
                    WI_CNT = 10;
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
    crate::game::statdump::stat_copy(wbstartstruct);
    unsafe {
        WI_STATE = WiStateEnum::StatCount;
        WI_BCNT = 0;
        WI_CNT = SHOWNEXTLOCDELAY * TICRATE;
        WI_ACCELERATE = false;
        wi_load_data(wbstartstruct);
        let deathmatch = DEATHMATCH != 0;
        let netgame = NETGAME;
        if deathmatch {
            wi_init_deathmatch_stats();
        } else if netgame {
            wi_init_netgame_stats();
        } else {
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
                if deathmatch {
                    wi_update_deathmatch_stats();
                } else if netgame {
                    wi_update_netgame_stats();
                } else {
                    wi_update_stats();
                }
            }
            WiStateEnum::ShowNextLoc => {
                WI_CNT -= 1;
                if WI_CNT <= 0 || WI_ACCELERATE {
                    WI_STATE = WiStateEnum::NoState;
                    WI_CNT = 10;
                }
            }
            WiStateEnum::NoState => {
                WI_CNT -= 1;
                if WI_CNT <= 0 {
                    crate::game::g_game::g_world_done();
                }
            }
        }
    }
}

pub fn wi_drawer() {
    let (state, deathmatch, netgame) = unsafe { (WI_STATE, DEATHMATCH != 0, NETGAME) };
    match state {
        WiStateEnum::StatCount => {
            if deathmatch {
                wi_draw_deathmatch_stats();
            } else if netgame {
                wi_draw_netgame_stats();
            } else {
                wi_draw_stats();
            }
        }
        WiStateEnum::ShowNextLoc => {
            wi_slam_background();
            wi_draw_el();
        }
        WiStateEnum::NoState => {}
    }
}
