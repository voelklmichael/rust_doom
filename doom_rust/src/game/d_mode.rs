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

// What version are we emulating?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameVersion {
    ExeDoom12,       // Doom 1.2
    ExeDoom1666,     // Doom 1.666
    ExeDoom17,       // Doom 1.7/1.7a
    ExeDoom18,       // Doom 1.8
    ExeDoom19,       // Doom 1.9
    ExeHacx,         // Hacx
    ExeUltimate,     // Ultimate Doom (retail)
    ExeFinal,        // Final Doom
    ExeFinal2,       // Final Doom (alternate exe)
    ExeChex,         // Chex Quest
    ExeHeretic13,    // Heretic 1.3
    ExeHexen11,      // Hexen 1.1
    ExeStrife12,     // Strife v1.2
    ExeStrife131,    // Strife v1.31
}

// Skill level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Skill {
    NoItems = -1,    // the "-skill 0" hack
    Baby = 0,
    Easy,
    Medium,
    Hard,
    Nightmare,
}
