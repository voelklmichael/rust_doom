//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Item/mobj interactions - pickups, damage.
//
// Original: p_inter.h / p_inter.c (stub)

use super::p_mobj::Mobj;

/// Touch special thing (item pickup). Original: P_TouchSpecialThing
pub fn p_touch_special_thing(_special: *mut Mobj, _toucher: *mut Mobj) {
    // TODO: require am_map, s_sound, doomstat (player_t), deh_main
}

/// Damage a mobj. Original: P_DamageMobj
pub fn p_damage_mobj(
    _target: *mut Mobj,
    _inflictor: *mut Mobj,
    _source: *mut Mobj,
    _damage: i32,
) {
    // TODO: require doomstat, s_sound, g_game
}

// TODO: P_GivePower, P_GiveAmmo, P_GiveWeapon, P_GiveBody, P_GiveArmor, etc.
// TODO: maxammo, clipammo - in p_local.h
