//! Rust translation of doomgeneric/doomdef.h
//! Internally used data structures for virtually everything.

use crate::d_mode::*;
use crate::doomtype::*;

/// C #define: DOOM_VERSION
pub const DOOM_VERSION: i32 = 109;

/// C #define: DOOM_191_VERSION
pub const DOOM_191_VERSION: i32 = 111;

/// C #define: RANGECHECK
pub const RANGECHECK: bool = true;

/// C #define: MAXPLAYERS
pub const MAXPLAYERS: i32 = 4;

/// C typedef: gamestate_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: gamestate_t
pub enum GamestateT {
    Level,
    Intermission,
    Finale,
    Demoscreen,
}

/// C typedef: gameaction_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: gameaction_t
pub enum GameactionT {
    Nothing,
    Loadlevel,
    Newgame,
    Loadgame,
    Savegame,
    Playdemo,
    Completed,
    Victory,
    Worlddone,
    Screenshot,
}

/// C #define: MTF_EASY
pub const MTF_EASY: i32 = 1;
/// C #define: MTF_NORMAL
pub const MTF_NORMAL: i32 = 2;
/// C #define: MTF_HARD
pub const MTF_HARD: i32 = 4;
/// C #define: MTF_AMBUSH
pub const MTF_AMBUSH: i32 = 8;

/// C typedef: card_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: card_t
pub enum CardT {
    Bluecard,
    Yellowcard,
    Redcard,
    Blueskull,
    Yellowskull,
    Redskull,
    Numcards,
}

/// C typedef: weapontype_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: weapontype_t
pub enum WeapontypeT {
    Fist,
    Pistol,
    Shotgun,
    Chaingun,
    Missile,
    Plasma,
    Bfg,
    Chainsaw,
    Supershotgun,
    Numweapons,
    Nochange,  // No pending weapon change
}

/// C typedef: ammotype_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: ammotype_t
pub enum AmmotypeT {
    Clip,   // Pistol / chaingun ammo
    Shell,  // Shotgun / double barreled shotgun
    Cell,   // Plasma rifle, BFG
    Misl,   // Missile launcher
    Numammo,
    Noammo, // Unlimited for chainsaw / fist
}

/// C: NUMCARDS
pub const NUMCARDS: usize = 6;
/// C: NUMPOWERS
pub const NUMPOWERS: usize = 6;
/// C: NUMAMMO
pub const NUMAMMO: usize = 4;

/// C typedef: powertype_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: powertype_t
pub enum PowertypeT {
    Invulnerability,
    Strength,
    Invisibility,
    Ironfeet,
    Allmap,
    Infrared,
    Numpowers,
}

/// C typedef: powerduration_t - constants (C enum allows duplicate values)
#[allow(non_camel_case_types)]
pub mod powerduration_t {
/// C #define: INVULNTICS
    pub const INVULNTICS: i32 = 30 * 35;   // 30*TICRATE
/// C #define: INVISTICS
    pub const INVISTICS: i32 = 60 * 35;    // 60*TICRATE
/// C #define: INFRATICS
    pub const INFRATICS: i32 = 120 * 35;   // 120*TICRATE
/// C #define: IRONTICS
    pub const IRONTICS: i32 = 60 * 35;     // 60*TICRATE
}
