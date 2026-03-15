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
use std::sync::{Mutex, OnceLock};

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

// =============================================================================
// StStuffState - thread-safe via OnceLock + Mutex
// =============================================================================

static ST_STUFF_STATE: OnceLock<Mutex<StStuffState>> = OnceLock::new();

/// Safety: Raw pointers point to zone-allocated patches and player data.
unsafe impl Send for StStuffState {}

pub struct StStuffState {
    pub backing_screen: *mut u8,
    pub cheat_mus: CheatSeq,
    pub cheat_god: CheatSeq,
    pub cheat_ammo: CheatSeq,
    pub cheat_ammonokey: CheatSeq,
    pub cheat_noclip: CheatSeq,
    pub cheat_clev: CheatSeq,
    pub cheat_mypos: CheatSeq,
    pub cheat_choppers: CheatSeq,
    pub cheat_powerup: [CheatSeq; 7],
    pub sbar: *mut crate::rendering::patch_t,
    pub tallnum: [*mut crate::rendering::patch_t; 10],
    pub tallpercent: *mut crate::rendering::patch_t,
    pub shortnum: [*mut crate::rendering::patch_t; 10],
    pub keys: [*mut crate::rendering::patch_t; 6],
    pub armsbg: *mut crate::rendering::patch_t,
    pub arms: [[*mut crate::rendering::patch_t; 2]; 6],
    pub faces: [*mut crate::rendering::patch_t; 3],
    pub statusbaron: Boolean,
    pub armson: Boolean,
    pub notdeathmatch: Boolean,
    pub firsttime: Boolean,
    pub gamestate: StStateEnum,
    pub clock: u32,
    pub oldhealth: i32,
    pub faceindex: i32,
    pub stopped: bool,
    pub large_ammo: i32,
    pub w_health: StPercent,
    pub w_armor: StPercent,
    pub w_ready: StNumber,
    pub keyboxes: [i32; 3],
    pub arms_display: [i32; 6],
    pub w_arms: [StMultIcon; 6],
    pub w_armsbg: StBinIcon,
    pub w_faces: StMultIcon,
    pub w_keyboxes: [StMultIcon; 3],
    pub sttmminus: *mut crate::rendering::patch_t,
}

impl Default for StStuffState {
    fn default() -> Self {
        Self {
            backing_screen: ptr::null_mut(),
            cheat_mus: CheatSeq::EMPTY,
            cheat_god: CheatSeq::EMPTY,
            cheat_ammo: CheatSeq::EMPTY,
            cheat_ammonokey: CheatSeq::EMPTY,
            cheat_noclip: CheatSeq::EMPTY,
            cheat_clev: CheatSeq::EMPTY,
            cheat_mypos: CheatSeq::EMPTY,
            cheat_choppers: CheatSeq::EMPTY,
            cheat_powerup: [CheatSeq::EMPTY; 7],
            sbar: ptr::null_mut(),
            tallnum: [ptr::null_mut(); 10],
            tallpercent: ptr::null_mut(),
            shortnum: [ptr::null_mut(); 10],
            keys: [ptr::null_mut(); 6],
            armsbg: ptr::null_mut(),
            arms: [[ptr::null_mut(); 2]; 6],
            faces: [ptr::null_mut(); 3],
            statusbaron: true,
            armson: true,
            notdeathmatch: true,
            firsttime: true,
            gamestate: StStateEnum::FirstPersonState,
            clock: 0,
            oldhealth: -1,
            faceindex: 0,
            stopped: true,
            large_ammo: 1994,
            w_health: StPercent {
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
            },
            w_armor: StPercent {
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
            },
            w_ready: StNumber {
                x: 0,
                y: 0,
                width: 0,
                oldnum: 0,
                num: ptr::null_mut(),
                on: ptr::null_mut(),
                p: ptr::null_mut(),
                data: 0,
            },
            keyboxes: [-1, -1, -1],
            arms_display: [0, 0, 0, 0, 0, 0],
            w_arms: [
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
            ],
            w_armsbg: StBinIcon {
                x: 0,
                y: 0,
                oldval: false,
                val: ptr::null_mut(),
                on: ptr::null_mut(),
                p: ptr::null_mut(),
                data: 0,
            },
            w_faces: StMultIcon {
                x: 0,
                y: 0,
                oldinum: -1,
                inum: ptr::null_mut(),
                on: ptr::null_mut(),
                p: ptr::null_mut(),
                data: 0,
            },
            w_keyboxes: [
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
                StMultIcon { x: 0, y: 0, oldinum: -1, inum: ptr::null_mut(), on: ptr::null_mut(), p: ptr::null_mut(), data: 0 },
            ],
            sttmminus: ptr::null_mut(),
        }
    }
}

