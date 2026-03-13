//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game logic.
//
// Original: g_game.h + g_game.c (stub)

use super::d_event::Event;
use super::d_ticcmd::Ticcmd;
use crate::deh::misc::DEH_DEFAULT_INITIAL_HEALTH;
use crate::doomstat::{Player, PlayerState, PLAYERS};
use crate::player::p_tick;

/// Advance game one tic. Calls P_Ticker.
/// Original: G_Ticker
pub fn g_ticker() {
    p_tick::p_ticker();
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

/// Handle input event. Stub - returns false (event not consumed).
/// Original: G_Responder
pub fn g_responder(_ev: &Event) -> bool {
    false
}

/// Build ticcmd from current input state. Stub - zeroes cmd.
/// Original: G_BuildTiccmd
pub fn g_build_ticcmd(cmd: &mut Ticcmd, _maketic: i32) {
    *cmd = Ticcmd::default();
}
