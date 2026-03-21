// doomdef.h / doomdef.c

pub use crate::d_mode::*;
pub use crate::doomtype::*;
pub use crate::i_timer::*;

// Original: #define DOOM_VERSION 109
pub const DOOM_VERSION: i32 = 109;

// Original: #define DOOM_191_VERSION 111
pub const DOOM_191_VERSION: i32 = 111;

// Original: #define RANGECHECK (compile flag; present in original)
pub const RANGECHECK: bool = true;

// Original: #define MAXPLAYERS 4
pub const MAXPLAYERS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GamestateT {
    GsLevel = 0,
    GsIntermission = 1,
    GsFinale = 2,
    GsDemoscreen = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameactionT {
    GaNothing = 0,
    GaLoadlevel = 1,
    GaNewgame = 2,
    GaLoadgame = 3,
    GaSavegame = 4,
    GaPlaydemo = 5,
    GaCompleted = 6,
    GaVictory = 7,
    GaWorlddone = 8,
    GaScreenshot = 9,
}

// Original: #define MTF_EASY 1
pub const MTF_EASY: i32 = 1;
// Original: #define MTF_NORMAL 2
pub const MTF_NORMAL: i32 = 2;
// Original: #define MTF_HARD 4
pub const MTF_HARD: i32 = 4;
// Original: #define MTF_AMBUSH 8
pub const MTF_AMBUSH: i32 = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CardT {
    ItBluecard = 0,
    ItYellowcard = 1,
    ItRedcard = 2,
    ItBlueskull = 3,
    ItYellowskull = 4,
    ItRedskull = 5,
}

// Original: NUMCARDS
pub const NUMCARDS: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WeapontypeT {
    WpFist = 0,
    WpPistol = 1,
    WpShotgun = 2,
    WpChaingun = 3,
    WpMissile = 4,
    WpPlasma = 5,
    WpBfg = 6,
    WpChainsaw = 7,
    WpSupershotgun = 8,
    Numweapons = 9,
    WpNochange = 10,
}

// Original: NUMWEAPONS (count of real weapons, before wp_nochange)
pub const NUMWEAPONS: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AmmotypeT {
    AmClip = 0,
    AmShell = 1,
    AmCell = 2,
    AmMisl = 3,
    Numammo = 4,
    AmNoammo = 5,
}

pub const NUMAMMO: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PowertypeT {
    PwInvulnerability = 0,
    PwStrength = 1,
    PwInvisibility = 2,
    PwIronfeet = 3,
    PwAllmap = 4,
    PwInfrared = 5,
}

pub const NUMPOWERS: usize = 6;

// Original: typedef enum { ... } powerduration_t
pub mod powerduration {
    use super::TICRATE;

    // Original: INVULNTICS = (30*TICRATE)
    pub const INVULNTICS: i32 = 30 * TICRATE;
    // Original: INVISTICS = (60*TICRATE)
    pub const INVISTICS: i32 = 60 * TICRATE;
    // Original: INFRATICS = (120*TICRATE)
    pub const INFRATICS: i32 = 120 * TICRATE;
    // Original: IRONTICS = (60*TICRATE)
    pub const IRONTICS: i32 = 60 * TICRATE;
}

#[allow(non_camel_case_types)]
pub struct DoomdefState;

impl DoomdefState {
    pub fn new() -> Self {
        Self
    }
}
