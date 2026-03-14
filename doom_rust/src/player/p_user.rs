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

/// Apply thrust to player. Original: P_Thrust
pub fn p_thrust(_player: *mut std::ffi::c_void, _angle: u32, _move: crate::m_fixed::Fixed) {
    let _ = (_player, _angle, _move);
}

/// Recalculate view height. Original: P_CalcHeight
pub fn p_calc_height(_player: *mut std::ffi::c_void) {
    let _ = _player;
}

/// Move player (walk, run). Original: P_MovePlayer
pub fn p_move_player(_player: *mut std::ffi::c_void) {
    let _ = _player;
}

// TODO: P_DeathThink, P_RespawnPlayer - require doomstat
