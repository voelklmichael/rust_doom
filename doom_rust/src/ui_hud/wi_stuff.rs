//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Intermission screen.
// Original: wi_stuff.h + wi_stuff.c

use crate::deh::deh_string;
use crate::doomstat::{GAMEMODE, WbStartStruct};
use crate::game::d_mode::GameMode;
use crate::rendering::v_draw_patch;
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

// =============================================================================
// Internal state (from wi_stuff.c)
// =============================================================================

static mut WI_STATE: WiStateEnum = WiStateEnum::NoState;
static mut WI_BCNT: i32 = 0;
static mut WI_BACKGROUND: *mut crate::rendering::patch_t = ptr::null_mut();
static mut WI_BACKGROUND_NAME: [u8; 9] = [0; 9];

// =============================================================================
// Implementation (from wi_stuff.c)
// =============================================================================

fn wi_load_data(wbs: &WbStartStruct) {
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
    }
}

fn wi_slam_background() {
    unsafe {
        if !WI_BACKGROUND.is_null() {
            v_draw_patch(0, 0, WI_BACKGROUND);
        }
    }
}

pub fn wi_start(wbstartstruct: &WbStartStruct) {
    unsafe {
        WI_STATE = WiStateEnum::StatCount;
        WI_BCNT = 0;
        wi_load_data(wbstartstruct);
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
    }
}

pub fn wi_drawer() {
    match unsafe { WI_STATE } {
        WiStateEnum::StatCount => {
            wi_slam_background();
        }
        WiStateEnum::ShowNextLoc => {
            wi_slam_background();
        }
        WiStateEnum::NoState => {}
    }
}
