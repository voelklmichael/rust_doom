//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Internally used data structures for virtually everything.
//
// Original: doomdef.h (partial - types needed by doomstat)

// The maximum number of players, multiplayer/networking.
pub const MAXPLAYERS: usize = 4;

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