fn get_st_stuff_state() -> &'static Mutex<StStuffState> {
    ST_STUFF_STATE.get_or_init(|| Mutex::new(StStuffState::default()))
}

/// Access StStuffState.
pub fn with_st_stuff_state<F, R>(f: F) -> R
where
    F: FnOnce(&StStuffState) -> R,
{
    let guard = get_st_stuff_state().lock().unwrap();
    f(&guard)
}

/// Mutably access StStuffState.
pub fn with_st_stuff_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut StStuffState) -> R,
{
    let mut guard = get_st_stuff_state().lock().unwrap();
    f(&mut guard)
}

/// Backing screen for status bar. Use with_st_stuff_state to access.
pub fn st_backing_screen() -> *mut u8 {
    with_st_stuff_state(|s| s.backing_screen)
}

// =============================================================================
// Implementation (from st_stuff.c)
// =============================================================================

fn st_load_graphics(s: &mut StStuffState) {
    for i in 0..10 {
        let name = format!("STTNUM{}", i);
        s.tallnum[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        let name = format!("STYSNUM{}", i);
        s.shortnum[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
    }
    s.tallpercent = w_cache_lump_name(deh_string("STTPRCNT"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
    s.sbar = w_cache_lump_name(deh_string("STBAR"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;

    for i in 0..6 {
        let name = format!("STKEYS{}", i);
        s.keys[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
    }
    s.armsbg = w_cache_lump_name(deh_string("STARMS"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;

    for i in 0..6 {
        let name = format!("STGNUM{}", i + 2);
        s.arms[i][0] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
        s.arms[i][1] = s.shortnum[i + 2];
    }

    s.faces[0] = w_cache_lump_name(deh_string("STFST00"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
    s.faces[1] = w_cache_lump_name(deh_string("STFGOD0"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
    s.faces[2] = w_cache_lump_name(deh_string("STFDEAD0"), PU_STATIC).as_ptr_mut() as *mut crate::rendering::patch_t;
}

fn st_load_data(s: &mut StStuffState) {
    st_load_graphics(s);
}

fn st_init_data(s: &mut StStuffState) {
    s.firsttime = true;
    s.clock = 0;
    s.gamestate = StStateEnum::FirstPersonState;
    s.statusbaron = true;
    s.oldhealth = -1;
    s.faceindex = 0;
    stlib_init(&mut s.sttmminus);
}

fn st_create_widgets(s: &mut StStuffState) {
    let idx = CONSOLEPLAYER as usize;
    let plyr = unsafe { &mut PLAYERS[idx] };
    let health_ptr = &mut plyr.health as *mut i32;
    let armor_ptr = &mut plyr.armorpoints as *mut i32;

    stlib_init_num(
        &mut s.w_ready,
        ST_AMMOX,
        ST_AMMOY,
        s.tallnum.as_mut_ptr(),
        &mut plyr.ammo[0] as *mut i32,
        &mut s.statusbaron,
        ST_AMMOWIDTH,
    );

    stlib_init_percent(
        &mut s.w_health,
        ST_HEALTHX,
        ST_HEALTHY,
        s.tallnum.as_mut_ptr(),
        health_ptr,
        &mut s.statusbaron,
        s.tallpercent,
    );

    stlib_init_percent(
        &mut s.w_armor,
        ST_ARMORX,
        ST_ARMORY,
        s.tallnum.as_mut_ptr(),
        armor_ptr,
        &mut s.statusbaron,
        s.tallpercent,
    );

    stlib_init_bin_icon(
        &mut s.w_armsbg,
        ST_ARMSBGX,
        ST_ARMSBGY,
        s.armsbg,
        &mut s.notdeathmatch,
        &mut s.statusbaron,
    );

    for i in 0..6 {
        stlib_init_mult_icon(
            &mut s.w_arms[i],
            ST_ARMSX + ((i % 3) as i32) * ST_ARMSXSPACE,
            ST_ARMSY + ((i / 3) as i32) * ST_ARMSYSPACE,
            s.arms[i].as_mut_ptr(),
            &mut s.arms_display[i] as *mut i32,
            &mut s.armson,
        );
    }

    stlib_init_mult_icon(
        &mut s.w_faces,
        ST_FACESX,
        ST_FACESY,
        s.faces.as_mut_ptr(),
        &mut s.faceindex as *mut i32,
        &mut s.statusbaron,
    );

    stlib_init_mult_icon(
        &mut s.w_keyboxes[0],
        ST_KEY0X,
        ST_KEY0Y,
        s.keys.as_mut_ptr(),
        &mut s.keyboxes[0] as *mut i32,
        &mut s.statusbaron,
    );
    stlib_init_mult_icon(
        &mut s.w_keyboxes[1],
        ST_KEY1X,
        ST_KEY1Y,
        s.keys.as_mut_ptr(),
        &mut s.keyboxes[1] as *mut i32,
        &mut s.statusbaron,
    );
    stlib_init_mult_icon(
        &mut s.w_keyboxes[2],
        ST_KEY2X,
        ST_KEY2Y,
        s.keys.as_mut_ptr(),
        &mut s.keyboxes[2] as *mut i32,
        &mut s.statusbaron,
    );
}

/// Update widget pointers and state. Called from st_ticker.
fn st_update_widgets(s: &mut StStuffState) {
    let idx = CONSOLEPLAYER as usize;
    let plyr = unsafe { &mut PLAYERS[idx] };
    let weapon = plyr.readyweapon as usize;
    let ammo_type = WEAPONINFO[weapon].ammo;

    if ammo_type == Ammotype::Noammo {
        s.w_ready.num = &mut s.large_ammo as *mut i32;
    } else {
        let ammo_idx = ammo_type as usize;
        s.w_ready.num = &mut plyr.ammo[ammo_idx] as *mut i32;
    }

    for i in 0..3 {
        s.keyboxes[i] = if plyr.cards[i] { i as i32 } else { -1 };
        if plyr.cards[i + 3] {
            s.keyboxes[i] = (i + 3) as i32;
        }
    }

    for i in 0..6 {
        s.arms_display[i] = if plyr.weaponowned[i + 1] { 1 } else { 0 };
    }

    if plyr.health <= 0 {
        s.faceindex = 2;
    } else if (plyr.cheats & CF_GODMODE) != 0 || plyr.powers[0] > 0 {
        s.faceindex = 1;
    } else {
        s.faceindex = 0;
    }

    s.notdeathmatch = DEATHMATCH == 0;
    s.armson = s.statusbaron && (DEATHMATCH == 0);
}

fn st_refresh_background(s: &StStuffState) {
    if !s.statusbaron || s.backing_screen.is_null() || s.sbar.is_null() {
        return;
    }
    v_use_buffer(s.backing_screen);
    v_draw_patch(ST_X, 0, s.sbar);
    v_restore_buffer();
    v_copy_rect(ST_X, 0, s.backing_screen, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y);
}

fn st_draw_widgets(s: &mut StStuffState, refresh: bool) {
    let backing = s.backing_screen;
    if backing.is_null() {
        return;
    }
    stlib_update_num(&mut s.w_ready, refresh, backing, ST_Y, s.sttmminus);
    stlib_update_percent(&mut s.w_health, refresh, backing, ST_Y, s.sttmminus);
    stlib_update_percent(&mut s.w_armor, refresh, backing, ST_Y, s.sttmminus);
    stlib_update_bin_icon(&mut s.w_armsbg, refresh, backing, ST_Y);
    for i in 0..6 {
        stlib_update_mult_icon(&mut s.w_arms[i], refresh, backing, ST_Y);
    }
    stlib_update_mult_icon(&mut s.w_faces, refresh, backing, ST_Y);
    for i in 0..3 {
        stlib_update_mult_icon(&mut s.w_keyboxes[i], refresh, backing, ST_Y);
    }
}

pub fn st_init() {
    with_st_stuff_state_mut(|s| {
        st_load_data(s);
        stlib_init(&mut s.sttmminus);
        let size = (ST_WIDTH * ST_HEIGHT) as usize;
        s.backing_screen = z_malloc(size, PU_STATIC, ptr::null_mut());
        s.cheat_god = CheatSeq::new("iddqd", 0);
        s.cheat_ammo = CheatSeq::new("idkfa", 0);
        s.cheat_ammonokey = CheatSeq::new("idfa", 0);
        s.cheat_noclip = CheatSeq::new("idspispopd", 0);
        s.cheat_clev = CheatSeq::new("idclev", 2);
        s.cheat_mypos = CheatSeq::new("idmypos", 0);
        s.cheat_mus = CheatSeq::new("idmus", 2);
        s.cheat_choppers = CheatSeq::new("idchoppers", 0);
        s.cheat_powerup[0] = CheatSeq::new("idbeholdv", 0);
        s.cheat_powerup[1] = CheatSeq::new("idbeholds", 0);
        s.cheat_powerup[2] = CheatSeq::new("idbeholdi", 0);
        s.cheat_powerup[3] = CheatSeq::new("idbeholdr", 0);
        s.cheat_powerup[4] = CheatSeq::new("idbeholda", 0);
        s.cheat_powerup[5] = CheatSeq::new("idbeholdl", 0);
        s.cheat_powerup[6] = CheatSeq::new("idbehold", 0);
    });
}

pub fn st_start() {
    with_st_stuff_state_mut(|s| {
        if !s.stopped {
            s.stopped = true;
        }
        st_init_data(s);
        st_create_widgets(s);
        s.stopped = false;
    });
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

    with_st_stuff_state_mut(|s| {
        if ev.ev_type == EvType::KeyUp && (ev.data1 as u32 & 0xffff0000) == AM_MSGHEADER as u32 {
            match ev.data1 {
                AM_MSGENTERED => {
                    s.gamestate = StStateEnum::AutomapState;
                    s.firsttime = true;
                }
                AM_MSGEXITED => {
                    s.gamestate = StStateEnum::FirstPersonState;
                }
                _ => {}
            }
        }

        if ev.ev_type == EvType::KeyDown {
            let idx = CONSOLEPLAYER as usize;
            let plyr = unsafe { &mut PLAYERS[idx] };
            let netgame = NETGAME;
            let skill_nightmare = GAMESKILL == crate::game::d_mode::Skill::Nightmare;

            if !netgame && !skill_nightmare {
                if cht_check_cheat(&mut s.cheat_god, ev.data2 as u8) {
                    plyr.cheats ^= CF_GODMODE;
                    if (plyr.cheats & CF_GODMODE) != 0 {
                        plyr.health = DEH_DEFAULT_GOD_MODE_HEALTH;
                    }
                    return true;
                }
                if cht_check_cheat(&mut s.cheat_ammonokey, ev.data2 as u8) {
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
                if cht_check_cheat(&mut s.cheat_ammo, ev.data2 as u8) {
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
                if cht_check_cheat(&mut s.cheat_noclip, ev.data2 as u8) {
                    plyr.cheats ^= CF_NOCLIP;
                    return true;
                }
                // idmus: change music (2-digit param)
                if cht_check_cheat(&mut s.cheat_mus, ev.data2 as u8) {
                    let mut buf = [0u8; 3];
                    cht_get_param(&s.cheat_mus, &mut buf);
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
                    if cht_check_cheat(&mut s.cheat_powerup[i], ev.data2 as u8) {
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
                if cht_check_cheat(&mut s.cheat_powerup[6], ev.data2 as u8) {
                    plyr.message = Some("BEHOLD!".to_string());
                    return true;
                }
                if cht_check_cheat(&mut s.cheat_choppers, ev.data2 as u8) {
                    plyr.weaponowned[Weapontype::Chainsaw as usize] = true;
                    plyr.powers[Powertype::Invulnerability as usize] = INVULNTICS;
                    plyr.message = Some("CHOPPERS!".to_string());
                    return true;
                }
            }

            // idclev: change level (outside netgame check, but still !netgame)
            if !netgame && cht_check_cheat(&mut s.cheat_clev, ev.data2 as u8) {
                let mut buf = [0u8; 3];
                cht_get_param(&s.cheat_clev, &mut buf);
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
        false
    })
}

pub fn st_ticker() {
    with_st_stuff_state_mut(|s| {
        st_update_widgets(s);
        s.clock = s.clock.wrapping_add(1);
        let idx = CONSOLEPLAYER as usize;
        s.oldhealth = unsafe { PLAYERS[idx].health };
    });
    st_update_palette_effects();
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
    with_st_stuff_state_mut(|s| {
        if fullscreen {
            st_refresh_background(s);
        }
        st_draw_widgets(s, refresh);
    });
}
