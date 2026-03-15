//! Rust translation of doomgeneric/d_items.h
//! Items: key cards, artifacts, weapon, ammunition.

use crate::doomdef::*;

/// C typedef: weaponinfo_t
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WeaponinfoT {
    pub ammo: AmmotypeT,
    pub upstate: i32,
    pub downstate: i32,
    pub readystate: i32,
    pub atkstate: i32,
    pub flashstate: i32,
}

/// C: NUMWEAPONS (enum value, 9 weapons before wp_nochange)
pub const NUMWEAPONS: usize = 9;

/// C extern: weaponinfo[NUMWEAPONS]
pub static mut weaponinfo: [WeaponinfoT; NUMWEAPONS] = [WeaponinfoT {
    ammo: AmmotypeT::Noammo,
    upstate: 0,
    downstate: 0,
    readystate: 0,
    atkstate: 0,
    flashstate: 0,
}; NUMWEAPONS];
