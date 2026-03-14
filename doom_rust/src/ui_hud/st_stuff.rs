//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Status bar code.
// Original: st_stuff.h + st_stuff.c

use crate::deh::deh_string;
use crate::doomdef::{Ammotype, SCREENHEIGHT, SCREENWIDTH};
use crate::doomdef::Weapontype;
use crate::doomdef::Powertype;
use crate::doomstat::{
    CF_GODMODE, CF_NOCLIP, CONSOLEPLAYER, DEATHMATCH, GAMEMODE, GAMESKILL, NETGAME, PLAYERS,
};
use crate::player::p_user::INVERSECOLORMAP;
use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::game::d_items::WEAPONINFO;
use crate::rendering::{v_copy_rect, v_draw_patch, v_restore_buffer, v_use_buffer};
use crate::ui_hud::cheat::{cht_check_cheat, cht_get_param, CheatSeq};
use crate::ui_hud::st_lib::{
    stlib_init, stlib_init_bin_icon, stlib_init_mult_icon, stlib_init_num, stlib_init_percent,
    stlib_update_bin_icon, stlib_update_mult_icon, stlib_update_num, stlib_update_percent,
    StBinIcon, StMultIcon, StNumber, StPercent,
};
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
const ST_ARMSX: i32 = 111;
const ST_ARMSY: i32 = 172;
const ST_ARMSBGX: i32 = 104;
const ST_ARMSBGY: i32 = 168;
const ST_ARMSXSPACE: i32 = 12;
const ST_ARMSYSPACE: i32 = 10;
const ST_FACESX: i32 = 143;
const ST_FACESY: i32 = 168;
const ST_KEY0X: i32 = 239;
const ST_KEY0Y: i32 = 171;
const ST_KEY1X: i32 = 239;
const ST_KEY1Y: i32 = 181;
const ST_KEY2X: i32 = 239;
const ST_KEY2Y: i32 = 191;

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
pub static mut CHEAT_CHOPPERS: CheatSeq = CheatSeq::EMPTY;
static mut CHEAT_POWERUP: [CheatSeq; 7] = [CheatSeq::EMPTY; 7];

// =============================================================================
// Internal state (from st_stuff.c)
// =============================================================================

static mut SBAR: *mut crate::rendering::patch_t = ptr::null_mut();
static mut TALLNUM: [*mut crate::rendering::patch_t; 10] = [ptr::null_mut(); 10];
static mut TALLPERCENT: *mut crate::rendering::patch_t = ptr::null_mut();
static mut SHORTNUM: [*mut crate::rendering::patch_t; 10] = [ptr::null_mut(); 10];

// Key card patches (STKEYS0..STKEYS5)
static mut KEYS: [*mut crate::rendering::patch_t; 6] = [ptr::null_mut(); 6];
// Arms background
static mut ARMSBG: *mut crate::rendering::patch_t = ptr::null_mut();
// Weapon ownership patches: arms[i][0]=gray, arms[i][1]=yellow (shortnum)
static mut ARMS: [[*mut crate::rendering::patch_t; 2]; 6] = [[ptr::null_mut(); 2]; 6];
// Face patches: simplified (normal, god, dead)
const ST_NUM_FACES: usize = 3;
static mut FACES: [*mut crate::rendering::patch_t; 3] = [ptr::null_mut(); 3];

static mut ST_STATUSBARON: Boolean = true;
static mut ST_ARMSON: Boolean = true;
static mut ST_NOTDEATHMATCH: Boolean = true;
static mut ST_FIRSTTIME: Boolean = true;
static mut ST_GAMESTATE: StStateEnum = StStateEnum::FirstPersonState;
static mut ST_CLOCK: u32 = 0;
static mut ST_OLDHEALTH: i32 = -1;
static mut ST_FACEINDEX: i32 = 0;
static mut ST_STOPPED: bool = true;

/// N/A display for weapons with no ammo (fist, chainsaw).
static mut LARGE_AMMO: i32 = 1994;

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

/// Ready weapon ammo display.
static mut W_READY: StNumber = StNumber {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: ptr::null_mut(),
    on: ptr::null_mut(),
    p: ptr::null_mut(),
    data: 0,
};

