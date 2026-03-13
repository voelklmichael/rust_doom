//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//   Functions and definitions relating to the game type and operational
//   mode.
//
// Original: d_mode.h

use crate::doomtype::Boolean;

// The "mission" controls what game we are playing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMission {
    Doom = 0,      // Doom 1
    Doom2,         // Doom 2
    PackTnt,       // Final Doom: TNT: Evilution
    PackPlut,      // Final Doom: The Plutonia Experiment
    PackChex,      // Chex Quest (modded doom)
    PackHacx,      // Hacx (modded doom2)
    Heretic,       // Heretic
    Hexen,         // Hexen
    Strife,        // Strife
    None,
}

// The "mode" allows more accurate specification of the game mode we are
// in: eg. shareware vs. registered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMode {
    Shareware,       // Doom/Heretic shareware
    Registered,      // Doom/Heretic registered
    Commercial,      // Doom II/Hexen
    Retail,          // Ultimate Doom
    Indetermined,    // Unknown.
}
