//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Status bar code.
// Original: st_stuff.h + st_stuff.c

use crate::deh::deh_string;
use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::doomstat::{CONSOLEPLAYER, PLAYERS};
use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::rendering::{v_copy_rect, v_draw_patch, v_restore_buffer, v_use_buffer};
use crate::ui_hud::cheat::{cht_check_cheat, CheatSeq};
use crate::ui_hud::st_lib::{stlib_init, stlib_init_percent, stlib_update_percent, StNumber, StPercent};
use crate::wad::w_cache_lump_name;
use crate::z_zone::{z_malloc, PU_STATIC};
use std::ptr;

// =============================================================================
// Public API (from st_stuff.h)
// =============================================================================

pub const ST_HEIGHT: i32 = 32;
pub const ST_WIDTH: i32 = SCREENWIDTH;
pub const ST_Y: i32 = SCREENHEIGHT - ST_HEIGHT;

// Layout constants (from st_stuff.c)
const ST_X: i32 = 0;
const ST_AMMOX: i32 = 44;
const ST_AMMOY: i32 = 171;
const ST_AMMOWIDTH: i32 = 3;
const ST_HEALTHX: i32 = 90;
const ST_HEALTHY: i32 = 171;
const ST_ARMORX: i32 = 221;
const ST_ARMORY: i32 = 171;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum StStateEnum {
    #[default]
    AutomapState,
    FirstPersonState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum StChatStateEnum {
    #[default]
    StartChatState,
    WaitDestState,
    GetChatState,
}

pub static mut ST_BACKING_SCREEN: *mut u8 = ptr::null_mut();

pub static mut CHEAT_MUS: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_GOD: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_AMMO: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_AMMONOKEY: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_NOCLIP: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_CLEV: CheatSeq = CheatSeq::EMPTY;
pub static mut CHEAT_MYPOS: CheatSeq = CheatSeq::EMPTY;

// =============================================================================
// Internal state (from st_stuff.c)
// =============================================================================

static mut SBAR: *mut crate::rendering::patch_t = ptr::null_mut();
static mut TALLNUM: [*mut crate::rendering::patch_t; 10] = [ptr::null_mut(); 10];
static mut TALLPERCENT: *mut crate::rendering::patch_t = ptr::null_mut();
static mut SHORTNUM: [*mut crate::rendering::patch_t; 10] = [ptr::null_mut(); 10];

static mut ST_STATUSBARON: Boolean = true;
static mut ST_FIRSTTIME: Boolean = true;
static mut ST_GAMESTATE: StStateEnum = StStateEnum::FirstPersonState;
static mut ST_CLOCK: u32 = 0;
static mut ST_OLDHEALTH: i32 = -1;
static mut ST_FACEINDEX: i32 = 0;
static mut ST_STOPPED: bool = true;

// Dummy values for widgets until Player has full fields
static mut ST_DUMMY_ARMOR: i32 = 0;
static mut ST_DUMMY_AMMO: i32 = 0;

static mut W_HEALTH: StPercent = StPercent {
    n: StNumber {
        x: 0,
        y: 0,
        width: 0,
        oldnum: 0,
        num: ptr::null_mut(),
        on: ptr::null_mut(),
        p: ptr::null_mut(),
        data: 0,
    },
    p: ptr::null_mut(),
};

static mut W_ARMOR: StPercent = StPercent {
    n: StNumber {
        x: 0,
        y: 0,
        width: 0,
        oldnum: 0,
        num: ptr::null_mut(),
        on: ptr::null_mut(),
        p: ptr::null_mut(),
        data: 0,
    },
    p: ptr::null_mut(),
};

// =============================================================================
// Implementation (from st_stuff.c)
// =============================================================================

fn st_load_graphics() {
    unsafe {
        for i in 0..10 {
            let name = format!("STTNUM{}", i);
            TALLNUM[i] = w_cache_lump_name(deh_string(&name), PU_STATIC) as *mut crate::rendering::patch_t;
            let name = format!("STYSNUM{}", i);
            SHORTNUM[i] = w_cache_lump_name(deh_string(&name), PU_STATIC) as *mut crate::rendering::patch_t;
        }
        TALLPERCENT = w_cache_lump_name(deh_string("STTPRCNT"), PU_STATIC) as *mut crate::rendering::patch_t;
        SBAR = w_cache_lump_name(deh_string("STBAR"), PU_STATIC) as *mut crate::rendering::patch_t;
    }
}

fn st_load_data() {
    st_load_graphics();
}

fn st_init_data() {
    unsafe {
        ST_FIRSTTIME = true;
        ST_CLOCK = 0;
        ST_GAMESTATE = StStateEnum::FirstPersonState;
        ST_STATUSBARON = true;
        ST_OLDHEALTH = -1;
        ST_FACEINDEX = 0;
        stlib_init();
    }
}

fn st_create_widgets() {
    unsafe {
        let idx = CONSOLEPLAYER as usize;
        let plyr = &mut PLAYERS[idx];
        let health_ptr = &mut plyr.health as *mut i32;

        stlib_init_percent(
            &mut W_HEALTH,
            ST_HEALTHX,
            ST_HEALTHY,
            TALLNUM.as_mut_ptr(),
            health_ptr,
            &mut ST_STATUSBARON,
            TALLPERCENT,
        );

        stlib_init_percent(
            &mut W_ARMOR,
            ST_ARMORX,
            ST_ARMORY,
            TALLNUM.as_mut_ptr(),
            &mut ST_DUMMY_ARMOR,
            &mut ST_STATUSBARON,
            TALLPERCENT,
        );
    }
}

fn st_refresh_background() {
    unsafe {
        if !ST_STATUSBARON || ST_BACKING_SCREEN.is_null() || SBAR.is_null() {
            return;
        }
        v_use_buffer(ST_BACKING_SCREEN);
        v_draw_patch(ST_X, 0, SBAR);
        v_restore_buffer();
        v_copy_rect(ST_X, 0, ST_BACKING_SCREEN, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y);
    }
}

fn st_draw_widgets(refresh: bool) {
    unsafe {
        let backing = ST_BACKING_SCREEN;
        if backing.is_null() {
            return;
        }
        stlib_update_percent(&mut W_HEALTH, refresh, backing, ST_Y);
        stlib_update_percent(&mut W_ARMOR, refresh, backing, ST_Y);
    }
}

pub fn st_init() {
    st_load_data();
    stlib_init();
    let size = (ST_WIDTH * ST_HEIGHT) as usize;
    unsafe {
        ST_BACKING_SCREEN = z_malloc(size, PU_STATIC, ptr::null_mut());
        // Init cheat sequences (C: CHEAT("iddqd", 0) etc.)
        CHEAT_GOD = CheatSeq::new("iddqd", 0);
        CHEAT_AMMO = CheatSeq::new("idkfa", 0);
        CHEAT_AMMONOKEY = CheatSeq::new("idfa", 0);
        CHEAT_NOCLIP = CheatSeq::new("idspispopd", 0);
        CHEAT_CLEV = CheatSeq::new("idclev", 2);
        CHEAT_MYPOS = CheatSeq::new("idmypos", 0);
        CHEAT_MUS = CheatSeq::new("idmus", 2);
    }
}

pub fn st_start() {
    unsafe {
        if !ST_STOPPED {
            ST_STOPPED = true;
        }
        st_init_data();
        st_create_widgets();
        ST_STOPPED = false;
    }
}

pub fn st_responder(ev: &Event) -> Boolean {
    use crate::game::d_event::EvType;
    unsafe {
        if ev.ev_type == EvType::KeyDown {
            let idx = CONSOLEPLAYER as usize;
            let plyr = &mut PLAYERS[idx];
            if cht_check_cheat(&mut CHEAT_GOD, ev.data2 as u8) {
                // Toggle god mode - stub: just set message
                plyr.health = 100;
                return true;
            }
        }
    }
    false
}

pub fn st_ticker() {
    unsafe {
        ST_CLOCK = ST_CLOCK.wrapping_add(1);
        let idx = CONSOLEPLAYER as usize;
        ST_OLDHEALTH = PLAYERS[idx].health;
    }
}

pub fn st_drawer(fullscreen: bool, refresh: bool) {
    if fullscreen {
        st_refresh_background();
    }
    st_draw_widgets(refresh);
}
