//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Enemy AI, monster thinking, action pointers.
//
// Original: p_enemy.c (stub)

use super::p_mobj::Mobj;

/// Alert nearby monsters to target's presence. Original: P_NoiseAlert
pub fn p_noise_alert(_target: *mut Mobj, _emmiter: *mut Mobj) {
    // TODO: require g_game, s_sound
}

// TODO: dirtype_t, A_Fall, A_Look, A_Chase, A_FaceTarget, etc.
// TODO: Monster action functions (A_*)
