//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game logic.
//
// Original: g_game.h + g_game.c (stub)

use super::d_event::Event;
use super::d_main::{GAMEACTION, SAVEGAMESLOT};
use super::d_ticcmd::Ticcmd;
use crate::deh::misc::DEH_DEFAULT_INITIAL_HEALTH;
use crate::doomdef::Gameaction;
use crate::doomstat::{Player, PlayerState, GAMEEPISODE, GAMEMAP, GAMESKILL, PLAYERS};
use crate::game::d_mode::Skill;
use crate::player::p_tick;

/// Advance game one tic. Calls P_Ticker, ST_Ticker, AM_Ticker, HU_Ticker.
/// Original: G_Ticker
pub fn g_ticker() {
    p_tick::p_ticker();
    crate::ui_hud::st_ticker();
    crate::ui_hud::am_ticker();
    crate::ui_hud::hu_ticker();
}

/// Reset player for respawn. Original: G_PlayerReborn
pub fn g_player_reborn(player: usize) {
    if player >= crate::doomdef::MAXPLAYERS {
        return;
    }
    unsafe {
        let p = &mut PLAYERS[player];
        let mut pl = Player::default();
        pl.mo = std::ptr::null_mut();
        pl.playerstate = PlayerState::Live;
        pl.viewheight = crate::player::VIEWHEIGHT;
        pl.health = DEH_DEFAULT_INITIAL_HEALTH;
        *p = pl;
    }
}

/// Handle input event. Dispatches to am_responder, st_responder, etc.
/// Original: G_Responder
pub fn g_responder(ev: &Event) -> bool {
    crate::ui_hud::am_responder(ev) || crate::ui_hud::st_responder(ev) as bool
}

/// Build ticcmd from current input state. Stub - zeroes cmd.
/// Original: G_BuildTiccmd
pub fn g_build_ticcmd(cmd: &mut Ticcmd, _maketic: i32) {
    *cmd = Ticcmd::default();
}

/// Defer new game start. Used by idclev cheat and menu.
/// Original: G_DeferedInitNew
pub fn g_defered_init_new(skill: Skill, episode: i32, map: i32) {
    unsafe {
        GAMESKILL = skill;
        GAMEEPISODE = episode;
        GAMEMAP = map;
        GAMEACTION = Gameaction::NewGame;
    }
}

/// Defer load game. Used by menu Load slot selection.
/// Original: G_LoadGame (deferred via gameaction)
pub fn g_defered_load_game(slot: i32) {
    unsafe {
        SAVEGAMESLOT = slot;
        GAMEACTION = Gameaction::LoadGame;
    }
}

/// Defer save game. Used by menu Save slot selection.
/// Original: G_SaveGame (deferred via gameaction)
pub fn g_defered_save_game(slot: i32) {
    unsafe {
        SAVEGAMESLOT = slot;
        GAMEACTION = Gameaction::SaveGame;
    }
}
