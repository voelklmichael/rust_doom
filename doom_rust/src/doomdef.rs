//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Internally used data structures for virtually everything.
//
// Original: doomdef.h

// The maximum number of players, multiplayer/networking.
pub const MAXPLAYERS: usize = 4;

/// Block size for screen blocks (used in status bar, automap).
pub const BLOCK_SIZE: i32 = 32;

/// Screen dimensions (from i_video.h).
pub const SCREENWIDTH: i32 = 320;
pub const SCREENHEIGHT: i32 = 200;
pub const SCREENWIDTH_USIZE: usize = SCREENWIDTH as usize;

// The current state of the game: whether we are playing, gazing at the
// intermission screen, the game final animation, or a demo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Gamestate {
    Level,
    Intermission,
    Finale,
    DemoScreen,
}

/// Game action - what the game should do next (from d_main).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Gameaction {
    #[default]
    Nothing,
    LoadLevel,
    NewGame,
    LoadGame,
    SaveGame,
    PlayDemo,
    Completed,
    Victory,
    WorldDone,
    Screenshot,
}

/// Key cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Card {
    BlueCard,
    YellowCard,
    RedCard,
    BlueSkull,
    YellowSkull,
    RedSkull,
}

pub const NUMCARDS: usize = 6;

/// Weapon types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Weapontype {
    Fist,
    Pistol,
    Shotgun,
    Chaingun,
    Missile,
    Plasma,
    Bfg,
    Chainsaw,
    SuperShotgun,
}

pub const NUMWEAPONS: usize = 9;

/// No pending weapon change.
pub const WP_NOCHANGE: i32 = 9;

/// Ammunition types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Ammotype {
    Clip,
    Shell,
    Cell,
    Misl,
    /// Unlimited for chainsaw / fist.
    Noammo,
}

pub const NUMAMMO: usize = 4;

/// Power up artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Powertype {
    Invulnerability,
    Strength,
    Invisibility,
    Ironfeet,
    Allmap,
    Infrared,
}

pub const NUMPOWERS: usize = 6;

/// TICRATE from i_timer - used for power durations.
pub const TICRATE: i32 = 35;

/// Power up durations (ticks).
pub const INVULNTICS: i32 = 30 * TICRATE;
pub const INVISTICS: i32 = 60 * TICRATE;
pub const INFRATICS: i32 = 120 * TICRATE;
pub const IRONTICS: i32 = 60 * TICRATE;

/// Skill flags.
pub const MTF_EASY: i32 = 1;
pub const MTF_NORMAL: i32 = 2;
pub const MTF_HARD: i32 = 4;
pub const MTF_AMBUSH: i32 = 8;
