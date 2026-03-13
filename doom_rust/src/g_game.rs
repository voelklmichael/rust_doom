//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game logic.
//
// Original: g_game.h + g_game.c (stub)

use crate::d_event::Event;
use crate::d_ticcmd::Ticcmd;
use crate::player::p_tick;

/// Advance game one tic. Calls P_Ticker.
/// Original: G_Ticker
pub fn g_ticker() {
    p_tick::p_ticker();
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