/// Which key icon to show per slot (0-5 or -1 for none). Updated in st_update_widgets.
static mut KEYBOXES: [i32; 3] = [-1, -1, -1];

/// Weapon owned display (0=gray, 1=yellow) for arms 1-6. Updated in st_update_widgets.
static mut ARMS_DISPLAY: [i32; 6] = [0, 0, 0, 0, 0, 0];

static mut W_ARMS: [StMultIcon; 6] = [
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
];
static mut W_ARMSBG: StBinIcon = StBinIcon {
    x: 0,
    y: 0,
    oldval: false,
    val: ptr::null_mut(),
    on: ptr::null_mut(),
    p: ptr::null_mut(),
    data: 0,
};
static mut W_FACES: StMultIcon = StMultIcon {
    x: 0,
    y: 0,
    oldinum: -1,
    inum: ptr::null_mut(),
    on: ptr::null_mut(),
    p: ptr::null_mut(),
    data: 0,
};
static mut W_KEYBOXES: [StMultIcon; 3] = [
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
    StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
];

// =============================================================================
// Implementation (from st_stuff.c)
// =============================================================================

fn st_load_graphics() {
    unsafe {
        for i in 0..10 {
            let name = format!("STTNUM{}", i);
            TALLNUM[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
            let name = format!("STYSNUM{}", i);
            SHORTNUM[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        }
        TALLPERCENT = w_cache_lump_name(deh_string("STTPRCNT"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        SBAR = w_cache_lump_name(deh_string("STBAR"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;

        // Key card patches
        for i in 0..6 {
            let name = format!("STKEYS{}", i);
            KEYS[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        }
        ARMSBG = w_cache_lump_name(deh_string("STARMS"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;

        // Weapon ownership: gray (STGNUM2..7), yellow = shortnum[2..8]
        for i in 0..6 {
            let name = format!("STGNUM{}", i + 2);
            ARMS[i][0] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
            ARMS[i][1] = SHORTNUM[i + 2];
        }

        // Face patches: normal, god, dead
        FACES[0] = w_cache_lump_name(deh_string("STFST00"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        FACES[1] = w_cache_lump_name(deh_string("STFGOD0"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        FACES[2] = w_cache_lump_name(deh_string("STFDEAD0"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
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
        let armor_ptr = &mut plyr.armorpoints as *mut i32;

        // Ready weapon ammo (pointer updated each tick in st_update_widgets)
        stlib_init_num(
            &mut W_READY,
            ST_AMMOX,
            ST_AMMOY,
            TALLNUM.as_mut_ptr(),
            &mut plyr.ammo[0] as *mut i32,
            &mut ST_STATUSBARON,
            ST_AMMOWIDTH,
        );

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
            armor_ptr,
            &mut ST_STATUSBARON,
            TALLPERCENT,
        );

        // Arms background (shown when !deathmatch)
        stlib_init_bin_icon(
            &mut W_ARMSBG,
            ST_ARMSBGX,
            ST_ARMSBGY,
            ARMSBG,
            &mut ST_NOTDEATHMATCH,
            &mut ST_STATUSBARON,
        );

        // Weapon ownership (6 weapons: pistol..bfg)
        for i in 0..6 {
            stlib_init_mult_icon(
                &mut W_ARMS[i],
                ST_ARMSX + ((i % 3) as i32) * ST_ARMSXSPACE,
                ST_ARMSY + ((i / 3) as i32) * ST_ARMSYSPACE,
                ARMS[i].as_mut_ptr(),
                &mut ARMS_DISPLAY[i] as *mut i32,
                &mut ST_ARMSON,
            );
        }

        // Face (simplified: 0=normal, 1=god, 2=dead)
        stlib_init_mult_icon(
            &mut W_FACES,
            ST_FACESX,
            ST_FACESY,
            FACES.as_mut_ptr(),
            &mut ST_FACEINDEX as *mut i32,
            &mut ST_STATUSBARON,
        );

        // Key boxes (3 slots)
        stlib_init_mult_icon(
            &mut W_KEYBOXES[0],
            ST_KEY0X,
            ST_KEY0Y,
            KEYS.as_mut_ptr(),
            &mut KEYBOXES[0] as *mut i32,
            &mut ST_STATUSBARON,
        );
        stlib_init_mult_icon(
            &mut W_KEYBOXES[1],
            ST_KEY1X,
            ST_KEY1Y,
            KEYS.as_mut_ptr(),
            &mut KEYBOXES[1] as *mut i32,
            &mut ST_STATUSBARON,
        );
        stlib_init_mult_icon(
            &mut W_KEYBOXES[2],
            ST_KEY2X,
            ST_KEY2Y,
            KEYS.as_mut_ptr(),
            &mut KEYBOXES[2] as *mut i32,
            &mut ST_STATUSBARON,
        );
    }
}

/// Update widget pointers and state. Called from st_ticker.
fn st_update_widgets() {
    unsafe {
        let idx = CONSOLEPLAYER as usize;
        let plyr = &mut PLAYERS[idx];
        let weapon = plyr.readyweapon as usize;
        let ammo_type = WEAPONINFO[weapon].ammo;

        if ammo_type == Ammotype::Noammo {
            W_READY.num = &mut LARGE_AMMO as *mut i32;
        } else {
            let ammo_idx = ammo_type as usize;
            W_READY.num = &mut plyr.ammo[ammo_idx] as *mut i32;
        }

        // Key boxes: slot i shows card i or skull i+3 (skull overrides)
        for i in 0..3 {
            KEYBOXES[i] = if plyr.cards[i] { i as i32 } else { -1 };
            if plyr.cards[i + 3] {
                KEYBOXES[i] = (i + 3) as i32;
            }
        }

        // Arms display: weaponowned[1..7] -> 0 or 1
        for i in 0..6 {
            ARMS_DISPLAY[i] = if plyr.weaponowned[i + 1] { 1 } else { 0 };
        }

        // Face: 0=normal, 1=god, 2=dead
        if plyr.health <= 0 {
            ST_FACEINDEX = 2;
        } else if (plyr.cheats & CF_GODMODE) != 0 || plyr.powers[0] > 0 {
            ST_FACEINDEX = 1;
        } else {
            ST_FACEINDEX = 0;
        }

        ST_NOTDEATHMATCH = DEATHMATCH == 0;
        ST_ARMSON = ST_STATUSBARON && (DEATHMATCH == 0);
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
        stlib_update_num(&mut W_READY, refresh, backing, ST_Y);
        stlib_update_percent(&mut W_HEALTH, refresh, backing, ST_Y);
        stlib_update_percent(&mut W_ARMOR, refresh, backing, ST_Y);
        stlib_update_bin_icon(&mut W_ARMSBG, refresh, backing, ST_Y);
        for i in 0..6 {
            stlib_update_mult_icon(&mut W_ARMS[i], refresh, backing, ST_Y);
        }
        stlib_update_mult_icon(&mut W_FACES, refresh, backing, ST_Y);
        for i in 0..3 {
            stlib_update_mult_icon(&mut W_KEYBOXES[i], refresh, backing, ST_Y);
        }
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
        CHEAT_CHOPPERS = CheatSeq::new("idchoppers", 0);
        CHEAT_POWERUP[0] = CheatSeq::new("idbeholdv", 0);
        CHEAT_POWERUP[1] = CheatSeq::new("idbeholds", 0);
        CHEAT_POWERUP[2] = CheatSeq::new("idbeholdi", 0);
        CHEAT_POWERUP[3] = CheatSeq::new("idbeholdr", 0);
        CHEAT_POWERUP[4] = CheatSeq::new("idbeholda", 0);
        CHEAT_POWERUP[5] = CheatSeq::new("idbeholdl", 0);
        CHEAT_POWERUP[6] = CheatSeq::new("idbehold", 0);
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

const TICRATE: i32 = 35;
const INVULNTICS: i32 = 30 * TICRATE;
const INVISTICS: i32 = 60 * TICRATE;
const INFRATICS: i32 = 120 * TICRATE;
const IRONTICS: i32 = 60 * TICRATE;

const AM_MSGHEADER: i32 = (('a' as i32) << 24) | (('m' as i32) << 16);
const AM_MSGENTERED: i32 = AM_MSGHEADER | (('e' as i32) << 8);
const AM_MSGEXITED: i32 = AM_MSGHEADER | (('x' as i32) << 8);

pub fn st_responder(ev: &Event) -> Boolean {
    use crate::deh::misc::{
        DEH_DEFAULT_GOD_MODE_HEALTH, DEH_DEFAULT_IDFA_ARMOR, DEH_DEFAULT_IDFA_ARMOR_CLASS,
        DEH_DEFAULT_IDKFA_ARMOR, DEH_DEFAULT_IDKFA_ARMOR_CLASS,
    };
    use crate::doomdef::{Powertype, NUMAMMO, NUMCARDS, NUMWEAPONS};
    use crate::game::d_event::EvType;
    use crate::game::d_mode::GameMode;
    use crate::game::g_game::g_defered_init_new;
    use crate::sound::{s_change_music, MusicEnum};
    unsafe {
        // Filter automap on/off (from am_map AM_MSGENTERED/AM_MSGEXITED)
        if ev.ev_type == EvType::KeyUp && (ev.data1 as u32 & 0xffff0000) == AM_MSGHEADER as u32 {
            match ev.data1 {
                AM_MSGENTERED => {
                    ST_GAMESTATE = StStateEnum::AutomapState;
                    ST_FIRSTTIME = true;
                }
                AM_MSGEXITED => {
                    ST_GAMESTATE = StStateEnum::FirstPersonState;
                }
                _ => {}
            }
        }

        if ev.ev_type == EvType::KeyDown {
            let idx = CONSOLEPLAYER as usize;
            let plyr = &mut PLAYERS[idx];
            let netgame = NETGAME;
            let skill_nightmare = GAMESKILL == crate::game::d_mode::Skill::Nightmare;

            if !netgame && !skill_nightmare {
                if cht_check_cheat(&mut CHEAT_GOD, ev.data2 as u8) {
                    plyr.cheats ^= CF_GODMODE;
                    if (plyr.cheats & CF_GODMODE) != 0 {
                        plyr.health = DEH_DEFAULT_GOD_MODE_HEALTH;
                    }
                    return true;
                }
                if cht_check_cheat(&mut CHEAT_AMMONOKEY, ev.data2 as u8) {
                    plyr.armorpoints = DEH_DEFAULT_IDFA_ARMOR;
                    plyr.armortype = DEH_DEFAULT_IDFA_ARMOR_CLASS;
                    for i in 0..NUMWEAPONS {
                        plyr.weaponowned[i] = true;
                    }
                    for i in 0..NUMAMMO {
                        plyr.ammo[i] = plyr.maxammo[i];
                    }
                    return true;
                }
                if cht_check_cheat(&mut CHEAT_AMMO, ev.data2 as u8) {
                    plyr.armorpoints = DEH_DEFAULT_IDKFA_ARMOR;
                    plyr.armortype = DEH_DEFAULT_IDKFA_ARMOR_CLASS;
                    for i in 0..NUMWEAPONS {
                        plyr.weaponowned[i] = true;
                    }
                    for i in 0..NUMAMMO {
                        plyr.ammo[i] = plyr.maxammo[i];
                    }
                    for i in 0..NUMCARDS {
                        plyr.cards[i] = true;
                    }
                    return true;
                }
                if cht_check_cheat(&mut CHEAT_NOCLIP, ev.data2 as u8) {
                    plyr.cheats ^= CF_NOCLIP;
                    return true;
                }
                // idmus: change music (2-digit param)
                if cht_check_cheat(&mut CHEAT_MUS, ev.data2 as u8) {
                    let mut buf = [0u8; 3];
                    cht_get_param(&CHEAT_MUS, &mut buf);
                    let musnum = if GAMEMODE == GameMode::Commercial
                        || GAMEMODE == GameMode::Indetermined
                    {
                        let n = (buf[0].saturating_sub(b'0')) as i32 * 10
                            + (buf[1].saturating_sub(b'0')) as i32;
                        if n > 35 {
                            return true;
                        }
                        (MusicEnum::Runnin as i32) + n - 1
                    } else {
                        let n = (buf[0].saturating_sub(b'1')) as i32 * 9
                            + (buf[1].saturating_sub(b'1')) as i32;
                        if n > 31 {
                            return true;
                        }
                        (MusicEnum::E1m1 as i32) + n
                    };
                    s_change_music(musnum.max(0) as usize, true);
                    return true;
                }
                // idbehold? power-up cheats
                for i in 0..6 {
                    if cht_check_cheat(&mut CHEAT_POWERUP[i], ev.data2 as u8) {
                        if plyr.powers[i] == 0 {
                            match i as i32 {
                                x if x == Powertype::Invulnerability as i32 => {
                                    plyr.powers[i] = INVULNTICS;
                                }
                                x if x == Powertype::Invisibility as i32 => {
                                    plyr.powers[i] = INVISTICS;
                                }
                                x if x == Powertype::Infrared as i32 => {
                                    plyr.powers[i] = INFRATICS;
                                }
                                x if x == Powertype::Ironfeet as i32 => {
                                    plyr.powers[i] = IRONTICS;
                                }
                                x if x == Powertype::Strength as i32 => {
                                    plyr.health = 100;
                                    plyr.powers[i] = 1;
                                }
                                _ => plyr.powers[i] = 1,
                            }
                        } else if i != Powertype::Strength as usize {
                            plyr.powers[i] = 1;
                        } else {
                            plyr.powers[i] = 0;
                        }
                        return true;
                    }
                }
                if cht_check_cheat(&mut CHEAT_POWERUP[6], ev.data2 as u8) {
                    plyr.message = Some("BEHOLD!".to_string());
                    return true;
                }
                if cht_check_cheat(&mut CHEAT_CHOPPERS, ev.data2 as u8) {
                    plyr.weaponowned[Weapontype::Chainsaw as usize] = true;
                    plyr.powers[Powertype::Invulnerability as usize] = INVULNTICS;
                    plyr.message = Some("CHOPPERS!".to_string());
                    return true;
                }
            }

            // idclev: change level (outside netgame check, but still !netgame)
            if !netgame && cht_check_cheat(&mut CHEAT_CLEV, ev.data2 as u8) {
                let mut buf = [0u8; 3];
                cht_get_param(&CHEAT_CLEV, &mut buf);
                let (epsd, map) = if GAMEMODE == GameMode::Commercial {
                    (1, (buf[0].saturating_sub(b'0')) as i32 * 10
                        + (buf[1].saturating_sub(b'0')) as i32)
                } else {
                    (
                        (buf[0].saturating_sub(b'0')) as i32,
                        (buf[1].saturating_sub(b'0')) as i32,
                    )
                };
                if epsd >= 1 && map >= 1 {
                    let mut valid = true;
                    match GAMEMODE {
                        GameMode::Retail if epsd > 4 || map > 9 => valid = false,
                        GameMode::Registered if epsd > 3 || map > 9 => valid = false,
                        GameMode::Shareware if epsd > 1 || map > 9 => valid = false,
                        GameMode::Commercial if epsd > 1 || map > 40 => valid = false,
                        _ => {}
                    }
                    if valid {
                        g_defered_init_new(GAMESKILL, epsd, map);
                    }
                }
                return true;
            }
        }
    }
    false
}

pub fn st_ticker() {
    st_update_widgets();
    st_update_palette_effects();
    unsafe {
        ST_CLOCK = ST_CLOCK.wrapping_add(1);
        let idx = CONSOLEPLAYER as usize;
        ST_OLDHEALTH = PLAYERS[idx].health;
    }
}

/// Update player fixedcolormap for damage/bonus/invulnerability flash. Original: P_PlayerThink palette part
fn st_update_palette_effects() {
    unsafe {
        let idx = CONSOLEPLAYER as usize;
        if idx >= crate::doomdef::MAXPLAYERS {
            return;
        }
        let plyr = &mut PLAYERS[idx];
        let invuln = (plyr.cheats & CF_GODMODE) != 0 || plyr.powers[Powertype::Invulnerability as usize] > 0;

        if invuln {
            plyr.fixedcolormap = INVERSECOLORMAP;
        } else if plyr.damagecount > 0 {
            plyr.fixedcolormap = 1 + (plyr.damagecount - 1) / 2;
            plyr.damagecount -= 1;
        } else if plyr.bonuscount > 0 {
            plyr.fixedcolormap = 1;
            plyr.bonuscount -= 1;
        } else {
            plyr.fixedcolormap = 0;
        }
    }
}

pub fn st_drawer(fullscreen: bool, refresh: bool) {
    if fullscreen {
        st_refresh_background();
    }
    st_draw_widgets(refresh);
}
