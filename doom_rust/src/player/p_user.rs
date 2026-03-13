//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Player movement, POV bobbing, weapon state.
//
// Original: p_user.c (stub)

/// INVERSECOLORMAP - index for invulnerability effect.
pub const INVERSECOLORMAP: i32 = 32;

/// MAXBOB - 16 pixels of view bobbing.
pub const MAXBOB: i32 = 0x100000;

/// Main player think - movement, weapon, etc. Original: P_PlayerThink
pub fn p_player_think(_player: *mut std::ffi::c_void) {
    // TODO: require player_t, d_event, doomstat
}

// TODO: P_Thrust, P_CalcHeight, P_MovePlayer, P_DeathThink, etc.
