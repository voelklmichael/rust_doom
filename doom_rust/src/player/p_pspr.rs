//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Player weapon sprites (psprites).
//
// Original: p_pspr.h / p_pspr.c (stub)

use crate::m_fixed::Fixed;

/// Overlay psprites - weapon and muzzle flash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Psprnum {
    Weapon = 0,
    Flash = 1,
    NumPsprites = 2,
}

/// Player sprite overlay - weapon/flash position on screen.
#[repr(C)]
#[derive(Debug)]
pub struct Pspdef {
    /// NULL state means not active.
    pub state: *mut std::ffi::c_void,
    pub tics: i32,
    pub sx: Fixed,
    pub sy: Fixed,
}

/// Setup psprites for player. Original: P_SetupPsprites
pub fn p_setup_psprites(_curplayer: *mut std::ffi::c_void) {
    // TODO: require player_t, deh_misc, s_sound
}

/// Move psprites (weapon bob, etc.). Original: P_MovePsprites
pub fn p_move_psprites(_curplayer: *mut std::ffi::c_void) {
    // TODO: require player_t, doomstat
}

/// Drop current weapon. Original: P_DropWeapon
pub fn p_drop_weapon(_player: *mut std::ffi::c_void) {
    // TODO: require player_t, s_sound
}
