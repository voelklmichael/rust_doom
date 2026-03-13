//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Items: key cards, artifacts, weapon, ammunition.
//
// Original: d_items.h + d_items.c

use crate::doomdef::{Ammotype, NUMWEAPONS};

/// Weapon info: sprite frames, ammunition use.
/// Original: weaponinfo_t
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Weaponinfo {
    pub ammo: Ammotype,
    pub upstate: i32,
    pub downstate: i32,
    pub readystate: i32,
    pub atkstate: i32,
    pub flashstate: i32,
}

// State indices from info.h (statenum_t) - use raw i32 until info is ported.
// S_NULL=0, S_PUNCH*, S_PISTOL*, S_SGUN*, S_CHAIN*, S_MISSILE*, S_PLASMA*, S_BFG*, S_SAW*, S_DSGUN*
const S_NULL: i32 = 0;
const S_PUNCHUP: i32 = 2;
const S_PUNCHDOWN: i32 = 1;
const S_PUNCH: i32 = 4;
const S_PUNCH1: i32 = 5;
const S_PISTOLUP: i32 = 12;
const S_PISTOLDOWN: i32 = 11;
const S_PISTOL: i32 = 14;
const S_PISTOL1: i32 = 15;
const S_PISTOLFLASH: i32 = 17;
const S_SGUNUP: i32 = 29;
const S_SGUNDOWN: i32 = 28;
const S_SGUN: i32 = 30;
const S_SGUN1: i32 = 31;
const S_SGUNFLASH1: i32 = 33;
const S_CHAINUP: i32 = 99;
const S_CHAINDOWN: i32 = 98;
const S_CHAIN: i32 = 100;
const S_CHAIN1: i32 = 101;
const S_CHAINFLASH1: i32 = 103;
const S_MISSILEUP: i32 = 109;
const S_MISSILEDOWN: i32 = 108;
const S_MISSILE: i32 = 110;
const S_MISSILE1: i32 = 111;
const S_MISSILEFLASH1: i32 = 113;
const S_PLASMAUP: i32 = 119;
const S_PLASMADOWN: i32 = 118;
const S_PLASMA: i32 = 120;
const S_PLASMA1: i32 = 121;
const S_PLASMAFLASH1: i32 = 123;
const S_BFGUP: i32 = 129;
const S_BFGDOWN: i32 = 128;
const S_BFG: i32 = 130;
const S_BFG1: i32 = 131;
const S_BFGFLASH1: i32 = 133;
const S_SAWUP: i32 = 139;
const S_SAWDOWN: i32 = 138;
const S_SAW: i32 = 140;
const S_SAW1: i32 = 141;
const S_DSGUNUP: i32 = 147;
const S_DSGUNDOWN: i32 = 146;
const S_DSGUN: i32 = 148;
const S_DSGUN1: i32 = 149;
const S_DSGUNFLASH1: i32 = 151;

/// Weapon info table. Original: weaponinfo[NUMWEAPONS]
pub static WEAPONINFO: [Weaponinfo; NUMWEAPONS] = [
    Weaponinfo {
        ammo: Ammotype::Noammo,
        upstate: S_PUNCHUP,
        downstate: S_PUNCHDOWN,
        readystate: S_PUNCH,
        atkstate: S_PUNCH1,
        flashstate: S_NULL,
    },
    Weaponinfo {
        ammo: Ammotype::Clip,
        upstate: S_PISTOLUP,
        downstate: S_PISTOLDOWN,
        readystate: S_PISTOL,
        atkstate: S_PISTOL1,
        flashstate: S_PISTOLFLASH,
    },
    Weaponinfo {
        ammo: Ammotype::Shell,
        upstate: S_SGUNUP,
        downstate: S_SGUNDOWN,
        readystate: S_SGUN,
        atkstate: S_SGUN1,
        flashstate: S_SGUNFLASH1,
    },
    Weaponinfo {
        ammo: Ammotype::Clip,
        upstate: S_CHAINUP,
        downstate: S_CHAINDOWN,
        readystate: S_CHAIN,
        atkstate: S_CHAIN1,
        flashstate: S_CHAINFLASH1,
    },
    Weaponinfo {
        ammo: Ammotype::Misl,
        upstate: S_MISSILEUP,
        downstate: S_MISSILEDOWN,
        readystate: S_MISSILE,
        atkstate: S_MISSILE1,
        flashstate: S_MISSILEFLASH1,
    },
    Weaponinfo {
        ammo: Ammotype::Cell,
        upstate: S_PLASMAUP,
        downstate: S_PLASMADOWN,
        readystate: S_PLASMA,
        atkstate: S_PLASMA1,
        flashstate: S_PLASMAFLASH1,
    },
    Weaponinfo {
        ammo: Ammotype::Cell,
        upstate: S_BFGUP,
        downstate: S_BFGDOWN,
        readystate: S_BFG,
        atkstate: S_BFG1,
        flashstate: S_BFGFLASH1,
    },
    Weaponinfo {
        ammo: Ammotype::Noammo,
        upstate: S_SAWUP,
        downstate: S_SAWDOWN,
        readystate: S_SAW,
        atkstate: S_SAW1,
        flashstate: S_NULL,
    },
    Weaponinfo {
        ammo: Ammotype::Shell,
        upstate: S_DSGUNUP,
        downstate: S_DSGUNDOWN,
        readystate: S_DSGUN,
        atkstate: S_DSGUN1,
        flashstate: S_DSGUNFLASH1,
    },
];
