//! Game definitions (doomdef.h, doomdef.c)
//! Original: doomdef.h, doomdef.c

use crate::doomtype::Boolean;
use crate::d_mode::{GameMissionT, GameModeT, GameVersionT, SkillT};
// #define DOOM_VERSION 109
pub const DOOM_VERSION: i32 = 109;
// #define DOOM_191_VERSION 111
pub const DOOM_191_VERSION: i32 = 111;
// #define MAXPLAYERS 4
pub const MAXPLAYERS: i32 = 4;

// #define MTF_EASY 1, etc.
pub const MTF_EASY: i32 = 1;
pub const MTF_NORMAL: i32 = 2;
pub const MTF_HARD: i32 = 4;
pub const MTF_AMBUSH: i32 = 8;

// typedef enum cheat_t (d_player.h)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheatT {
    CfNoclip = 1,
    CfGodmode = 2,
    CfNomomentum = 4,
}

// typedef enum gamestate_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamestateT {
    GsLevel,
    GsIntermission,
    GsFinale,
    GsDemoscreen,
}

// typedef enum gameaction_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameactionT {
    GaNothing,
    GaLoadlevel,
    GaNewgame,
    GaLoadgame,
    GaSavegame,
    GaPlaydemo,
    GaCompleted,
    GaVictory,
    GaWorlddone,
    GaScreenshot,
}

// typedef enum card_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardT {
    ItBluecard,
    ItYellowcard,
    ItRedcard,
    ItBlueskull,
    ItYellowskull,
    ItRedskull,
    Numcards,
}

// typedef enum weapontype_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeapontypeT {
    WpFist,
    WpPistol,
    WpShotgun,
    WpChaingun,
    WpMissile,
    WpPlasma,
    WpBfg,
    WpChainsaw,
    WpSupershotgun,
    Numweapons,
    WpNochange,
}

// typedef enum ammotype_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmotypeT {
    AmClip,
    AmShell,
    AmCell,
    AmMisl,
    Numammo,
    AmNoammo,
}

// typedef enum powertype_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowertypeT {
    PwInvulnerability,
    PwStrength,
    PwInvisibility,
    PwIronfeet,
    PwAllmap,
    PwInfrared,
    Numpowers,
}

// typedef enum powerduration_t (TICRATE=35)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerdurationT {
    Invulntics = 1050,  // 30*TICRATE
    Invistics = 2100,   // 60*TICRATE
    Infratics = 4200,   // 120*TICRATE
    Irontics = 2101,    // 60*TICRATE (same duration as Invistics; Rust requires unique discriminant)
}
