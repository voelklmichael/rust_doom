//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Item/mobj interactions - pickups, damage.
//
// Original: p_inter.h / p_inter.c (stub)

use crate::doomstat::PLAYERS;

use super::p_mobj::Mobj;

/// Damage flash duration (tics). Original: DAMAGE_TICCOUNT
const DAMAGE_TICCOUNT: i32 = 2 * 35; // 2 seconds

/// Bonus flash duration (tics). Original: BONUS_TICCOUNT
const BONUS_TICCOUNT: i32 = 1 * 35; // 1 second

/// Touch special thing (item pickup). Original: P_TouchSpecialThing
pub fn p_touch_special_thing(special: *mut Mobj, toucher: *mut Mobj) {
    if special.is_null() || toucher.is_null() {
        return;
    }
    unsafe {
        if (*toucher).player.is_null() {
            return;
        }
        let player_idx = player_index_from_mobj(toucher);
        if let Some(idx) = player_idx {
            PLAYERS[idx].bonuscount = BONUS_TICCOUNT;
        }
    }
    // TODO: full pickup logic (ammo, health, weapon, etc.)
}

/// Damage a mobj. Original: P_DamageMobj
pub fn p_damage_mobj(
    target: *mut Mobj,
    _inflictor: *mut Mobj,
    _source: *mut Mobj,
    _damage: i32,
) {
    if target.is_null() {
        return;
    }
    unsafe {
        if (*target).player.is_null() {
            return;
        }
        let player_idx = player_index_from_mobj(target);
        if let Some(idx) = player_idx {
            PLAYERS[idx].damagecount = DAMAGE_TICCOUNT;
        }
    }
    // TODO: full damage logic (health, pain state, death)
}

/// Get player index (0..MAXPLAYERS-1) from player mobj, or None.
fn player_index_from_mobj(mo: *mut Mobj) -> Option<usize> {
    if mo.is_null() {
        return None;
    }
    unsafe {
        let player_ptr = (*mo).player;
        if player_ptr.is_null() {
            return None;
        }
        for i in 0..crate::doomdef::MAXPLAYERS {
            if std::ptr::eq(PLAYERS[i].mo, mo as *mut std::ffi::c_void) {
                return Some(i);
            }
        }
    }
    None
}

// TODO: P_GivePower, P_GiveAmmo, P_GiveWeapon, P_GiveBody, P_GiveArmor, etc.
// TODO: maxammo, clipammo - in p_local.h
